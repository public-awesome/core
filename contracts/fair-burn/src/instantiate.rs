use crate::{
    constants::{CONTRACT_NAME, CONTRACT_VERSION},
    error::ContractError,
    helpers::bps_to_decimal,
    msg::InstantiateMsg,
    state::Config,
};

use cosmwasm_std::{DepsMut, Env, Event, MessageInfo};
use cw2::set_contract_version;
use sg_std::Response;

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

    let config = Config {
        fee_percent: bps_to_decimal(msg.fee_bps),
        fee_manager: deps.api.addr_validate(&msg.fee_manager)?,
    };
    config.save(deps.storage)?;

    let event = Event::new("instantiate")
        .add_attribute("action", "instantiate")
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("contract_version", CONTRACT_VERSION)
        .add_attribute("fee_percent", config.fee_percent.to_string())
        .add_attribute("fee_manager", config.fee_manager.to_string());

    Ok(Response::new().add_event(event))
}
