use crate::ContractError;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{ensure, Addr, Decimal, Storage, Timestamp};
use cw_storage_plus::{Item, Map};
use std::cmp::min;

#[cw_serde]
pub struct Config {
    /// The number of seconds to wait before updating a royalty entry.
    pub update_wait_period: u64,
    /// The maximum that can be added or removed from a royalty entry in a single update.
    pub max_share_delta: Decimal,
}

impl Config {
    pub fn save(&self, storage: &mut dyn Storage) -> Result<(), ContractError> {
        self.validate()?;
        CONFIG.save(storage, self)?;
        Ok(())
    }

    fn validate(&self) -> Result<(), ContractError> {
        ensure!(
            self.max_share_delta < Decimal::one(),
            ContractError::InvalidConfig("max_share_delta must be greater than 0".to_string())
        );
        Ok(())
    }
}

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct RoyaltyEntry {
    /// The address that will receive the royalty payments
    pub recipient: Addr,
    /// The percentage of sales that should be paid to the recipient
    pub share: Decimal,
    /// The last time the royalty entry was updated
    pub updated: Option<Timestamp>,
}

impl RoyaltyEntry {
    pub fn validate(&self) -> Result<(), ContractError> {
        ensure!(
            self.share <= Decimal::one(),
            ContractError::InvalidCollectionRoyalty(
                "Royalty share must be less than or equal to 1".to_string()
            )
        );
        Ok(())
    }

    pub fn update_share(
        &mut self,
        config: &Config,
        share_delta: Decimal,
        decrement: Option<bool>,
    ) -> Result<(), ContractError> {
        let delta = min(share_delta, config.max_share_delta);
        let decrement = decrement.is_some() && decrement.unwrap();

        if decrement {
            self.share -= delta;
        } else {
            self.share += delta;
        }

        Ok(())
    }
}

#[cw_serde]
pub struct RoyaltyDefault {
    pub collection: Addr,
    pub royalty_entry: RoyaltyEntry,
}

pub const ROYALTY_DEFAULTS: Map<Addr, RoyaltyDefault> = Map::new("rd");

// (collection, protocol) -> RoyaltyProtocol
pub type RoyaltyProtocolKey = (Addr, Addr);

#[cw_serde]
pub struct RoyaltyProtocol {
    pub collection: Addr,
    pub protocol: Addr,
    pub royalty_entry: RoyaltyEntry,
}

pub const ROYALTY_PROTOCOLS: Map<RoyaltyProtocolKey, RoyaltyProtocol> = Map::new("rp");
