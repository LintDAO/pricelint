use candid::types::internal::type_of;
use candid::types::TypeId;
use ic_cdk::api::time;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn hash_salt<T>(t:T,salt:String) -> String {
    hash::<String>(stringify!(t).to_owned() + salt.as_str())
}

pub fn hash<T>(t:T) -> String {
    let hasher = &mut DefaultHasher::new();
    let str = stringify!(t);
    str.hash(hasher);
    format!("{:x}", hasher.finish())
}
//todo:  uuid
