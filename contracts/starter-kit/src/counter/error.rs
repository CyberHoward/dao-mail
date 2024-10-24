use cosmwasm_std::Addr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CounterError {
    #[error("{0}")]
    Std(#[from] cosmwasm_std::StdError),

    #[error("Counter not found for account {address} and authenticator {authenticator_id}")]
    CounterNotFound {
        address: Addr,
        authenticator_id: String,
    },
}

pub type CounterResult<T> = Result<T, CounterError>;