use std::fmt::{Display, Formatter};
use crate::web::common::errors::AuthenticationError;
use crate::web::models::context::Context;
use crate::web::models::user_model::User;
use crate::{impl_storable, map_get, map_insert, USER_CONTEXT};
use candid::{CandidType, Principal};
use ic_cdk::{call, caller, query, update};
use crate::web::services::user_service::{ExtendUserService, UserService};
use crate::web::common::guard::{is_named_user,is_admin};
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

#[query(guard = "is_admin")]
fn find_user_lists()->Vec<User>{
    let option_users=User::find_all();
    match option_users{
        None => {vec![]}
        Some(vec_users) => {
            vec_users
        }
    }
}
