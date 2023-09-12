use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub collection: String,
    pub tiers: Vec<Uint128>,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateTiers { tiers: Vec<Uint128> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(u64)]
    Tier { address: String },
    #[returns(Vec<Uint128>)]
    Tiers {},
    #[returns(String)]
    Collection {},
}
