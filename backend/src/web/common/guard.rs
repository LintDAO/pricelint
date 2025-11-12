use crate::impl_storable::{StringVec, UserAffiliation};
use crate::web::common::errors::GuardError;
use crate::ROLE_USER_TREE;
use ic_cdk::caller;
use std::string::ToString;

pub fn band_role(user: UserAffiliation) {
    ROLE_USER_TREE.with_borrow_mut(|rc| match user {
        UserAffiliation::Administrator(principal) => {
            rc.insert(UserAffiliation::Administrator(principal));
        }
        UserAffiliation::NormalNamedUser(principal) => {
            rc.insert(UserAffiliation::NormalNamedUser(principal));
        }
        UserAffiliation::Anonymous(_) => {}
    })
}

pub fn is_admin() -> Result<(), String> {
    ROLE_USER_TREE.with_borrow_mut(|rc| {
        let current_user = caller().to_string();
        let is_admin = rc
            .iter()
            .filter(|k| matches!(k, UserAffiliation::Administrator(_)))
            .any(|k| matches!(k, UserAffiliation::Administrator(principal) if principal == current_user));
        if is_admin {
            return Ok(());
        }
        Err(GuardError::IsNotAdministrator.to_string())
    })
}

pub fn is_named_user() -> Result<(), String> {
    ROLE_USER_TREE.with_borrow_mut(|rc| {
        let current_user = caller().to_string();
        let is_admin = rc
            .iter()
            .filter(|k| matches!(k, UserAffiliation::NormalNamedUser(_)))
            .any(|k| matches!(k, UserAffiliation::NormalNamedUser(principal) if principal == current_user));
        if is_admin {
            return Ok(());
        }
        Err(GuardError::IsAnonymousUser.to_string())
    })
}

//  安装pred的canister代码后 记录canister id 只允许这些canister id调用
pub fn is_canister() -> Result<(), String> {
    let caller = caller();
    let bytes = caller.as_slice();
    let error = match bytes.last() {
        Some(0x01) => return Ok(()),
        Some(0x02) => "error:self-authenticating (user)",
        Some(0x04) => "error:anonymous",
        Some(0x7f) => "error:reserved",
        _ => "error:unknown authenticating",
    };
    Err(error.to_string())
}

pub fn init_admin() {
    let init_admin = vec![
        //dfx
        "vsqls-6k2en-jqrej-7dvmj-x27gn-e6bzr-asyr6-k7k6f-zl4xe-yykp4-uqe",
        "gq66f-io24f-torxu-ftmfi-chzvv-umdni-3jkai-s734u-wdzps-jmlkk-2ae",
        //nns/ii
        "y4qkv-s2rge-6ux5s-n7h2x-mvapy-cqc54-kjxmu-fgspq-3hpz3-ooqa3-pae",
    ];

    // 启动时候初始化管理员还有 用户列表  还有 bind role
    for admin in init_admin {
        //只是绑定了角色和初始化列表  具体的用户注册需要手动user_login api去手动注册 ，避免流程过于复杂
        band_role(UserAffiliation::Administrator(admin.to_string()));
    }
    //todo 将管理员添加在canisters的control人员里面里面 包括nns/ii/dfx身份
}
