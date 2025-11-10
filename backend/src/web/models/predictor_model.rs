use crate::impl_storable;
use candid::CandidType;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

//个人预测数据
#[derive(Serialize, Deserialize, CandidType, Clone, Default)]
pub struct Predictor {
    pub id: String,
    pub user_id: String,
    pub canister_id: String,
    pub price: f64,
    pub trend: Option<String>, //实际结果 涨跌      // up down none  ,none表示尚未预测
    pub pred: Pred,
    pub stake: (f64, f64), //amount:个人的质押   //change:24小时内变化
    pub create_time: u64,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Default)]
pub struct PredictorResult {
    pub price: Option<f64>,    //历史价格
    pub trend: Option<String>, //实际结果 涨跌      // up down none  ,none表示尚未预测
    pub pred: Pred,
}
//所有人的预测数据的集合总体统计
#[derive(Serialize, Deserialize, CandidType, Clone, Default)]
pub struct PredictorView {
    pub id: String,
    pub token_name: String,
    pub last_2: Option<PredictorResult>,
    pub last_1: Option<PredictorResult>,
    pub now: Option<PredictorResult>,
    pub next: Option<PredictorResult>,
    pub accuracy: f64,
    pub stake: (f64, f64),
    pub create_time: u64,
}
//预测结果
#[derive(Serialize, Deserialize, CandidType, Clone, Default)]
pub struct Pred {
    pub staked: f64,
    pub up: f64,
    pub down: f64,
    //预测结果 涨跌
    pub trend: String
}

impl_storable!(Predictor);

#[derive(Serialize, Deserialize, CandidType, Clone, Default)]
//历史预测结果
pub struct Prediction{
    //代币
    token_name:String,
    //实际价格
    price:u64,
    //预测趋势
    trend:String,
    //质押和预测结果
    pred:Pred,
    //记录时间
    time:u64
}
impl_storable!(Prediction);