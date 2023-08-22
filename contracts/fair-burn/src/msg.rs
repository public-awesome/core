#[cfg_attr(not(debug_assertions), allow(unused_imports))]
use crate::state::Config;

use cosmwasm_schema::{cw_serde, QueryResponses};
#[cfg_attr(not(debug_assertions), allow(unused_imports))]
use cw2::ContractVersion;

#[cw_serde]
pub struct InstantiateMsg {
    /// The percentage of funds to be burned, represented as basis points
    pub fee_bps: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    FairBurn { recipient: Option<String> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ContractVersion)]
    ContractVersion {},
    #[returns(Config)]
    Config {},
}

#[cw_serde]
pub enum SudoMsg {
    UpdateConfig { fee_bps: Option<u64> },
}
