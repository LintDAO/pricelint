use std::fmt::{Display, Formatter};
use crate::web::common::errors::{AuthenticationError, UserError};
use crate::web::models::context::Context;
use crate::web::models::user_model::User;
use crate::{impl_storable, map_get, map_insert, USER_CONTEXT};
use candid::{export_service, CandidType, Principal};
use ic_cdk::{call, caller, query, update};
use crate::web::services::user_service::{ExtendUserService, UserService};
use crate::web::common::guard::{is_named_user,is_admin};
#[query]
async fn user_login() -> Result<Option<User>, String> {
    if caller() == Principal::anonymous() {
        return Err(AuthenticationError::AnonymousUser.to_string());
    }
    if !User::is_exist(caller()) {
       let new_user=create_user();
        match new_user {
            None => {
                return  Err(UserError::CreateUserFailed.to_string());
            }
            Some(_) => {
                return Ok(new_user);
            }
        }
    };
    Ok(None) //表示存在
}

#[update]
fn create_user()->Option<User> {
    User::create_deafult_user()
}
#[update(guard="is_admin")]
fn data_clear()->(){
    USER_CONTEXT.with(|s| s.borrow_mut().clear_new());
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