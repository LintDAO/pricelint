use std::fmt::{Display, Formatter};
use crate::web::common::errors::AuthenticationError;
use crate::web::models::context::Context;
use crate::web::models::user_model::User;
use crate::{impl_storable, map_get, map_insert, USER_CONTEXT};
use candid::{CandidType, Principal};
use ic_cdk::{call, caller, query, update};
use crate::web::services::user_service::{ExtendUserService, UserService};

#[query]
async fn user_login() -> Result<bool, String> {
    if caller() == Principal::anonymous() {
        return Err(AuthenticationError::AnonymousUser.to_string());
    }
    if !User::is_exist(caller()) {
        create_user();
    }
    Ok(true)
}

#[update]
fn create_user(){
    User::create_deafult_user();
}