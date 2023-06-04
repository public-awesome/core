use cosmwasm_schema::cw_serde;
use cw_item_set::Set;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub bech32: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const WHITELIST: Set<&str> = Set::new("wl", "whitelist__counter");
