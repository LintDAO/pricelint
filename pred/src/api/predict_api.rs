//以后从config里面读取默认模型

pub mod prediction_api {
    use crate::api::config::default_model_config::get_default_model;
    use crate::api::predict_api::prediction_domain::{Pred, Prediction};
    use crate::common::constants::canister_id::{
        IC_BACKEND_CANISTER_ID, LOCAL_BACKEND_CANISTER_ID,
    };
    use crate::common::lifecycle::{MODEL_MAP, PREDICTION};
    use crate::models::interface::predict::Predict;
    use crate::models::interface::train::Train;
    use crate::models::lstm::v1::lstm_domain::LstmModel;
    use burn::backend::ndarray::NdArrayDevice;
    use burn::backend::{Autodiff, NdArray};
    use burn::prelude::{Tensor, TensorData};
    use burn::LearningRate;
    use candid::Principal;
    use ic_cdk::api::call::CallResult;
    use ic_cdk::api::time;
    use ic_cdk::{api, call, query, update};
    use std::collections::{hash_map, BTreeMap, HashMap};
    use std::env::args;
    use std::fmt::format;

    type AutodiffBackend = Autodiff<NdArray<f32>>;

    const LSTM_V1_0_0: &str = "lstm_v1.0.0";

    pub async fn predict_trend() -> Result<(), String> {
        let device = &NdArrayDevice::Cpu;
        let model_name = get_default_model()?;
        match model_name.as_str() {
            LSTM_V1_0_0 => {
                let model = MODEL_MAP
                    .with_borrow(|rc| rc.get(&LSTM_V1_0_0.to_string()))
                    .ok_or("Has not init this model,please init model first.")?;
                let mut log = Vec::<(u64, String)>::new();
                let model = LstmModel::<AutodiffBackend>::default().restore_from_bytes(model)?;
                // 最近100个时间步 从后台获取
                let token_name = "ICPUSDT".to_string();
                let step = 100usize;

                let (seq_data,) = call::<(String, usize), (Vec<(f32, f32)>,)>(
                    LOCAL_BACKEND_CANISTER_ID.parse().unwrap(),
                    "user_query_exchange",
                    (token_name, step),
                )
                .await
                .map_err(|(r, e)| e.to_string())?;
                let last1 = seq_data[0].1;
                let last2 = seq_data[1].1;

                let seq_data = seq_data
                    .iter()
                    .map(|(time, price)| [*time, *price])
                    .flatten()
                    .collect::<Vec<_>>();
                if seq_data.len() <= 0 {
                    return Err(String::from("seq_data length cannot be zero"));
                }

                let result = lstm_predict(model, seq_data)?;
                //对比确定 trend

                let pred_ = if result > last1 {
                    Pred {
                        staked: 0.0,
                        up: 0.0,
                        down: 0.0,
                        trend: "up".to_string(),
                    }
                } else {
                    Pred {
                        staked: 0.0,
                        up: 0.0,
                        down: 0.0,
                        trend: "down".to_string(),
                    }
                };
                let trend_ = if last1 > last2 { "up" } else { "down" };
                //todo user_principal&stake
                let prediction = Prediction {
                    id: ic_cdk::api::id().to_text() + time().to_string().as_str(),
                    user_id: "".to_string(),
                    canister_id: ic_cdk::api::id().to_text(),
                    price: last1 as f64,
                    trend: trend_.to_string(),
                    pred: pred_,
                    create_time: time(),
                };
                PREDICTION.with_borrow_mut(|rc| rc.insert(time(), prediction.clone()));
                let prediction_resp = push_to_backend(prediction)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(())
            }
            _ => Err(format!("Unknown model name: {}", model_name)),
        }
    }

    //通过自己提供的数据预测
    fn lstm_predict(
        lstm_model: LstmModel<Autodiff<NdArray>>,
        data: Vec<f32>,
    ) -> Result<f32, String> {
        let seq_length = 100;
        let feature_size = 2;
        //暂时固定值
        let max = 124658.54f32;
        let min = 15781.29f32;

        let device = &NdArrayDevice::Cpu;
        let input = Tensor::<Autodiff<NdArray>, 3>::from_data(
            TensorData::new::<f32, Vec<usize>>(
                data.clone(),
                [data.len(), seq_length, feature_size].into(),
            ),
            device,
        );
        let result = lstm_model.predict(input);
        //还原
        let original_value = result * (max - min) + min;
        Ok(original_value)
    }

    pub async fn push_to_backend(prediction: Prediction) -> Result<Prediction, String> {
        let canister_id =
            Principal::from_text(LOCAL_BACKEND_CANISTER_ID).map_err(|e| e.to_string())?;
        let result: CallResult<(Prediction,)> =
            call(canister_id, "prediction_record", (prediction,)).await;
        let (ret,) = result.map_err(|(r, e)| e.to_string())?;
        Ok(ret)
    }

    //预测准确率的图表
    // #[query]
    fn prediction_chart_data(begin_time: u64, end_time: u64) -> Vec<(u64, f64)> {
        const SECONDS_PER_DAY: u64 = 60 * 60 * 24;
        PREDICTION.with_borrow_mut(|rc| {
            rc.iter()
                .filter(|(k, _)| k >= &begin_time && k <= &end_time)
                .fold(HashMap::new(), |mut acc, (ts, value)| {
                    // 计算当前数据点所属的日期的起始时间
                    let day_start_ts = (ts / SECONDS_PER_DAY) * SECONDS_PER_DAY;
                    acc.entry(day_start_ts)
                        .or_insert(Vec::new())
                        .push((ts, value));
                    acc
                })
                .iter()
                .map(|(k, vec)| {
                    let total_size = vec.len() as f64;
                    let predict_true = vec
                        .iter()
                        .filter(|((t, p))| p.trend == p.pred.trend)
                        .count() as f64;
                    (*k, predict_true / total_size)
                })
                .collect::<Vec<_>>()
        })
    }
    //最新预测结果
    // #[query]
    fn latest_predictionlatest_prediction() -> Option<(u64, Prediction)> {
        PREDICTION.with_borrow(|rc| rc.iter().max_by_key(|(a, _)| *a))
    }
}

pub mod prediction_domain {
    use crate::impl_storable;
    use ic_stable_structures::storable::{Bound, Storable};
    use serde::Serialize;
    use std::borrow::Cow;
    #[derive(Serialize, Deserialize, CandidType, Clone, Default)]
    pub struct Prediction {
        pub id: String,
        pub user_id: String,
        pub canister_id: String,
        pub price: f64,
        pub trend: String, //实际结果 涨跌      // up down none  ,none表示尚未预测
        pub pred: Pred,
        pub create_time: u64,
    }
    //预测结果
    #[derive(Serialize, Deserialize, CandidType, Clone, Default)]
    pub struct Pred {
        pub staked: f64,
        pub up: f64,
        pub down: f64,
        //预测结果 涨跌
        pub trend: String,
    }
    impl_storable!(Prediction);
}
