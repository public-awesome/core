use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

use crate::state::Metadata;

#[cw_serde]
pub struct InstantiateMsg {
    pub minter_code_id: u64,
    pub name_collection: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    // TODO: move this to the minter?
    UpdateMetadata {
        address: String,
        staked_amount: Uint128,
        data: Option<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Metadata)]
    Metadata { address: String },
    #[returns(Uint128)]
    TotalStaked { owner: String },
}
