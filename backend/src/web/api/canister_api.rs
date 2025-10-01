use crate::impl_storable::WasmFile;

use crate::web::common::errors::BtreeMapError;
use crate::web::services::canister_service::canister_info;
use crate::WASM_FILES;
use candid::types::principal::PrincipalError;
use candid::{Error, Principal};
use ic_cdk::api::management_canister::main::uninstall_code;
use ic_cdk::{caller, id, query, update};
use std::fs::read;

// #[query]
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
            let mut ret=rc.borrow_mut()
                .iter()
                .filter(|(k, v)| v.update_type == update_type)
                .max_by(|(k, v), (k2, v2)| v.upload_time.cmp(&v2.upload_time))
                .map(|(k, v)| v.clone())
                .ok_or(BtreeMapError::GetKeyIsNotExist.to_string())?;
            //向前端隐藏此内容
            ret.wasm_bin=None;
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
