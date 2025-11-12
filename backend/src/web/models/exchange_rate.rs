use serde::Serialize;
use crate::common::utils::xrc::ExchangeRate;

//代币 时间 共同作为唯一键
#[derive(Serialize, Deserialize, Debug, Clone, CandidType, Ord, PartialOrd, Eq, PartialEq)]
pub struct ExchangeRateRecordKey(pub String, pub u64);
//历史导入和xrc查询汇总
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct ExchangeRateRecord {
    pub symbol: String,
    pub xrc_data: Option<ExchangeRate>,
    pub exchange_rate: u64,
    pub time: u64,
}