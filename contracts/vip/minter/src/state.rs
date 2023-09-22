use cosmwasm_std::{Addr, StdResult, Storage};
use cw_storage_plus::{Item, Map};

pub const COLLECTION: Item<Addr> = Item::new("collection_address");
pub const PAUSED: Item<bool> = Item::new("paused");

/// (name, block_height)
pub const TOKEN_UPDATE_HEIGHT: Map<u64, u64> = Map::new("tuh");

pub const TOKEN_INDEX: Item<u64> = Item::new("token_index");

pub fn increment_token_index(store: &mut dyn Storage) -> StdResult<u64> {
    let val = TOKEN_INDEX.may_load(store)?.unwrap_or_default() + 1;
    TOKEN_INDEX.save(store, &val)?;
    Ok(val)
}
