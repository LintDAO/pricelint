use candid::CandidType;
use ic_cdk::{call, init, post_upgrade, pre_upgrade, spawn};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{
    DefaultMemoryImpl, Log, StableBTreeMap, StableLog, StableVec, Storable,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::io::Read;
use std::slice;
use std::time::Duration;

type Memory = VirtualMemory<DefaultMemoryImpl>;
macro_rules! init_stable_memory {
    // StableBTreeMap 模式
    ($name:ident, $memory_id:expr, map<$K:ty, $V:ty>) => {
        thread_local! {
            pub static $name: RefCell<StableBTreeMap<$K, $V, Memory>> = RefCell::new(
                StableBTreeMap::init(
                    MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new($memory_id)))
                )
            );
        }
    };

    // StableVec 模式
    ($name:ident, $memory_id:expr, vec<$V:ty>) => {
        thread_local! {
           pub static $name: RefCell<StableVec<$V, Memory>> = RefCell::new(
                StableVec::init(
                    MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new($memory_id)))
                ).expect("Failed to initialize StableVec")
            );
        }
    };
}

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    pub static TIMER_ID: RefCell<TimerId> = RefCell::new(TimerId::default());
}
init_stable_memory!(MODEL_MAP,MODEL_MAP_MEMORY_ID,map<String, Vec<u8>>);
init_stable_memory!(CONFIG,CONFIG_MEMORY_ID,map<String, Value<String>>);
init_stable_memory!(CANISTER_MONITOR,CANISTER_MONITOR_MEMORY_ID,map<u64,CanisterLog>);

#[derive(Serialize, candid::Deserialize, Debug, Clone, candid::CandidType)]
pub enum Value<K: Ord, V = String> {
    Text(String),
    Number(u64),
    Tuple1(V),
    Tuple2(V, V),
    Tuple3(V, V, V),
    BtreeMap(std::collections::BTreeMap<K, V>),
    Vector(Vec<V>), // 可以添加更多变体
}
#[derive(CandidType, Deserialize, Debug, Serialize, Clone)]
pub struct CanisterLog {
    pub time: u64,
    pub message: String,
}
impl_storable!(Value<K,V>);
impl_storable!(CanisterLog);

use crate::api::config::config_entity::Config;
use crate::api::monitor::monitor_api::record_canister_info;
use crate::common::constants::config::{
    FIVE_MIN_TIMER_INTERVAL, ONE_HOUR_IMER_INTERVAL, PREDICT_FLAG_KEY, TIMER_INTERVAL_KEY, T_FLAG,
};
use crate::common::constants::duration::NANOS_PER_SEC;
use crate::common::constants::memory_manager::{
    CANISTER_MONITOR_MEMORY_ID, CONFIG_MEMORY_ID, MODEL_MAP_MEMORY_ID,
};
use crate::impl_storable;
use crate::services::user_predict_service::predict_entity::Prediction;
use crate::services::user_predict_service::predict_service::push_to_backend;
use getrandom::Error;
use ic_cdk::api::time;
use ic_cdk_timers::{set_timer, set_timer_interval, TimerId};

#[no_mangle]
unsafe extern "Rust" fn __getrandom_v03_custom(dest: *mut u8, len: usize) -> Result<(), Error> {
    Ok(())
}

#[init]
fn init() {
    // init_timer();
}
#[pre_upgrade]
fn pre_upgrade_function() {}

#[post_upgrade]
fn post_upgrade_function() {
    init_timer();
}

fn schedule_tasklists_15m() {
    ic_cdk::println!("schedule_tasklists_15m:{}", time());
}
fn schedule_tasklists_60m() {
    ic_cdk::println!("schedule_tasklists_60m:{}", time());
    spawn(async move {})
}

fn schedule_tasklists_1d() {
    ic_cdk::println!("schedule_tasklists_1d:{}", time());
    let ret = record_canister_info();
}
fn init_timer() {
    const NANOS_PER_SEC: u64 = 1_000_000_000;
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
//需要执行的所有任务
pub fn periodic_task() -> () {
    CONFIG.with(|rc| {
        let is_running = rc.borrow().get(&PREDICT_FLAG_KEY.to_string());
        if is_running.is_some() {
            let x = is_running.unwrap();
            if let Value::Text(x) = x {
                if x != T_FLAG {
                    ic_cdk::println!("{}", "is not running periodic_task, task is stop");
                    return;
                } else {
                    ic_cdk::println!("{}", "running periodic_task");
                }
            }
        } else {
            ic_cdk::println!("{}", "is not running periodic_task,stable is none");
            return;
        }
        let mut value = rc.borrow_mut().get(&TIMER_INTERVAL_KEY.to_string());
        if value.is_none() {
            rc.borrow_mut().insert(
                TIMER_INTERVAL_KEY.to_string(),
                Value::Number(ONE_HOUR_IMER_INTERVAL),
            );
            value = Some(Value::Number(ONE_HOUR_IMER_INTERVAL));
        };
        if let Value::Number(duration) = value.unwrap() {
            ic_cdk::println!("duration:{:?}", duration);
            let duration = duration * NANOS_PER_SEC;
            //分钟
            let now = time();
            // 计算距离下一个整点的秒数
            let next_running_duration = duration - (now % duration);
            let next_running_time = now + next_running_duration;
            schedule_next_tick(
                duration,
                next_running_duration,
                next_running_time,
                task_list,
            );
        }
    })
}
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
pub fn task_list() -> () {
    ic_cdk::println!("running task_list");
    spawn(async move {
        let x = push_to_backend().await;
    });
}
