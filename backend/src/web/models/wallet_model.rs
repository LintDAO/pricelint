use candid::{CandidType, Principal};
use ic_cdk::caller;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,CandidType,Clone)]
pub struct Wallet {
    pub id: String,
    pub holder: Principal,
    pub address: String,
    pub from: String,
    pub name: String,
    pub principal_id: Option<Principal>,
    pub create_time: u64,
    pub transactions: u64, //transactions count
    pub last_sync_time: u64,
    pub last_transaction_time: u64,
}

impl Default for Wallet {
    fn default() -> Self {
        Wallet {
            holder: caller(),
            ..Default::default()
        }
    }
}