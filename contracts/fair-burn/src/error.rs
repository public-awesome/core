use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Zero funds: must send non-zero funds to this contract")]
    ZeroFunds,

    #[error("Invalid config: {0}")]
    InvalidConfig(String),
}
