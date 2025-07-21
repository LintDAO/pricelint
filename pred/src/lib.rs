mod common;
mod models;
mod api;
mod services;

pub mod export_canid{
    use ic_cdk::export_candid;
    use canistergeek_ic_rust::api_type::GetInformationRequest;
    use canistergeek_ic_rust::api_type::GetInformationResponse;
    use canistergeek_ic_rust::api_type::UpdateInformationRequest;
    use crate::services::pred_service::Predictor;
    export_candid!();
    
}

