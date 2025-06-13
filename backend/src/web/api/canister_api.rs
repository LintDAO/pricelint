use crate::web::services::canister_service::canister_info;
use candid::types::principal::PrincipalError;
use candid::{Error, Principal};
use ic_cdk::{caller, id, query, update};

// #[query]
#[update]
async fn get_canister_info() -> Result<String, String> {
    let principal: String="eov5t-niaaa-aaaah-arepa-cai".to_string();

    let canister_id = Principal::from_text(principal).map_err(|e| e.to_string())?;
    let user_principal=Principal::management_canister();

    let (ret,) = canister_info(user_principal, canister_id).await.map_err(|(r,e)|
        format!("[rejectionCode]:{:?} ,[messages]:{}",r,e)
    )?;
    let ret = ret.controllers.get(0).unwrap().to_string();
    //eov5t-niaaa-aaaah-arepa-cai
    Ok(ret)
}

//local
#[update]
async fn get_canister_info1() -> Result<String, String> {
    let principal: String="bd3sg-teaaa-aaaaa-qaaba-cai".to_string();

    let canister_id = Principal::from_text(principal).map_err(|e| e.to_string())?;
    let user_principal=Principal::management_canister();

    let (ret,) = canister_info(user_principal, canister_id).await.map_err(|(r,e)|
        format!("[rejectionCode]:{:?} ,[messages]:{}",r,e)
    )?;

    let ret = ret.controllers.get(0).unwrap().to_string();
    //eov5t-niaaa-aaaah-arepa-cai
    Ok(ret)
}