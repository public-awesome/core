use std::env;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, instantiate2_address, to_binary, Addr, Binary, CodeInfoResponse, Deps, DepsMut, Env,
    Event, MessageInfo, Response, StdError, StdResult, Timestamp, Uint128, WasmMsg,
};
use cw2::set_contract_version;
use cw721::TokensResponse;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    increment_token_index, BASE_URI, COLLECTION, PAUSED, TIERS, TOKEN_INDEX, TOKEN_UPDATE_HEIGHT,
};
use stargaze_vip_collection::state::Metadata;

const CONTRACT_NAME: &str = "crates.io:stargaze-vip-minter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(info.sender.as_str()))?;
    let minter = env.contract.address;
    BASE_URI.save(deps.storage, &msg.base_uri)?;
    TIERS.save(deps.storage, &msg.tiers)?;

    let canonical_creator = deps.api.addr_canonicalize(minter.as_str())?;
    let CodeInfoResponse { checksum, .. } =
        deps.querier.query_wasm_code_info(msg.collection_code_id)?;
    let salt = b"vip_collection1";

    // create collection address with instantiate2
    let canonical_addr = instantiate2_address(&checksum, &canonical_creator, salt)
        .map_err(|_| StdError::generic_err("Could not calculate addr"))?;
    let collection = deps.api.addr_humanize(&canonical_addr)?;

    COLLECTION.save(deps.storage, &deps.api.addr_validate(collection.as_str())?)?;

    PAUSED.save(deps.storage, &false)?;

    let collection_init_msg = WasmMsg::Instantiate2 {
        admin: Some(String::from(info.sender)),
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

    let event = Event::new("instantiate").add_attribute("collection", collection);

    Ok(Response::new()
        .add_message(collection_init_msg)
        .add_event(event))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint {} => execute_mint(deps, env, info),
        ExecuteMsg::Update { token_id } => execute_update(deps, env, info, token_id),
        ExecuteMsg::Pause {} => execute_pause(deps, info),
        ExecuteMsg::Resume {} => execute_resume(deps, info),
        ExecuteMsg::UpdateTiers { tiers } => execute_update_tiers(deps, info, tiers),
    }
}

pub fn execute_mint(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    ensure!(!PAUSED.load(deps.storage)?, ContractError::Paused {});

    let vip_collection = COLLECTION.load(deps.storage)?;

    let mint_msg = mint(
        deps.branch(),
        info.sender,
        env.block.time,
        vip_collection,
        None,
    )?;
    let token_id = TOKEN_INDEX.load(deps.storage)?;
    TOKEN_UPDATE_HEIGHT.update(deps.storage, token_id, |_| -> StdResult<_> {
        Ok(env.block.height)
    })?;
    let event = Event::new("mint");
    Ok(Response::new().add_message(mint_msg).add_event(event))
}

pub fn execute_update(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: u64,
) -> Result<Response, ContractError> {
    ensure!(!PAUSED.load(deps.storage)?, ContractError::Paused {});
    let vip_collection = COLLECTION.load(deps.storage)?;

    let last_update_height = TOKEN_UPDATE_HEIGHT.may_load(deps.storage, token_id)?;
    if last_update_height.is_none() {
        return Err(ContractError::TokenNotFound {});
    }

    let mint_msg = mint(
        deps.branch(),
        info.sender,
        env.block.time,
        vip_collection,
        Some(token_id),
    )?;

    TOKEN_UPDATE_HEIGHT.update(deps.storage, token_id, |_| -> StdResult<_> {
        Ok(env.block.height)
    })?;
    let event = Event::new("update");
    Ok(Response::new().add_message(mint_msg).add_event(event))
}
pub fn mint(
    deps: DepsMut,
    sender: Addr,
    block_time: Timestamp,
    vip_collection: Addr,
    token_id: Option<u64>,
) -> Result<WasmMsg, ContractError> {
    if token_id.is_none() {
        // ensure that the sender did not mint any tokens yet
        let tokens_response: TokensResponse = deps.querier.query_wasm_smart(
            vip_collection.clone(),
            &cw721_base::msg::QueryMsg::<TokensResponse>::Tokens {
                owner: sender.to_string(),
                start_after: None,
                limit: None,
            },
        )?;
        ensure!(
            tokens_response.tokens.is_empty(),
            ContractError::AlreadyMinted {}
        );
    }

    let token_id_to_mint = match token_id {
        Some(id) => id.to_string(), // to be used for updates
        None => increment_token_index(deps.storage)?.to_string(),
    };

    let msg = stargaze_vip_collection::ExecuteMsg::Mint {
        token_id: token_id_to_mint,
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

pub fn execute_update_tiers(
    deps: DepsMut,
    info: MessageInfo,
    tiers: Vec<Uint128>,
) -> Result<Response, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)
        .map_err(|_| ContractError::Unauthorized {})?;
    TIERS.save(deps.storage, &tiers)?;
    let event = Event::new("update_tiers").add_attribute(
        "tiers",
        tiers
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(","),
    );
    Ok(Response::new().add_event(event))
}

pub fn execute_pause(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)
        .map_err(|_| ContractError::Unauthorized {})?;

    ensure!(!PAUSED.load(deps.storage)?, ContractError::AlreadyPaused {});
    PAUSED.save(deps.storage, &true)?;

    let event = Event::new("pause");
    Ok(Response::new().add_event(event))
}

pub fn execute_resume(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)
        .map_err(|_| ContractError::Unauthorized {})?;

    ensure!(PAUSED.load(deps.storage)?, ContractError::NotPaused {});
    PAUSED.save(deps.storage, &false)?;

    let event = Event::new("resume");
    Ok(Response::new().add_event(event))
}

fn total_staked(deps: DepsMut, address: Addr) -> StdResult<Uint128> {
    let total = deps
        .querier
        .query_all_delegations(address)?
        .iter()
        .fold(0, |acc, d| acc + d.amount.amount.u128());

    Ok(Uint128::from(total))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Collection {} => to_binary(&COLLECTION.load(deps.storage)?.to_string()),
        QueryMsg::IsPaused {} => to_binary(&PAUSED.load(deps.storage)?),
        QueryMsg::TokenUpdateHeight { token_id } => {
            to_binary(&TOKEN_UPDATE_HEIGHT.load(deps.storage, token_id)?)
        }
        QueryMsg::Tier { address } => {
            let tokens_response: cw721::TokensResponse = deps.querier.query_wasm_smart(
                COLLECTION.load(deps.storage)?,
                &cw721::Cw721QueryMsg::Tokens {
                    owner: address,
                    start_after: None,
                    limit: None,
                },
            )?;
            let token_id = tokens_response
                .tokens
                .first()
                .ok_or_else(|| StdError::generic_err("No token found for address"))?;

            let token_info: cw721::NftInfoResponse<Metadata> = deps.querier.query_wasm_smart(
                COLLECTION.load(deps.storage)?,
                &cw721::Cw721QueryMsg::NftInfo {
                    token_id: token_id.to_string(),
                },
            )?;
            let staked_amount = token_info.extension.staked_amount;

            let tiers = TIERS.load(deps.storage)?;
            let index = tiers
                .iter()
                .position(|&x| x >= staked_amount)
                .unwrap_or(tiers.len());

            Ok(to_binary(&index)?)
        }
        QueryMsg::Tiers {} => {
            let tiers = TIERS.load(deps.storage)?;
            Ok(to_binary(&tiers)?)
        }
    }
}

#[cfg(test)]
mod tests {}
