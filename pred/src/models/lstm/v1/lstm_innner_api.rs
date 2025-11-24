use crate::models::interface::predict::Predict;
use crate::models::interface::train::Train;
use crate::models::interface::validate::Validate;
use crate::models::linear::v1::linear_domain::LinearModel;
use crate::models::lstm::v1::lstm_domain::LstmModel;
use burn::backend::ndarray::NdArrayDevice;
use burn::backend::{Autodiff, NdArray};
use burn::prelude::{Tensor, TensorData};
use burn::LearningRate;
use ic_cdk::update;
use serde::Serialize;
use serde_json::{to_vec, Value};
use std::cell::RefCell;
use std::ops::Deref;
// 1704067200000,
// "13.29600000",
// "14.07800000",
// "12.65000000",
// "12.93600000",
// "8564412.71000000",
// 1704153599999,
// "114225757.51177000",
// 543061,
// "4093636.42000000",
// "54642171.04268000",
// "0

type AutodiffBackend = Autodiff<NdArray<f32>>;
thread_local! {
    static M:RefCell<LstmModel<AutodiffBackend>>=RefCell::new(LstmModel::<AutodiffBackend>::default());
}
#[update]
pub fn lstm_epochs(epochs: usize, lr: f32, data: Vec<u8>) -> Result<(), String> {
    let device = NdArrayDevice::default();
    let json_str = String::from_utf8(data).map_err(|e| e.to_string())?;
    let raw_data: Vec<Value> = serde_json::from_str(&json_str).map_err(|e| e.to_string())?;

    let raw_data_f32: Vec<Vec<f32>> = raw_data
        .iter()
        .map(|value| {
            value
                .as_array()
                .expect("Each row must be an array")
                .iter()
                .map(|x| match x {
                    Value::Number(n) => n.as_f64().unwrap_or(0.0) as f32,
                    Value::String(s) => s.parse::<f32>().unwrap_or(0.0),
                    _ => 0.0,
                })
                .collect()
        })
        .collect();

    // 提取目标数据（第9个特征）
    let price_data: Vec<f32> = raw_data_f32
        .iter()
        .map(|v| v[4]) // 使用第9个特征作为目标
        .collect();

    // 重新组织输入数据：以12个为一组
    let train_data = raw_data_f32
        .iter()
        .map(|x| {
            let open = x[1];
            let high = x[2];
            let low = x[3];
            let close = x[4];
            let volume = x[5];
            [open, high, low, close, volume]
        })
        .flatten()
        .collect::<Vec<_>>();

    // 创建输入张量 [num_sequences, sequence_length, 1]
    let input = Tensor::<AutodiffBackend, 3>::from_data(
        TensorData::new::<f32, Vec<usize>>(train_data, [347, 1, 5].into()),
        &device,
    );

    // 创建目标张量 [num_sequences, 1]
    let target = Tensor::<AutodiffBackend, 2>::from_data(
        TensorData::new::<f32, Vec<usize>>(price_data, [347, 1].into()),
        &device,
    );

    let mut model = M.with_borrow_mut(|rc| rc.clone());
    for epoch in 0..epochs {
        ic_cdk::println!("训练轮次 {}", epoch);
        model.train_step(input.clone(), target.clone(), lr as LearningRate);
        ic_cdk::println!("轮次 {} ", epoch);
    }
    M.with_borrow_mut(|rc| {
        *rc = model;
    });
    Ok(())
}
//   /home/prj/burn_price/temp/icp_history_price_2024.json
