use crate::web::common::errors::WalletError;
use crate::web::models::context::Context;
use crate::web::models::wallet_model::Wallet;
use crate::{map_get, map_insert, Memory, WALLET_CONTEXT};
use candid::Principal;
use ic_cdk::api::canister_balance;
use ic_cdk::call;
use lazy_static::lazy_static;
use proc_macro::{generate_service_impl, generate_service_trait};
use std::cell::RefCell;
use std::thread::LocalKey;

generate_service_trait!(Wallet);
generate_service_impl!(Wallet, WALLET_CONTEXT);

pub trait ExtendWalletService: WalletService {
    fn is_exist(principal: Principal) -> bool;
    async fn get_icp_balance(principal: Principal) -> Result<f64, WalletError>;
    async fn get_cycles_balance(principal: Principal) -> Result<u64, WalletError>;
}
impl ExtendWalletService for Wallet {
    fn is_exist(principal: Principal) -> bool {
        let ret = map_get!(MAP, &principal.to_string());
        ret.is_some()
    }
    async fn get_icp_balance(user_principal: Principal) -> Result<f64, WalletError> {
        // 构造查询参数
        let args = (user_principal,);

        // 发起跨 Canister 调用
        let result: Result<(f64,), _> = call(user_principal, "icrc1_balance_of", args).await;

        match result {
            Ok((balance,)) => Ok(balance),
            Err((code, msg)) => Err(WalletError::GetICPBalanceFailed),
        }
    }
    async fn get_cycles_balance(canister_principal: Principal) -> Result<u64, WalletError> {
        // 调用目标 Canister 的 get_cycles_balance 方法
        let result: Result<(u64,), _> =
            call(canister_principal, "get_cycles_balance", ()).await;
        match result {
            Ok((balance,)) => Ok(balance),
            Err((code, msg)) => Err(WalletError::GetCyclesFailed),
        }
    }
}
