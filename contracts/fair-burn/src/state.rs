use crate::error::ContractError;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{ensure, Decimal, Storage};
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    /// The percentage of funds to be burned
    pub fee_percent: Decimal,
}

impl Config {
    pub fn save(&self, storage: &mut dyn Storage) -> Result<(), ContractError> {
        self.validate()?;
        CONFIG.save(storage, self)?;
        Ok(())
    }

    fn validate(&self) -> Result<(), ContractError> {
        ensure!(
            self.fee_percent > Decimal::zero(),
            ContractError::InvalidConfig("fee_percent must be positive".to_string())
        );
        Ok(())
    }
}

pub const CONFIG: Item<Config> = Item::new("cfg");
