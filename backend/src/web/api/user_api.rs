use std::fmt::{Display, Formatter};
use crate::web::common::errors::{AuthenticationError, UserError};
use crate::web::models::context::Context;
use crate::web::models::user_model::User;
use crate::{impl_storable, map_get, map_insert, USER_CONTEXT};
use candid::{export_service, CandidType, Error, Principal};
use ic_cdk::{call, caller, query, update};
use crate::web::common::constants::OWNER_ROLE_TAG;
use crate::web::services::user_service::{ExtendUserService, UserService};
use crate::web::common::guard::{is_named_user, is_admin, band_role};
#[query]
 fn user_login() -> Result<User, String> {
    if caller() == Principal::anonymous() {
        return Err(AuthenticationError::AnonymousUser.to_string());
    }
    if !User::is_exist(caller()) {
        return Err(UserError::UserIsNotExist.to_string());
    };
    let user=User::find_one_by_principal(caller()).ok_or(UserError::UserIsNotFound.to_string())?;
    Ok(user)
    //表示该用户存在  不用注册了
}
#[update]
fn user_register()-> Result<User, String> {
    if caller() == Principal::anonymous() {
        return Err(AuthenticationError::AnonymousUser.to_string());
    }
    if !User::is_exist(caller()) {
        let new_user=create_user().ok_or(UserError::CreateUserFailed.to_string())?;
        return Ok(new_user);
    };
   Err(UserError::RegisterUserHasExist.to_string()) //表示已存在 不需要注册了
}

//管理员使用用户身份创建 而不是用户直接创建
fn create_user()->Option<User> {
    let user=User::create_deafult_user();
    let user= user.map(|u| {
        band_role( u.owner.to_string(),OWNER_ROLE_TAG.to_string());
        return  u
    });
    user
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

#[query]
fn  get_principal()->Principal{
    caller()
}
