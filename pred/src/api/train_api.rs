use crate::api::config::default_model_config::get_default_model;
use crate::common::guard::is_owner;
use crate::common::lifecycle::MODEL_MAP;
use crate::models::interface::train::Train;
use crate::models::lstm::v1::lstm_domain::{LstmModel, LstmModelConfig};
use burn::backend::ndarray::NdArrayDevice;
use burn::backend::{Autodiff, NdArray};
use burn::prelude::{Tensor, TensorData};
use burn::LearningRate;
use ic_cdk::api::time;
use ic_cdk::{query, update};
use std::time;

type AutodiffBackend = Autodiff<NdArray<f32>>;
const LSTM_V1_0_0: &str = "lstm_v1.0.0";

//初始化或者重置
#[update(guard = "is_owner")]
fn lstm_train_init(mut config: Option<LstmModelConfig>) -> Result<Vec<(u64, String)>, String> {
    ic_cdk::println!("Training started...");
    let model_name = get_default_model()?;
    let device = &NdArrayDevice::Cpu;
    match model_name.as_str() {
        LSTM_V1_0_0 => {
            let mut model;
            if config.is_none() {
                model = LstmModel::<AutodiffBackend>::default();
            } else {
                model = LstmModel::<AutodiffBackend>::new(config.unwrap(), device);
            }
            let record = model.record_as_bytes()?;
            MODEL_MAP.with_borrow_mut(|rc| rc.insert(LSTM_V1_0_0.to_string(), record));
            Ok(vec![(time(), String::from("Init or reset model success"))])
        }
        _ => Ok(vec![(
            time(),
            String::from("Has not matched default model in this canister!"),
        )]),
    }
}

//  批次训练
// 为了方便暂时用vec<u8>
#[update]
fn train_epochs(
    epochs: usize,
    mut lr: Option<f64>,
    data: Vec<u8>,
) -> Result<Vec<(u64, String)>, String> {
    if lr.is_none() {
        lr = Some(0.01);
    }
    let device = &NdArrayDevice::Cpu;
    let model_name = get_default_model()?;
    match model_name.as_str() {
        LSTM_V1_0_0 => {
            let model = MODEL_MAP
                .with(|rc| rc.borrow().get(&model_name))
                .ok_or("Has not init training config,please set training config first.")?;
            let init = LstmModel::<AutodiffBackend>::default();
            let mut model = init.restore_from_bytes(model)?;
            let vec = serde_json::from_slice::<Vec<(f32, f32)>>(data.as_slice())
                .map_err(|e| e.to_string())?;

            //进行数据的缩放 
            let max_price = 124658.54f32;
            let min_price = 15781.29f32;

            let min_time = vec[0].0;
            let max_time = vec[vec.len() - 1].0;
            let train_data = vec
                .clone()
                .iter()
                .map(|(x,y)| [(x - min_time) / (max_time - min_time), (y - min_price) / (max_price - min_price)])
                .flatten()
                .collect::<Vec<_>>();
            let price_data = vec
                .iter()
                .map(|(x, price)| (price - min_price) / (max_price - min_price))
                .collect::<Vec<_>>();
            let num_sequences = price_data.len();
            let seq_length = 10;

            let input = Tensor::<AutodiffBackend, 3>::from_data(
                TensorData::new::<f32, Vec<usize>>(
                    train_data.repeat(seq_length),
                    [num_sequences, seq_length, 2].into(),
                ),
                device,
            );

            // 创建目标张量 [num_sequences, 1]
            let target = Tensor::<AutodiffBackend, 2>::from_data(
                TensorData::new::<f32, Vec<usize>>(price_data, [num_sequences, 1].into()),
                &device,
            );
            let mut log = Vec::<(u64, String)>::new();
            for epoch in 0..epochs {
                log.push((time(), String::from(format!("Start epoch {}", epoch))));
                let (_, train_log) =
                    model.train_step(input.clone(), target.clone(), lr.unwrap() as LearningRate);
                log.extend(train_log);
            }
            let record = model.record_as_bytes()?;

            MODEL_MAP.with_borrow_mut(|rc| rc.insert(LSTM_V1_0_0.to_string(), record));
            log.push((time(), String::from("Record this model successfully")));
            Ok(log)
        }
        _ => Ok(vec![(
            time(),
            String::from("Has not matched default model in this canister!"),
        )]),
    }
}
