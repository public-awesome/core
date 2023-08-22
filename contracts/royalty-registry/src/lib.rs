//! # Stargaze Royalty Registry
//!
//! The Stargaze Royalty Registry contract is a CosmWasm smart contract deployed on the Stargaze chain. It allows NFT collection admins to define the royalties that should be paid to them when their NFTs are sold on the Stargaze chain. The royalty registry logic is applied as described below:
//!
//! - Only the collection admin can register a royalty for a collection. The collection admin is defined to be the admin on the NFT collection contract. If that contract admin does not exist, then the collection admin is the contract creator.
//! - The collection admin can set a default royalty percentage for the collection. This default royalty percentage is applied when there is no specific protocol royalty percentage set for a given protocol.
//! - The collection admin can set a protocol royalty percentage for a given protocol. This protocol royalty percentage is applied when the protocol itself is calculation a royalty for the NFT sale.
//! - Any royalty percentage set by a given collection owner can only be changed by a the maximum amount of config parameter `max_share_delta` per invocation. After changing a royalty percentage, the collection owner must wait `update_wait_period` to update the percentage again.

mod error;
pub mod execute;
mod helpers;
pub mod instantiate;
pub mod migrate;
pub mod msg;
pub mod query;
pub mod state;
pub mod sudo;

pub use crate::error::ContractError;
pub use crate::helpers::fetch_or_set_royalties;
