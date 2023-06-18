use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_ownable::{cw_ownable_execute, cw_ownable_query};
use sg_basic_whitelist::sg_basic_whitelist_query;

use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {
    /// Set to `true` for Cosmos and Stargaze addresses, `false` for Ethereum and others.
    pub bech32: bool,
    /// The address that can add and remove addresses from the whitelist.
    pub owner: String,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    AddAddress { address: String },
    RemoveAddress { address: String },
    Purge {},
}

#[sg_basic_whitelist_query]
#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
    #[returns(u64)]
    Count {},
    #[returns(Vec<String>)]
    List {},
}
