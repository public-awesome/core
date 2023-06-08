use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    /// Mint a loyalty token for the given address
    Mint { address: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
