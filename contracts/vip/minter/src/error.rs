use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("InvalidUpdateInterval")]
    InvalidUpdateInterval {},

    #[error("Paused")]
    Paused {},

    #[error("AlreadyPaused")]
    AlreadyPaused {},

    #[error("NotPaused")]
    NotPaused {},

    #[error("UpdateIntervalNotPassed")]
    UpdateIntervalNotPassed {},

    #[error("TokenNotFound")]
    TokenNotFound {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
