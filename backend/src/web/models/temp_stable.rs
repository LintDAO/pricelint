use std::collections::BTreeMap;
use serde::Serialize;

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