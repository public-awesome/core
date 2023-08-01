use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const COLLECTION: Item<Addr> = Item::new("collection");
