use crate::common::utils::xrc;
use crate::common::utils::xrc::{
    Asset, AssetClass, ExchangeRate, GetExchangeRateRequest, GetExchangeRateResult,
};
use crate::impl_storable::{ExchangeRateRecord, ExchangeRateRecordKey};
use crate::web::common::constants::{
    API_VERSION, BASE_BIANCE_API, BIANCE_KLINES_API, BIANCE_TICKER_API, XRC_CANISTER_ID,
};
use crate::web::models::context::Context;
use crate::web::models::predictor_model::Predictor;
use crate::{
    map_get, map_insert, Memory, EXCHANGE_RATE, PREDICTOR_CONTEXT, STAKE, STAKING_RECORD,
    STATE_MAP, USER_CONTEXT,
};
use candid::{Nat, Principal};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_cdk::api::time;
use ic_cdk::caller;
use ic_stable_structures::StableBTreeSet;
use lazy_static::lazy_static;
use proc_macro::{generate_service_impl, generate_service_trait};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::iter::Sum;
use std::ops::Deref;
use std::thread::LocalKey;
use urlencoding::encode;

generate_service_trait!(Predictor);
generate_service_impl!(Predictor, PREDICTOR_CONTEXT);

pub trait ExtendPredictorService: PredictorService {
    //get coins from other platform

    fn predictor_config();

    fn get_accuracy() -> BTreeMap<String, f64>;

    fn get_total_stake() -> std::collections::BTreeMap<String, u64>;

    async fn get_exchange_rate(
        base_asset: Asset,
        quote_asset: Asset,
    ) -> Result<ExchangeRate, String>;

    //每隔一段时间存储一条预测数据 并且调用的是用户的canisters
    async fn autosave_predictor() -> Result<(), String>;

    async fn autosave_exchange_rate() -> Result<(), String>;

    fn get_real_stake_24hours() -> BTreeMap<String, u64>;

    fn get_real_stake_util_yesterday() -> BTreeMap<String, u64>;

    fn get_stake_growth_rate() -> BTreeMap<String, f64>;

    fn get_total_stake_util_yesterday() -> BTreeMap<String, u64>;
}
impl ExtendPredictorService for Predictor {
    fn predictor_config() {
        todo!()
    }



    fn get_accuracy() -> BTreeMap<String, f64> {
        let x:BTreeMap<String,f64>=BTreeMap::new();
        return x;
    }

    //获取当前所有用户质押的总金额(OK)
    fn get_total_stake() -> BTreeMap<String, u64> {
        STAKE.with_borrow_mut(|r| {
            r.iter()
                .fold(BTreeMap::new(), |mut acc, (_k, v)| {
                    if let Ok(stake) = u64::try_from(&v.token_balance.0) {
                        *acc.entry(v.stake_detail.token_name.clone()).or_insert(0) += stake;
                    }
                    acc
                })
                .into_iter()
                .collect()
        })
    }
    //获取到昨天24：00的质押实际金额(OK)
    fn get_real_stake_util_yesterday() -> BTreeMap<String, u64> {
        let yesterday_staking_token = STAKING_RECORD.with_borrow_mut(|rc| {
            let now = time();
            let seconds_per_day = 24 * 60 * 60;
            let yesterday_start = (now / seconds_per_day - 1) * seconds_per_day;
            let yesterday_end = yesterday_start + seconds_per_day - 1;
            let until_yesterday_end = rc.iter().filter(|(_, v)| {
                v.cost.is_none()
                    && v.reward.is_none()
                    && yesterday_start <= v.stake_time
                    && v.stake_time <= yesterday_end
            }).fold(BTreeMap::new(), |mut acc, (_k, v)| {
                *acc.entry(v.token_name).or_insert(0) +=v.amount;
                acc
            });
            until_yesterday_end
        });
        yesterday_staking_token
    }

    //24小时内质押的实际金额  ,从昨天24：00开始到现在的时间 (OK)
    fn get_real_stake_24hours() -> BTreeMap<String, u64> {
        STAKING_RECORD.with_borrow_mut(|rc| {
            let now = time();
            let seconds_per_day = 24 * 60 * 60;
            let yesterday_start = (now / seconds_per_day - 1) * seconds_per_day;
            let yesterday_end = yesterday_start + seconds_per_day - 1;
            let total_stake_24hours = rc.iter().filter(|(_, v)| {
                v.cost.is_none()
                    && v.reward.is_none()
                    && yesterday_end <= v.stake_time
                    && v.stake_time <= now
            }).fold(BTreeMap::new(), |mut acc, (_k, v)| {
                *acc.entry(v.token_name).or_insert(0) +=v.amount;
                acc
            });
            total_stake_24hours
        })
    }
    
    fn get_total_stake_util_yesterday() -> BTreeMap<String, u64> {
        Self::get_real_stake_util_yesterday()
    }
    //质押增长率 24小时计（OK)
    fn get_stake_growth_rate() -> BTreeMap<String, f64> {
        let yesterday_staking_token = Self::get_total_stake_util_yesterday();
        let now_staking_token = Self::get_total_stake();
        let mut stake_growth_rate = BTreeMap::new();
        for (token_name, now_stake) in now_staking_token.iter() {
            let yesterday_stake = yesterday_staking_token.get(token_name).unwrap_or(&0);
            let growth_rate = (*now_stake  - *yesterday_stake ) as f64
                / (*yesterday_stake as f64);
            stake_growth_rate.insert(token_name.clone(), growth_rate);
        }
        stake_growth_rate
    }

    //获取当前的汇率 从xrc canister获取
    //OK
    async fn get_exchange_rate(
        base_asset: Asset,
        quote_asset: Asset,
    ) -> Result<ExchangeRate, String> {
        let principal = Principal::from_text(XRC_CANISTER_ID).map_err(|e| e.to_string())?;
        let (ret,) = xrc::get_exchange_rate(
            principal,
            1_000_000_000,
            GetExchangeRateRequest {
                timestamp: None,
                base_asset,
                quote_asset,
            },
        )
        .await
        .map_err(|(r, e)| e.to_string())?;
        match ret {
            GetExchangeRateResult::Ok(v) => Ok(v),
            GetExchangeRateResult::Err(e) => Err(e.to_string()),
        }
    }

    //每隔一段时间存储一条预测数据 并且调用的是用户的canisters
    //Ok
    async fn autosave_predictor() -> Result<(), String> {
        //该内容已在用户canisters中实现
        //用户canisters定时推送数据
        Ok(())
    }

    //保存汇率到稳定内存
    //OK
    async fn autosave_exchange_rate() -> Result<(), String> {
        const ICP_SYMBOL: &str = "ICP";
        const BTC_SYMBOL: &str = "BTC";
        const TARGET_SYMBOL: &str = "USDT";
        let icp_to_usdt = Self::get_exchange_rate(
            Asset {
                class: AssetClass::Cryptocurrency,
                symbol: ICP_SYMBOL.to_string(),
            },
            Asset {
                class: AssetClass::FiatCurrency,
                symbol: TARGET_SYMBOL.to_string(),
            },
        )
        .await?;
        let btc_to_usdt = Self::get_exchange_rate(
            Asset {
                class: AssetClass::Cryptocurrency,
                symbol: BTC_SYMBOL.to_string(),
            },
            Asset {
                class: AssetClass::FiatCurrency,
                symbol: TARGET_SYMBOL.to_string(),
            },
        )
        .await?;

        //存储到稳定内存
        EXCHANGE_RATE.with(|rate| {
            let now = time();
            let mut borrowed_rate = rate.borrow_mut();
            borrowed_rate.insert(
                ExchangeRateRecordKey(
                    ICP_SYMBOL.to_string() + TARGET_SYMBOL,
                    icp_to_usdt.clone().timestamp,
                ),
                ExchangeRateRecord {
                    symbol: ICP_SYMBOL.to_string() + TARGET_SYMBOL,
                    xrc_data: Some(icp_to_usdt.clone()),
                    exchange_rate: icp_to_usdt.clone().rate,
                    time: icp_to_usdt.clone().timestamp,
                },
            );
            borrowed_rate.insert(
                ExchangeRateRecordKey(
                    BTC_SYMBOL.to_string() + TARGET_SYMBOL,
                    btc_to_usdt.clone().timestamp,
                ),
                ExchangeRateRecord {
                    symbol: BTC_SYMBOL.to_string() + TARGET_SYMBOL,
                    xrc_data: Some(btc_to_usdt.clone()),
                    exchange_rate: btc_to_usdt.clone().rate,
                    time: btc_to_usdt.clone().timestamp,
                },
            );
        });

        Ok(())
    }
}
