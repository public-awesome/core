use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

#[cw_serde]
pub struct Metadata {
    pub staked_amount: Coin,
    pub data: Option<String>,
    pub updated_at: u64,
}
