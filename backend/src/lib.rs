use crate::impl_storable::{ExchangeRate, StringVec, WasmFile};
use crate::ml::model::{default_model, record};
use crate::web::models::context::Context;
use crate::web::models::predictor_model::{Predictor, PredictorView};
use crate::web::models::user_model::User;
use crate::web::models::wallet_model::Wallet;
use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::main::raw_rand;
use ic_cdk::{export_candid, post_upgrade, pre_upgrade};
use ic_cdk_macros::{init, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableVec, Storable};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cell::RefCell;
use std::clone::Clone;

type Memory = VirtualMemory<DefaultMemoryImpl>;
// 全局内存管理器
thread_local! {
      static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =RefCell::new(MemoryManager::init(
        DefaultMemoryImpl::default()
    ));
}
macro_rules! init_stable_memory {
    // StableBTreeMap 模式
    ($name:ident, $memory_id:expr, map<$K:ty, $V:ty>) => {
        thread_local! {
            static $name: RefCell<StableBTreeMap<$K, $V, Memory>> = RefCell::new(
                StableBTreeMap::init(
                    MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new($memory_id)))
                )
            );
        }
    };

    // StableVec 模式
    ($name:ident, $memory_id:expr, vec<$V:ty>) => {
        thread_local! {
            static $name: RefCell<StableVec<$V, Memory>> = RefCell::new(
                StableVec::init(
                    MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new($memory_id)))
                ).expect("Failed to initialize StableVec")
            );
        }
    };
}



init_stable_memory!(MODEL_MAP,0,map<String, Vec<u8>>);
// init_btree_map!(STATE_MAP,1,<String, State>);
init_stable_memory!(USER_CONTEXT,2,map<String, Context<User>>);
init_stable_memory!(WALLET_CONTEXT,3,map<String, Context<Wallet>>);
init_stable_memory!(PREDICTOR_CONTEXT,4,map<String, Context<Predictor>>);
init_stable_memory!(ROLE_USER_TREE,5,map<String, StringVec>);
init_stable_memory!(WASM_FILES,6,map<String, WasmFile>);
init_stable_memory!(EXCHANGE_RATE,7,map<String, ExchangeRate>);
init_stable_memory!(PREDICTOR_QUANTIFY,8,vec<PredictorView>);



//存储于内存的context上下文
thread_local! {
    //todo 备份
}
thread_local! {
    static RANDOM_BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}


mod common;
mod ml;
mod web;

pub mod impl_storable {
    pub(crate) use crate::common::utils::xrc::ExchangeRate;
    use crate::web::models::predictor_model::PredictorView;
    use crate::{impl_storable, Memory};
    use candid::{CandidType, Principal};
    use ic_stable_structures::storable::Bound;
    use ic_stable_structures::{StableVec, Storable};
    use serde::{Deserialize, Serialize};
    use std::borrow::Cow;
    use std::collections::BTreeMap;

    #[derive(Deserialize, Serialize, Clone)]
    pub struct StringVec(pub Vec<String>);
    impl_storable!(StringVec);

    #[derive(Deserialize, Serialize, Clone, CandidType)]
    pub struct WasmFile {
        pub wasm_name: String,
        pub wasm_version: String,
        pub wasm_bin: Vec<u8>,
    }
    impl_storable!(WasmFile);
    impl_storable!(ExchangeRate);
    impl_storable!(PredictorView);
}

pub mod export_candid {
    use crate::common::utils::time::DurationRange;
    use crate::common::utils::xrc::Asset;
    use crate::common::utils::xrc::ExchangeRate;
    use crate::web::models::predictor_model::{Predictor, PredictorView};
    use crate::web::models::user_model::User;
    use crate::WasmFile;
    use ic_cdk::{export_candid, query};
    export_candid!();
}
//TODO: lifecycles和api canid 导出先写到一起  后续需要分canisters再进行重构分离
