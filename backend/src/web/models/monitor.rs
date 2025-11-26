use candid::{CandidType, Principal};
use ic_cdk::caller;
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,CandidType,Clone)]
pub struct CanisterLog {
    pub(crate) time:u64,
    pub(crate) message: String,
}

