pub mod contract;
mod error;
pub mod msg;
pub mod state;
use cosmwasm_std::Empty;
use schemars::JsonSchema;
use state::Metadata;

pub use crate::error::ContractError;

pub type VipCollection<'a> = cw721_base::Cw721Contract<'a, Metadata, Empty, Empty, Empty>;

pub type ExecuteMsg = cw721_base::ExecuteMsg<Metadata, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;
