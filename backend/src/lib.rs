use crate::impl_storable::{ExchangeRate, StringVec, WasmFile};
use crate::ml::api::default_api::State;
use crate::ml::model::{default_model, record};
use crate::web::models::context::Context;
use crate::web::models::predictor_model::{Predictor, PredictorView};
use crate::web::models::user_model::User;
use crate::web::models::wallet_model::Wallet;
use burn::backend::ndarray::{NdArray, NdArrayDevice};
use burn::backend::Autodiff;
use burn::record::{PrecisionSettings, Record, Recorder};
use burn::tensor::Tensor;
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

thread_local! {
    static RANDOM_BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}
// 全局内存管理器

thread_local! {
      static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =RefCell::new(MemoryManager::init(
        DefaultMemoryImpl::default()
    ));

      static MODEL_MAP: RefCell<StableBTreeMap<String, Vec<u8>, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
    ));

      static STATE_MAP: RefCell<StableBTreeMap<String, State, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)) )
    ));

      static USER_CONTEXT: RefCell<StableBTreeMap<String, Context<User>, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

      static WALLET_CONTEXT: RefCell<StableBTreeMap<String, Context<Wallet>, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

      static PREDICTOR_CONTEXT: RefCell<StableBTreeMap<String, Context<Predictor>, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));


     static ROLE_USER_TREE: RefCell<StableBTreeMap<String,  StringVec, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));

      static WASM_FILES: RefCell<StableBTreeMap<String,WasmFile, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
    ));
       static EXCHANGE_RATE: RefCell<StableBTreeMap<String,ExchangeRate, Memory>> = RefCell::new(StableBTreeMap::init(
       MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7)))
    ));
    
        static PREDICTOR_QUANTIFY: RefCell<StableVec<PredictorView, Memory>> = RefCell::new(StableVec::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(8)))
    ).expect("Couldn't get memory manager, can't initialize stable vec"));
}

//存储于内存的context上下文
thread_local! {
    //todo 备份
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
    use ic_cdk::{export_candid, query};
    use crate::common::utils::xrc::Asset;
    use crate::common::utils::xrc::ExchangeRate;
    use crate::ml::api::default_api::{PriceData, State};
    use crate::web::models::predictor_model::{Predictor, PredictorView};
    use crate::web::models::user_model::User;
    use crate::WasmFile;
    use crate::common::utils::time::DurationRange;
    export_candid!();
}
//TODO: lifecycles和api canid 导出先写到一起  后续需要分canisters再进行重构分离
