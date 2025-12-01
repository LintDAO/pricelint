#[macro_use]
extern crate candid;

mod common;
mod models;
mod api;
mod services;

pub mod export_canid{
    use ic_cdk::export_candid;

    use crate::api::config::config_entity::TrainConfig;
    use crate::common::lifecycle::Value;
    use crate::models::lstm::v1::lstm_domain::LstmModelConfig;
    use crate::common::lifecycle::CanisterLog;
    use crate::api::predict_api::prediction_domain::Prediction;
    export_candid!();
    
}

