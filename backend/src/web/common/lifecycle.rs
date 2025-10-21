use crate::common::utils::xrc;
use crate::common::utils::xrc::{
    Asset, ExchangeRate, GetExchangeRateRequest, GetExchangeRateResult,
};
use crate::web::common::constants::XRC_CANISTER_ID;
use crate::web::common::guard::init_admin;
use crate::web::models::predictor_model::Predictor;
use crate::web::services::predictor_service::ExtendPredictorService;
use candid::Principal;
use ic_cdk::api::time;
use ic_cdk::{block_on, init, post_upgrade, pre_upgrade, spawn};
use ic_cdk_timers::{set_timer, set_timer_interval};
use std::fmt;
use std::fmt::Debug;
use std::future::IntoFuture;
use std::time::Duration;
use tokio::runtime::Handle;

#[init]
fn init() {
    init_admin();
    init_timer();
}
#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("pre_upgrade:");
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("post_upgrade:");
    init_admin();
    init_timer();
}

fn init_timer() {
    let duration = 60 * 60;
    let now = Duration::from_nanos(time()).as_secs();
    // 计算距离下一个整点的秒数
    let next_running_duration = duration - (now % duration);
    let next_running_time = now + next_running_duration;
    schedule_next_tick(
        duration,
        next_running_duration,
        next_running_time,
        schedule_tasklists,
    );
}

fn schedule_tasklists() {
    ic_cdk::println!("schedule_tasklists:{}", time());
    spawn(async move {
        let x=Predictor::autosave_predictor().await;       
        let x=Predictor::autosave_exchange_rate().await;
    })
}
// 计划下一个任务执行时间
fn schedule_next_tick(
    duration: u64,
    next_running_duration: u64,
    next_running_time: u64,
    func: fn(),
) {
    let timer_id = set_timer(Duration::from_secs(next_running_duration), move || {
        spawn(async move {
            // 这里是你的核心异步任务逻辑
            ic_cdk::println!("start schedule_next_tick");
            func();
            let next_running_duration = next_running_time - Duration::from_nanos(time()).as_secs();
            if duration < 0 {
                panic!("Failed to schedule next tick");
            }
            let next_running_time = next_running_time + duration;
            // 任务完成后，立即安排下一次执行
            schedule_next_tick(duration, next_running_duration, next_running_time, func);
        });
    });
}
