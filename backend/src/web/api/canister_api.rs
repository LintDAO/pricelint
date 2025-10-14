use crate::impl_storable::WasmFile;

use crate::web::common::errors::BtreeMapError;
use crate::web::services::canister_service::canister_info;
use crate::{BACKUP_DATA, TEMP_MAP, TEMP_VEC, WASM_FILES};
use candid::types::principal::PrincipalError;
use candid::{Error, Nat, Principal};
use ic_cdk::api::management_canister::main::uninstall_code;
use ic_cdk::{caller, id, query, update};
use ic_stable_structures::Storable;
use std::fs::read;

#[update]
async fn get_canister_info() -> Result<String, String> {
    let principal: String = "eov5t-niaaa-aaaah-arepa-cai".to_string();

    let canister_id = Principal::from_text(principal).map_err(|e| e.to_string())?;
    let user_principal = Principal::management_canister();

    let (ret,) = canister_info(user_principal, canister_id)
        .await
        .map_err(|(r, e)| format!("[rejectionCode]:{:?} ,[messages]:{}", r, e))?;
    let ret = ret.controllers.get(0).unwrap().to_string();
    //eov5t-niaaa-aaaah-arepa-cai
    Ok(ret)
}
pub mod backup_api {
    use crate::impl_storable::{
        BackupData, ExchangeRate, StringVec, TempMapValue, TempVecValue, WasmFile,
    };
    use crate::web::common::constants::memory_manager::EXPORT_MEMORY_IDS;
    use crate::web::common::errors::BtreeMapError;
    use crate::web::models::context::Context;
    use crate::web::models::predictor_model::{Predictor, PredictorView};
    use crate::web::models::stake_model::Stake;
    use crate::web::models::user_model::User;
    use crate::web::models::wallet_model::Wallet;
    use crate::{
        BACKUP_DATA, CANISTER_LIST, EXCHANGE_RATE, MEMORY_MANAGER, PREDICTOR_CONTEXT,
        PREDICTOR_QUANTIFY, ROLE_USER_TREE, STAKE, TEMP_MAP, TEMP_VEC, USER_CONTEXT,
        WALLET_CONTEXT, WASM_FILES,
    };
    use candid::{Nat, Principal};
    use ic_cdk::api::time;
    use ic_cdk::{query, update};
    use ic_stable_structures::memory_manager::MemoryId;
    use ic_stable_structures::Storable;
    use serde::Serialize;

    macro_rules! collect_memory_data {
        ($name:ident) => {
            $name.with(|rc| rc.borrow().iter().collect::<Vec<_>>())
        };
    }
    macro_rules! restore_from_data {
        ($name:ident,$data:expr,map) => {
            $name.with(|rc| {
                let mut bm = rc.borrow_mut();
                for data in $data {
                    bm.insert(data.0.clone(), data.1.clone());
                }
            })
        };
        ($name:ident,$data:expr,vec) => {
            $name.with(|rc| {
                let mut bm = rc.borrow_mut();
                for data in $data {
                    let _ = bm.push(&data);
                }
            })
        };

    }
    #[derive(Serialize, Deserialize)]
    struct ExportData {
        temp_vec_data: Vec<TempVecValue<String>>,
        temp_map_data: Vec<(String, TempMapValue<String>)>,
        user_context_data: Vec<(String, Context<User>)>,
        wallet_context_data: Vec<(String, Context<Wallet>)>,
        predictor_context_data: Vec<(String, Context<Predictor>)>,
        role_user_tree_data: Vec<(String, StringVec)>,
        wasm_files_data: Vec<(String, WasmFile)>,
        exchange_rate_data: Vec<(String, ExchangeRate)>,
        predictor_quantify_data: Vec<PredictorView>,
        stake_data: Vec<(String, Stake)>,
        canister_list_data: Vec<(String, StringVec)>,
    }

    //查询所有备份数据概览
    #[query]
    fn find_backup_lists() -> Vec<(u64, usize)> {
        let ret = BACKUP_DATA.with(|rc| {
            let bm = rc.borrow_mut();
            let ret = bm.iter().map(|(k, v)| (k, v.0.len())).collect::<Vec<_>>();
            ret
        });
        ret
    }

    //仅备份稳定内存的数据 手动触发或者pre_upgrade时自动
    #[update]
    fn backup_stable_memory() -> Result<(), String> {
        let temp_vec_data = collect_memory_data!(TEMP_VEC);
        let temp_map_data = collect_memory_data!(TEMP_MAP);
        let user_context_data = collect_memory_data!(USER_CONTEXT);
        let wallet_context_data = collect_memory_data!(WALLET_CONTEXT);
        let predictor_context_data = collect_memory_data!(PREDICTOR_CONTEXT);
        let role_user_tree_data = collect_memory_data!(ROLE_USER_TREE);
        let wasm_files_data = collect_memory_data!(WASM_FILES);
        let exchange_rate_data = collect_memory_data!(EXCHANGE_RATE);
        let predictor_quantify_data = collect_memory_data!(PREDICTOR_QUANTIFY);
        let stake_data = collect_memory_data!(STAKE);
        let canister_list_data = collect_memory_data!(CANISTER_LIST);
        let export_data = ExportData {
            temp_vec_data,
            temp_map_data,
            user_context_data,
            wallet_context_data,
            predictor_context_data,
            role_user_tree_data,
            wasm_files_data,
            exchange_rate_data,
            predictor_quantify_data,
            stake_data,
            canister_list_data,
        };
        let export_data_json = serde_json::to_string(&export_data).map_err(|e| e.to_string())?;
        BACKUP_DATA.with(|rc| rc.borrow_mut().insert(time(), BackupData(export_data_json)));
        Ok(())
    }

    //删除备份数据
    #[update]
    fn delete_backup_data(key: u64) -> bool {
        BACKUP_DATA.with(|rc| rc.borrow_mut().remove(&key).is_some())
    }
    //删除备份数据
    #[update]
    fn find_backup_data(key: u64) -> Option<String> {
        BACKUP_DATA.with(|rc| {
            let bm = rc.borrow_mut();
            if bm.contains_key(&key) {
                Some(bm.get(&key).unwrap().0.clone())
            } else {
                None
            }
        })
    }
    // 定义 HTTP 请求和响应类型
    pub type HeaderField = (String, String);
    #[derive(CandidType, Deserialize)]
    pub struct HttpRequest {
        pub method: String,
        pub url: String,
        pub headers: Vec<HeaderField>,
        pub body: Vec<u8>,
    }

    #[derive(CandidType, Deserialize)]
    pub struct HttpResponse {
        pub status_code: u16,
        pub headers: Vec<HeaderField>,
        pub body: Vec<u8>,
    }
    //导出稳定内存的数据到json文件
    #[query]
    fn dump_stable_memory(key: Option<u64>) -> HttpResponse {
        let ret = BACKUP_DATA.with(|rc| {
            let ref_mut = rc.borrow_mut();
            if key.is_none() {
                let last_element = ref_mut.iter().next_back();
                if last_element.is_none() {
                    return None;
                }
                return Some(last_element.unwrap().1.clone());
            }
            ref_mut.get(&key.unwrap())
        });
        let body = if ret.is_some() {
            ret.unwrap().to_bytes().to_vec()
        } else {
            vec![]
        };
        HttpResponse {
            status_code: 200,
            body,
            headers: vec![
                (
                    "Content-Type".to_string(),
                    "application/octet-stream".to_string(),
                ),
                (
                    "Content-Disposition".to_string(),
                    "attachment; filename=\"backup.json\"".to_string(),
                ),
            ],
        }
    }

    //从json文件中恢复稳定内存的数据
    #[update]
    fn restore_from_file(export_data_json: String) -> Result<(), String> {

        let export_data: ExportData = serde_json::from_str(&export_data_json).map_err(|e| e.to_string())?;
        //因为导入json的情况是手动触发，事先需要清空数据 所以不需要清空map和vec
        restore_from_data!(TEMP_VEC,export_data.temp_vec_data,vec);
        restore_from_data!(TEMP_MAP,export_data.temp_map_data,map);
        restore_from_data!(USER_CONTEXT,export_data.user_context_data,map);
        restore_from_data!(WALLET_CONTEXT,export_data.wallet_context_data,map);
        restore_from_data!(PREDICTOR_CONTEXT,export_data.predictor_context_data,map);
        restore_from_data!(ROLE_USER_TREE,export_data.role_user_tree_data,map);
        restore_from_data!(WASM_FILES,export_data.wasm_files_data,map);
        restore_from_data!(EXCHANGE_RATE,export_data.exchange_rate_data,map);
        restore_from_data!(PREDICTOR_QUANTIFY,export_data.predictor_quantify_data,vec);
        restore_from_data!(STAKE,export_data.stake_data,map);
        restore_from_data!(CANISTER_LIST,export_data.canister_list_data,map);
        Ok(())
    }
}

pub mod wasm_api {
    use crate::impl_storable::{UpdateType, WasmFile};
    use crate::web::common::errors::BtreeMapError;
    use crate::web::common::guard::is_admin;
    use crate::web::common::guard::is_named_user;
    use crate::WASM_FILES;
    use candid::MotokoResult::ok;
    use ic_cdk::api::time;
    use ic_cdk::{query, update};

    #[update(guard = "is_admin")]
    fn upload_wasm(
        name: String,
        version: String,
        wasm_bin: Vec<u8>,
        update_type: UpdateType,
    ) -> Result<String, String> {
        if wasm_bin.len() > 10_000_000 {
            return Err("File too large".to_string());
        }

        let ret = WASM_FILES.with(|rc| {
            let key = name.clone() + version.as_str();
            let wasm_file = WasmFile {
                wasm_name: name,
                wasm_version: version,
                wasm_bin: Some(wasm_bin),
                upload_time: time(),
                update_type,
            };
            let ret = rc.borrow_mut().insert(key, wasm_file.clone());

            if ret.is_some() {
                return format!(
                    "Update old wasm file,Name:{},Version:{}",
                    wasm_file.wasm_name, wasm_file.wasm_version
                );
            } else {
                return format!(
                    "Insert new wasm file,Name:{},Version:{}",
                    wasm_file.wasm_name, wasm_file.wasm_version
                );
            }
        });
        Ok(ret)
    }

    #[query(guard = "is_admin")]
    fn get_wasm_lists() -> Result<Vec<WasmFile>, String> {
        WASM_FILES.with(|rc| {
            let ret = rc
                .borrow_mut()
                .iter()
                .map(|(k, v)| WasmFile {
                    wasm_name: v.wasm_name.clone(),
                    wasm_version: v.wasm_version.clone(),
                    wasm_bin: None, //为了节约不显示具体bin 具体的直接通过get_wasm_bin获取
                    upload_time: v.upload_time.clone(),
                    update_type: v.update_type.clone(),
                })
                .collect();
            Ok(ret)
        })
    }
    #[update(guard = "is_admin")]
    fn delete_wasm(name: String, version: String) -> Result<WasmFile, String> {
        WASM_FILES.with(|rc| {
            let ret = rc
                .borrow_mut()
                .remove(&(name + version.as_str()))
                .ok_or(BtreeMapError::RemoveKeyIsNotExist.to_string())?;
            Ok(ret)
        })
    }

    #[query(guard = "is_named_user")]
    fn get_wasm_bin(name: String, version: String) -> Result<WasmFile, String> {
        WASM_FILES.with(|rc| {
            let key = name + version.as_str();
            let ret = rc
                .borrow_mut()
                .get(&key)
                .ok_or(BtreeMapError::GetKeyIsNotExist.to_string())?;
            Ok(ret)
        })
    }

    // 此版本根据上传的时间排序的 获取最后上传的
    // 此api只获取版本信息 不返回wasm_bin，获取wasm_bin使用get_wasm_bin api
    #[query(guard = "is_named_user")]
    fn get_latest_version(update_type: UpdateType) -> Result<WasmFile, String> {
        let latest_version: Result<WasmFile, String> = WASM_FILES.with(|rc| {
            let mut ret = rc
                .borrow_mut()
                .iter()
                .filter(|(k, v)| v.update_type == update_type)
                .max_by(|(k, v), (k2, v2)| v.upload_time.cmp(&v2.upload_time))
                .map(|(k, v)| v.clone())
                .ok_or(BtreeMapError::GetKeyIsNotExist.to_string())?;
            //向前端隐藏此内容
            ret.wasm_bin = None;
            Ok(ret)
        });
        latest_version
    }
}

pub mod canister_list {
    use crate::impl_storable::StringVec;
    use crate::web::common::errors::BtreeMapError::GetKeyIsNotExist;
    use crate::{impl_storable, CANISTER_LIST, TEMP_MAP};
    use candid::Principal;
    use ic_cdk::caller;

    pub struct CanisterParameter {
        pub canister_id: Principal,
        pub canister_name: String,
    }

    //从链上同步canister列表
    //前端同步
    pub fn sync_canister() {}
    pub fn add_canister(canister_id: String) -> Result<Vec<String>, String> {
        CANISTER_LIST.with(|map| {
            let string_vec = map.borrow_mut().get(&caller().to_text());
            let mut string_vec = string_vec.ok_or(GetKeyIsNotExist.to_string())?.0;
            string_vec.push(canister_id);
            map.borrow_mut()
                .insert(caller().to_text(), StringVec(string_vec.clone()));
            Ok(string_vec)
        })
    }
    pub fn find_canister_list() -> Result<Vec<String>, String> {
        CANISTER_LIST.with(|map| {
            let string_vec = map.borrow_mut().get(&caller().to_text());
            let string_vec = string_vec.ok_or(GetKeyIsNotExist.to_string())?.0;
            Ok(string_vec)
        })
    }

    //仅删除canister列表中的canister  实际操作前端agent调用api完成
    pub fn remove_canister(canister_id: String) -> Result<Vec<String>, String> {
        // CANISTER_LIST.with(|map | {
        //     let string_vec=map.borrow_mut().get(&caller().to_text());
        //     let mut  string_vec =string_vec.ok_or(GetKeyIsNotExist.to_string())?.0;
        //     string_vec.retain(|x| *x != canister_id);
        //     map.borrow_mut().insert(caller().to_text(), StringVec(string_vec.clone()));
        //     Ok(string_vec)
        // })
        Ok(vec![])
    }
}
