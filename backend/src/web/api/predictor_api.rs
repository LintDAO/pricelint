use std::fmt::Error;
use crate::web::models::predictor_model::Predictor;
use crate::web::services::predictor_service::{ExtendPredictorService, PredictorService};
use ic_cdk::{query, update};
use ic_cdk::api::management_canister::http_request::{http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, TransformContext};
use crate::web::common::constants::{API_VERSION, BASE_BIANCE_API, BIANCE_KLINES_API};
use crate::web::common::errors::PredictorError;
use crate::web::common::errors::PredictorError::NotExistedPredictions;

#[query]
fn show_predictions() -> Result<Vec<Predictor>, String> {
    let vec = Predictor::find_all();
    match vec {
        None => Err(NotExistedPredictions.to_string()),
        Some(value) => Ok(value),
    }
}


#[update]
async fn test_http()->Result<String,String> {
    <Predictor as ExtendPredictorService>::get_coins_prices();
    let url = format! {"{}{}/{}?symbol={}&interval={}&limit=10", BASE_BIANCE_API, API_VERSION, BIANCE_KLINES_API,"BTCUSDT","1m"};
    ic_cdk::println!("{}",url);
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        headers: vec![HttpHeader { name: "Accept".to_string(), value: "application/json".to_string() }],
        body: None,
        max_response_bytes: Some(2000000),
        transform: None,
    };
    let cycles = 2_000_000_000; // 调整 Cycles 成本
    let (response,) = http_request(request, cycles).await.map_err(|e| format!("{:?}", e))?;
    String::from_utf8(response.body).map_err(|e| e.to_string())
}

