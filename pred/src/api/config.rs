use crate::common::constants::config::DEFAULT_MODEL_KEY;
use crate::common::lifecycle::{Value, CONFIG, MODEL_MAP};
use ic_cdk::{query, update};
use crate::common::guard::is_owner;
#[update(guard = "is_owner")]
pub fn set_model_version(model_version: String) -> Result<(), String> {
    CONFIG.with(|rc| {
        rc.borrow_mut()
            .insert(DEFAULT_MODEL_KEY.to_string(), Value::Text(model_version));
    });
    Ok(())
}

#[query(guard = "is_owner")]
pub fn find_model_lists() -> Result<Vec<String>, String> {
    let keys = MODEL_MAP.with(|m| m.borrow_mut().keys().collect());
    Ok(keys)
}

//设置模型参数 存入CONFIG 内存
#[update(guard = "is_owner")]
fn set_train_params() {}
