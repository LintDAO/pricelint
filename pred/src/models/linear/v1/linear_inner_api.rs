use crate::models::interface::predict::Predict;
use crate::models::interface::train::Train;
use crate::models::linear::v1::linear_domain::{AdamConfigWrap, LinearModel, LinearModelConfig, OptimizerConfigs};
use burn::backend::ndarray::NdArrayDevice;
use burn::backend::{Autodiff, NdArray};
use burn::LearningRate;
use burn::optim::{AdamConfig, GradientsParams, Optimizer};
use burn::prelude::{Module, Tensor, TensorData};
use ic_cdk::update;
use serde_json::Value;

#[update]
pub fn linear_epochs(epochs:usize,lr:f32,data:Vec<u8>)->Result<(),String> {
    // 使用 Autodiff 后端 (用于计算梯度) 和 NdArray 设备
    type AutodiffBackend = Autodiff<NdArray<f32>>;
    let device = NdArrayDevice::default();

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

    let x = Tensor::<AutodiffBackend, 2>::from_data(
        TensorData::new::<f32, Vec<usize>>(train_data, [347, 5].into()),
        &device,
    );

    let y = Tensor::<AutodiffBackend, 2>::from_data(
        TensorData::new::<f32, Vec<usize>>(price_data, [347, 1].into()),
        &device,
    );



    // 2. 模型和优化器初始化
    let mut model = LinearModel::<AutodiffBackend>::default();

    for epoch in 0..epochs  {
        ic_cdk::println!("epoch {}", epoch);
        model.train_step(x.clone(),y.clone(),lr as LearningRate);
        ic_cdk::println!("predict {}", model.forward(x.clone()));
    }
    Ok(())

}
//       /home/prj/burn_price/temp/icp_history_price_2024.json