use crate::{
    constants::{CONTRACT_NAME, CONTRACT_VERSION},
    error::ContractError,
    state::Config,
};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{ensure, Decimal, DepsMut, Env, Event, StdError};
use cw2::{get_contract_version, set_contract_version};
use cw_storage_plus::Item;
use semver::Version;
use sg_std::Response;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

#[cw_serde]
pub struct ConfigV1_0 {
    /// The percentage of funds to be burned
    pub fee_percent: Decimal,
}

pub const CONFIG_V1_0: Item<ConfigV1_0> = Item::new("cfg");

#[cw_serde]
pub struct MigrateMsg {
    fee_manager: String,
}

#[cfg_attr(not(feature = "library"), entry_point)]
#[allow(clippy::cmp_owned)]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    let prev_contract_version = get_contract_version(deps.storage)?;

    let valid_contract_names = vec![CONTRACT_NAME.to_string()];
    ensure!(
        valid_contract_names.contains(&prev_contract_version.contract),
        StdError::generic_err("Invalid contract name for migration")
    );

    ensure!(
        Version::parse(&prev_contract_version.version).unwrap()
            < Version::parse(CONTRACT_VERSION).unwrap(),
        StdError::generic_err("Must upgrade contract version")
    );

    let config_v1_0 = CONFIG_V1_0.load(deps.storage)?;
    let config = Config {
        fee_percent: config_v1_0.fee_percent,
        fee_manager: deps.api.addr_validate(&msg.fee_manager)?,
    };
    config.save(deps.storage)?;

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let response = Response::new()
        .add_event(
            Event::new("migrate")
                .add_attribute("action", "migrate")
                .add_attribute("contract_name", CONTRACT_NAME)
                .add_attribute("contract_version", CONTRACT_VERSION),
        )
        .add_event(
            Event::new("update_config")
                .add_attribute("fee_percent", config.fee_percent.to_string())
                .add_attribute("fee_manager", config.fee_manager.to_string()),
        );

    Ok(response)
}
