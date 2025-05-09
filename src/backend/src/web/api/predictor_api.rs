use crate::web::models::predictor_model::Predictor;
use crate::web::services::predictor_service::PredictorService;
use ic_cdk::{query, update};
use crate::web::common::errors::PredictorError::NotExistedPredictions;

#[query]
fn show_predictions() -> Result<Vec<Predictor>, String> {
    let vec = Predictor::find_all();
    match vec {
        None => Err(NotExistedPredictions.to_string()),
        Some(value) => Ok(value),
    }
}
