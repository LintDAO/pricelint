use crate::common::utils::xrc;
use crate::common::utils::xrc::{
    Asset, AssetClass, ExchangeRate, GetExchangeRateRequest, GetExchangeRateResult,
};
use crate::impl_storable::ExchangeRateRecord;
use crate::web::common::constants::{
    API_VERSION, BASE_BIANCE_API, BIANCE_KLINES_API, BIANCE_TICKER_API, XRC_CANISTER_ID,
};
use crate::web::models::context::Context;
use crate::web::models::predictor_model::Predictor;
use crate::{
    map_get, map_insert, Memory, EXCHANGE_RATE, PREDICTOR_CONTEXT, STAKE, STATE_MAP, USER_CONTEXT,
};
use candid::{Nat, Principal};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_cdk::api::time;
use ic_cdk::caller;
use lazy_static::lazy_static;
use proc_macro::{generate_service_impl, generate_service_trait};
use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Sum;
use std::ops::Deref;
use std::thread::LocalKey;
use urlencoding::encode;

generate_service_trait!(Predictor);
generate_service_impl!(Predictor, PREDICTOR_CONTEXT);

pub trait ExtendPredictorService: PredictorService {
    //get coins from other platform

    fn predictor_config();

    // call predictor results
    fn get_predictor_results() -> f32;
    fn get_last_pred(principal: Principal) -> Vec<Predictor>;
    fn get_accuracy() -> f64;

    fn get_total_stake() -> u64;
    async fn get_exchange_rate(
        base_asset: Asset,
        quote_asset: Asset,
    ) -> Result<ExchangeRate, String>;

    //每隔一段时间存储一条预测数据 并且调用的是用户的canisters
    async fn autosave_predictor() -> Result<(), String>;
    async fn autosave_exchange_rate() -> Result<(), String>;
    fn get_total_stake_24hours() -> f64;
    fn get_stake_growth_rate() -> f64;
}
impl ExtendPredictorService for Predictor {
    fn predictor_config() {
        todo!()
    }

    fn get_predictor_results() -> f32 {
        //todo 暂定如此  后续逻辑需修改 或者重写预测的具体过程和数据存储
        // predict()
        1.0
    }

    //获取最后两次数据 数据不足则获取一次
    fn get_last_pred(principal: Principal) -> Vec<Predictor> {
        let mut predictors: Vec<Predictor> = MAP.with(|map| {
            map.borrow()
                .iter()
                .filter(|(_, p)| p.owner.unwrap() == principal)
                .map(|(_, p)| p.context.unwrap())
                .collect()
        });

        predictors.sort_by(|a, b| b.create_time.cmp(&a.create_time));
        predictors.into_iter().take(2).collect() // 取最新的两个
    }

    fn get_accuracy() -> f64 {
        MAP.with(|map| {
            let mut borrowed_map = map.borrow_mut();
            //预测所有人的,总人数是所有已经预测了的,也就是 predictor.trend!="none"的
            let mut current_user_predictors = borrowed_map
                .iter()
                .filter(|(_, p)| p.clone().context.unwrap().trend.is_some());
            //预测正确的
            let true_count = current_user_predictors
                .by_ref()
                .filter(|(_, v)| {
                    let predictor = v.clone().context.unwrap();
                    //匹配已有实际结果的predictor历史 不匹配尚未预测的和正在预测的
                    if let Some(_) = predictor.trend {
                        //预测结果与实际结果相等
                        if predictor.trend.unwrap() == predictor.pred.trend {
                            return true;
                        }
                    }
                    return false;
                })
                .count();

            let total_count = current_user_predictors.count().clone();
            (true_count as f64 / total_count as f64)
        })
    }

    //获取当前所有用户质押的总金额
    //OK
    fn get_total_stake() -> u64 {
        STAKE.with_borrow_mut(|r| {
            let total_stake = r.iter().map(|(_, v)| {
                u64::try_from(&v.token_balance.0).unwrap()
            }).sum::<u64>();
           total_stake
        })
    }

    fn get_total_stake_24hours() -> f64 {
        0.0
    }
    fn get_stake_growth_rate() -> f64 {
        let growth_rate =
            (Self::get_total_stake() - Self::get_total_stake()) / Self::get_total_stake();
        0.0 * 100.0
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
            borrowed_rate.insert(ExchangeRateRecord {
                symbol: ICP_SYMBOL.to_string() + TARGET_SYMBOL,
                xrc_data: Some(icp_to_usdt.clone()),
                exchange_rate: icp_to_usdt.clone().rate,
                time: icp_to_usdt.clone().timestamp,
            });
            borrowed_rate.insert(ExchangeRateRecord {
                symbol: BTC_SYMBOL.to_string() + TARGET_SYMBOL,
                xrc_data: Some(btc_to_usdt.clone()),
                exchange_rate: btc_to_usdt.clone().rate,
                time: btc_to_usdt.clone().timestamp,
            });
        });

        Ok(())
    }
}
