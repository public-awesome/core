#[cfg_attr(not(debug_assertions), allow(unused_imports))]
use crate::state::Config;

use cosmwasm_schema::{cw_serde, QueryResponses};

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
    #[returns(Config)]
    Config {},
}

#[cw_serde]
pub enum SudoMsg {
    UpdateConfig { fee_bps: Option<u64> },
}
