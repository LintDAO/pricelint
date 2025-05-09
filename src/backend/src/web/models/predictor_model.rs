use serde::{Deserialize, Serialize};
use crate::impl_storable;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use std::borrow::Cow;
use candid::CandidType;

#[derive(Serialize, Deserialize,CandidType,Clone)]
pub struct Predictor {
    pub id:String,
    pub asset: String, //币种
    pub accuracy: f64, //准确率
    pub past_price:f64,
    pub past_prediction:f64,
    pub now_price:f64,
    pub now_prediction:f64,
    pub stake: f64,    //份额
    pub create_time:u64
}
//et.关于定时 理论上前端定时即可
impl Default for Predictor {
    fn default() -> Self {
        Self{
            id:"".to_string(),
            asset: "".to_string(),
            accuracy: 0.0,
            past_price: 0.0,
            past_prediction: 0.0,
            now_price: 0.0,
            now_prediction: 0.0,
            stake: 0.0,
            create_time: 0,
        }
    }
}
impl Predictor {
    fn new(id:String,asset:String,accuracy:f64,past_price:f64,past_prediction:f64,now_price:f64,now_prediction:f64,stake:f64,create_time:u64)->Predictor{
        Self{
            id,
            asset,
            accuracy,
            past_price,
            past_prediction,
            now_price,
            now_prediction,
            stake,
            create_time,
        }
    }
}
impl_storable!(Predictor);