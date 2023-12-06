//! # Stargaze Fair Burn
//!
//! The Stargaze Fair Burn contract is a CosmWasm smart contract deployed on the Stargaze chain. It is responsible for handlintg fees paid by other contracts. Fees can be paid in multiple denoms. The Fair Burn contract performs the following logic:
//!
//! - If the funds transferred are in STARS, then a percentage of the funds are burned, and the remaining funds are sent either to the treasury, or a specified recipient address.
//! - If the funds transferred are not in STARS, then a percentage of the funds are sent to the treasury, and the remaining funds are sent either to the treasury, or a specified recipient address.
//!
//! ## Addresses
//!
//! - `elfagar-1: stars1mp4dg9mst3hxn5xvcd9zllyx6gguu5jsp5tyt9nsfrtghhwj2akqudhls8`

mod constants;
mod error;
pub mod execute;
mod helpers;
pub mod instantiate;
pub mod migrate;
#[doc(hidden)]
pub mod msg;
pub mod query;
mod state;
pub mod sudo;

#[cfg(test)]
mod tests;

pub use crate::helpers::append_fair_burn_msg;
