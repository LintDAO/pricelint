use crate::web::common::errors::AuthenticationError;
use crate::web::models::auth::{Authentication, ICPAuthenticationType};
use serde::{Deserialize, Serialize};
use std::fmt::Error as FmtError;
use std::error::Error ;
use candid::Principal;
use ic_cdk::query;
use serde::__private::de::IdentifierDeserializer;

// 用户数据结构
#[derive(Serialize, Deserialize, Clone, Debug)]
struct UserProfile {
    principal: String,
    auth_method: String,
    username: Option<String>,
    created_at: u64,
    last_login: u64,
}

impl Authentication<ICPAuthenticationType> for UserProfile {
    fn authenticate(&self, t: ICPAuthenticationType) -> Result<String, impl Error> {
        match t {
            ICPAuthenticationType::InternetIdentity => {
                Ok("".to_string())
            }
            _=>{
                Err(AuthenticationError::UnsupportedAuthenticationType)
            }
        }
    }
}
