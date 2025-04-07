use ic_cdk::storage;
use ic_cdk_macros::{init, update, query, pre_upgrade, post_upgrade};
use candid::{CandidType, Deserialize};
use serde::Serialize;
use burn::backend::ndarray::{NdArray, NdArrayDevice};
use burn::backend::Autodiff;
use burn::tensor::Tensor;
use getrandom::Error;
use ic_cdk::api::management_canister::main::raw_rand;
use std::cell::RefCell;

thread_local! {
    static RANDOM_BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}


#[no_mangle]
//TODO 目前随机数是固定的，没有请求IC自带的随机数生成随机数，会导致问题，需要后续修改，但IC的随机数raw_rand不能异步填充
//需要在 post_upgrade 或 add_price 中用 raw_rand 填充随机数
unsafe extern "Rust" fn __getrandom_v03_custom(dest: *mut u8, len: usize) -> Result<(), Error> {
    RANDOM_BUFFER.with(|buffer| {
        let mut buffer = buffer.borrow_mut();
        if buffer.len() < len {
            // 如果缓冲区不足，填充更多数据
            let needed = len - buffer.len();
            buffer.extend(vec![42; needed]); // 用固定值填充，避免无限循环
        }
        let slice = core::slice::from_raw_parts_mut(dest, len);
        slice.copy_from_slice(&buffer[..len]);
        buffer.drain(..len);
        Ok(())
    })
}


mod model;

const SEQUENCE_LENGTH: usize = 5;

#[derive(CandidType, Serialize, Deserialize, Clone, Default)]
struct State {
    weights: Option<Vec<f32>>, // [5]，而不是 [80]
    bias: Option<Vec<f32>>,    // [1]，而不是 [16]
    prices: Vec<f32>,
}

#[init]
fn init() {
    RANDOM_BUFFER.with(|buffer| {
        buffer.borrow_mut().extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    });

    let mut state = State::default();
    let model = model::PricePredictor::<Autodiff<NdArray>>::new(1, 1, SEQUENCE_LENGTH); // output_size = 1
    state.weights = Some(model.get_weights().val().into_data().to_vec().unwrap()); // 5 个元素
    state.bias = Some(model.get_bias().unwrap().val().into_data().to_vec().unwrap()); // 1 个元素
    state.prices = vec![100.0, 101.0, 102.0, 103.0, 104.0];
    storage::stable_save((state,)).unwrap();
}

#[update]
async fn refresh_random_buffer() {
    let randomness = raw_rand().await.unwrap().0;
    RANDOM_BUFFER.with(|buffer| {
        buffer.borrow_mut().clear();
        buffer.borrow_mut().extend_from_slice(&randomness);
    });
}

// #[pre_upgrade]
// fn pre_upgrade() {
//     let state: State = storage::stable_restore::<(State,)>().unwrap().0;
//     storage::stable_save((state,)).unwrap();
// }

// #[post_upgrade]
// fn post_upgrade() {
//     RANDOM_BUFFER.with(|buffer| {
//         buffer.borrow_mut().extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
//     });
// }

#[update]
fn add_price(price: f32) {
    let mut state: State = storage::stable_restore::<(State,)>().unwrap().0;
    state.prices.push(price);

    if state.prices.len() >= SEQUENCE_LENGTH + 1 {
        if let (Some(weights), Some(bias)) = (&state.weights, &state.bias) {
            let mut model = model::PricePredictor::<Autodiff<NdArray>>::from_weights(weights.clone(), bias.clone(), SEQUENCE_LENGTH);
            let (inputs, targets) = prepare_data(&state.prices);
            model.train(inputs, targets, 0.01, 5);
            state.weights = Some(model.get_weights().val().into_data().to_vec().unwrap());
            state.bias = Some(model.get_bias().unwrap().val().into_data().to_vec().unwrap());
        }
    }
    storage::stable_save((state,)).unwrap();
}


#[query]
fn predict() -> f32 {
    let state: State = storage::stable_restore::<(State,)>().unwrap().0;
    if state.prices.len() < SEQUENCE_LENGTH || state.weights.is_none() || state.bias.is_none() {
        return 0.0;
    }

    let weights = state.weights.unwrap();
    let bias = state.bias.unwrap();
    if weights.len() != 5 || bias.len() != 1 { // 调整为 5 和 1
        ic_cdk::trap("Invalid weights or bias length");
    }

    let model = model::PricePredictor::<Autodiff<NdArray>>::from_weights(weights, bias, SEQUENCE_LENGTH);
    let last_sequence = &state.prices[state.prices.len() - SEQUENCE_LENGTH..];
    let input = Tensor::<Autodiff<NdArray>, 1>::from_floats(last_sequence, &NdArrayDevice::Cpu)
        .reshape([1, SEQUENCE_LENGTH, 1]);
    let output = model.forward(input); // [1, 1]
    output.into_scalar() // 现在可以正确转换为标量
}

fn prepare_data(prices: &[f32]) -> (Tensor<Autodiff<NdArray>, 3>, Tensor<Autodiff<NdArray>, 2>) {
    let mut inputs = Vec::new();
    let mut targets = Vec::new();

    for i in 0..(prices.len() - SEQUENCE_LENGTH) {
        let input_slice = &prices[i..i + SEQUENCE_LENGTH];
        let target = prices[i + SEQUENCE_LENGTH];
        inputs.extend_from_slice(input_slice);
        targets.push(target);
    }

    let batch_size = prices.len() - SEQUENCE_LENGTH;
    let inputs = Tensor::<Autodiff<NdArray>, 1>::from_floats(&inputs[..], &NdArrayDevice::Cpu)
        .reshape([batch_size, SEQUENCE_LENGTH, 1])
        .to_device(&NdArrayDevice::Cpu);
    let targets = Tensor::<Autodiff<NdArray>, 1>::from_floats(&targets[..], &NdArrayDevice::Cpu)
        .reshape([batch_size, 1])
        .to_device(&NdArrayDevice::Cpu);

    (inputs, targets)
}

#[query]
fn get_state() -> State {
    storage::stable_restore::<(State,)>().unwrap().0
}