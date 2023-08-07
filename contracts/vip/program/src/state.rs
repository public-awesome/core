use std::collections::BTreeMap;

use cosmwasm_std::{Addr, Coin, Coins, Uint128};
use cw_storage_plus::{Item, Map};

pub const COLLECTION: Item<Addr> = Item::new("collection");

// TODO: waiting on https://github.com/CosmWasm/cosmwasm/pull/1809
// pub const TIERS: Map<u16, Coins> = Map::new("t");

pub const TIERS: Map<u16, Vec<Coin>> = Map::new("t");

// pub const TIERS: Map<u16, BTreeMap<String, Uint128>> = Map::new("t");
