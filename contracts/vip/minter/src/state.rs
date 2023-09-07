use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub vip_collection: Addr,
    pub update_interval: u64, // in blocks
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PAUSED: Item<bool> = Item::new("paused");

/// (block_height, [name1, name2, ...])
pub const NAME_QUEUE: Map<u64, Vec<String>> = Map::new("nq");

/// (name, block_height)
pub const TOKEN_UPDATE_HEIGHT: Map<u64, u64> = Map::new("nuh");

pub const TOKEN_INDEX: Item<u64> = Item::new("token_index");

pub fn increment_token_index(store: &mut dyn Storage) -> StdResult<u64> {
    let val = TOKEN_INDEX.may_load(store)?.unwrap_or_default() + 1;
    TOKEN_INDEX.save(store, &val)?;
    Ok(val)
}
