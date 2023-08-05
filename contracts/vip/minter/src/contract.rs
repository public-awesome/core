#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw2::set_contract_version;
use sg_vip::minter::InstantiateMsg;
use stargaze_vip_collection::contract::total_staked;
use stargaze_vip_collection::state::Metadata;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::state::COLLECTION;

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

    COLLECTION.save(deps.storage, &deps.api.addr_validate(&msg.collection)?)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { name } => execute_mint(deps, env, name),
    }
}

pub fn execute_mint(deps: DepsMut, env: Env, name: String) -> Result<Response, ContractError> {
    // TODO: query name to get associated address
    let address = "address".to_string();
    let total_staked = total_staked(deps.as_ref(), deps.api.addr_validate(&address)?)?;

    let metadata = Metadata {
        staked_amount: Uint128::from(total_staked),
        data: None,
        updated_at: env.block.time,
    };

    // TODO: get collection and mint token with metadata
    let collection = COLLECTION.load(deps.storage)?;

    // // Create mint msgs
    // let mint_msg = Sg721ExecuteMsg::<Extension, Empty>::Mint {
    //     token_id: increment_token_index(deps.storage)?.to_string(),
    //     owner: info.sender.to_string(),
    //     token_uri: Some(token_uri.clone()),
    //     extension: None,
    // };
    // let msg = CosmosMsg::Wasm(WasmMsg::Execute {
    //     contract_addr: collection_address.to_string(),
    //     msg: to_binary(&mint_msg)?,
    //     funds: vec![],
    // });
    // res = res.add_message(msg);

    // TODO: add the address to an end block queue

    Ok(Response::new())
}

// TODO: add end block function
// TODO: pop address off the queue and update metadata

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
