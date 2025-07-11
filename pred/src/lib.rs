mod common;
mod models;
mod api;

use ic_cdk::export_candid;
//example
//use ic_cdk_macros::*;
// use ic_cdk::export::candid;
//
// #[import(canister_id = "", candid_path = "multiply_deps.did")]
// struct CounterCanister;
//
// #[update]
// async fn read() -> candid::Nat {
//     CounterCanister::read().await.0
// }


export_candid!();