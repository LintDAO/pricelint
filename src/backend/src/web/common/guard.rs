use std::cell::RefCell;
use std::collections::BTreeMap;
use candid::Principal;
use ic_cdk::caller;
use crate::{map_get, map_insert};
use crate::web::common::constants::{ADMIN_ROLE_TAG, OWNER_ROLE_TAG};
use crate::web::common::errors::GuardError;

thread_local! {
    static ROLE_USER_TREE:RefCell<BTreeMap<String, Vec<Principal>>>=RefCell::new(BTreeMap::new());
}
//todo 初始化插入 tag和管理人员
//todo 创建用户的时候插入 ROLE_USER_TREE的用户
fn band_role<'a>(  principal:Principal,role_name:String){
    let user_lists=map_get!(ROLE_USER_TREE,&role_name,'a);
    match user_lists {
       None => {
           map_insert!(ROLE_USER_TREE,role_name,vec![principal]);
       }
       Some( _) => {
          ROLE_USER_TREE.with_borrow_mut(|map| {
              map_get!(ROLE_USER_TREE,&role_name,'a).unwrap().push(principal);
          })
       }
   }
}

pub fn is_admin<'admin>() -> Result<(), String> {
    let current_user=caller();
    let admin_lists=map_get!(ROLE_USER_TREE,&ADMIN_ROLE_TAG.to_string(),'admin);
    if  admin_lists.unwrap().contains(&current_user){
        return Ok(());
    }
    Err(GuardError::IsNotAdministrator.to_string())
}

pub fn is_named_user<'user>() -> Result<(), String>{
    let current_user=caller();
    let normal_users=map_get!(ROLE_USER_TREE,&OWNER_ROLE_TAG.to_string(),'user);
    if  normal_users.unwrap().contains(&current_user){
        return Ok(());
    }
    Err(GuardError::IsAnonymousUser.to_string())
}