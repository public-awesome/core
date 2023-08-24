use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub vip_collection: Addr,
    pub name_collection: Addr,
    pub update_interval: u64, // in blocks
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PAUSED: Item<bool> = Item::new("paused");

/// (block_height, [name1, name2, ...])
pub const NAME_QUEUE: Map<u64, Vec<String>> = Map::new("nq");

// TODO: need secondary queue for name <> height mapping
