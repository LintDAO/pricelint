use std::hash::{DefaultHasher, Hash, Hasher};
use candid::{CandidType, Principal};
use ic_cdk::api::time;
use ic_cdk::caller;
use serde::{Deserialize, Serialize};
use crate::impl_storable;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use std::borrow::Cow;


#[derive(Serialize,Deserialize,Debug,CandidType,Clone)]
pub struct  User{
    pub id:String,
    pub owner: Principal ,
    pub name: String,
    pub create_time: u64,

}
pub struct UserProfile{
    pub user: User,
    pub ext:String
}
pub struct UserInformation{
    pub user: User,
    pub ext:String
}
impl User{
    fn new(id:String,owner: Principal, name: String,create_time:u64) -> Self {
        User { id,owner, name, create_time }
    }
}

impl Default for User{


    fn default() -> Self
    {
        let hasher = &mut DefaultHasher::new();
        (caller().to_text()+time().to_string().as_str()).hash(hasher);
        User{
            id:format!("{:}", hasher.finish()),
            owner: caller(),
            name: format!("{:x}",hasher.finish()),
            create_time: time(),
        }
    }
}



#[derive(Deserialize, Serialize, Clone, CandidType, Ord, PartialOrd, Eq, PartialEq)]
pub enum UserAffiliation  {
    Administrator(String),
    NormalNamedUser(String),
    Anonymous(String),
}