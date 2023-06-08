use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

use crate::state::Metadata;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateMetadata {
        address: String,
        staked_amount: Coin,
        data: Option<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Metadata)]
    Metadata { address: String },
}
