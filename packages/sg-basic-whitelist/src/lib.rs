use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_ownable::{cw_ownable_execute, cw_ownable_query};

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    AddAddress { address: String },
    RemoveAddress { address: String },
    Purge {},
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(bool)]
    IncludesAddress { address: String },
}
