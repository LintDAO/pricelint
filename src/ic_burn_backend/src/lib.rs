use ic_cdk::storage;
use ic_cdk_macros::{init, update, query};
use ic_cdk::api::management_canister::main::raw_rand;
use candid::{CandidType};
use serde::{Serialize, Deserialize};
use burn::backend::ndarray::{NdArray, NdArrayDevice};
use burn::backend::Autodiff;
use burn::tensor::Tensor;
use serde_json;
use serde_json::Value;
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

const SEQUENCE_LENGTH: usize = 50;

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
struct PriceData {
    open: f32,
    high: f32,
    low: f32,
    close: f32,
    volume: f32,
    price_diff: f32,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default)]
struct State {
    weights: Option<Vec<f32>>,
    bias: Option<Vec<f32>>,
    prices: Vec<PriceData>,
    min_values: Vec<f32>,
    max_values: Vec<f32>,
}

#[init]
async fn init() {
    // 使用 raw_rand 填充 RANDOM_BUFFER
    let random_bytes = raw_rand()
        .await
        .map(|(bytes,)| bytes)
        .map_err(|_| getrandom::Error::new_custom(1)) 
        .expect("Failed to generate random bytes");
    
    RANDOM_BUFFER.with(|buffer| {
        let mut buffer = buffer.borrow_mut();
        buffer.extend_from_slice(&random_bytes);
        ic_cdk::println!("Initialized RANDOM_BUFFER with bytes: {:?}", &random_bytes); // 打印随机字节
    });

    let mut state = State::default();
    let model = model::PricePredictor::<Autodiff<NdArray>>::new(6, 16, 1, SEQUENCE_LENGTH);
    state.weights = Some(model.get_weights().val().into_data().to_vec().unwrap());
    state.bias = Some(model.get_bias().unwrap().val().into_data().to_vec().unwrap());
    state.prices = vec![PriceData { open: 100.0, high: 101.0, low: 99.0, close: 100.5, volume: 1000.0, price_diff: 0.5 }; 50];
    let (min_vals, max_vals) = compute_min_max(&state.prices);
    state.min_values = min_vals;
    state.max_values = max_vals;
    storage::stable_save((state,)).unwrap();
}

#[update]
async fn refill_random_buffer() {
    // 补充 RANDOM_BUFFER 的随机字节
    let random_bytes = raw_rand()
        .await
        .map(|(bytes,)| bytes)
        .map_err(|_| getrandom::Error::new_custom(1)) 
        .expect("Failed to generate random bytes");
    
    RANDOM_BUFFER.with(|buffer| {
        let mut buffer = buffer.borrow_mut();
        buffer.extend_from_slice(&random_bytes);
        ic_cdk::println!("Refilled RANDOM_BUFFER with bytes: {:?}", &random_bytes); // 打印随机字节
    });
}


#[update]
fn upload_json_file(file_content: Vec<u8>) {
    let mut state: State = storage::stable_restore::<(State,)>().unwrap().0;
    let json_str = String::from_utf8(file_content).expect("Invalid UTF-8");
    let raw_data: Value = serde_json::from_str(&json_str).expect("Invalid JSON format");
    let raw_array = raw_data.as_array().expect("JSON must be an array");

    state.prices = raw_array.iter().map(|row| {
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
    }).collect();

    let (min_vals, max_vals) = compute_min_max(&state.prices);
    state.min_values = min_vals;
    state.max_values = max_vals;
    storage::stable_save((state,)).unwrap();
}

#[update]
fn add_price(data: PriceData) {
    let mut state: State = storage::stable_restore::<(State,)>().unwrap().0;
    state.prices.push(data);
    storage::stable_save((state,)).unwrap();
}

#[update]
fn train(epochs: u64) {
    let mut state: State = storage::stable_restore::<(State,)>().unwrap().0;
    if state.prices.len() < SEQUENCE_LENGTH + 1 {
        ic_cdk::trap("Not enough data to train");
    }
    let mut model = model::PricePredictor::<Autodiff<NdArray>>::new(6, 16, 1, SEQUENCE_LENGTH);

    // 如果已有权重和偏置，加载它们（可选）
    if let (Some(weights), Some(bias)) = (&state.weights, &state.bias) {
        // 这里可以添加逻辑加载权重和偏置到模型中（当前仅保存 Linear 层的权重）
    }

    let (inputs, targets) = prepare_data(&state.prices, &state.min_values, &state.max_values);
    // 单次调用只执行 5 个 epoch，避免指令限制
    let epochs_per_call = 5.min(epochs as usize);
    model.train(inputs, targets, 0.001, epochs_per_call); // 提高学习率到 0.001

    state.weights = Some(model.get_weights().val().into_data().to_vec().unwrap());
    state.bias = Some(model.get_bias().unwrap().val().into_data().to_vec().unwrap());
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
    if weights.len() != 16 || bias.len() != 1 { // hidden_size = 16
        ic_cdk::trap("Invalid weights or bias length");
    }

    let model = model::PricePredictor::<Autodiff<NdArray>>::new(6, 16, 1, SEQUENCE_LENGTH);
    let last_sequence = &state.prices[state.prices.len() - SEQUENCE_LENGTH..];
    let input = normalize_sequence(last_sequence, &state.min_values, &state.max_values);
    let output = model.forward(input);
    denormalize(output.into_scalar(), state.min_values[3], state.max_values[3])
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

fn normalize_sequence(sequence: &[PriceData], min_values: &[f32], max_values: &[f32]) -> Tensor<Autodiff<NdArray>, 3> {
    let data: Vec<f32> = sequence.iter().flat_map(|p| {
        vec![
            (p.open - min_values[0]) / (max_values[0] - min_values[0]),
            (p.high - min_values[1]) / (max_values[1] - min_values[1]),
            (p.low - min_values[2]) / (max_values[2] - min_values[2]),
            (p.close - min_values[3]) / (max_values[3] - min_values[3]),
            (p.volume - min_values[4]) / (max_values[4] - min_values[4]),
            (p.price_diff - min_values[5]) / (max_values[5] - min_values[5]),
        ]
    }).collect();
    Tensor::<Autodiff<NdArray>, 1>::from_floats(&data[..], &NdArrayDevice::Cpu).reshape([1, SEQUENCE_LENGTH, 6])
}

fn denormalize(value: f32, min_val: f32, max_val: f32) -> f32 {
    value * (max_val - min_val) + min_val
}

fn prepare_data(prices: &[PriceData], min_values: &[f32], max_values: &[f32]) -> (Tensor<Autodiff<NdArray>, 3>, Tensor<Autodiff<NdArray>, 2>) {
    let mut inputs = Vec::new();
    let mut targets = Vec::new();
    if prices.len() <= SEQUENCE_LENGTH {
        ic_cdk::trap("Not enough data for training");
    }
    for i in 0..(prices.len() - SEQUENCE_LENGTH) {
        let input_slice = &prices[i..i + SEQUENCE_LENGTH];
        let target = prices[i + SEQUENCE_LENGTH].close;
        let scaled_input: Vec<f32> = input_slice.iter().flat_map(|p| {
            vec![
                (p.open - min_values[0]) / (max_values[0] - min_values[0]),
                (p.high - min_values[1]) / (max_values[1] - min_values[1]),
                (p.low - min_values[2]) / (max_values[2] - min_values[2]),
                (p.close - min_values[3]) / (max_values[3] - min_values[3]),
                (p.volume - min_values[4]) / (max_values[4] - min_values[4]),
                (p.price_diff - min_values[5]) / (max_values[5] - min_values[5]),
            ]
        }).collect();
        inputs.extend(scaled_input);
        targets.push((target - min_values[3]) / (max_values[3] - min_values[3]));
    }
    let batch_size = prices.len() - SEQUENCE_LENGTH;
    let inputs = Tensor::<Autodiff<NdArray>, 1>::from_floats(&inputs[..], &NdArrayDevice::Cpu).reshape([batch_size, SEQUENCE_LENGTH, 6]);
    let targets = Tensor::<Autodiff<NdArray>, 1>::from_floats(&targets[..], &NdArrayDevice::Cpu).reshape([batch_size, 1]);
    (inputs, targets)
}

#[query]
fn get_state() -> State {
    let state = storage::stable_restore::<(State,)>().unwrap().0;
    ic_cdk::println!("Prices length: {}", state.prices.len());
    state
}