use std::error::Error;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

// 认证错误类型
#[derive(Serialize, Deserialize, Debug)]
pub enum AuthenticationError {
    AnonymousUser=1,
    Unauthorized=2,
    UnsupportedAuthenticationType=3,
}
impl Display for AuthenticationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthenticationError:{:?}", self)
    }
}
impl Error for AuthenticationError {}
impl From<AuthenticationError> for std::fmt::Error {
    fn from(_err: AuthenticationError) -> std::fmt::Error {
        std::fmt::Error
    }
}

