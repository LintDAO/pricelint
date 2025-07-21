use crate::ml::api::default_api::init as init_ml;
use crate::web::common::guard::init_admin;
use burn::module::{DisplaySettings, ModuleDisplay};
use ic_cdk::api::time;
use ic_cdk::{block_on, init, post_upgrade, pre_upgrade, spawn};
use ic_cdk_timers::{set_timer, set_timer_interval};
use std::fmt;
use std::fmt::Debug;
use std::future::IntoFuture;
use std::time::Duration;
use tokio::runtime::Handle;
use crate::web::models::predictor_model::Predictor;
use crate::web::services::predictor_service::ExtendPredictorService;

#[init]
fn init() {
    init_ml();
    init_admin();
    spawn(async move {
        init_timer().await;
    })
}
#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("pre_upgrade:");
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("post_upgrade:");
    spawn(async move {
        init_timer().await;
    })
}
async fn init_timer() {
    const TASK_INTERVAL: u64 = 3600;
    // 获取当前时间戳（秒）
    let now = Duration::from_nanos(time()).as_secs();
    // 计算距离下一个整点的秒数
    let seconds_to_next_hour = TASK_INTERVAL - (now % TASK_INTERVAL);

    // 设置一次性定时器，到达下一个整点
    set_timer(Duration::from_secs(seconds_to_next_hour), || {
        // 先执行一次任务
        trigger_task();
        // 然后每小时执行一次
        set_timer_interval(Duration::from_secs(TASK_INTERVAL), trigger_task );
        //TODO: 时间校准为整时
    });
}

fn trigger_task() {
    ic_cdk::println!("trigger_task:{}", time());
    spawn(async move {
        // Predictor::autosave_predictor().await;
    })
}
