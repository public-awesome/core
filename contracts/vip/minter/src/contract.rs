#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
    WasmMsg,
};
use cw2::set_contract_version;
use sg_vip::minter::InstantiateMsg;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::state::{Config, CONFIG};

const CONTRACT_NAME: &str = "crates.io:stargaze-vip-minter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(
        deps.storage,
        &Config {
            vip_collection: deps.api.addr_validate(&msg.vip_collection)?,
            name_collection: deps.api.addr_validate(&msg.name_collection)?,
        },
    )?;

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
        ExecuteMsg::Mint { name } => execute_mint(deps, env, info, name),
        ExecuteMsg::Update { name } => todo!(),
        ExecuteMsg::Pause {} => todo!(),
    }
}

pub fn execute_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    let Config {
        vip_collection,
        name_collection,
    } = CONFIG.load(deps.storage)?;

    ensure!(
        info.sender == associated_address(deps.as_ref(), name.clone())?,
        ContractError::Unauthorized {}
    );

    let staked_amount = total_staked(deps.as_ref(), info.sender.clone())?;

    let msg = sg_vip::collection::ExecuteMsg::Mint {
        name,
        owner: info.sender.to_string(),
        staked_amount: Uint128::from(staked_amount),
        data: None,
    };
    let mint_msg = WasmMsg::Execute {
        contract_addr: vip_collection.to_string(),
        msg: to_binary(&msg)?,
        funds: vec![],
    };

    // TODO: add the `token_id` to an end block queue

    Ok(Response::new().add_message(mint_msg))
}

pub fn execute_update(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    let Config {
        vip_collection,
        name_collection,
    } = CONFIG.load(deps.storage)?;

    ensure!(
        info.sender == associated_address(deps.as_ref(), name.clone())?,
        ContractError::Unauthorized {}
    );

    let staked_amount = total_staked(deps.as_ref(), info.sender.clone())?;

    let msg = sg_vip::collection::ExecuteMsg::Mint {
        name,
        owner: info.sender.to_string(),
        staked_amount: Uint128::from(staked_amount),
        data: None,
    };
    let mint_msg = WasmMsg::Execute {
        contract_addr: vip_collection.to_string(),
        msg: to_binary(&msg)?,
        funds: vec![],
    };

    // TODO: update metadata and call update on collection contract

    Ok(Response::new())
}

fn associated_address(deps: Deps, name: String) -> Result<Addr, ContractError> {
    let associated_addr: Addr = deps.querier.query_wasm_smart(
        CONFIG.load(deps.storage)?.name_collection,
        &sg_name::SgNameQueryMsg::AssociatedAddress { name },
    )?;

    Ok(associated_addr)
}

fn total_staked(deps: Deps, address: Addr) -> StdResult<u128> {
    let total = deps
        .querier
        .query_all_delegations(address)?
        .iter()
        .fold(0, |acc, d| acc + d.amount.amount.u128());

    Ok(total)
}

// TODO: add end block function
// TODO: pop address off the queue and update metadata

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
