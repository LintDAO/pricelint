pub mod record_from_user {
    use crate::impl_storable::{StakeRecord, StakeRecordKey};
    use crate::web::common::errors::BtreeMapError::InsertMapError;
    use crate::web::models::prediction_model::{Prediction, PredictionKey};
    use crate::{PREDICTION, STAKING_RECORD};
    use ic_cdk::{caller, update};

    //用户推送预测结果到我们的canisters ,只允许安装了pred的功能的canisters调用此api
    //
    #[update]
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
