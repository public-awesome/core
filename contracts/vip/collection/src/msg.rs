use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::Metadata;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Metadata)]
    Metadata { token_id: String },
}
