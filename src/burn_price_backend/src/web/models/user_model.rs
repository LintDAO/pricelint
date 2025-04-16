use std::fmt::format;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ptr::hash;
use candid::Principal;
use ic_cdk::api::time;
use ic_cdk::caller;
use rand::distr::Alphanumeric;
use rand::{random, rng, Rng};

pub struct  User{
    pub owner: Principal ,
    pub name: String,
    pub create_time: u64,

}
pub struct UserProfile{
    pub user: User,
    pub ext:String
}
pub struct UserInfomation{
    pub user: User,
    pub ext:String
}
impl User{
    fn new(owner: Principal, name: String,create_time:u64) -> Self {
        User { owner, name, create_time }
    }
}

impl Default for User{


    fn default() -> Self
    {
        let hasher = &mut DefaultHasher::new();
        (caller().to_text()+time().to_string().as_str()).hash(hasher);
        User{
            owner: caller(),
            name: format!("{:x}",hasher.finish()),
            create_time: time(),
        }
    }
}
