use crate::error::ContractError;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{ensure, Addr, Decimal, Storage};
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    /// The percentage of funds to be burned
    pub fee_percent: Decimal,
    /// The address to send fees to if the funds are not in STARS
    pub fee_manager: Addr,
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
