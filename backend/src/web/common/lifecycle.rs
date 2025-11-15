use crate::common::utils::time::NANOS_PER_SEC;
use crate::web::common::guard::init_admin;
use crate::web::services::prediction_service::autosave;
use ic_cdk::api::time;
use ic_cdk::{init, post_upgrade, pre_upgrade, spawn};
use ic_cdk_timers::set_timer;
use std::time::Duration;
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
    let duration_15m = NANOS_PER_SEC * 60 * 15;
    let duration_60m = NANOS_PER_SEC * 60 * 60;
    let duration_1d = duration_60m * 24;
    let now = time();
    schedule_next_tick(
        duration_15m,
        duration_15m - (now % duration_15m),
        now + (duration_15m - (now % duration_15m)),
        schedule_tasklists_15m,
    );
    schedule_next_tick(
        duration_60m,
        duration_60m - (now % duration_60m),
        now + (duration_60m - (now % duration_60m)),
        schedule_tasklists_60m,
    );
    schedule_next_tick(
        duration_1d,
        duration_1d - (now % duration_1d),
        now + (duration_1d - (now % duration_1d)),
        schedule_tasklists_1d,
    );
}

fn schedule_tasklists_15m() {
    ic_cdk::println!("schedule_tasklists_15m:{}", time());
    autosave::autosave_stake_amount();
    autosave::autosave_prediction_history();
    spawn(async move {
        let exchange_error=autosave::autosave_exchange_rate().await;
    })
}
fn schedule_tasklists_60m() {
    ic_cdk::println!("schedule_tasklists_60m:{}", time());
    spawn(async move {})
}

fn schedule_tasklists_1d() {
    ic_cdk::println!("schedule_tasklists_1d:{}", time());
    autosave::autosave_predict_accuracy();
    spawn(async move {})
}
// 计划下一个任务执行时间
fn schedule_next_tick(
    duration: u64,
    next_running_duration: u64,
    next_running_time: u64,
    func: fn(),
) {
    let timer_id = set_timer(Duration::from_nanos(next_running_duration), move || {
        ic_cdk::println!("now {}, next_running_time:{} ", time(), next_running_time);
        func();
        let now = time(); // 在闭包内部获取当前时间
                          // 基于当前时间重新计算下一次执行时间
        let next_running_duration = duration - (now % duration);
        let next_running_time = now + next_running_duration;
        schedule_next_tick(duration, next_running_duration, next_running_time, func);
    });
}
