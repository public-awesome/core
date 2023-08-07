#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    instantiate2_address, to_binary, Binary, CodeInfoResponse, ContractInfoResponse, Deps, DepsMut,
    Env, MessageInfo, Response, StdError, StdResult, WasmMsg,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteExt, InstantiateMsg, QueryMsg};
use crate::state::Metadata;
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

    let collection = env.contract.address.as_str();
    let canonical_creator = deps.api.addr_canonicalize(collection)?;
    let CodeInfoResponse { checksum, .. } =
        deps.querier.query_wasm_code_info(msg.minter_code_id)?;
    let salt = b"vip_minter1";

    // create minter address with instantiate2
    let canonical_addr = instantiate2_address(&checksum, &canonical_creator, salt)
        .map_err(|_| StdError::generic_err("Could not calculate addr"))?;
    let minter = deps.api.addr_humanize(&canonical_addr)?;

    let ContractInfoResponse { admin, .. } = deps.querier.query_wasm_contract_info(collection)?;

    let minter_init_msg = WasmMsg::Instantiate2 {
        admin,
        code_id: msg.minter_code_id,
        label: String::from("vip-minter"),
        msg: to_binary(&sg_vip::minter::InstantiateMsg {
            vip_collection: collection.to_string(),
            name_collection: msg.name_collection,
        })?,
        funds: vec![],
        salt: Binary::from(salt.to_vec()),
    };

    // TODO: `minter` may need to change to be this contract instead
    let collection_init_msg = cw721_base::msg::InstantiateMsg {
        name: String::from("Stargaze VIP Collection"),
        symbol: String::from("SGVIP"),
        minter: minter.to_string(),
    };

    // This configures the collection with the minter as the owner, the only one that can mint.
    let res =
        VipCollection::default().instantiate(deps.branch(), env, info, collection_init_msg)?;

    Ok(res
        .add_message(minter_init_msg)
        .add_attribute("vip-minter", minter))
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
        cw721_base::ExecuteMsg::Extension { msg } => match msg {
            ExecuteExt::UpdateToken {
                token_id,
                owner,
                token_uri,
                extension,
            } => todo!(),
        },
        // cw721_base::ExecuteMsg::UpdateOwnership(_) => todo!(),
        _ => VipCollection::default()
            .execute(deps, env, info, msg)
            .map_err(Into::into),
    }
}

pub fn execute_update_token(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    owner: String,
    extension: Metadata,
) -> Result<Response, ContractError> {
    only_minter(deps.as_ref())?;

    // We can just overwrite the previous token with the new metadata
    VipCollection::default()
        .mint(deps, info, token_id, owner, None, extension)
        .map_err(ContractError::Cw721Base)
}

fn only_minter(deps: Deps) -> Result<String, ContractError> {
    VipCollection::default()
        .minter(deps)?
        .minter
        .ok_or(ContractError::Unauthorized {})
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
