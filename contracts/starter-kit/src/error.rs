use thiserror::Error;

use crate::{authenticator::AuthenticatorError, counter::error::CounterError};
use cosmwasm_std::{CoinsError, StdError};
use cw3::DepositError;
use cw_ownable::OwnershipError;
use cw_utils::{PaymentError, ThresholdError};

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

    // cw3
    #[error("{0}")]
    Threshold(#[from] ThresholdError),

    #[error("Group contract invalid address '{addr}'")]
    InvalidGroup { addr: String },

    #[error("Proposal is not open")]
    NotOpen {},

    #[error("Proposal voting period has expired")]
    Expired {},

    #[error("Proposal must expire before you can close it")]
    NotExpired {},

    #[error("Wrong expiration option")]
    WrongExpiration {},

    #[error("Already voted on this proposal")]
    AlreadyVoted {},

    #[error("Proposal must have passed and not yet been executed")]
    WrongExecuteStatus {},

    #[error("Cannot close completed or passed proposals")]
    WrongCloseStatus {},

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("{0}")]
    Deposit(#[from] DepositError),

    #[error("Invalid reply id: {0}")]
    InvalidReplyId(u64),

    #[error("No voters")]
    NoVoters {},
}
