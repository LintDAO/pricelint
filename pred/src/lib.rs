mod common;
mod models;
mod api;
mod services;

pub mod export_canid{
    use ic_cdk::export_candid;

    use crate::services::pred_service::Predictor;
    export_candid!();
    
}

