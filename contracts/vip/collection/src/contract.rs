#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use cw721::{Cw721Query};
use cw721_base::InstantiateMsg;

use crate::error::ContractError;
use crate::{ExecuteMsg, QueryMsg, VipCollection};

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
        _ => VipCollection::default()
            .execute(deps, env, info, msg)
            .map_err(Into::into),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    VipCollection::default().query(deps, env, msg)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Empty;
    use cw_multi_test::{Contract, ContractWrapper};

    fn collection_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::instantiate,
            crate::contract::execute,
            crate::contract::query,
        );
        Box::new(contract)
    }
}
