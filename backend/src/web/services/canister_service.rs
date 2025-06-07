use std::env::args;
use crate::web::common::errors::CanisterError;
use candid::{export_service, Principal};
use ic_cdk::api::call::{call_with_payment, call_with_payment128, CallResult};
use ic_cdk::api::management_canister::main::{ CanisterId, CanisterIdRecord, CanisterInfoRequest, CanisterInfoResponse, CanisterSettings, CanisterStatusResponse, CreateCanisterArgument};
use ic_cdk::{call, query};
use ic_cdk::api::canister_version;
use crate::web::services::canister_service;

const DEFAULT_CYCLES:u128=1_000_000_000;
pub async fn create_canister(user_principal: Principal, cycles: u128,) -> CallResult<(CanisterIdRecord,)> {

    let settings=CanisterSettings{
           controllers: Some(vec![user_principal]),
           compute_allocation: None,
           memory_allocation: None,
           freezing_threshold: None,
           reserved_cycles_limit: None,
           log_visibility: None,
           wasm_memory_limit: None ,
       };
        let sender_canister_version=Some(canister_version());
    call_with_payment128(
        user_principal,
        "create_canister",
        (settings,sender_canister_version),
        cycles,
    )
        .await
}

//指示有关canister的各种信息。只有容器的controller可以请求其状态。它包含了：
//  status。 它可以是运行、停止或停止之一。
// SHA256 哈希值。安装在容器上的模块的 SHA256 哈希值。 如果容器为空，则为 null。
// controller。控制器列表
// allocations。占用的内存大小。
// cycle数量。
pub async fn canister_status(user_principal: Principal,canister_id: Principal) -> CallResult<(CanisterStatusResponse,)> {
    call(user_principal, "canister_status", (canister_id,)).await
}


//容器的controller可以停止容器（例如，为容器升级做准备）。
//  停止canister不是原子操作。 直接效果是容器的状态更改为正在停止（除非容器已停止）。
// 系统将reject对正在停止的容器的所有调用，表明容器正在停止。 对停止canister的响应照常处理。
// 处理完所有未完成的响应后（因此没有打开的调用上下文），容器状态更改为停止，并且管理容器响应 stop_canister 请求的调用者。
pub async fn stop_canister(user_principal: Principal,canister_id: Principal) -> CallResult<()> {
    call(user_principal, "stop_canister", (canister_id,)).await
}
//容器可以由其controller启动。
//如果容器状态已停止或正在停止，则容器状态仅设置为运行。 在后一种情况下，所有正在处理的 stop_canister 调用都失败（并被拒绝）。
//如果容器已在运行，则状态保持不变。
pub async fn start_canister(user_principal: Principal,canister_id: Principal) -> CallResult<()> {
    call(user_principal, "start_canister", (canister_id,)).await
}
//  此方法从 IC 中删除一个canister。只有容器的 controllers 可以删除它，并且容器必须已经停止。
//  删除容器无法撤消，存储在容器上的任何状态都将被永久删除并丢弃其cycle。容器一旦被删除，其 ID 就不能再使用。
pub async fn delete_canister(user_principal:Principal,canister_id: Principal) -> CallResult<()> {
    call(user_principal, "delete_canister", (canister_id,)).await
}


//  此方法将包含在此调用中的cycle存放到指定的容器中。
// 对谁可以调用此方法，没有controller限制。
pub async fn deposit_cycles(user_principal:Principal,canister_id: Principal, cycles: u128) -> CallResult<()> {
    call_with_payment128(
        user_principal,
        "deposit_cycles",
        (canister_id,),
        cycles,
    )
        .await
}


pub async fn canister_info(user_principal:Principal,canister_id: Principal) -> CallResult<(CanisterInfoResponse,)> {
    call(user_principal, "canister_info", (canister_id,)).await
}


pub async  fn get_all_canisters(){

}
//todo 中间平台
//todo不确定这个call走的用户还是本人的