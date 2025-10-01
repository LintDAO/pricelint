use candid::CandidType;
use ic_cdk::{call, init, post_upgrade, pre_upgrade, spawn};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, Log, StableBTreeMap, StableLog, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::BTreeMap;
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
    ($name:ident,$event:ident,log<index:$index_mem:tt,data:$data_mem:tt>) => {
        thread_local! {
            pub static $name: RefCell<Log<$event,Memory,Memory>> = RefCell::new(
                Log::init(
                    MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new($index_mem))),
                    MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new($data_mem)))
                ).expect("Failed to initialize StableLog")
            );
        }
    };
}

thread_local! {
        static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =RefCell::new(MemoryManager::init(
        DefaultMemoryImpl::default()
    ));
    pub static TIMER_ID: RefCell<TimerId> = RefCell::new(TimerId::default());
}
init_stable_memory!(MODEL_MAP,0,map<String, Vec<u8>>);
init_stable_memory!(CONFIG,1,map<String, Value<String>>);
init_stable_memory!(LOG,String,log<index:2, data:3>);

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum Value<K: Ord, V = String> {
    Text(String),
    Number(u64),
    BtreeMap(BTreeMap<K, V>),
    Vector(Vec<V>), // 可以添加更多变体
}
impl<K> Storable for Value<K>
where
    K: Ord + Clone + Serialize + for<'de> Deserialize<'de>,
{
    fn to_bytes(&self) -> Cow<[u8]> {
        let bytes = bincode::serialize(self).expect("Failed to serialize object");
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        bincode::deserialize(&bytes).expect("Failed to deserialize enum object")
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 10_000_000,
        is_fixed_size: false,
    };
}
use crate::api::config::config_entity::Config;
use crate::common::constants::config::{
    FIVE_MIN_TIMER_INTERVAL, ONE_HOUR_IMER_INTERVAL, PREDICT_FLAG_KEY, TIMER_INTERVAL_KEY, T_FLAG,
};
use crate::services::pred_service::predict_entity::Predictor;
use crate::services::pred_service::predict_service::push_predictor_to_backend;
use getrandom::Error;
use ic_cdk::api::time;
use ic_cdk_timers::{set_timer, set_timer_interval, TimerId};

#[no_mangle]
unsafe extern "Rust" fn __getrandom_v03_custom(dest: *mut u8, len: usize) -> Result<(), Error> {
    Ok(())
}

#[init]
fn init() {}
#[pre_upgrade]
fn pre_upgrade_function() {}

#[post_upgrade]
fn post_upgrade_function() {}

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
            //分钟
            let now = Duration::from_nanos(time()).as_secs();
            // 计算距离下一个整点的秒数
            let next_running_duration = duration - (now % duration);
            let next_running_time=now+next_running_duration;
            schedule_next_tick(duration,next_running_duration,next_running_time);
            
            // // 设置一次性定时器，到达下一个整点
            // let timer=set_timer(Duration::from_secs(seconds_to_next_hour), move || {
            //     task_list();
            //     ic_cdk::println!("start set_timer");
            //     // 先执行一次任务
            //     set_timer_interval(Duration::from_secs(duration), task_list);
            // });
        }
    })
}
fn schedule_next_tick(duration: u64,next_running_duration:u64,next_running_time:u64) {
    // 设置一个10秒后触发的一次性定时器
    let timer_id = set_timer(Duration::from_secs(next_running_duration), move|| {
        spawn(async move {
            // 这里是你的核心异步任务逻辑
            ic_cdk::println!("start schedule_next_tick");
            task_list();
            
            let next_running_duration=next_running_time-Duration::from_nanos(time()).as_secs();
            if duration<0 {
                panic!("Failed to schedule next tick");
            }
            let next_running_time=next_running_time+duration;
            // 任务完成后，立即安排下一次执行
            schedule_next_tick(duration,next_running_duration, next_running_time);
        });
    });

    // 存储定时器ID
    TIMER_ID.with(|timer_id_cell| {
        *timer_id_cell.borrow_mut() = timer_id;
    });
}

pub fn task_list() -> () {
    ic_cdk::println!("running task_list");
    spawn(async move {
        let x = push_pred().await;
    });
}
async fn push_pred() -> Result<(), String> {
    //推送预测结果
    ic_cdk::println!("running push_pred");
    let p = push_predictor_to_backend().await?;
    Ok(())
}
