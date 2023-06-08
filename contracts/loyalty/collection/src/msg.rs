use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw_ownable::{cw_ownable_execute, cw_ownable_query};

use crate::state::Metadata;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    // TODO: move this to the minter?
    UpdateMetadata {
        address: String,
        staked_amount: Uint128,
        data: Option<String>,
    },
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Metadata)]
    Metadata { address: String },
    #[returns(Uint128)]
    TotalStaked { owner: String },
}
