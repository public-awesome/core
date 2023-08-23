#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use stargaze_vip_collection::state::Metadata;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{COLLECTION, TIERS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:stargaze-vip-program";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // TODO: add cw_ownable so an admin can update tier limits

    COLLECTION.save(deps.storage, &deps.api.addr_validate(&msg.collection)?)?;

    TIERS.save(deps.storage, &msg.tiers)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Tier { name } => {
            let token_info: cw721::NftInfoResponse<Metadata> = deps.querier.query_wasm_smart(
                COLLECTION.load(deps.storage)?,
                &cw721::Cw721QueryMsg::NftInfo { token_id: name },
            )?;
            let staked_amount = token_info.extension.staked_amount;

            let tiers = TIERS.load(deps.storage)?;
            let index = tiers
                .iter()
                .position(|&x| x >= staked_amount)
                .unwrap_or(tiers.len());

            Ok(to_binary(&index)?)
        }
    }
}

#[cfg(test)]
mod tests {}
