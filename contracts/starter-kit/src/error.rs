use thiserror::Error;

use cosmwasm_std::{CoinsError, StdError};

use crate::{
authenticator::AuthenticatorError,
counter::error::CounterError,
};
use cw_ownable::OwnershipError;

/// Never is a placeholder to ensure we don't return any errors
#[derive(Error, Debug)]
pub enum Never {}

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    CoinsError(#[from] CoinsError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Authenticator error: {0}")]
    AuthenticatorError(#[from] AuthenticatorError),

    #[error("Counter error: {0}")]
    CounterError(#[from] CounterError),

    #[error("Ownership error: {0}")]
    OwnershipError(#[from] OwnershipError),
}
