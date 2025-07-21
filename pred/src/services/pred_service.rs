use candid::utils::ArgumentEncoder;
use candid::{CandidType, Principal};
use serde::{Serialize,Deserialize};
use crate::{ impl_storable};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use std::borrow::Cow;
use ic_cdk::api::call::CallResult;
use ic_cdk::api::management_canister::main::CanisterIdRecord;
use ic_cdk::call;
use crate::common::constants::IC_BACKEND_CANISTER_ID;

#[derive(Serialize, Deserialize, CandidType, Clone, Default)]
pub struct Predictor {
    pub id: String,
    pub user_id: String,
    pub canister_id: String,
    pub price: f64,
    pub trend: Option<String>, //实际结果 涨跌      // up down none  ,none表示尚未预测
    pub pred: Pred,
    pub stake: (f64, f64),  //amount:总的质押   //change:24小时内变化
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





// 推送到 backend canister
async fn push_predictor_to_backend()->Result<Predictor,String> {
    let canister_id=Principal::from_text(IC_BACKEND_CANISTER_ID).map_err(|e|e.to_string())?;
    let args=Predictor::default();
    let result:CallResult<(Predictor,)> =call(canister_id, "push_user_pred", (args,)).await;
    let (ret,)=result.map_err(|(r,e)| e.to_string())?;
    Ok(ret)
}


