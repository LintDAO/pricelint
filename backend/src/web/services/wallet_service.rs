use crate::web::models::context::Context;
use crate::web::models::wallet_model::Wallet;
use crate::{map_get,map_insert, WALLET_CONTEXT,Memory};
use proc_macro::{generate_service_impl, generate_service_trait};
use candid::Principal;
use std::cell::RefCell;
use std::thread::LocalKey;
use lazy_static::lazy_static;

generate_service_trait!(Wallet);
generate_service_impl!(Wallet, WALLET_CONTEXT);

pub trait ExtendWalletService: WalletService {
    fn is_exist(principal: Principal) -> bool;

}
impl ExtendWalletService for Wallet {
    fn is_exist(principal: Principal) -> bool {
        let ret= map_get!(MAP, &principal.to_string());
        ret.is_some()
    }
}
