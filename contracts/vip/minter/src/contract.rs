#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw2::set_contract_version;
use stargaze_vip_collection::contract::total_staked;
use stargaze_vip_collection::state::Metadata;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

const CONTRACT_NAME: &str = "crates.io:stargaze-vip-minter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

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
        ExecuteMsg::Mint { address } => execute_mint(deps, env, address),
    }
}

pub fn execute_mint(deps: DepsMut, env: Env, address: String) -> Result<Response, ContractError> {
    let total_staked = total_staked(deps.as_ref(), deps.api.addr_validate(&address)?)?;

    let metadata = Metadata {
        staked_amount: Uint128::from(total_staked),
        data: None,
        updated_at: env.block.time,
    };

    // TODO: get collection and mint token with metadata

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
