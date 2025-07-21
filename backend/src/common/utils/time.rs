use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize,CandidType,Clone)]
pub enum DurationRange {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days
}
const NANOS_PER_MICRO: u64 = 1_000;
const NANOS_PER_MILLI: u64 = 1_000_000;
const NANOS_PER_SEC: u64 = 1_000_000_000;
const NANOS_PER_MINUTE: u64 = 60 * NANOS_PER_SEC;
const NANOS_PER_HOUR: u64 = 60 * NANOS_PER_MINUTE;
const NANOS_PER_DAY: u64 = 24 * NANOS_PER_HOUR;

//TODO: timezone时区问题 
pub fn get_time_range(timestamp:u64,range: DurationRange) -> (u64, u64) {
    let now = timestamp;
    let (start, end) = match range {
        DurationRange::Nanoseconds => {
            (now , now)
        },
        DurationRange::Microseconds => {
            (now/NANOS_PER_MICRO*NANOS_PER_MICRO, (now/NANOS_PER_MICRO+1)*NANOS_PER_MICRO-1)
        },
        DurationRange::Milliseconds => {
            (now/NANOS_PER_MILLI*NANOS_PER_MILLI, (now/NANOS_PER_MILLI+1)*NANOS_PER_MILLI-1)
        },
        DurationRange::Seconds => {
            (now/NANOS_PER_SEC*NANOS_PER_SEC, (now/NANOS_PER_SEC+1)*NANOS_PER_SEC-1)
        },
        DurationRange::Minutes => {
            (now/NANOS_PER_MINUTE*NANOS_PER_MINUTE, (now/NANOS_PER_MINUTE+1)*NANOS_PER_MINUTE-1)
        },
        DurationRange::Hours => {
            (now/NANOS_PER_HOUR*NANOS_PER_HOUR, (now/NANOS_PER_HOUR+1)*NANOS_PER_HOUR-1)
        },
        DurationRange::Days => {
            (now/NANOS_PER_DAY*NANOS_PER_DAY, (now/NANOS_PER_DAY+1)*NANOS_PER_DAY-1)
        }
    };
    (start, end)
}
