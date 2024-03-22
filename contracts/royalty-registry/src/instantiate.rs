use crate::{error::ContractError, msg::InstantiateMsg};

use cosmwasm_std::{DepsMut, Env, Event, MessageInfo, Response};
use cw2::set_contract_version;

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = msg.config;
    config.save(deps.storage)?;

    let mut response = Response::new();
    response = response.add_event(
        Event::new("instantiate-contract")
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION),
    );

    response = response.add_event(
        Event::new("initialize-config")
            .add_attribute("update_wait_period", config.update_wait_period.to_string())
            .add_attribute("max_share_delta", config.max_share_delta.to_string()),
    );

    Ok(response)
}
