use ic_cdk::api::time;
use crate::common::utils::time::{get_time_range, DurationRange};
use ic_cdk::query;

#[query]
pub fn test_1(d:DurationRange)-> (u64,u64)  {
    let x=1_000_000_000*60*60*24;
    let (start, end) = get_time_range(time(),d.clone());
    ic_cdk::println!("{} {}",start,end);
    let (start, end) = get_time_range((time()/x-1)*x,DurationRange::Days);
    ic_cdk::println!("{} {}",start,end);
    (start,end)
}

