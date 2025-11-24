use crate::api::config::default_model_config::get_default_model;
use crate::common::constants::config::DEFAULT_MODEL_KEY;
use crate::common::errors::ConfigError;
use crate::common::guard::is_owner;
use crate::common::lifecycle::{Value, CONFIG};
use crate::models::lstm::v1::lstm::{load_model, LstmModel};
use burn::backend::ndarray::NdArrayDevice;
use burn::backend::{Autodiff, NdArray};
use burn::tensor::{DataError, Tensor, TensorData};
use ic_cdk::{query, update};

#[update(guard = "is_owner")]
fn train() -> Result<(), String> {
    ic_cdk::println!("Training started...");
    let model_name = get_default_model()?;

    match model_name.as_str() {
        "lstm_v1.0.0" => {
            // 示例参数（实际可通过接口传入或配置）
            let device = NdArrayDevice::Cpu;
            let data = vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
            ];
            let seq_len = 5;
            let test_ratio = 0.2;
            let num_epochs = 10;
            let batch_size = 2;
            let learning_rate = 0.01;
            let model_save_path = "model.bin";
            let lstm_model = LstmModel::<Autodiff<NdArray>>::default();
            lstm_model.train(
                device,
                data,
                seq_len,
                test_ratio,
                num_epochs,
                batch_size,
                learning_rate,
            );

            ic_cdk::println!("Training finished.");
        }
        "linear" => {}
        _ => {}
    }

    Ok(())
}

#[update(guard = "is_owner")]
fn predict() -> Result<Vec<f32>, String> {
    let model_name = get_default_model()?;
    match model_name.as_str() {
        "lstm_v1.0.0" => Ok(vec![]),
        "linear" => Ok(vec![]),
        _ => Ok(vec![]),
    }
}
