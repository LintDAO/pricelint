pub mod monitor_api {
    use crate::common::constants::memory_manager::EXPORT_MEMORY_IDS;
    use crate::common::guard::is_owner;
    use crate::common::lifecycle::{
        CanisterLog, Value, CANISTER_MONITOR, CONFIG, MEMORY_MANAGER, MODEL_MAP,
    };
    use ic_cdk::api::time;
    use ic_cdk::{api, call, caller, heartbeat, post_upgrade, pre_upgrade, query, update};
    use ic_stable_structures::memory_manager::MemoryId;
    use ic_stable_structures::Memory;
    use std::arch::wasm32;
    use std::fmt::format;

    const WASM_PAGE_SIZE: u64 = 65536;
    #[query(guard = "is_owner")]
    pub fn get_cansiter_info() -> Result<CanisterLog, String> {
        let cycles = ic_cdk::api::canister_balance();
        let version = ic_cdk::api::canister_version();
        let heap_size = unsafe { wasm32::memory_size::<0>() } as u64 * WASM_PAGE_SIZE;
        let stable_size = MEMORY_MANAGER.with_borrow(|rc| {
            let mut vec = Vec::new();
            for mem_id in EXPORT_MEMORY_IDS {
                let x = rc.get(MemoryId::new(*mem_id)).size() * WASM_PAGE_SIZE;
                vec.push(format!("mem_id:{},size:{} ;", mem_id, x));
            }
            vec
        });
        let record = format!(
            "version:{} , cycles:{},heap_memory:{},stable:{:?}",
            version, cycles, heap_size, stable_size
        );

        Ok(CanisterLog {
            time: time(),
            message: record,
        })
        //dfx canister status pred
    }
    #[query(guard = "is_owner")]
    pub fn find_last_info() -> Option<CanisterLog> {
        CANISTER_MONITOR.with_borrow(|rc| rc.iter().max_by_key(|(a, b)| *a).map(|(k, v)| v))
    }
    #[query(guard = "is_owner")]
    pub fn find_all_info() -> Vec<CanisterLog> {
        CANISTER_MONITOR.with_borrow(|rc| rc.iter().map(|(k, v)| v).collect())
    }

    #[update(guard = "is_owner")]
    pub fn record_canister_info() -> Result<(), String> {
        let info = get_cansiter_info()?;
        CANISTER_MONITOR
            .with_borrow_mut(|rc| {
                if (rc.len()) >= 1000 {
                    let (min, _) = rc.iter().min_by_key(|(a, b)| *a).unwrap();
                    rc.remove(&min);
                }
                rc.insert(time(),info);
            });
        Ok(())
    }
}

mod test {
    use crate::common::constants::canister_id::LOCAL_BACKEND_CANISTER_ID;
    use crate::common::lifecycle::{Value, CONFIG, MODEL_MAP};
    use crate::services::user_predict_service::predict_entity::{Pred, Prediction};
    use candid::Principal;
    use ic_cdk::api::call::CallResult;
    use ic_cdk::api::time;
    use ic_cdk::{api, call, caller, update};

    fn test_insert_data(t: String, v1: Vec<u8>) -> usize {
        MODEL_MAP.with_borrow_mut(|rc| rc.insert(t.to_string(), v1.clone()));
        CONFIG.with_borrow_mut(|rc| {
            rc.insert(t.to_string(), Value::Text(String::from_utf8(v1).unwrap()))
        });
        MODEL_MAP
            .with_borrow_mut(|rc| rc.get(&"test".to_string()))
            .unwrap()
            .len()
    }

    #[update]
    pub async fn test1() -> Result<Prediction, String> {
        //TODO:实际运行改成ic的canister_id
        let canister_id =
            Principal::from_text(LOCAL_BACKEND_CANISTER_ID).map_err(|e| e.to_string())?;
        let args = Prediction {
            id: "".to_string(),
            user_id: caller().to_text(),
            canister_id: api::id().to_string(),
            price: 10.0, //TODO:
            trend: None, //TODO:
            pred: Pred {
                staked: 0.0,
                up: 0.0,
                down: 0.0,
                trend: "".to_string(),
            }, //TODO:
            stake: (0.0, 0.0), //TODO:
            create_time: time(),
        };
        //TODO:default重新赋值
        let result: CallResult<(Result<Prediction, String>,)> =
            call(canister_id, "push_user_pred", (args,)).await;
        let (ret,) = result.map_err(|(r, e)| e.to_string())?;
        ret
    }
}
