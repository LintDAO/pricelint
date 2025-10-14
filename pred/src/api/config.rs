pub mod config_entity {
    use crate::common::errors::SerializeError;
    use burn::data::dataloader::DataLoader;
    use candid::CandidType;
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};
    use serde_json::{from_value, Value};
    use std::collections::BTreeMap;

    #[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
    pub struct TrainConfig {
        pub learning_rate: Option<f64>, //学习率，控制模型参数更新的步长
        pub batch_size: Option<usize>,  //批量大小，每次迭代使用的样本数量
        pub epochs: Option<usize>,      // 训练轮数，完整遍历数据集的次数
        pub optimizer: Option<String>,  //优化器类型，如 "adam", "sgd", "rmsprop
        pub momentum: Option<f64>,      //动量参数，用于加速 SGD 优化器
        pub weight_decay: Option<f64>,  //权重衰减（L2 正则化）系数
        pub advance_train_config: Option<AdvanceTrainConfig>,
    }
    #[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
    pub struct AdvanceTrainConfig {
        pub dropout_rate: Option<f64>,              //	Dropout 比率，防止过拟合
        pub hidden_units: Option<Vec<usize>>,       //	隐藏层单元数量（对于神经网络）
        pub activation: Option<String>,             //	激活函数类型，如 "relu", "sigmoid", "tanh"
        pub loss_function: Option<String>,          //	损失函数类型，如 "cross_entropy", "mse"
        pub validation_split: Option<f64>,          //	验证集比例
        pub early_stopping_patience: Option<usize>, //	早停耐心值，多少轮没有改进后停止训练
    }
    pub trait Config<T, F> {
        fn struct_to_map(&self) -> Result<BTreeMap<String, Value>, String>;
        fn map_to_struct(map: BTreeMap<String, Value>) -> Result<F, String>;
    }
    impl<T, F> Config<T, F> for T
    where
        T: Serialize,
        F: DeserializeOwned,
    {
        fn struct_to_map(&self) -> Result<BTreeMap<String, Value>, String> {
            let value = serde_json::to_value(self).map_err(|e| e.to_string())?;

            match value {
                Value::Object(map) => Ok(map.into_iter().collect()),
                _ => Err(SerializeError::SerializeFailed.to_string()),
            }
        }
        fn map_to_struct(map: BTreeMap<String, Value>) -> Result<F, String> {
            let json_value = Value::Object(serde_json::Map::from_iter(
                map.iter().map(|(k, v)| (k.clone(), v.clone())),
            ));
            from_value(json_value).map_err(|e| e.to_string())
        }
    }
}
pub mod mian_config {
    use crate::api::config::config_entity;
    use crate::api::config::config_entity::{Config, TrainConfig};
    use crate::common::constants::config::{DEFAULT_MODEL_KEY, MODEL_PARAMETERS_KEY};
    use crate::common::guard::is_owner;
    use crate::common::lifecycle::{Value, CONFIG, MODEL_MAP};
    use ic_cdk::{call, query, update};
    use std::collections::BTreeMap;

    #[query(guard = "is_owner")]
    pub fn find_model_lists() -> Result<Vec<String>, String> {
        let keys = MODEL_MAP.with(|m| m.borrow_mut().keys().collect());
        Ok(keys)
    }

    //设置模型参数 存入CONFIG 内存
    #[update(guard = "is_owner")]
    fn set_train_params(train_config: TrainConfig) -> Result<(), String> {
        let map = <TrainConfig as Config<TrainConfig, TrainConfig>>::struct_to_map(&train_config)
            .map_err(|e| e.to_string())?;
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
pub mod default_model_config {
    use crate::common::constants::config::DEFAULT_MODEL_KEY;
    use crate::common::errors::ConfigError;
    use crate::common::guard::is_owner;
    use crate::common::lifecycle::{Value, CONFIG};
    use ic_cdk::{query, update};

    // #[update(guard = "is_owner")]
    // pub fn set_model_version(model_version: String) -> Result<(), String> {
    //     CONFIG.with(|rc| {
    //         rc.borrow_mut()
    //             .insert(DEFAULT_MODEL_KEY.to_string(), Value::Text(model_version));
    //     });
    //     Ok(())
    // }
    #[query(guard = "is_owner")]
    pub fn get_default_model() -> Result<String, String> {
        let default_model_name = CONFIG
            .with(|rc| rc.borrow_mut().get(&DEFAULT_MODEL_KEY.to_string()))
            .ok_or(ConfigError::HasNotSetDefaultModel.to_string())?;
        if let Value::Text(val) = default_model_name {
            Ok(val)
        } else {
            Err(ConfigError::ValueMatchPatternError.to_string())
        }
    }

    #[update(guard = "is_owner")]
    fn set_default_model(model_name: String) -> Result<Value<String>, String> {
        let default_model_name = CONFIG
            .with(|rc| {
                rc.borrow_mut()
                    .insert(DEFAULT_MODEL_KEY.to_string(), Value::Text(model_name))
            })
            .ok_or(ConfigError::SetDefaultModelError.to_string())?;
        Ok(default_model_name)
    }
}
pub mod predict_config {
    use crate::common::constants::config::{CURRENT_VERSION_KEY, FIVE_MIN_TIMER_INTERVAL, F_FLAG, ONE_HOUR_IMER_INTERVAL, PREDICT_FLAG_KEY, TIMER_INTERVAL_KEY, T_FLAG};
    use crate::common::guard::is_owner;
    use crate::common::lifecycle::{periodic_task, Value, CONFIG, TIMER_ID};
    use ic_cdk::{query, update};
    use std::time::Duration;
    use ic_cdk::api::management_canister::main::WasmModule;
    use ic_cdk_timers::clear_timer;

    #[update(guard = "is_owner")]
    pub async fn set_start_predict() -> Result<(), String> {

        CONFIG.with(|rc| {
            let mut bm=rc.borrow_mut();
            bm.insert(
                PREDICT_FLAG_KEY.to_string(),
                Value::Text(T_FLAG.to_string()),
            );
        });

        clear_timer(TIMER_ID.with(|t| t.borrow().clone()));
        //执行任务
        periodic_task();
        Ok(())
    }
    #[update(guard = "is_owner")]
    pub fn set_stop_predict() -> Result<(), String> {
        CONFIG.with(|rc| {
            rc.borrow_mut().insert(
                PREDICT_FLAG_KEY.to_string(),
                Value::Text(F_FLAG.to_string()),
            );
        });
        Ok(())
    }

    //前端需传递时间间隔interval  单位second
    #[update(guard = "is_owner")]
     pub async fn set_timer_interval(interval: u64) -> Result<(), String> {
        CONFIG.with(|rc| {
            match interval {
                ONE_HOUR_IMER_INTERVAL | FIVE_MIN_TIMER_INTERVAL => {
                    rc.borrow_mut()
                        .insert(TIMER_INTERVAL_KEY.to_string(), Value::Number(interval));
                }
                _ => {
                    //测试用10s 实际为1hour
                    rc.borrow_mut().insert(
                        TIMER_INTERVAL_KEY.to_string(),
                        Value::Number(ONE_HOUR_IMER_INTERVAL),
                    );
                }
            };
        });

        clear_timer(TIMER_ID.with(|t| t.borrow().clone()));
        //执任务
        periodic_task();
        Ok(())
    }
    #[query(guard = "is_owner")]
    pub fn is_predict_running() -> bool {
        let ret = CONFIG.with(|rc| rc.borrow_mut().get(&PREDICT_FLAG_KEY.to_string()));
        if ret.is_some() {
            match ret.unwrap() {
                Value::Text(val) => val == T_FLAG,
                _ => false,
            }
        } else {
            false
        }
    }

    //安装wasm后设置当前版本
    #[query(guard = "is_owner")]
    pub fn set_current_version(name:String,version:String)->(){
        CONFIG.with(|rc| {
            let ret = rc.borrow_mut().insert(CURRENT_VERSION_KEY.to_string(), Value::Tuple2(name,version));
        })
    }
    #[query(guard = "is_owner")]
    pub fn get_current_version()->Option<(String,String)>{
        CONFIG.with(|rc| {
            let ret = rc.borrow_mut().get(&CURRENT_VERSION_KEY.to_string());
            if ret.is_some(){
                if let Value::Tuple2(name,version)=ret.unwrap(){
                    return Some((name.clone(),version.clone()));
                }  else {
                    return None 
                }
            }
            return None
            
        })  
    }
}
