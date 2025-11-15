mod common;
mod models;
mod api;
mod services;

pub mod export_canid{
    use ic_cdk::export_candid;

    use crate::services::user_predict_service::predict_entity::Predictor;
    use crate::api::config::config_entity::TrainConfig;
    use crate::common::lifecycle::Value;
    export_candid!();
    
}

