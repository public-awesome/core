use std::env;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, instantiate2_address, to_binary, Addr, Binary, CodeInfoResponse, ContractInfoResponse,
    Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Timestamp, Uint128, WasmMsg,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, NAME_QUEUE};

const CONTRACT_NAME: &str = "crates.io:stargaze-vip-minter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let minter = env.contract.address;

    let canonical_creator = deps.api.addr_canonicalize(minter.as_str())?;
    let CodeInfoResponse { checksum, .. } =
        deps.querier.query_wasm_code_info(msg.collection_code_id)?;
    let salt = b"vip_collection1";

    // create collection address with instantiate2
    let canonical_addr = instantiate2_address(&checksum, &canonical_creator, salt)
        .map_err(|_| StdError::generic_err("Could not calculate addr"))?;
    let collection = deps.api.addr_humanize(&canonical_addr)?;

    let ContractInfoResponse { admin, .. } =
        deps.querier.query_wasm_contract_info(minter.clone())?;

    CONFIG.save(
        deps.storage,
        &Config {
            vip_collection: deps.api.addr_validate(collection.as_str())?,
            name_collection: deps.api.addr_validate(&msg.name_collection)?,
            update_interval: msg.update_interval,
        },
    )?;

    let collection_init_msg = WasmMsg::Instantiate2 {
        admin,
        code_id: msg.collection_code_id,
        label: String::from("vip-collection"),
        msg: to_binary(&cw721_base::InstantiateMsg {
            name: "Stargaze VIP Collection".to_string(),
            symbol: "SGVIP".to_string(),
            minter: minter.to_string(),
        })?,
        funds: vec![],
        salt: Binary::from(salt.to_vec()),
    };

    Ok(Response::new()
        .add_message(collection_init_msg)
        .add_attribute("collection", collection))
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
        ExecuteMsg::Update { name } => execute_update(deps, env, info, name),
        ExecuteMsg::Pause {} => todo!(),
    }
}

pub fn execute_mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    ensure!(
        info.sender == associated_address(deps.as_ref(), name.clone())?,
        ContractError::Unauthorized {}
    );

    let Config {
        vip_collection,
        update_interval,
        ..
    } = CONFIG.load(deps.storage)?;

    let mint_msg = mint(
        deps.as_ref(),
        info.sender,
        env.block.time,
        name.clone(),
        vip_collection,
    )?;

    NAME_QUEUE.update(
        deps.storage,
        env.block.height + update_interval,
        |names| -> StdResult<_> {
            let mut names = names.unwrap_or_default();
            names.push(name);
            Ok(names)
        },
    )?;

    Ok(Response::new().add_message(mint_msg))
}

pub fn execute_update(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    ensure!(
        info.sender == associated_address(deps.as_ref(), name.clone())?,
        ContractError::Unauthorized {}
    );

    let Config { vip_collection, .. } = CONFIG.load(deps.storage)?;

    let mint_msg = mint(
        deps.as_ref(),
        info.sender,
        env.block.time,
        name,
        vip_collection,
    )?;

    Ok(Response::new().add_message(mint_msg))
}

pub fn mint(
    deps: Deps,
    sender: Addr,
    block_time: Timestamp,
    name: String,
    vip_collection: Addr,
) -> Result<WasmMsg, ContractError> {
    let msg = stargaze_vip_collection::ExecuteMsg::Mint {
        token_id: name,
        owner: sender.to_string(),
        token_uri: None,
        extension: stargaze_vip_collection::state::Metadata {
            staked_amount: total_staked(deps, sender)?,
            data: None,
            updated_at: block_time,
        },
    };

    Ok(WasmMsg::Execute {
        contract_addr: vip_collection.to_string(),
        msg: to_binary(&msg)?,
        funds: vec![],
    })
}

pub fn associated_address(deps: Deps, name: String) -> Result<Addr, ContractError> {
    let associated_addr: Addr = deps.querier.query_wasm_smart(
        CONFIG.load(deps.storage)?.name_collection,
        &sg_name::SgNameQueryMsg::AssociatedAddress { name },
    )?;

    Ok(associated_addr)
}

fn total_staked(deps: Deps, address: Addr) -> StdResult<Uint128> {
    let total = deps
        .querier
        .query_all_delegations(address)?
        .iter()
        .fold(0, |acc, d| acc + d.amount.amount.u128());

    Ok(Uint128::from(total))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
