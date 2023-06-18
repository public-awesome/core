#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw2::set_contract_version;
use stargaze_loyalty_collection::state::Metadata;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

const CONTRACT_NAME: &str = "crates.io:stargaze-loyalty-minter";
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
    // TODO: get the total staked for the address
    let delegations = deps.querier.query_all_delegations(address)?;
    // NOTE: assuming the staked denom is the same as the stake weight denom
    let total_staked = delegations
        .iter()
        .fold(0, |acc, d| acc + d.amount.amount.u128());

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
