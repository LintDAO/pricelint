use crate::web::services::prediction_service::ExtendPredictorService;
use candid::{CandidType, Deserialize,};

pub mod prediction_api{
    use ic_cdk::{caller, query, update};
    use crate::PREDICTOR_CONTEXT;
    use crate::web::models::context::Context;
    use crate::web::models::prediction_model::{Pred, Prediction, PredictorResult, PredictorView};
    use crate::web::services::prediction_service::ExtendPredictorService;
    use crate::web::common::guard::is_canister;
    #[query]
    fn get_predictor_vec() -> Result<Vec<Prediction>, String> {
        todo!()
    }
    // 查询聚合数据
    //目前是请求一次查询一次 以后可能修改成从记录里定时查询数据
    #[query]
    fn show_predictions() -> Result<Vec<PredictorView>, String> {
        let accuracy = Prediction::get_accuracy_record(7);
        let total_stake = Prediction::get_total_stake();
        let t1 = PredictorResult {
            price: Some(12.1),
            trend: Some("up".to_string()),
            pred: Pred {
                staked: 22,
                up: 12,
                down: 10,
                trend: "up".to_string(),
            },
        };
        let mut view1 = PredictorView {
            id: "".to_string(),
            token_name: "ICPUSDT".to_string(),
            last_2: Some(t1.clone()),
            last_1: Some(t1.clone()),
            now: None,
            next: Some(t1.clone()),
            accuracy: 0.0,
            stake: (12.0, 13.0),
            create_time: 0,
        };
        let mut view2 = PredictorView {
            id: "".to_string(),
            token_name: "BTCUSDT".to_string(),
            last_2: Some(t1.clone()),
            last_1: Some(t1.clone()),
            now: None,
            next: Some(t1.clone()),
            accuracy: 0.0,
            stake: (11.0, 15.0),
            create_time: 0,
        };
        Ok(vec![view1, view2])
    }


}

pub mod exchange_rate_api {
    use crate::web::models::exchange_rate::{ExchangeRateRecord, ExchangeRateRecordKey};
    use crate::{ EXCHANGE_RATE};
    use ic_cdk::api::{canister_balance};
    use ic_cdk::{query, update};
    use std::borrow::Cow;

    // 导入历史数据
    // 导入大量数据的时候可能因为内存泄漏或者循环引用原因导致短期内增长大量的内存触发icp的机制导致panic
    // 如果触发panic则修改 Freezing threshold ,默认 2_592_000 Seconds ,内存短期过高可以降低Freezing threshold数值
    // dfx canister update-settings backend --freezing-threshold <seconds>
    #[update]
    pub fn import_history_records(
        symbol: String,
        history_data: Vec<(u64, f64)>,
    ) -> Result<(), String> {
        ic_cdk::println!("cycles: {}", canister_balance());

        // let mut data: Vec<(u64, f64)> =serde_json::from_slice(&history_data).map_err(|e| e.to_string())?;
        let mut vec_exchange_rate = history_data
            .iter()
            .map(|&(time, exchange_rate)| {
                let k = ExchangeRateRecordKey(symbol.clone(), time);
                let v = ExchangeRateRecord {
                    symbol: Cow::Borrowed(&symbol).into_owned(),
                    xrc_data: None,
                    exchange_rate: (exchange_rate * 10_f64.powf(8f64)) as u64,
                    time,
                };
                (k, v)
            })
            .collect::<Vec<_>>();

        EXCHANGE_RATE.with(|rc| {
            let mut map = rc.borrow_mut();
            for (k, v) in vec_exchange_rate.into_iter() {
                map.insert(k, v);
            }
        });
        Ok(())
    }

    //查询所有的数据 统计条数
    #[query]
    fn count_all_symbols() -> usize {
        EXCHANGE_RATE.with_borrow_mut(|rc| rc.iter().count())
    }
    //查询指定symbol的数据 统计条数
    #[query]
    fn count_by_symbol(symbol: String) -> usize {
        EXCHANGE_RATE.with_borrow_mut(|rc| rc.iter().filter(|(k, v)| v.symbol == symbol).count())
    }
    //查询所有的symbols
    #[query]
    fn find_all_symbols() -> std::collections::BTreeMap<String, Vec<ExchangeRateRecord>> {
        EXCHANGE_RATE.with_borrow_mut(|rc| {
            let mut map = std::collections::BTreeMap::new();
            for (k, v) in rc.iter() {
                map.entry(v.symbol.clone())
                    .or_insert_with(Vec::new)
                    .push(v.clone());
            }
            map
        })
    }

    //查询指定symbol的数据
    #[query]
    fn find_by_symbol(symbol: String) -> Vec<ExchangeRateRecord> {
        EXCHANGE_RATE.with_borrow(|rc| {
            rc.iter()
                .filter(|(_, v)| v.symbol == symbol)
                .map(|(_, v)| v.clone())
                .collect()
        })
    }
    //查询所有的symbol种类
    #[query]
    fn list_symbol_kind() -> std::collections::BTreeSet<String> {
        EXCHANGE_RATE.with_borrow_mut(|rc| {
            rc.iter()
                .map(|(k, v)| v.symbol.clone())
                .collect::<std::collections::BTreeSet<_>>()
        })
    }
}
