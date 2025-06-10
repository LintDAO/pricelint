use std::borrow::Cow;
use burn::backend::{Autodiff, NdArray};
use burn::backend::ndarray::NdArrayDevice;
use burn::prelude::Tensor;
use candid::{CandidType, Deserialize};
use ic_cdk::api::management_canister::main::raw_rand;
use ic_cdk_macros::{init, query, update};
use ic_stable_structures::Storable;
use ic_stable_structures::storable::Bound;
use serde::Serialize;
use serde_json::Value;
use crate::{RANDOM_BUFFER, STATE_MAP};
use crate::ml::model::{default_model, record};
#[no_mangle]
unsafe extern "Rust" fn __getrandom_v03_custom(
    dest: *mut u8,
    len: usize,
) -> Result<(), getrandom::Error> {
    RANDOM_BUFFER.with(|buffer| {
        let mut buffer = buffer.borrow_mut();
        if buffer.len() < len {
            ic_cdk::println!(
                "RANDOM_BUFFER insufficient, needed: {}, available: {}",
                len,
                buffer.len()
            );
            return Err(getrandom::Error::new_custom(1));
        }
        let slice = core::slice::from_raw_parts_mut(dest, len);
        slice.copy_from_slice(&buffer[..len]);
        ic_cdk::println!(
            "Consumed {} bytes from RANDOM_BUFFER: {:?}",
            len,
            &buffer[..len]
        );
        buffer.drain(..len);
        Ok(())
    })
}
pub const SEQUENCE_LENGTH: usize = 50;


#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct PriceData {
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    volume: f32,
    price_diff: f32,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default)]
pub struct State {
    weights: Option<Vec<f32>>,
    bias: Option<Vec<f32>>,
    prices: Vec<PriceData>,
    min_values: Vec<f32>,
    max_values: Vec<f32>,
}
pub fn init(){
    let mut state = State::default();
    state.prices = vec![
        PriceData {
            open: 100.0,
            high: 101.0,
            low: 99.0,
            close: 100.5,
            volume: 1000.0,
            price_diff: 0.5
        };
        50
    ];
    let (min_vals, max_vals) = compute_min_max(&state.prices);
    state.min_values = min_vals;
    state.max_values = max_vals;
    // storage::stable_save((state,)).unwrap();
    STATE_MAP.with(|map| {
        map.borrow_mut().insert(String::from("state"), state);
    });
}
impl Storable for State {
    fn to_bytes(&self) -> Cow<[u8]> {
        let bytes = bincode::serialize(self).expect("Failed to serialize State");
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        bincode::deserialize(&bytes).expect("Failed to deserialize State")
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 10_000_000, // 调整为 State 的最大预期大小（字节）
        is_fixed_size: false,
    };
}



#[update]
async fn refill_random_buffer(count: u32) {
    let initial_cycles = ic_cdk::api::canister_balance();
    for _ in 0..count {
        let random_bytes = raw_rand()
            .await
            .map(|(bytes,)| bytes)
            .map_err(|_| getrandom::Error::new_custom(1))
            .expect("Failed to generate random bytes");
        RANDOM_BUFFER.with(|buffer| {
            buffer.borrow_mut().extend_from_slice(&random_bytes);
        });
    }
    ic_cdk::println!("Refilled RANDOM_BUFFER with {} bytes", count * 32);
    ic_cdk::println!(
        "refill_random_buffer consumed {} cycles",
        initial_cycles - ic_cdk::api::canister_balance()
    );
}

#[update]
fn upload_json_file(file_content: Vec<u8>) {
    // let mut state: State = storage::stable_restore::<(State,)>().unwrap().0;
    let mut state: State =
        STATE_MAP.with(|map| map.borrow_mut().get(&String::from("state")).unwrap());
    let json_str = String::from_utf8(file_content).expect("Invalid UTF-8");
    let raw_data: Value = serde_json::from_str(&json_str).expect("Invalid JSON format");
    let raw_array = raw_data.as_array().expect("JSON must be an array");

    state.prices = raw_array
        .iter()
        .map(|row| {
            let row_array = row.as_array().expect("Each row must be an array");
            let to_f32 = |v: &Value| -> f32 {
                match v {
                    Value::Number(n) => n.as_f64().unwrap_or(0.0) as f32,
                    Value::String(s) => s.parse::<f32>().unwrap_or(0.0),
                    _ => 0.0,
                }
            };
            let open = to_f32(&row_array[1]);
            let high = to_f32(&row_array[2]);
            let low = to_f32(&row_array[3]);
            let close = to_f32(&row_array[4]);
            let volume = to_f32(&row_array[5]);
            PriceData {
                open,
                high,
                low,
                close,
                volume,
                price_diff: close - open,
            }
        })
        .collect();

    let (min_vals, max_vals) = compute_min_max(&state.prices);
    state.min_values = min_vals;
    state.max_values = max_vals;
    // storage::stable_save((state,)).unwrap();
    STATE_MAP.with(|map| {
        map.borrow_mut().insert(String::from("state"), state);
    });
}

#[update]
fn add_price(data: PriceData) {
    // let mut state: State = storage::stable_restore::<(State,)>().unwrap().0;
    let mut state: State =
        STATE_MAP.with(|map| map.borrow_mut().get(&String::from("state")).unwrap());
    state.prices.push(data);
    // storage::stable_save((state,)).unwrap();
    STATE_MAP.with(|map| {
        map.borrow_mut().insert(String::from("state"), state);
    });
}

#[update]
async fn train(epochs: u64) {
    let initial_cycles = ic_cdk::api::canister_balance();
    ic_cdk::println!("Cycles init{}", initial_cycles);
    // 检查并填充 RANDOM_BUFFER
    let needs_fill = RANDOM_BUFFER.with(|buffer| buffer.borrow().len() < 32);
    if needs_fill {
        let mut random_bytes = Vec::new();
        for _ in 0..10 {
            // 填充 320 字节
            let bytes = raw_rand()
                .await
                .map(|(bytes,)| bytes)
                .map_err(|_| getrandom::Error::new_custom(1))
                .expect("Failed to generate random bytes");
            random_bytes.extend_from_slice(&bytes);
        }
        RANDOM_BUFFER.with(|buffer| {
            buffer.borrow_mut().extend_from_slice(&random_bytes);
            ic_cdk::println!(
                "Train initialized RANDOM_BUFFER with 320 bytes: {:?}",
                &random_bytes
            );
        });
    }

    let mut state = STATE_MAP.with(|map| {
        let state = map.borrow_mut().get(&String::from("state"));
        let result = state.unwrap_or_else(|| {
            ic_cdk::println!("Failed to restore state");
            State::default() // 提供默认状态
        });
        result
    });

    if state.prices.len() < SEQUENCE_LENGTH + 1 {
        ic_cdk::trap("Not enough data to train");
    }
    let mut model =
        default_model::PricePredictor::<Autodiff<NdArray>>::new(6, 16, 1, SEQUENCE_LENGTH);

    // 如果已有权重和偏置，加载它们（可选）
    if let (Some(weights), Some(bias)) = (&state.weights, &state.bias) {
        // 这里可以添加逻辑加载权重和偏置到模型中（当前仅保存 Linear 层的权重）
    }
    let (inputs, targets) = prepare_data(&state.prices, &state.min_values, &state.max_values);
    let epochs_per_call = 5.min(epochs as usize);
    model.train(inputs, targets, 0.001, epochs_per_call);
    state.weights = Some(model.get_weights().val().into_data().to_vec().unwrap());
    state.bias = Some(
        model
            .get_bias()
            .unwrap()
            .val()
            .into_data()
            .to_vec()
            .unwrap(),
    );
    ic_cdk::println!(
        "Train consumed {} cycles",
        initial_cycles - ic_cdk::api::canister_balance()
    );
    // storage::stable_save((state,)).unwrap();
    STATE_MAP.with(|map| {
        map.borrow_mut().insert(String::from("state"), state);
    });

    //todo 先暂时在训练后自动保存,实际使用根据用户的需要手动保存和加载
    record::save_model(model);

    ic_cdk::println!("Cycles used end: {}", ic_cdk::api::canister_balance());
}

#[query]
fn predict() -> f32 {
    let mut state: State =
        STATE_MAP.with(|map| map.borrow_mut().get(&String::from("state")).unwrap());
    if state.prices.len() < SEQUENCE_LENGTH || state.weights.is_none() || state.bias.is_none() {
        return 0.0;
    }

    let weights = state.weights.unwrap();
    let bias = state.bias.unwrap();
    if weights.len() != 16 || bias.len() != 1 {
        // hidden_size = 16
        ic_cdk::trap("Invalid weights or bias length");
    }

    // let model = model::PricePredictor::<Autodiff<NdArray>>::new(6, 16, 1, SEQUENCE_LENGTH);
    //todo 先暂时在训练后自动保存,实际使用根据用户的需要手动保存和加载
    let model = record::load_model();
    let last_sequence = &state.prices[state.prices.len() - SEQUENCE_LENGTH..];
    let input = normalize_sequence(last_sequence, &state.min_values, &state.max_values);
    let output = model.forward(input);
    denormalize(
        output.into_scalar(),
        state.min_values[3],
        state.max_values[3],
    )
}

fn compute_min_max(prices: &[PriceData]) -> (Vec<f32>, Vec<f32>) {
    let mut min_values = vec![f32::MAX; 6];
    let mut max_values = vec![f32::MIN; 6];
    for p in prices {
        let row = vec![p.open, p.high, p.low, p.close, p.volume, p.price_diff];
        for (i, &val) in row.iter().enumerate() {
            min_values[i] = min_values[i].min(val);
            max_values[i] = max_values[i].max(val);
        }
    }
    (min_values, max_values)
}

fn normalize_sequence(
    sequence: &[PriceData],
    min_values: &[f32],
    max_values: &[f32],
) -> Tensor<Autodiff<NdArray>, 3> {
    let data: Vec<f32> = sequence
        .iter()
        .flat_map(|p| {
            vec![
                (p.open - min_values[0]) / (max_values[0] - min_values[0]),
                (p.high - min_values[1]) / (max_values[1] - min_values[1]),
                (p.low - min_values[2]) / (max_values[2] - min_values[2]),
                (p.close - min_values[3]) / (max_values[3] - min_values[3]),
                (p.volume - min_values[4]) / (max_values[4] - min_values[4]),
                (p.price_diff - min_values[5]) / (max_values[5] - min_values[5]),
            ]
        })
        .collect();
    Tensor::<Autodiff<NdArray>, 1>::from_floats(&data[..], &NdArrayDevice::Cpu).reshape([
        1,
        SEQUENCE_LENGTH,
        6,
    ])
}

fn denormalize(value: f32, min_val: f32, max_val: f32) -> f32 {
    value * (max_val - min_val) + min_val
}

fn prepare_data(
    prices: &[PriceData],
    min_values: &[f32],
    max_values: &[f32],
) -> (Tensor<Autodiff<NdArray>, 3>, Tensor<Autodiff<NdArray>, 2>) {
    let mut inputs = Vec::new();
    let mut targets = Vec::new();
    if prices.len() <= SEQUENCE_LENGTH {
        ic_cdk::trap("Not enough data for training");
    }
    for i in 0..(prices.len() - SEQUENCE_LENGTH) {
        let input_slice = &prices[i..i + SEQUENCE_LENGTH];
        let target = prices[i + SEQUENCE_LENGTH].close;
        let scaled_input: Vec<f32> = input_slice
            .iter()
            .flat_map(|p| {
                vec![
                    (p.open - min_values[0]) / (max_values[0] - min_values[0]),
                    (p.high - min_values[1]) / (max_values[1] - min_values[1]),
                    (p.low - min_values[2]) / (max_values[2] - min_values[2]),
                    (p.close - min_values[3]) / (max_values[3] - min_values[3]),
                    (p.volume - min_values[4]) / (max_values[4] - min_values[4]),
                    (p.price_diff - min_values[5]) / (max_values[5] - min_values[5]),
                ]
            })
            .collect();
        inputs.extend(scaled_input);
        targets.push((target - min_values[3]) / (max_values[3] - min_values[3]));
    }
    let batch_size = prices.len() - SEQUENCE_LENGTH;
    let inputs = Tensor::<Autodiff<NdArray>, 1>::from_floats(&inputs[..], &NdArrayDevice::Cpu)
        .reshape([batch_size, SEQUENCE_LENGTH, 6]);
    let targets = Tensor::<Autodiff<NdArray>, 1>::from_floats(&targets[..], &NdArrayDevice::Cpu)
        .reshape([batch_size, 1]);
    (inputs, targets)
}
#[query]
fn get_state() -> State {
    // let state = storage::stable_restore::<(State,)>().unwrap().0;
    let mut state: State =
        STATE_MAP.with(|map| map.borrow_mut().get(&String::from("state")).unwrap());
    ic_cdk::println!("Prices length: {}", state.prices.len());
    state
}