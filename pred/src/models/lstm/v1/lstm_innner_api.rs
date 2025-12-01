use crate::models::interface::predict::Predict;
use crate::models::interface::train::Train;
use crate::models::interface::validate::Validate;
use crate::models::linear::v1::linear_domain::LinearModel;
use crate::models::lstm::v1::lstm_domain::LstmModel;
use burn::backend::ndarray::NdArrayDevice;
use burn::backend::{Autodiff, NdArray};
use burn::prelude::{Int, Tensor, TensorData};
use burn::LearningRate;
use ic_cdk::update;
use serde::Serialize;
use serde_json::{to_vec, Value};
use std::cell::RefCell;
use std::ops::Deref;
use burn::record::Record;
use burn::tensor::cast::ToElement;
use burn::tensor::ElementComparison;
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
        .map(|v| v[1]) // 使用第9个特征作为目标
        .collect();

    // 重新组织输入数据：以12个为一组
    let train_data = raw_data_f32
        .iter()
        .map(|x| {
            let open = x[0];
            let high = x[1];
            [open, high]
        })
        .flatten()
        .collect::<Vec<_>>();

    let num_sequences = price_data.len();
    let seq_length = 10;
    // 创建输入张量 [num_sequences, sequence_length, 1]
    let input = Tensor::<AutodiffBackend, 3>::from_data(
        TensorData::new::<f32, Vec<usize>>(
            train_data.clone().repeat(5),
            [num_sequences, 5, 2].into(),
        ),
        &device,
    );
    // 创建目标张量 [num_sequences, 1]
    let target = Tensor::<AutodiffBackend, 2>::from_data(
        TensorData::new::<f32, Vec<usize>>(price_data.clone(), [num_sequences, 1].into()),
        &device,
    );

    let mut model = M.with_borrow_mut(|rc| rc.clone());
    for epoch in 0..epochs {
        ic_cdk::println!("训练轮次 {}", epoch + 1);
        model.train_step(input.clone(), target.clone(), lr as LearningRate);
        let min=price_data.iter()
            .copied()
            .filter(|x| !x.is_nan()) // 过滤掉 NaN
            .min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();// 显式使用 partial_cmp

        let max=price_data.iter()
            .copied()
            .filter(|x| !x.is_nan()) // 过滤掉 NaN
            .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();// 显式使用 partial_cmp

        ic_cdk::println!("max:{:?} min:{:?}",max,min);
    }
    M.with_borrow_mut(|rc| {
        *rc = model;
    });
    Ok(())
}

#[update]
fn p(){
    let device = NdArrayDevice::default();
    let test = Tensor::<AutodiffBackend, 3>::from_data(
        TensorData::new::<f32, Vec<usize>>(
            [1609459200000f32, 28923.63f32].to_vec().repeat(1),
            [1,1,2].into(),
        ),
        &device,
    );
    let mut model = M.with_borrow_mut(|rc| rc.clone());
    let out=model.predict(test);
    //max:124658.54 min:15781.29
    let max_value = 124658.54f32;
    let min = 15781.29f32;
    let original_value = out * (max_value - min) + min;
    ic_cdk::println!("original_value :{}",original_value);
}
//   /home/prj/burn_price/temp/icp_history_price_2024.json
//   /home/prj/burn_price/temp/BTCUSDT.json