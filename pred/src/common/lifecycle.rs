use ic_cdk::{call, init, post_upgrade, pre_upgrade};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;
use candid::Principal;
use ic_cdk::api::call::CallResult;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
        static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =RefCell::new(MemoryManager::init(
        DefaultMemoryImpl::default()
    ));
        pub static MODEL_MAP: RefCell<StableBTreeMap<String, Vec<u8>, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
    ));
         pub static CONFIG: RefCell<StableBTreeMap<String, Value<String>, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

         pub static N: RefCell<StableBTreeMap<String, Value<String>, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value<K: Ord, V = String> {
    Text(String),
    BtreeMap(BTreeMap<K, V>),
    Vector(Vec<V>),
    // 可以添加更多变体
}
impl<K> Storable for Value<K>
where
    K: Ord + Clone + Serialize + for<'de> Deserialize<'de>,
{
    fn to_bytes(&self) -> Cow<[u8]> {
        let bytes = bincode::serialize(self).expect("Failed to serialize State");
        Cow::Owned(bytes)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        bincode::deserialize(&bytes).expect("Failed to deserialize State")
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 10_000_000,
        is_fixed_size: false,
    };
}
#[no_mangle]
unsafe extern "Rust" fn __getrandom_v03_custom(
    dest: *mut u8,
    len: usize,
) -> Result<(), getrandom::Error> {
    //TODO:  complete it
    Ok(())
}

#[init]
fn init() {}
#[pre_upgrade]
fn pre_upgrade_function() {
    let monitor_stable_data = canistergeek_ic_rust::monitor::pre_upgrade_stable_data();
    let logger_stable_data = canistergeek_ic_rust::logger::pre_upgrade_stable_data();
    ic_cdk::storage::stable_save((monitor_stable_data, logger_stable_data));
}

#[post_upgrade]
fn post_upgrade_function() {
    let stable_data: Result<(canistergeek_ic_rust::monitor::PostUpgradeStableData, canistergeek_ic_rust::logger::PostUpgradeStableData), String> = ic_cdk::storage::stable_restore();
    match stable_data {
        Ok((monitor_stable_data, logger_stable_data)) => {
            canistergeek_ic_rust::monitor::post_upgrade_stable_data(monitor_stable_data);
            canistergeek_ic_rust::logger::post_upgrade_stable_data(logger_stable_data);
        }
        Err(e) => {}
    }
}
//TODO: stable可能和ic-structure 内存冲突  并且导致dfx无法正常启动和升级
async fn push_pred()-> Result<(), String>{
    Ok(())
}