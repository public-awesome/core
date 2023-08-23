use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

pub const COLLECTION: Item<Addr> = Item::new("collection");

pub const TIERS: Item<Vec<Uint128>> = Item::new("tiers");
