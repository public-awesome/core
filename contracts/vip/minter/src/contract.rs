#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, to_binary, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
    Uint128, WasmMsg,
};
use cw2::set_contract_version;
use sg_vip::minter::InstantiateMsg;
use stargaze_vip_collection::contract::total_staked;
use stargaze_vip_collection::state::Metadata;

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
    env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    let Config {
        vip_collection,
        name_collection,
    } = CONFIG.load(deps.storage)?;

    // query name so we know the name is associated with an address
    let associated_addr: Addr = deps.querier.query_wasm_smart(
        name_collection,
        &sg_name::SgNameQueryMsg::AssociatedAddress { name: name.clone() },
    )?;
    ensure!(
        info.sender == associated_addr,
        ContractError::Unauthorized {}
    );

    let total_staked = total_staked(deps.as_ref(), associated_addr)?;

    let metadata = Metadata {
        staked_amount: Uint128::from(total_staked),
        data: None,
        updated_at: env.block.time,
    };

    let msg = cw721_base::ExecuteMsg::<Metadata, Empty>::Mint {
        token_id: name,
        owner: info.sender.to_string(),
        token_uri: None,
        extension: metadata,
    };
    let mint_msg = WasmMsg::Execute {
        contract_addr: vip_collection.to_string(),
        msg: to_binary(&msg)?,
        funds: vec![],
    };

    // TODO: add the address to an end block queue

    Ok(Response::new().add_message(mint_msg))
}

// TODO: add end block function
// TODO: pop address off the queue and update metadata

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
