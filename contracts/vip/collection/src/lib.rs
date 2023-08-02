pub mod contract;
mod error;
pub mod helpers;
pub mod msg;
pub mod state;
use cosmwasm_std::Empty;
use state::Metadata;

pub use crate::error::ContractError;

pub type VipCollection<'a> = cw721_base::Cw721Contract<'a, Metadata, Empty, Empty, Empty>;

pub type ExecuteMsg = cw721_base::ExecuteMsg<Metadata, Empty>;
