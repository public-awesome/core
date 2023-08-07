use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct TierMsg {
    pub tier: u16,
    pub amount: Vec<Coin>,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub collection: String,
    pub tiers: Vec<TierMsg>,
}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
