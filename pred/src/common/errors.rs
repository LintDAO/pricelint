use std::fmt::{Display, Formatter};
use std::error::Error;
use serde::Deserialize;
use serde::Serialize;
use crate::impl_error;



#[derive(Serialize, Deserialize, Debug)]
pub enum GuardError{
    IsNotCanisterController,
}

impl_error!(GuardError);


#[derive(Serialize, Deserialize, Debug)]
pub enum ConfigError{
    HasNotSetDefaultModel,
    ValueMatchPatternError
}

impl_error!(ConfigError);