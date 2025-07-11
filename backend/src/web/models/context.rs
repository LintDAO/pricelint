use candid::{CandidType, Principal};
use ic_cdk::api::time;
use ic_cdk::caller;
use std::hash::{DefaultHasher, Hash, Hasher};
use burn::tensor::T;
use serde::{Deserialize, Serialize};
use crate::impl_storable;
use crate::web::models::predictor_model::Predictor;
use crate::web::models::user_model::User;
use crate::web::models::wallet_model::Wallet;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use std::borrow::Cow;

#[derive(Serialize,Deserialize,Debug,Clone,CandidType)]
pub struct Context<T> {
    pub id: Option<String>,
    pub context: Option<T>,
    pub owner: Option<Principal>,
    pub create_time: Option<u64>,
}

impl<T> Context<T> {
    pub fn new(c: T) -> Self {
        let mut context = Context::default();
        context.context = Some(c);
        context
    }
    pub fn default() -> Self {
        let user: String = caller().to_text();
        let hasher = &mut DefaultHasher::new();
        let salt = time() % 2 + 1;
        (user + salt.to_string().as_str()).hash(hasher);
        Context {
            id: Some(format!("{:x}", hasher.finish())),
            context: None,
            owner: Some(caller()),
            create_time: Some(time()),
        }
    }
    fn get_context<'a>(&'a self) -> &'a Option<T> {
        &self.context
    }
}

impl_storable!(Context<T>);