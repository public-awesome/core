use crate::{error::ContractError, helpers::bps_to_decimal, msg::SudoMsg, state::CONFIG};

use cosmwasm_std::{DepsMut, Env, Event};
use sg_std::Response;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::UpdateConfig { fee_bps } => sudo_update_config(deps, fee_bps),
    }
}

pub fn sudo_update_config(deps: DepsMut, fee_bps: Option<u64>) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    let mut event = Event::new("sudo-update-config");

    if let Some(fee_bps) = fee_bps {
        config.fee_percent = bps_to_decimal(fee_bps);
        event = event.add_attribute("fee_percent", config.fee_percent.to_string());
    }

    config.save(deps.storage)?;

    Ok(Response::new().add_event(event))
}
