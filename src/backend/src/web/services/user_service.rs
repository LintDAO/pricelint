use crate::web::models::context::Context;
use crate::web::models::user_model::User;
use crate::{map_get, map_insert, Memory, USER_CONTEXT};
use candid::Principal;
use ic_cdk::caller;
use lazy_static::lazy_static;
use proc_macro::{generate_service_impl, generate_service_trait};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::thread::LocalKey;

generate_service_trait!(User);
generate_service_impl!(User, USER_CONTEXT);

pub trait ExtendUserService: UserService {
    fn is_exist(principal: Principal) -> bool;
    fn f1() -> Option<Self::Output>;
    fn f2() -> Option<Vec<Self::Output>>;
}
impl ExtendUserService for User {
    fn is_exist(principal: Principal) -> bool {
        let ret = map_get!(MAP, &principal.to_string());
        match ret {
            None => false,
            Some(_) => true,
        }
    }
    fn f1() -> Option<Self::Output> {
        MAP.with(|map| {
            map.borrow_mut()
                .iter()
                .find(|(_, user)| user.owner.unwrap() == caller())
                .map(|(_, user)| user.context.clone().unwrap()) // 必须实现 Clone)
        })
    }

    fn f2() -> Option<Vec<Self::Output>> {
        MAP.with(|map| {
            map.borrow_mut()
                .iter()
                .filter(|(_, ctx)| ctx.owner.unwrap()==caller()) // 示例条件
                .map(|(_, ctx)| ctx.context.clone())
                .collect()
        })
    }
}
