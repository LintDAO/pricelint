use crate::impl_storable;
use candid::CandidType;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
    pub last_2: Option<PredictionHistory>,
    pub last_1: Option<PredictionHistory>,
    pub now: Option<PredictionHistory>,
    pub next: Option<PredictionHistory>,
    pub accuracy: f64,
    pub stake: (f64, f64),
    pub create_time: u64,
}
//预测结果
#[derive(Serialize, Deserialize, CandidType, Clone, Default)]
pub struct Pred {
    pub staked: u64,
    pub up: u64,
    pub down: u64,
    //预测结果 涨跌
    pub trend: String,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Ord, PartialOrd, Eq, PartialEq)]
//历史预测结果
pub struct PredictionHistory {
    //代币
    pub token_name: String,
    //实际价格
    pub price: u64,
    //这一次和上一次的实际家给对比 涨跌
    pub trend: String,
    //质押和预测结果 --> 质押金额  预测up金额  预测down金额  预测趋势
    pub pred: (u64, u64, u64, String),
    //记录时间
    pub time: u64,
}
//预测用户(canister_id)  预测时间 预测币种
#[derive(Serialize, Deserialize, CandidType, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct PredictionKey(pub(crate) String, pub(crate) u64, pub(crate) String);
//个人预测数据
#[derive(Serialize, Deserialize, CandidType, Clone, Default)]
pub struct Prediction {
    pub id: String,
    pub user_id: String,
    pub canister_id: String,
    //预测币种
    pub token_name: String,
    pub price: f64,
    pub trend: Option<String>, //实际结果 涨跌      // up down none  ,none表示尚未预测
    pub pred: Pred,
    pub stake: (f64, f64), //amount:个人的质押   //change:24小时内变化
    pub create_time: u64,
}
