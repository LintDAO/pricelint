use crate::impl_storable;
use candid::CandidType;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;




//et.关于定时 理论上前端定时即可
#[derive(Serialize, Deserialize, CandidType, Clone, Default)]
pub struct Predictor {
    pub id: String,
    pub name: String,
    pub price: f64,
    pub trend: Option<String>, //实际结果 涨跌      // up down none  ,none表示尚未预测
    pub pred: Pred,
    pub stake: (f64, f64),
    pub create_time: u64,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Default)]
pub struct PredictorView{
    pub id: String,
    pub last_2: Option<Predictor>,
    pub last_1: Option<Predictor>,
    pub now: Option<Predictor>,
    pub next: Option<Predictor>,
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
