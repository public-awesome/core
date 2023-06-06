#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, StdResult};
use cw2::set_contract_version;
use cw_ownable::get_ownership;
use sg_std::Response;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, WHITELIST};

const CONTRACT_NAME: &str = "crates.io:stargaze-whitelist-mutable";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    deps.api.addr_validate(&msg.owner)?;
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(&msg.owner))?;

    CONFIG.save(deps.storage, &Config { bech32: msg.bech32 })?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddAddress { address } => execute_add_address(deps, address),
        ExecuteMsg::RemoveAddress { address } => execute_remove_address(deps, address),
        ExecuteMsg::UpdateOwnership(action) => update_ownership(deps, env, info, action),
        ExecuteMsg::Purge {} => execute_purge(deps),
    }
}

pub fn execute_add_address(deps: DepsMut, address: String) -> Result<Response, ContractError> {
    if CONFIG.load(deps.storage)?.bech32 {
        deps.api.addr_validate(&address)?;
    }

    if !WHITELIST.insert(deps.storage, &address)? {
        return Err(ContractError::DuplicateAddress {});
    }

    Ok(Response::new())
}

pub fn execute_remove_address(deps: DepsMut, address: String) -> Result<Response, ContractError> {
    WHITELIST.remove(deps.storage, &address)?;

    Ok(Response::new())
}

pub fn execute_purge(deps: DepsMut) -> Result<Response, ContractError> {
    WHITELIST.clear(deps.storage);

    Ok(Response::new())
}

/// Wraps around cw_ownable::update_ownership to extract the result and wrap it in a Stargaze Response
fn update_ownership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    action: cw_ownable::Action,
) -> Result<Response, ContractError> {
    let ownership = cw_ownable::update_ownership(deps, &env.block, &info.sender, action)?;
    Ok(Response::new().add_attributes(ownership.into_attributes()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Ownership {} => to_binary(&get_ownership(deps.storage)?),
        QueryMsg::IncludesAddress { address } => to_binary(&query_includes_address(deps, address)),
        QueryMsg::Count {} => to_binary(&query_count(deps)?),
        QueryMsg::List {} => to_binary(&query_list(deps)?),
    }
}

pub fn query_includes_address(deps: Deps, address: String) -> bool {
    WHITELIST.contains(deps.storage, &address)
}

pub fn query_count(deps: Deps) -> StdResult<u64> {
    WHITELIST.count(deps.storage)
}

pub fn query_list(deps: Deps) -> StdResult<Vec<String>> {
    WHITELIST
        .items(deps.storage, None, None, Order::Ascending)
        .collect()
}
