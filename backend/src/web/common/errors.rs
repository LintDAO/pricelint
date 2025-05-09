use std::error::Error;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::impl_error;

// 认证错误类型
#[derive(Serialize, Deserialize, Debug)]
pub enum AuthenticationError {
    AnonymousUser=1,
    Unauthorized=2,
    UnsupportedAuthenticationType=3,
}
impl_error!(AuthenticationError);



#[derive(Serialize, Deserialize, Debug)]
pub enum UserError {

}
impl_error!(UserError);

#[derive(Serialize, Deserialize, Debug)]
pub enum WalletError {

}
impl_error!(WalletError);


#[derive(Serialize, Deserialize, Debug)]
pub enum PredictorError {
    NotExistedPredictions=1,
    UnknownError =999
}
impl_error!(PredictorError);

#[derive(Serialize, Deserialize, Debug)]
pub enum GuardError {
    IsNotAdministrator,
    IsAnonymousUser,
}
impl_error!(GuardError);