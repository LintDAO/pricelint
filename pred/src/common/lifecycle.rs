
use ic_cdk::{call, init, post_upgrade, pre_upgrade};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::Read;
use std::slice;
use candid::CandidType;

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
        static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =RefCell::new(MemoryManager::init(
        DefaultMemoryImpl::default()
    ));
}
init_stable_memory!(MODEL_MAP,0,map<String, Vec<u8>>);
init_stable_memory!(CONFIG,1,map<String, Value<String>>);



#[derive(CandidType,Serialize, Deserialize, Debug, Clone)]
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
use getrandom::Error;

#[no_mangle]
unsafe extern "Rust" fn __getrandom_v03_custom(
    dest: *mut u8,
    len: usize
) -> Result<(), Error> {


    Ok(())
}

#[init]
fn init() {}
#[pre_upgrade]
fn pre_upgrade_function() {

}

#[post_upgrade]
fn post_upgrade_function() {

}
//TODO: stable可能和ic-structure 内存冲突  并且导致dfx无法正常启动和升级
async fn push_pred()-> Result<(), String>{
    Ok(())
}