use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::Config;

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
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
pub enum SudoMsg {
    UpdateConfig { fair_burn_bps: Option<u64> },
}

#[cw_serde]
pub struct ConfigResponse {
    pub contract: String,
    pub version: String,
    pub config: Config,
}
