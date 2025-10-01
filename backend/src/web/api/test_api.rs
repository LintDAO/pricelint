use candid::Nat;
use ic_cdk::api::time;
use crate::common::utils::time::{get_time_range, DurationRange};
use crate::common::utils::hash::{hash,hash_salt};
use ic_cdk::{caller, query};

#[query]
pub fn test_1(d:DurationRange)-> (u64,u64)  {
    let x=1_000_000_000*60*60*24;
    let (start, end) = get_time_range(time(),d.clone());
    ic_cdk::println!("{} {}",start,end);
    let (start, end) = get_time_range((time()/x-1)*x,DurationRange::Days);
    ic_cdk::println!("{} {}",start,end);
    (start,end)
}


pub fn test_hash(){
    ic_cdk::println!("hash:{}",hash("hello".to_string()));
    ic_cdk::println!("hash_salt:{}",hash_salt("hello".to_string(),"123".to_string()));
}

// test
//btc记账罐子   mxzaz-hqaaa-aaaar-qaada-cai
// 后端本地罐子 uxrrr-q7777-77774-qaaaq-cai
// backend ic      eov5t-niaaa-aaaah-arepa-cai
//代币记账本     tx6gn-wqaaa-aaaac-qbrma-cai

//本地用户 default  jgpx7-55xpz-cslhk-rtpf3-3zsj2-sxu2e-yk5p2-gynvo-clo6g-gh5s2-bqe
//本地用户ic    vsqls-6k2en-jqrej-7dvmj-x27gn-e6bzr-asyr6-k7k6f-zl4xe-yykp4-uqe

// NNS       y4qkv-s2rge-6ux5s-n7h2x-mvapy-cqc54-kjxmu-fgspq-3hpz3-ooqa3-pae

// dfx identity get-principal



