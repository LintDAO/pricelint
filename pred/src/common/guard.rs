use ic_cdk::{call, caller};
use ic_cdk::api::is_controller;
use crate::common::errors::GuardError;


//仅限canister的控制者访问
pub fn is_owner() -> Result<(), String> {
    if !is_controller(&caller()) {
         return Err(GuardError::IsNotCanisterController.to_string());
    }
    Ok(())
}
