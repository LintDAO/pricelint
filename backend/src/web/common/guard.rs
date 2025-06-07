use crate::web::common::constants::{ADMIN_ROLE_TAG, OWNER_ROLE_TAG};
use crate::web::common::errors::GuardError;
use crate::{map_get, map_insert};
use candid::types::principal::PrincipalError;
use candid::Principal;
use ic_cdk::caller;
use lazy_static::lazy_static;
use serde::__private::de::IdentifierDeserializer;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::convert::Into;
use std::string::ToString;

thread_local! {
    static ROLE_USER_TREE:RefCell<BTreeMap<String, Vec<Principal>>>=RefCell::new(BTreeMap::new());
}
//todo 初始化插入 tag和管理人员
//todo 创建用户的时候插入 ROLE_USER_TREE的用户
fn band_role<'a>(principal: Principal, role_name: String) {
    let user_lists = map_get!(ROLE_USER_TREE,&role_name,'a);
    match user_lists {
        None => {
            map_insert!(ROLE_USER_TREE, role_name, vec![principal]);
        }
        Some(_) => ROLE_USER_TREE.with_borrow_mut(|map| {
            map_get!(ROLE_USER_TREE,&role_name,'a)
                .unwrap()
                .push(principal);
        }),
    }
}

lazy_static! {
    static ref DEFAULT_ADMIN_VEC: Vec<Result<Principal, PrincipalError>> = vec![
        Principal::from_text("vsqls-6k2en-jqrej-7dvmj-x27gn-e6bzr-asyr6-k7k6f-zl4xe-yykp4-uqe"),
        Principal::from_text("gq66f-io24f-torxu-ftmfi-chzvv-umdni-3jkai-s734u-wdzps-jmlkk-2ae")
    ];
}
pub fn is_admin<'admin>() -> Result<(), String> {
    let current_user = caller();
    let admin_lists = map_get!(ROLE_USER_TREE,&ADMIN_ROLE_TAG.to_string(),'admin);
    match admin_lists {
        None => {
            //如果为匹配管理列表空 则插入默认的管理员用户   相当于初始化了
            for admin in DEFAULT_ADMIN_VEC.iter() {
                match admin {
                    Ok(value) => {
                        {
                            band_role(*value, ADMIN_ROLE_TAG.to_string());
                        }
                        {
                            band_role(*value, OWNER_ROLE_TAG.to_string());
                        }
                    }
                    Err(e) => {
                        return Err(e.to_string());
                    }
                }
            }
        }
        Some(users) => {
            if users.contains(&current_user) {
                return Ok(());
            }
        }
    }

    Err(GuardError::IsNotAdministrator.to_string())
}

pub fn is_named_user<'user>() -> Result<(), String> {
    let current_user = caller();
    let named_users = map_get!(ROLE_USER_TREE,&OWNER_ROLE_TAG.to_string(),'user);
    if named_users.unwrap().contains(&current_user) {
        return Ok(());
    }
    Err(GuardError::IsAnonymousUser.to_string())
}
