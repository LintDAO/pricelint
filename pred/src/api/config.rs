pub mod config_entity {
    use crate::common::errors::SerializeError;
    use candid::CandidType;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::collections::BTreeMap;


    #[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
    pub struct TrainConfig {
        pub learning_rate: Option<f64>, //学习率，控制模型参数更新的步长
        pub batch_size: Option<usize>,  //批量大小，每次迭代使用的样本数量
        pub epochs: Option<usize>,  // 训练轮数，完整遍历数据集的次数
        pub optimizer: Option<String>, //优化器类型，如 "adam", "sgd", "rmsprop
        pub momentum: Option<f64>,  //动量参数，用于加速 SGD 优化器
        pub weight_decay: Option<f64>, //权重衰减（L2 正则化）系数
        pub advance_train_config: Option<AdvanceTrainConfig>
    }
    #[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
    pub struct AdvanceTrainConfig {
        pub dropout_rate:Option<f64>,	//	Dropout 比率，防止过拟合
        pub hidden_units:Option<Vec<usize>>,	//	隐藏层单元数量（对于神经网络）
        pub activation:Option<String>,	//	激活函数类型，如 "relu", "sigmoid", "tanh"
        pub loss_function:Option<String>,	//	损失函数类型，如 "cross_entropy", "mse"
        pub validation_split:Option<f64>,	//	验证集比例
        pub early_stopping_patience:Option<usize>,	//	早停耐心值，多少轮没有改进后停止训练

    }
    pub trait Config {
        fn struct_to_map(&self) -> Result<BTreeMap<String, Value>, String>;
    }
    impl<T: Serialize> Config for T {
        fn struct_to_map(&self) -> Result<BTreeMap<String, Value>, String> {
            let value = serde_json::to_value(self).map_err(|e| e.to_string())?;

            match value {
                Value::Object(map) => Ok(map.into_iter().collect()),
                _ => Err(SerializeError::SerializeFailed.to_string()),
            }
        }
    }
}

pub mod config_api {
    use crate::api::config::config_entity::{Config, TrainConfig};
    use crate::common::constants::config::{DEFAULT_MODEL_KEY, MODEL_PARAMETERS_KEY};
    use crate::common::guard::is_owner;
    use crate::common::lifecycle::{Value, CONFIG, MODEL_MAP};
    use ic_cdk::{query, update};
    use std::collections::BTreeMap;

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
    fn set_train_params(train_config: TrainConfig) -> Result<(), String> {
        let map = train_config.struct_to_map()?;
        let map = map
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<BTreeMap<String, String>>();
        CONFIG.with(|rc| {
            rc.borrow_mut()
                .insert(MODEL_PARAMETERS_KEY.to_string(), Value::BtreeMap(map));
        });
        Ok(())
    }
}
