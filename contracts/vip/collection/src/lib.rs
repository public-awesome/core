pub mod contract;
mod error;
pub mod msg;
pub mod state;
use cosmwasm_std::Empty;
use msg::ExecuteExt;
use state::Metadata;

pub use crate::error::ContractError;

pub type VipCollection<'a> = cw721_base::Cw721Contract<'a, Metadata, Empty, ExecuteExt, Empty>;

pub type ExecuteMsg = cw721_base::ExecuteMsg<Metadata, ExecuteExt>;
