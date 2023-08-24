use crate::error::ContractError;
use crate::msg::SudoMsg;
use crate::state::Config;

use cosmwasm_std::{DepsMut, Env, Event};
use sg_std::Response;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::UpdateConfig { config } => sudo_update_config(deps, env, config),
    }
}

pub fn sudo_update_config(
    deps: DepsMut,
    _env: Env,
    config: Config,
) -> Result<Response, ContractError> {
    config.save(deps.storage)?;

    let mut response = Response::new();
    response = response.add_event(
        Event::new("update-config")
            .add_attribute("update_wait_period", config.update_wait_period.to_string())
            .add_attribute("max_share_delta", config.max_share_delta.to_string()),
    );

    Ok(response)
}
