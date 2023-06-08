use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Timestamp, Uint128};

#[cw_serde]
pub struct Metadata {
    pub staked_amount: Uint128,
    pub data: Option<String>,
    pub updated_at: Timestamp,
}
