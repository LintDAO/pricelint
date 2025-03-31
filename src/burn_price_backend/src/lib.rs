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
unsafe extern "Rust" fn __getrandom_v03_custom(dest: *mut u8, len: usize) -> Result<(), getrandom::Error> {
    RANDOM_BUFFER.with(|buffer| {
        let mut buffer = buffer.borrow_mut();
        if buffer.len() < len {
            return Err(getrandom::Error::new_custom(1));
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
    weights: Option<Vec<f32>>,
    bias: Option<Vec<f32>>,
    prices: Vec<f32>,
}

#[init]
fn init() {
    RANDOM_BUFFER.with(|buffer| {
        buffer.borrow_mut().extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
    });

    let mut state = State::default();
    let model = model::PricePredictor::<Autodiff<NdArray>>::new(1, 16, 1, SEQUENCE_LENGTH);
    state.weights = Some(model.get_weights().val().into_data().to_vec().unwrap());
    state.bias = Some(model.get_bias().unwrap().val().into_data().to_vec().unwrap());
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

#[pre_upgrade]
fn pre_upgrade() {
    let state: State = storage::stable_restore::<(State,)>().unwrap().0;
    storage::stable_save((state,)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    RANDOM_BUFFER.with(|buffer| {
        buffer.borrow_mut().extend_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
    });
}

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

    let model = model::PricePredictor::<Autodiff<NdArray>>::from_weights(
        state.weights.unwrap(),
        state.bias.unwrap(),
        SEQUENCE_LENGTH,
    );
    let last_sequence = &state.prices[state.prices.len() - SEQUENCE_LENGTH..];
    let input = Tensor::<Autodiff<NdArray>, 1>::from_floats(&last_sequence[..], &NdArrayDevice::Cpu)
        .reshape([1, SEQUENCE_LENGTH, 1])
        .to_device(&NdArrayDevice::Cpu);

    let output = model.forward(input);
    output.into_scalar()
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