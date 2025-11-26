//以后从config里面读取默认模型

use crate::api::config::default_model_config::get_default_model;
use crate::common::lifecycle::MODEL_MAP;
use crate::models::interface::predict::Predict;
use crate::models::interface::train::Train;
use crate::models::lstm::v1::lstm_domain::LstmModel;
use burn::backend::ndarray::NdArrayDevice;
use burn::backend::{Autodiff, NdArray};
use burn::prelude::{Tensor, TensorData};
use burn::LearningRate;
use ic_cdk::api::time;
use crate::services::user_predict_service::predict_entity::Prediction;

type AutodiffBackend = Autodiff<NdArray<f32>>;

const LSTM_V1_0_0: &str = "lstm_v1.0.0";

fn predict() -> Result<Vec<(u64, String)>, String> {
    let device = &NdArrayDevice::Cpu;
    let model_name = get_default_model()?;
    match model_name.as_str() {
        LSTM_V1_0_0 => {
            let model = MODEL_MAP
                .with_borrow(|rc| rc.get(&LSTM_V1_0_0.to_string()))
                .ok_or("Has not init training config,please set training config first.")?;
            let mut log = Vec::<(u64, String)>::new();
            log.push((time(), String::from("Start to predict price")));
            let model = LstmModel::<AutodiffBackend>::default().restore_from_bytes(model)?;
            //todo 获取 raw
            let raw_data = vec![];
            let result = lstm_predict(model, raw_data)?;
            log.push((time(), String::from("Complete to predict price.")));
            log.push((time(), String::from(format!("Prediction of price is:{:?}", result))));
            //todo 对比确定 trend
            let trend="up".to_string();
            // todo Prediction
            log.push((time(), String::from(format!("Trend of this token is:{:?}", trend))));
            Ok(log)
        }
        _ => Ok(vec![(
            time(),
            String::from("Has not matched default model in this canister!"),
        )]),
    }
}
//通过自己提供的数据预测
fn lstm_predict(lstm_model: LstmModel<Autodiff<NdArray>>, data: Vec<f32>) -> Result<f32, String> {
    let device = &NdArrayDevice::Cpu;
    let input = Tensor::<Autodiff<NdArray>, 3>::from_data(
        TensorData::new::<f32, Vec<usize>>(data.clone(), [data.len(), 1, 5].into()),
        device,
    );
    let result = lstm_model.predict(input);
    Ok(result)
}
