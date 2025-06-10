use crate::impl_storable::StringVec;
use crate::web::common::constants::{ADMIN_ROLE_TAG, OWNER_ROLE_TAG};
use crate::web::common::errors::GuardError;
use crate::{impl_storable, map_get, map_insert, Memory, ROLE_USER_TREE};
use candid::types::principal::PrincipalError;
use candid::Principal;
use ic_cdk::caller;
use lazy_static::lazy_static;
use std::string::ToString;

pub fn band_role(principal: String, role_name: String) {
    //获取该角色tag的vec
    let user_lists = map_get!(ROLE_USER_TREE, &role_name);

    match user_lists {
        None => {
            map_insert!(ROLE_USER_TREE, role_name, StringVec(vec![principal]));
        }
        Some(StringVec(mut vec)) => {
            vec.push(principal);
            ic_cdk::println!("role:{}  len:{}",role_name,vec.len());
            map_insert!(ROLE_USER_TREE, role_name, StringVec(vec));
        }
    }
}

pub fn is_admin<'admin>() -> Result<(), String> {
    let current_user = caller().to_string();
    let admin_lists = map_get!(ROLE_USER_TREE,&ADMIN_ROLE_TAG.to_string(),'admin);
    match admin_lists {
        None => return Err(GuardError::UnknownEmptyAdminLists.to_string()),
        Some(StringVec(users)) => {
            if users.contains(&current_user) {
                return Ok(());
            }
        }
    }

    Err(GuardError::IsNotAdministrator.to_string())
}

pub fn is_named_user<'user>() -> Result<(), String> {
    let current_user = caller().to_string();
    let named_users = map_get!(ROLE_USER_TREE,&OWNER_ROLE_TAG.to_string(),'user);
    //查询user tag的vec
    match named_users {
        None => return Err(GuardError::UnknownEmptyUserLists.to_string()),
        Some(StringVec(users)) => {
            if users.contains(&current_user) {
                return Ok(());
            }
        }
    }

    Err(GuardError::IsAnonymousUser.to_string())
}

pub fn init_admin() {
    let init_admin = vec![
        //dfx
        "vsqls-6k2en-jqrej-7dvmj-x27gn-e6bzr-asyr6-k7k6f-zl4xe-yykp4-uqe",
        "gq66f-io24f-torxu-ftmfi-chzvv-umdni-3jkai-s734u-wdzps-jmlkk-2ae",
        //nns/ii
        "c6dif-csraj-5ckyd-3m3h6-r5td3-3b2wi-a7bpn-ccf3k-c6q3i-7fjb6-jae",
    ];

    // 启动时候初始化管理员还有 用户列表  还有 bind role
    for admin in init_admin {
        //只是绑定了角色和初始化列表  具体的用户注册需要手动user_login api去手动注册 ，避免流程过于复杂
        band_role(admin.to_string(), ADMIN_ROLE_TAG.to_string());
        band_role(admin.to_string(), OWNER_ROLE_TAG.to_string());
    }
    //todo 将管理员添加在canisters的control人员里面里面 包括nns/ii/dfx身份
}
