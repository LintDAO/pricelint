use crate::common::utils::xrc;
use crate::common::utils::xrc::{
    get_exchange_rate, Asset, ExchangeRate, GetExchangeRateRequest, GetExchangeRateResult,
};
use crate::web::common::constants::{
    API_VERSION, BASE_BIANCE_API, BIANCE_KLINES_API, BIANCE_TICKER_API,
};
use crate::web::common::errors::PredictorError;
use crate::web::common::errors::PredictorError::{NotExistedPredictions, UnknownError};
use crate::web::common::guard::is_admin;
use crate::web::common::guard::{is_canister, is_named_user};
use crate::web::models::context::Context;
use crate::web::models::predictor_model::{Pred, Predictor, PredictorResult, PredictorView};
use crate::web::services::predictor_service::{ExtendPredictorService, PredictorService};
use crate::{map_get, PREDICTOR_CONTEXT, PREDICTOR_QUANTIFY};
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::call::CallResult;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, TransformContext,
};
use ic_cdk::api::time;
use ic_cdk::{api, call, caller, export_candid, query, update};
use std::fmt::Error;

#[query]
fn get_predictor_vec() -> Result<Vec<Predictor>, String> {
    let vec = Predictor::find_all();
    match vec {
        None => Err(NotExistedPredictions.to_string()),
        Some(value) => Ok(value),
    }
}

// 查询自己的数据+公共数据
#[query(guard = "is_named_user")]
fn show_predictions() -> Result<PredictorView, String> {
    let mut view = PredictorView {
        id: format!("{}{}", caller(), time()),
        last_2: None,
        last_1: None,
        now: None, // TODO:前端
        next: None,
        accuracy: Predictor::get_accuracy(),
        stake: (
            Predictor::get_total_stake(),
            Predictor::get_stake_growth_rate(),
        ),
        create_time: time(),
    };

    //todo 完成质押代币

    let view_vec = PREDICTOR_QUANTIFY.with(|map| {
        map.borrow()
            .iter()
            .map(|p| p.clone())
            .collect::<Vec<PredictorView>>()
    });
    if view_vec.len() <= 0 {
        return Err(NotExistedPredictions.to_string());
    } else if view_vec.len() == 1 {
        view.last_1 = Some(view_vec.get(0).unwrap().last_1.clone().unwrap());
        view.last_2 = None;
    } else if view_vec.len() >= 2 {
        if let [.., last3, last2, last1] = view_vec.as_slice() {
            if last1.next.is_none() {
                //如果最后一个数据next为空 则说明尚未开始预测 则历史数据最新两条为last_2和last_3 , last_1为正在预测或者尚未预测的数据
                view.last_1 = Some(last2.last_1.clone().unwrap());
                view.last_2 = Some(last3.last_2.clone().unwrap());
                view.next = Some(PredictorResult {
                    price: Some(1.0),
                    trend: Some("up".to_string()),
                    pred: Pred {
                        staked: 1.0,
                        up: 1.0,
                        down: 1.0,
                        trend: "up".to_string(),
                    },
                })
            } else {
                //next 不为空  查看
            }
            //TODO: pop旧的 push新的
        }
    }
    Ok(view)
}

//用户推送预测结果到我们的canisters ,只允许安装了pred的功能的canisters调用此api
#[update(guard = "is_canister")]
async fn push_user_pred(predictor: Predictor) -> Result<Predictor, String> {
    ic_cdk::println!("caller:{}", caller().to_text());
    PREDICTOR_CONTEXT.with(|map| {
        let mut borrowed_map = map.borrow_mut();
        let mut ctx = Context::default();
        ctx.context = Some(predictor.clone());
        borrowed_map.insert(predictor.clone().id, ctx);
    });
    Ok(predictor)
}

pub mod exchange_rate_api {
    use crate::impl_storable::ExchangeRateRecord;
    use crate::EXCHANGE_RATE;
    use candid::MotokoResult::ok;
    use ic_cdk::{query, update};

    #[update]
    pub fn import_history_records(
        symbol: String,
        history_datas: Vec<(u64, f64)>,
    ) -> Result<(), String> {
        let history_vec: Vec<ExchangeRateRecord> = history_datas
            .iter()
            .map(|(t, x)| ExchangeRateRecord {
                symbol: symbol.clone(),
                time: *t,
                exchange_rate: *x,
                xrc_data: None,
            })
            .collect::<_>();

        EXCHANGE_RATE.with(|rc| {
            for record in history_vec.iter() {
                let bm = rc.borrow_mut();
                bm.push(record).unwrap();
            }
        });

        Ok(())
    }
    #[query]
    pub fn find_exchange_rates(symbol:String,start_time:u64,end_time:u64) -> Result<Vec<ExchangeRateRecord>, String> {
        let records = EXCHANGE_RATE.with(|rc| {
            let bm = rc.borrow();
            bm.iter().filter(|x| x.symbol == symbol && x.time >= start_time && x.time <= end_time)
                .collect::<_>()
        });
        Ok(records)
    }
}
