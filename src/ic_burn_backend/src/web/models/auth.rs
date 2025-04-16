// use std::fmt::Error as FmtError;
use std::error::Error;
use crate::web::common::errors::AuthenticationError;

pub enum AuthType{
    ICP(ICPAuthenticationType),
    ThirdPart(ThirdPartAuthenticationType)
}
pub enum ICPAuthenticationType {
    InternetIdentity,
    EmailAndPassword,
    NFID,
    IcAuth,
    PlugWallet
}
//以下工具可以使用来自其他链和生态系统的钱包地址对 ICP dapps 进行身份验证：
pub enum ThirdPartAuthenticationType{
    Bitcoin,
    Ethereum,
    Solana,
    MSQLibrary
}



pub trait Authentication<T> {
    // 1. 认证相关方法
    /// 验证凭证并返回用户信息
    fn authenticate(&self,t:T) -> Result<String,impl Error>;

    // /// 刷新认证令牌
    // fn refresh_token(&self, refresh_token: &str) -> Result<TokenPair, AuthError>;
    //
    // /// 注销当前会话
    // fn logout(&self, token: &str) -> Result<(), AuthError>;
    //
    // // 2. 用户状态检查方法
    // /// 检查用户是否已认证
    // fn is_authenticated(&self, token: &str) -> bool;
    //
    // /// 获取当前认证用户信息
    // fn get_current_user(&self, token: &str) -> Option<T>;
    //
    // // 3. 授权相关方法
    // /// 检查用户是否有特定权限
    // fn has_permission(&self, user: &T, permission: &str) -> bool;
    //
    // /// 检查用户是否有特定角色
    // fn has_role(&self, user: &T, role: &str) -> bool;
    //
    // // 4. 令牌管理
    // /// 验证令牌有效性
    // fn validate_token(&self, token: &str) -> Result<TokenClaims, AuthError>;
    //
    // /// 生成新令牌
    // fn generate_token(&self, user: &T) -> TokenPair;
}

