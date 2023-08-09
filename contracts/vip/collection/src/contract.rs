#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use cw721_base::InstantiateMsg;

use crate::error::ContractError;
use crate::msg::QueryMsg;
use crate::{ExecuteMsg, VipCollection};

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

    // This configures the collection with the minter as the owner, the only one that can mint
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
        cw721_base::ExecuteMsg::TransferNft {
            recipient,
            token_id,
        } => Err(ContractError::Unauthorized {}),
        cw721_base::ExecuteMsg::SendNft {
            contract,
            token_id,
            msg,
        } => Err(ContractError::Unauthorized {}),
        cw721_base::ExecuteMsg::Approve {
            spender,
            token_id,
            expires,
        } => Err(ContractError::Unauthorized {}),
        cw721_base::ExecuteMsg::Revoke { spender, token_id } => Err(ContractError::Unauthorized {}),
        cw721_base::ExecuteMsg::ApproveAll { operator, expires } => {
            Err(ContractError::Unauthorized {})
        }
        cw721_base::ExecuteMsg::RevokeAll { operator } => Err(ContractError::Unauthorized {}),
        // cw721_base::ExecuteMsg::Mint {
        //     token_id,
        //     owner,
        //     token_uri,
        //     extension,
        // } => todo!(),
        // cw721_base::ExecuteMsg::Burn { token_id } => todo!(),
        // cw721_base::ExecuteMsg::Extension { msg } => todo!(),
        // cw721_base::ExecuteMsg::UpdateOwnership(_) => todo!(),
        _ => VipCollection::default()
            .execute(deps, env, info, msg)
            .map_err(Into::into),
    }
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

// pub fn query_stake_weight(deps: Deps, env: Env, name: String) -> StdResult<Uint128> {
//     let res: NftInfoResponse<Metadata> = VipCollection::default().query(
//         deps,
//         env,
//         cw721_base::msg::QueryMsg::NftInfo { token_id: name },
//     );

//     res.extension.staked_amount
// }

#[cfg(test)]
mod tests {}
