#![allow(dead_code, unused_imports)]
use ic_cdk::api::call::call_with_payment;
use candid::{self, CandidType, Decode, Encode,Principal};
use crate::impl_error;
use ic_cdk::api::call::CallResult;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter::Cycle;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone,Debug)]
pub enum AssetClass {
    Cryptocurrency,
    FiatCurrency,
}

#[derive(CandidType, Serialize, Deserialize, Clone,Debug)]
pub struct Asset {
    pub class: AssetClass,
    pub symbol: String,
}

#[derive(CandidType, Serialize,Deserialize)]
pub struct GetExchangeRateRequest {
    pub timestamp: Option<u64>,
    pub quote_asset: Asset,
    pub base_asset: Asset,
}

#[derive(CandidType, Serialize,Deserialize,Clone,Debug)]
pub struct ExchangeRateMetadata {
    pub decimals: u32,
    pub forex_timestamp: Option<u64>,
    pub quote_asset_num_received_rates: u64,
    pub base_asset_num_received_rates: u64,
    pub base_asset_num_queried_sources: u64,
    pub standard_deviation: u64,
    pub quote_asset_num_queried_sources: u64,
}



//xrc查询用
#[derive(CandidType, Serialize,Deserialize,Clone,Debug)]
pub struct ExchangeRate {
    pub metadata: ExchangeRateMetadata,
    pub rate: u64,
    pub timestamp: u64,
    pub quote_asset: Asset,
    pub base_asset: Asset,
}

#[derive(CandidType, Serialize,Deserialize,Debug)]
pub enum ExchangeRateError {
    AnonymousPrincipalNotAllowed,
    CryptoQuoteAssetNotFound,
    FailedToAcceptCycles,
    ForexBaseAssetNotFound,
    CryptoBaseAssetNotFound,
    StablecoinRateTooFewRates,
    ForexAssetsNotFound,
    InconsistentRatesReceived,
    RateLimited,
    StablecoinRateZeroRate,
    ForexInvalidTimestamp,
    NotEnoughCycles,
    ForexQuoteAssetNotFound,
    StablecoinRateNotFound,
    Pending,
}

#[derive(CandidType, Deserialize)]
pub enum GetExchangeRateResult {
    Ok(ExchangeRate),
    Err(ExchangeRateError),
}
impl_error!(ExchangeRateError);

pub async fn get_exchange_rate(
    canister_id: Principal,cycles: u64,
    arg0: GetExchangeRateRequest,
) -> CallResult<(GetExchangeRateResult,)> {
    call_with_payment(canister_id, "get_exchange_rate", (arg0,),cycles).await
}
