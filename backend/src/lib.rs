#[macro_use]
extern crate candid;

use crate::impl_storable::{BackupRecord, ExchangeRateRecord, StringVec, TempMapValue, TempVecValue, WasmFile};
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
use ic_stable_structures::{BTreeMap, DefaultMemoryImpl, StableBTreeMap, StableVec, Storable};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cell::RefCell;
use std::clone::Clone;
use std::collections::HashMap;
use crate::ml::api::default_api::State;
use crate::web::common::constants::memory_manager::{BACKUP_DATA_MEMORY_ID, CANISTER_LIST_MEMORY_ID, EXCHANGE_RATE_MEMORY_ID, PREDICTOR_CONTEXT_MEMORY_ID, PREDICTOR_QUANTIFY_MEMORY_ID, ROLE_USER_TREE_MEMORY_ID, STAKE_MEMORY_ID, TEMP_MAP_MEMORY_ID, TEMP_VEC_MEMORY_ID, USER_CONTEXT_MEMORY_ID, WALLET_CONTEXT_MEMORY_ID, WASM_FILES_MEMORY_ID};
use crate::web::models::stake_model::Stake;

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



init_stable_memory!(MODEL_MAP,111,map<String, Vec<u8>>);
init_stable_memory!(STATE_MAP,112,map<String, State>);


init_stable_memory!(BACKUP_DATA,BACKUP_DATA_MEMORY_ID,map<u64,BackupRecord>);
//存储各种临时数据
init_stable_memory!(TEMP_VEC,TEMP_VEC_MEMORY_ID,vec<TempVecValue<String>>);
init_stable_memory!(TEMP_MAP,TEMP_MAP_MEMORY_ID,map<String,TempMapValue<String>>);

init_stable_memory!(USER_CONTEXT,USER_CONTEXT_MEMORY_ID,map<String, Context<User>>);
init_stable_memory!(WALLET_CONTEXT,WALLET_CONTEXT_MEMORY_ID,map<String, Context<Wallet>>);
init_stable_memory!(PREDICTOR_CONTEXT,PREDICTOR_CONTEXT_MEMORY_ID,map<String, Context<Predictor>>);
init_stable_memory!(ROLE_USER_TREE,ROLE_USER_TREE_MEMORY_ID,map<String, StringVec>);
init_stable_memory!(WASM_FILES,WASM_FILES_MEMORY_ID,map<String, WasmFile>);
init_stable_memory!(EXCHANGE_RATE,EXCHANGE_RATE_MEMORY_ID,vec<ExchangeRateRecord>);
init_stable_memory!(PREDICTOR_QUANTIFY,PREDICTOR_QUANTIFY_MEMORY_ID,vec<PredictorView>);
init_stable_memory!(STAKE,STAKE_MEMORY_ID,map<String,Stake>);
init_stable_memory!(CANISTER_LIST,CANISTER_LIST_MEMORY_ID,map<String,StringVec>);
// init_stable_memory!(XRC,XRC_MEMORY_ID,map<String,Vec<ExchangeRate>>);



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
    use crate::web::models::predictor_model::PredictorView;
    use crate::{impl_storable, Memory};
    use candid::{CandidType, Principal};
    use ic_stable_structures::storable::Bound;
    use ic_stable_structures::{StableVec, Storable};
    use serde::{Deserialize, Serialize};
    use std::borrow::Cow;
    use std::collections::BTreeMap;
    use crate::web::models::stake_model::Stake;
    use candid::{Decode, Encode};
    use crate::common::utils::xrc::{ExchangeRate};

    #[derive(Deserialize, Serialize, Clone, CandidType)]
    pub struct StringVec(pub Vec<String>);
    impl_storable!(StringVec);

    #[derive(Deserialize, Serialize, Clone, CandidType)]
    pub struct WasmFile {
        pub wasm_name: String,
        pub wasm_version: String,
        pub wasm_bin: Option<Vec<u8>>,
        pub upload_time: u64,
        pub update_type: UpdateType //功能性更新或者模型更新
    }
    #[derive(Deserialize, Serialize, Clone, CandidType)]
    #[derive(PartialEq)]
    pub  enum UpdateType {
        FunctionUpdate,
        ModelUpdate
    }
    impl_storable!(WasmFile);

    //历史导入和xrc查询汇总
    #[derive(CandidType, Serialize,Deserialize,Clone)]
    pub struct ExchangeRateRecord{
        pub symbol:String,
        pub xrc_data:Option<ExchangeRate>,
        pub exchange_rate:f64,
        pub time:u64,
    }



    impl_storable!(ExchangeRateRecord);
    impl_storable!(PredictorView);
    impl_storable!(Stake);

    #[derive(Serialize, Deserialize, Debug, Clone, CandidType)]
    pub enum TempMapValue<K: Ord, V = String> {
        Text(String),
        Number(u64),
        BtreeMap(BTreeMap<K, V>),
        Vector(Vec<V>),
    }
    #[derive(Serialize, Deserialize, Debug, Clone, CandidType)]
    pub enum TempVecValue<T> {
        Text(String),
        Number(u64),
        Vector(Vec<T>),
    }
    impl_storable!(TempMapValue<K,V>);
    impl_storable!(TempVecValue<T>);

     #[derive(Deserialize, Serialize, Clone, CandidType)]
    pub  struct  BackupRecord(pub String);
    impl Storable for BackupRecord {
        fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
            Cow::Owned(Encode!(self).unwrap())
        }
        fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
            Decode!(bytes.as_ref(), Self).unwrap()
        }
        const BOUND: Bound = Bound::Bounded {
            max_size: 100*1024*1024, //100m
            is_fixed_size: false,
        };
    }


}

pub mod export_candid {
    use crate::State;
    use crate::ml::api::default_api::PriceData;
    use candid::{CandidType,Nat, Deserialize, Principal};
    use crate::common::utils::time::DurationRange;
    use crate::common::utils::xrc::Asset;
    use crate::common::utils::xrc::ExchangeRate;
    use crate::web::models::predictor_model::{Predictor, PredictorView};
    use crate::web::models::user_model::User;
    use crate::WasmFile;
    use ic_cdk::{export_candid, query};
    use icrc_ledger_types::icrc1::transfer::{BlockIndex};
    use crate::web::api::stake_api::transfer_log::QueryBlocksResponse;
    use crate::web::models::stake_model::{ICRC2AllowanceResponse,ICRC1BalanceOfArgs,WithdrawArgs};
    use icrc_ledger_types::icrc2::{transfer_from::TransferFromArgs,allowance::AllowanceArgs,approve::ApproveArgs,approve::ApproveError};
    use icrc_ledger_types::icrc1::transfer::{TransferArg,Memo};
    use icrc_ledger_types::icrc1::account::Account;
    use icrc_ledger_types::icrc3::blocks::{GetBlocksResponse,GetBlocksRequest};
    use icrc_ledger_types::icrc3::transactions::{GetTransactionsResponse,GetTransactionsRequest};
    use crate::impl_storable::UpdateType;
    use crate::web::api::canister_api::backup_api::{HttpResponse,HttpRequest};
    use crate::impl_storable::ExchangeRateRecord;
    export_candid!();
}
//TODO: lifecycles和api canid 导出先写到一起  后续需要分canisters再进行重构分离
