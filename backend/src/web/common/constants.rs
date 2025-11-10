pub const BASE_BIANCE_API: &str = "https://api.binance.com/api/";
pub const API_VERSION: &str = "v3";
pub const BIANCE_KLINES_API: &str = "klines";
pub const BIANCE_TICKER_API: &str = "ticker";

pub const ADMIN_ROLE_TAG: &str = "admin";
pub const OWNER_ROLE_TAG: &str = "owner";

pub const ICRC1_LEDGER_CANISTER_ID: &str = "tx6gn-wqaaa-aaaac-qbrma-cai";
pub const XRC_CANISTER_ID: &str = "uf6dk-hyaaa-aaaaq-qaaaq-cai";

pub mod memory_manager {

    pub const BACKUP_DATA_MEMORY_ID: u8 = 254;  //255特殊保留 使用了会报错

    
    pub const TEMP_VEC_MEMORY_ID: u8 = 0;
    pub const TEMP_MAP_MEMORY_ID: u8 = 1;
    pub const USER_CONTEXT_MEMORY_ID: u8 = 2;
    pub const WALLET_CONTEXT_MEMORY_ID: u8 = 3;
    pub const PREDICTOR_CONTEXT_MEMORY_ID: u8 = 4;
    pub const ROLE_USER_TREE_MEMORY_ID: u8 = 5;
    pub const WASM_FILES_MEMORY_ID: u8 = 6;
    pub const EXCHANGE_RATE_MEMORY_ID: u8 = 7;
    pub const PREDICTOR_QUANTIFY_MEMORY_ID: u8 = 8;
    pub const STAKE_MEMORY_ID: u8 = 9;
    pub const CANISTER_LIST_MEMORY_ID: u8 = 10;
    pub const RECORD_MEMORY_ID: u8 = 11;
    pub const STAKING_RECORD_MEMORY_ID: u8 = 12;
    
    pub const EXPORT_MEMORY_IDS: &[u8] = [
        TEMP_VEC_MEMORY_ID,
        TEMP_MAP_MEMORY_ID,
        USER_CONTEXT_MEMORY_ID,
        WALLET_CONTEXT_MEMORY_ID,
        PREDICTOR_CONTEXT_MEMORY_ID,
        ROLE_USER_TREE_MEMORY_ID,
        WASM_FILES_MEMORY_ID,
        EXCHANGE_RATE_MEMORY_ID,
        PREDICTOR_QUANTIFY_MEMORY_ID,
        STAKE_MEMORY_ID,
        CANISTER_LIST_MEMORY_ID,
        RECORD_MEMORY_ID,
        STAKING_RECORD_MEMORY_ID
    ]
    .as_slice();
}
