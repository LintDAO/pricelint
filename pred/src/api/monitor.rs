use candid::Principal;
use ic_cdk::{api, call, caller, post_upgrade, pre_upgrade, query, update};
use crate::common::guard::is_owner;
use ic_cdk::api::call::CallResult;
use ic_cdk::api::time;
use crate::common::constants::{IC_BACKEND_CANISTER_ID, LOCAL_BACKEND_CANISTER_ID};
use crate::services::pred_service::predict_entity::{Pred, Predictor};

//监控canisters数据
#[query(guard = "is_owner")]
pub async fn get_canister_info() {
}

#[update(guard = "is_owner")]
pub async fn update_canister_info() -> () {
}


#[update(guard = "is_owner")]
pub async fn collect_metrics() -> () {
  
}
//TODO: 需要前端传递参数调用


#[update]
pub async fn test1()->Result<Predictor,String>{
    //TODO:实际运行改成ic的canister_id
    let canister_id=Principal::from_text(LOCAL_BACKEND_CANISTER_ID).map_err(|e|e.to_string())?;
    let args=Predictor{
        id: "".to_string(),
        user_id: caller().to_text(),
        canister_id: api::id().to_string(),
        price: 10.0,   //TODO:
        trend: None, //TODO:
        pred: Pred{
            staked: 0.0,
            up: 0.0,
            down: 0.0,
            trend: "".to_string(),
        }, //TODO:
        stake: (0.0, 0.0), //TODO:
        create_time: time(),
    };
    //TODO:default重新赋值
    let result:CallResult<(Result<Predictor,String>,)> =call(canister_id, "push_user_pred", (args,)).await;
    let (ret,)=result.map_err(|(r,e)| e.to_string())?;
    ret
}


