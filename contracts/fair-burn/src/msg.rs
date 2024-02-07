#[cfg_attr(not(debug_assertions), allow(unused_imports))]
use crate::state::Config;

use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    /// The percentage of funds to be burned, represented as basis points
    pub fee_bps: u64,
    /// The address to send fees to if the funds are not in STARS
    pub fee_manager: String,
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
    UpdateConfig {
        fee_bps: Option<u64>,
        fee_manager: Option<String>,
    },
}
