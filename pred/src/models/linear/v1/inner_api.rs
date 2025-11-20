use crate::models::interface::predict::Predict;
use crate::models::interface::train::Train;
use crate::models::linear::v1::domain::{
    AdamConfigWrap, LinearModel, LinearModelConfig, OptimizerConfigs, PriceDataset, PriceSample,
};
use burn::backend::ndarray::NdArrayDevice;
use burn::backend::NdArray;
use burn::nn;
use burn::nn::Initializer;
use burn::optim::AdamConfig;
use burn::prelude::{Tensor, TensorData};
use ic_cdk::update;
use crate::models::interface::validate::Validate;

#[update]
pub fn t() {
    let model = LinearModel::<NdArray>::default();
    // 生成假数据：y = 2*x + 1 + 噪声
    let x_data: Vec<f32> = (0..5).map(|i| i as f32 / 10.0).collect();
    let y_data: Vec<f32> = x_data
        .iter()
        .map(|&x| 2.0 * x + 1.0 + (rand::random::<f32>() - 0.5) * 0.5)
        .collect();
    let device = burn::backend::ndarray::NdArrayDevice::default();
    let model = LinearModel::new(
        LinearModelConfig {
            bias: true,
            input_size: 5,
            output_size: 1,
            initializer: Initializer::KaimingNormal {
                gain: 0.57735,
                fan_out_only: false,
            },
        },
        OptimizerConfigs::Adam(AdamConfigWrap(AdamConfig::new())),
        NdArrayDevice::Cpu,
    );
    let x = Tensor::<NdArray, 1>::from_floats(x_data.as_slice(), &device);
    let y = Tensor::<NdArray, 1>::from_floats(y_data.as_slice(), &device);

    for epoch in 1..=5 {
        let pred = model.forward(x.clone());
        let loss = model.mse_loss(&pred.clone(), &y.clone());
        ic_cdk::println!("Pred: {:?}", pred);
        ic_cdk::println!("Epoch {} | Loss: {:.6}", epoch, loss);
    }
}
