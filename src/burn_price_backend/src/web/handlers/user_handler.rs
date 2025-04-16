use std::cell::RefCell;
use std::collections::HashMap;
use candid::{CandidType, Principal};
use ic_cdk::{caller, query, update};
use std::error::Error;
use crate::web::common::errors::AuthenticationError;


#[update]
fn user_login() -> Result<bool, String> {
    let caller = caller();
    if  caller == Principal::anonymous() {
      return  Err(AuthenticationError::AnonymousUser.to_string());
    }
    

    Ok(true)
}
