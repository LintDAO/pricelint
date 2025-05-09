use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use crate::impl_storable;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use std::borrow::Cow;
use ic_cdk::api::time;
use ic_cdk::caller;

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
impl_storable!(Wallet);
impl Default for Wallet {
    fn default() -> Self {
        Self{
            id: time().to_string(),
            holder: caller(),
            address: "".to_string(),
            from: "".to_string(),
            name: "".to_string(),
            principal_id: None,
            create_time: 0,
            transactions: 0,
            last_sync_time: 0,
            last_transaction_time: 0,
        }
    }
}