pub mod monitor_api {
    use crate::web::common::guard::is_named_user;
    use ic_cdk::api::time;
    use ic_cdk::{api, call, caller, heartbeat, post_upgrade, pre_upgrade, query, update};
    use ic_stable_structures::memory_manager::MemoryId;
    use ic_stable_structures::Memory;
    use std::arch::wasm32;
    use crate::{CANISTER_MONITOR, MEMORY_MANAGER};
    use crate::web::common::constants::memory_manager::EXPORT_MEMORY_IDS;
    use crate::web::models::monitor::CanisterLog;

    const WASM_PAGE_SIZE: u64 = 65536;
    #[query(guard = "is_named_user")]
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
    #[query(guard = "is_named_user")]
    pub fn find_last_info() -> Option<CanisterLog> {
        CANISTER_MONITOR.with_borrow(|rc| rc.iter().max_by_key(|(a, b)| *a).map(|(k, v)| v))
    }
    #[query(guard = "is_named_user")]
    pub fn find_all_info() -> Vec<CanisterLog> {
        CANISTER_MONITOR.with_borrow(|rc| rc.iter().map(|(k, v)| v).collect())
    }
}