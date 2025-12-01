pub mod record_from_user {
    use crate::impl_storable::{StakeRecord, StakeRecordKey};
    use crate::web::common::errors::BtreeMapError::InsertMapError;
    use crate::web::models::prediction_model::{Prediction, PredictionKey};
    use crate::{PREDICTION, STAKING_RECORD};
    use ic_cdk::{caller, update};
    use crate::web::common::guard::is_canister;

    //用户推送预测结果到我们的canisters ,只允许安装了pred的功能的canisters调用此api
    //
    #[update(guard="is_canister")]
    pub fn prediction_record(prediction: Prediction) -> Result<Prediction, String> {
        ic_cdk::println!("caller:{}", caller().to_text());
        PREDICTION.with_borrow_mut(|rc| {
            let x = rc
                .insert(
                    PredictionKey(
                        prediction.clone().canister_id,
                        prediction.clone().create_time,
                        prediction.clone().token_name,
                    ),
                    prediction,
                )
                .ok_or(InsertMapError.to_string())?;
            Ok(x)
        })
    }

    #[update]
    pub fn staking_operation_record(staking_record: StakeRecord) -> Result<StakeRecord, String> {
        STAKING_RECORD.with_borrow_mut(|rc| {
            let x = rc
                .insert(
                    StakeRecordKey(
                        staking_record.clone().token_name,
                        staking_record.clone().account,
                        staking_record.clone().stake_time,
                    ),
                    staking_record.clone(),
                )
                .ok_or(InsertMapError.to_string())?;
            Ok(x)
        })
    }
}

pub mod user_query {
    use crate::common::utils::xrc::ExchangeRate;
    use crate::web::models::exchange_rate::ExchangeRateRecordKey;
    use crate::EXCHANGE_RATE;
    use ic_cdk::{query, update};
    use crate::web::common::guard::is_canister;
    
    #[update(guard="is_canister")]
    pub fn user_query_exchange(token_name: String, step: usize) -> Vec<(f32, f32)> {
        ic_cdk::println!("token_name:{},step:{}",token_name,step);
        EXCHANGE_RATE.with_borrow(|rc| {
            let mut collect_with_token = rc
                .iter()
                .filter(
                    |(k, _)| matches!(k, ExchangeRateRecordKey(token, _) if token.eq(&token_name)),
                )
                .collect::<Vec<_>>();

            collect_with_token.sort_by(|(k1, _), (k2, _)| {
                let ExchangeRateRecordKey(_, time1) = k1;
                let ExchangeRateRecordKey(_, time2) = k2;
                k1.cmp(&k2)
            });

            let final_data = collect_with_token
                .iter()
                .take(step)
                .map(|(k, v)| (v.time as f32, v.exchange_rate as f32))
                .collect::<Vec<_>>();
            final_data
        })
    }
}
