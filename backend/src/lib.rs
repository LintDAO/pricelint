#[macro_use]
extern crate candid;

use crate::impl_storable::{
    BackupRecord, Record, RecordKey, StakeRecordKey, StringVec, UserAffiliation, WasmFile,
};
use crate::ml::api::default_api::State;
use crate::ml::model::{default_model, record};
use crate::web::common::constants::memory_manager::{BACKUP_DATA_MEMORY_ID, CANISTER_LIST_MEMORY_ID, CANISTER_MONITOR_MEMORY_ID, EXCHANGE_RATE_MEMORY_ID, PREDICTION_MEMORY_ID, PREDICTOR_CONTEXT_MEMORY_ID, RECORD_MEMORY_ID, ROLE_USER_TREE_MEMORY_ID, STAKE_MEMORY_ID, STAKING_RECORD_MEMORY_ID, TEMP_MAP_MEMORY_ID, TEMP_VEC_MEMORY_ID, USER_CONTEXT_MEMORY_ID, WASM_FILES_MEMORY_ID};
use crate::web::models::context::Context;
use crate::web::models::exchange_rate::{ExchangeRateRecord, ExchangeRateRecordKey};
use crate::web::models::prediction_model::{
    Prediction, PredictionHistory, PredictionKey, PredictorView,
};
use crate::web::models::stake_model::{Stake, StakeKey, StakeRecord};
use crate::web::models::temp_stable::{TempMapValue, TempVecValue};
use crate::web::models::user_model::User;
use crate::web::models::monitor::{CanisterLog};
use candid::{CandidType, Principal};
use ic_cdk::api::management_canister::main::raw_rand;
use ic_cdk::{export_candid, post_upgrade, pre_upgrade};
use ic_cdk_macros::{init, query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{
    BTreeMap, BTreeSet, DefaultMemoryImpl, StableBTreeMap, StableBTreeSet, StableVec, Storable,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::cell::RefCell;
use std::clone::Clone;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

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
    // StableVec 模式
    ($name:ident, $memory_id:expr, set<$V:ty>) => {
        thread_local! {
            static $name: RefCell<StableBTreeSet<$V, Memory>> = RefCell::new(
                StableBTreeSet::init(
                    MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new($memory_id)))
                )
            );
        }
    };
}

init_stable_memory!(MODEL_MAP,111,map<String, Vec<u8>>);
init_stable_memory!(STATE_MAP,112,map<String, State>);

init_stable_memory!(BACKUP_DATA,BACKUP_DATA_MEMORY_ID,map<u64,BackupRecord>);
//存储各种临时数据
init_stable_memory!(TEMP_VEC, TEMP_VEC_MEMORY_ID, vec<TempVecValue<String>>);
init_stable_memory!(TEMP_MAP,TEMP_MAP_MEMORY_ID,map<String,TempMapValue<String>>);
//用户基础数据
init_stable_memory!(USER_CONTEXT,USER_CONTEXT_MEMORY_ID,map<String, Context<User>>);
//日志
init_stable_memory!(CANISTER_MONITOR,CANISTER_MONITOR_MEMORY_ID,map<u64,CanisterLog>);

//暂时未使用
init_stable_memory!(PREDICTOR_CONTEXT,PREDICTOR_CONTEXT_MEMORY_ID,map<String, Context<Prediction>>);

//权限和角色
init_stable_memory!(ROLE_USER_TREE,ROLE_USER_TREE_MEMORY_ID,set<UserAffiliation>);
//wasm的存储
init_stable_memory!(WASM_FILES,WASM_FILES_MEMORY_ID,map<String, WasmFile>);
//历史价格和实时汇率
init_stable_memory!(EXCHANGE_RATE,EXCHANGE_RATE_MEMORY_ID,map<ExchangeRateRecordKey,ExchangeRateRecord>);
//预测单个结果
init_stable_memory!(PREDICTION,PREDICTION_MEMORY_ID,map<PredictionKey,Prediction>);
//质押
init_stable_memory!(STAKE,STAKE_MEMORY_ID,map<StakeKey,Stake>);
//用户拥有的canisters
init_stable_memory!(CANISTER_LIST,CANISTER_LIST_MEMORY_ID,map<String,StringVec>);
//存储预测各种结果数据   //1.预测的聚合数据 PredictorView 2.历史预测数据 Prediction 3.总的质押金额（不是实际参与质押的金额）
init_stable_memory!(RECORD,RECORD_MEMORY_ID,map<RecordKey,Record>);
//质押staking具体的操作记录
init_stable_memory!(STAKING_RECORD,STAKING_RECORD_MEMORY_ID,map<StakeRecordKey,StakeRecord>);

thread_local! {
    static RANDOM_BUFFER: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

mod common;
mod ml;
mod web;

pub mod impl_storable {
    use crate::common::utils::xrc::ExchangeRate;
    use crate::impl_storable;
    use crate::web::models::context::Context;
    use crate::web::models::exchange_rate::{ExchangeRateRecord, ExchangeRateRecordKey};
    use crate::web::models::prediction_model::{
        Prediction, PredictionHistory, PredictionKey, PredictorView,
    };
    pub(crate) use crate::web::models::record::{Record, RecordKey};
    pub(crate) use crate::web::models::stake_model::{Stake, StakeRecord, StakeRecordKey};
    pub(crate) use crate::web::models::temp_stable::{TempMapValue, TempVecValue};
    pub(crate) use crate::web::models::user_model::{User, UserAffiliation};
    use crate::web::models::monitor::{CanisterLog};
    pub(crate) use crate::web::models::wasm_file::WasmFile;
    use candid::{CandidType, Principal};
    use candid::{Decode, Encode};
    use ic_stable_structures::storable::Bound;
    use ic_stable_structures::Storable;
    use icrc_ledger_types::icrc1::account::Account;
    use serde::{Deserialize, Serialize};
    use std::borrow::Cow;
    use std::collections::BTreeMap;
    use crate::web::models::stake_model::StakeKey;

    #[derive(Deserialize, Serialize, Clone, CandidType)]
    pub struct StringVec(pub Vec<String>);
    impl_storable!(StringVec);
    #[derive(Deserialize, Serialize, Clone, CandidType)]
    pub struct BackupRecord(pub String);

    impl_storable!(Context<T>);
    impl_storable!(User);
    impl_storable!(UserAffiliation);
    impl_storable!(CanisterLog);
    impl_storable!(PredictionHistory);
    impl_storable!(Prediction);
    impl_storable!(PredictionKey);
    impl_storable!(WasmFile);
    impl_storable!(ExchangeRateRecord);
    impl_storable!(ExchangeRateRecordKey);
    impl_storable!(PredictorView);
    impl_storable!(StakeKey);
    impl_storable!(Stake);

    impl_storable!(TempMapValue<K,V>);
    impl_storable!(TempVecValue<T>);

    impl_storable!(BackupRecord, [100 * 1024 * 1024, false]);

    impl_storable!(StakeRecord);
    impl_storable!(StakeRecordKey);

    impl_storable!(Record);
    impl_storable!(RecordKey);
}

pub mod export_candid {
    use crate::common::utils::time::DurationRange;
    use crate::common::utils::xrc::Asset;
    use crate::common::utils::xrc::ExchangeRate;
    use crate::ml::api::default_api::PriceData;
    use crate::web::api::canister_api::backup_api::{HttpRequest, HttpResponse};
    use crate::web::api::stake_api::transfer_log::QueryBlocksResponse;
    use crate::web::models::exchange_rate::ExchangeRateRecord;
    use crate::web::models::prediction_model::{Prediction, PredictorView};
    use crate::web::models::stake_model::{
        ICRC1BalanceOfArgs, ICRC2AllowanceResponse, WithdrawArgs,Stake,StakeKey,StakeRecord,StakeRecordKey
    };
    use crate::web::models::user_model::User;
    use crate::web::models::monitor::CanisterLog;
    use crate::web::models::wasm_file::{UpdateType, WasmFile};
    use crate::State;
    use candid::{CandidType, Deserialize, Nat, Principal};
    use ic_cdk::{export_candid, query};
    use icrc_ledger_types::icrc1::account::Account;
    use icrc_ledger_types::icrc1::transfer::BlockIndex;
    use icrc_ledger_types::icrc1::transfer::{Memo, TransferArg};
    use icrc_ledger_types::icrc2::{
        allowance::AllowanceArgs, approve::ApproveArgs, approve::ApproveError,
        transfer_from::TransferFromArgs,
    };
    use icrc_ledger_types::icrc3::blocks::{GetBlocksRequest, GetBlocksResponse};
    use icrc_ledger_types::icrc3::transactions::{GetTransactionsRequest, GetTransactionsResponse};
    use std::collections::BTreeSet;
    use std::collections::BTreeMap;
    export_candid!();
}
//TODO: lifecycles和api canid 导出先写到一起  后续需要分canisters再进行重构分离
