use crate::common::constants::config::DEFAULT_MODEL_KEY;
use crate::common::errors::ConfigError;
use crate::common::guard::is_owner;
use crate::common::lifecycle::{Value, CONFIG};
use crate::models::lstm_v1::{load_model, LstmModel};
use burn::backend::ndarray::NdArrayDevice;
use burn::backend::{Autodiff, NdArray};
use burn::tensor::{DataError, Tensor, TensorData};
use ic_cdk::{query, update};
use crate::api::config::default_model_config::get_default_model;

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
        "lstm_v1.0.0" => {
            let device = NdArrayDevice::Cpu;

            // 加载模型
            let model: LstmModel<Autodiff<NdArray>> =
                load_model(&device).map_err(|e| e.to_string())?;
            let input = vec![1.0f32, 2.0, 3.0];
            // 输入预处理
            let seq_len = input.len();
            let input_tensor =
                Tensor::<Autodiff<NdArray>, 1>::from_floats(input.as_slice(), &device)
                    .reshape([1, seq_len, 1]);

            // 推理
            let output = model.predict(input_tensor);

            // 输出后处理
            let output_data = output.into_data();
            let output_slice = output_data.as_slice::<f32>().map_err(|e| match e {
                DataError::CastError(v) => v.to_string(),
                DataError::TypeMismatch(v) => v,
            })?;
            Ok(output_slice.to_vec())
        }
        "linear" => Ok(vec![]),
        _ => Ok(vec![]),
    }
}

