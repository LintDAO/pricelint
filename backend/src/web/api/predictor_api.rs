use crate::common::utils::url;
use crate::web::common::constants::{
    API_VERSION, BASE_BIANCE_API, BIANCE_KLINES_API, BIANCE_TICKER_API,
};
use crate::web::common::errors::PredictorError;
use crate::web::common::errors::PredictorError::{NotExistedPredictions, UnknownError};
use crate::web::models::predictor_model::{Predictor, PredictorView};
use crate::web::services::predictor_service::{ExtendPredictorService, PredictorService};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, TransformContext,
};
use ic_cdk::{caller, export_candid, query, update};
use std::fmt::Error;
use crate::{map_get, PREDICTOR_CONTEXT};
use crate::web::common::guard::is_admin;
use crate::web::common::guard::is_named_user;
#[query]
fn show_predictions() -> Result<Vec<Predictor>, String> {
    let vec = Predictor::find_all();
    match vec {
        None => Err(NotExistedPredictions.to_string()),
        Some(value) => Ok(value),
    }
}


const DEFAULT_CYCLES: u128 = 3_000_000_000;
const MAX_RESPONSE_BYTES: u64 = 2000000;
async fn get_coins_prices(symbols: &[&str], window_size: String) -> Result<String, String> {
    //todo 计算整5分钟/整小时的时间戳
    let url_str = format!("{}{}/{}?symbols=[{}]&windowSize={}",
        BASE_BIANCE_API, API_VERSION, BIANCE_TICKER_API, symbols.join(","), window_size
    );
    let url = url::url_encode(url_str);
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        headers: vec![HttpHeader {
            name: "Accept".to_string(),
            value: "application/json".to_string(),
        }],
        body: None,
        max_response_bytes: Some(MAX_RESPONSE_BYTES),
        transform: None,
    };
    let (response,) = http_request(request, DEFAULT_CYCLES)
        .await
        .map_err(|e| format!("{:?}", e))?;
    //todo 解析
    String::from_utf8(response.body).map_err(|e| e.to_string())
    //todo 根据open和close时间取余计算整小时的时间戳
}
#[query(guard="is_named_user")]
fn pred()->Result<PredictorView,String>{
    let mut view=PredictorView::default();
    view.now=None;
    //todo 完成质押代币  和准确率 的统计
    view.accuracy=0.5;
    view.stake=(1000.0,0.05);

    let pred_vec = Predictor::get_last_pred(caller());
    if pred_vec.len()<=0 {
        return Err(NotExistedPredictions.to_string());
    }else if pred_vec.len()==1 {
        view.last_1=Some(pred_vec.get(0).cloned().unwrap());
        view.last_2=None;
    }else if pred_vec.len()==2 {
        view.last_1=Some(pred_vec.get(0).cloned().unwrap());
        view.last_2=Some(pred_vec.get(1).cloned().unwrap());
    }else {
         return Err(UnknownError.to_string());
    }
    Ok(view)
}

//每隔一段时间存储一条预测数据 并且调用的是用户的canisters
fn autosave_predictor(){

}
