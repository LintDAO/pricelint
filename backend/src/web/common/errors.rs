use std::error::Error;
use std::fmt::{Display, Formatter};
use candid::CandidType;
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
    CreateUserFailed,
    UserIsNotExist,
    RegisterUserHasExist,
    UserIsNotFound,
}
impl_error!(UserError);

#[derive(Serialize, Deserialize, Debug)]
pub enum WalletError {
    GetICPBalanceFailed,
    GetCyclesFailed,
    UnknownError
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
    UnknownEmptyAdminLists,
    UnknownEmptyUserLists,
}
impl_error!(GuardError);


#[derive(Serialize, Deserialize, Debug)]
pub enum CanisterError {
    CreateCanisterFailed,
    InsufficientCycles,
}
impl_error!(CanisterError);

#[derive(Serialize, Deserialize, Debug)]
pub enum BtreeMapError{
    GetKeyIsNotExist,
    RemoveKeyIsNotExist,
    InsertMapError,
    
}
impl_error!(BtreeMapError);
