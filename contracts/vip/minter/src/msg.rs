use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub collection: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Mint a loyalty token for the given name
    Mint { name: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
