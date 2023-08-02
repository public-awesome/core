#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    instantiate2_address, to_binary, Addr, Binary, CodeInfoResponse, ContractInfoResponse, Deps,
    DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw721_base::InstantiateMsg;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::VipCollection;

const CONTRACT_NAME: &str = "crates.io:stargaze-vip-collection";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // create minter address with instantiate2
    let contract_addr = env.contract.address.as_str();

    let canonical_creator = deps.api.addr_canonicalize(contract_addr)?;
    let ContractInfoResponse { code_id, .. } =
        deps.querier.query_wasm_contract_info(contract_addr)?;
    let CodeInfoResponse { checksum, .. } = deps.querier.query_wasm_code_info(code_id)?;
    let salt = b"vip_minter1";

    let canonical_addr = instantiate2_address(&checksum, &canonical_creator, salt)
        .map_err(|_| StdError::generic_err("Could not calculate addr"))?;

    let addr = deps.api.addr_humanize(&canonical_addr)?;

    // TODO: query minter to see if it exists...

    // TODO: instantiate minter with collection address

    // instantiate collection
    VipCollection::default().instantiate(deps.branch(), env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateMetadata {
            address,
            staked_amount,
            data,
        } => execute_update_metadata(deps, env, info, address, staked_amount, data),
    }
}

pub fn execute_update_metadata(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _address: String,
    _staked_amount: Uint128,
    _data: Option<String>,
) -> Result<Response, ContractError> {
    // TODO: only_owner...
    // TODO: get the nft based on the address (which is the token_id)
    // TODO: update metadata

    Ok(Response::new())
}

pub fn total_staked(deps: Deps, address: Addr) -> StdResult<u128> {
    let total = deps
        .querier
        .query_all_delegations(address)?
        .iter()
        .fold(0, |acc, d| acc + d.amount.amount.u128());

    Ok(total)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Metadata { address } => to_binary(&query_metadata(deps, address)?),
        QueryMsg::TotalStaked { owner } => to_binary(&query_total_staked(deps, owner)?),
    }
}

pub fn query_metadata(deps: Deps, address: String) -> StdResult<Binary> {
    // TODO: get metadata by address (token_id)

    todo!()
}

/// Total staked is the sum of all staked amounts for a given owner. If
/// an owner has multiple items, it will iterate through all of them and
/// sum the staked amounts.
pub fn query_total_staked(deps: Deps, owner: String) -> StdResult<Binary> {
    // TODO: get all tokens by owner of `address` (token_id)
    // TODO: iterate through metdata to get total stake weight

    todo!()
}

#[cfg(test)]
mod tests {}
