use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

use crate::state::Metadata;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Metadata)]
    Metadata { token_id: String },
    #[returns(Uint128)]
    TotalStaked { owner: String },
}
