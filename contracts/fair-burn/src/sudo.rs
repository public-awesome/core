use crate::{error::ContractError, helpers::bps_to_decimal, msg::SudoMsg, state::CONFIG};

use cosmwasm_std::{Addr, DepsMut, Env, Event};
use cw_utils::maybe_addr;
use sg_std::Response;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    let api = deps.api;

    match msg {
        SudoMsg::UpdateConfig {
            fee_bps,
            fee_manager,
        } => sudo_update_config(deps, fee_bps, maybe_addr(api, fee_manager)?),
    }
}

pub fn sudo_update_config(
    deps: DepsMut,
    fee_bps: Option<u64>,
    fee_manager: Option<Addr>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    let mut event = Event::new("sudo-update-config");

    if let Some(fee_bps) = fee_bps {
        config.fee_percent = bps_to_decimal(fee_bps);
        event = event.add_attribute("fee_percent", config.fee_percent.to_string());
    }

    if let Some(fee_manager) = fee_manager {
        config.fee_manager = fee_manager;
        event = event.add_attribute("fee_manager", config.fee_manager.to_string());
    }

    config.save(deps.storage)?;

    Ok(Response::new().add_event(event))
}
