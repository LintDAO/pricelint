use ic_cdk::init;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;

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
    //TODO  complete it
    Ok(())
}

#[init]
fn init() {}
