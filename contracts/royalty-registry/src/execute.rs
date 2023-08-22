use crate::{
    error::ContractError,
    helpers::only_collection_creator,
    msg::ExecuteMsg,
    state::{
        RoyaltyDefault, RoyaltyEntry, RoyaltyProtocol, RoyaltyProtocolKey, CONFIG,
        ROYALTY_DEFAULTS, ROYALTY_PROTOCOLS,
    },
};

use cosmwasm_std::{attr, ensure, Addr, Decimal, DepsMut, Env, Event, MessageInfo};
use cw_utils::{maybe_addr, nonpayable};
use sg721_base::msg::{CollectionInfoResponse, QueryMsg as Sg721QueryMsg};
use sg_std::Response;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let api = deps.api;

    match msg {
        ExecuteMsg::InitializeCollectionRoyalty { collection } => {
            execute_initialize_collection_royalty(deps, info, env, api.addr_validate(&collection)?)
        }
        ExecuteMsg::SetCollectionRoyaltyDefault {
            collection,
            recipient,
            share,
        } => execute_set_collection_royalty_default(
            deps,
            info,
            env,
            api.addr_validate(&collection)?,
            api.addr_validate(&recipient)?,
            share,
        ),
        ExecuteMsg::UpdateCollectionRoyaltyDefault {
            collection,
            recipient,
            share_delta,
            decrement,
        } => execute_update_collection_royalty_default(
            deps,
            info,
            env,
            api.addr_validate(&collection)?,
            maybe_addr(api, recipient)?,
            share_delta,
            decrement,
        ),
        ExecuteMsg::SetCollectionRoyaltyProtocol {
            collection,
            protocol,
            recipient,
            share,
        } => execute_set_collection_royalty_protocol(
            deps,
            info,
            env,
            api.addr_validate(&collection)?,
            api.addr_validate(&protocol)?,
            api.addr_validate(&recipient)?,
            share,
        ),
        ExecuteMsg::UpdateCollectionRoyaltyProtocol {
            collection,
            protocol,
            recipient,
            share_delta,
            decrement,
        } => execute_update_collection_royalty_protocol(
            deps,
            info,
            env,
            api.addr_validate(&collection)?,
            api.addr_validate(&protocol)?,
            maybe_addr(api, recipient)?,
            share_delta,
            decrement,
        ),
    }
}

pub fn execute_initialize_collection_royalty(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    collection: Addr,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;

    let mut response = Response::new();

    let royalty_default = ROYALTY_DEFAULTS.may_load(deps.storage, collection.clone())?;
    if royalty_default.is_some() {
        return Err(ContractError::InvalidCollectionRoyalty(
            "Collection royalty already initialized".to_string(),
        ));
    }

    let collection_info: CollectionInfoResponse = deps
        .querier
        .query_wasm_smart(collection.clone(), &Sg721QueryMsg::CollectionInfo {})?;

    if let Some(royalty_info) = collection_info.royalty_info {
        let royalty_entry = RoyaltyEntry {
            recipient: deps.api.addr_validate(&royalty_info.payment_address)?,
            share: royalty_info.share,
            updated: None,
        };

        royalty_entry.validate()?;

        ROYALTY_DEFAULTS.save(
            deps.storage,
            collection.clone(),
            &RoyaltyDefault {
                collection: collection.clone(),
                royalty_entry,
            },
        )?;

        response = response.add_event(Event::new("initialize-collection-royalty").add_attributes(
            vec![
                attr("collection", collection.to_string()),
                attr("recipient", royalty_info.payment_address.to_string()),
                attr("share", royalty_info.share.to_string()),
                attr("updated", env.block.time.to_string()),
            ],
        ));
    } else {
        return Err(ContractError::InvalidCollectionRoyalty(
            "Collection contract royalties not found".to_string(),
        ));
    }

    Ok(response)
}

pub fn execute_set_collection_royalty_default(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    collection: Addr,
    recipient: Addr,
    share: Decimal,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    only_collection_creator(deps.as_ref(), &info, &collection)?;

    let mut response = Response::new();

    let royalty_default = ROYALTY_DEFAULTS.may_load(deps.storage, collection.clone())?;
    if royalty_default.is_some() {
        return Err(ContractError::InvalidCollectionRoyalty(
            "Collection royalty already initialized".to_string(),
        ));
    }

    let royalty_default = RoyaltyDefault {
        collection: collection.clone(),
        royalty_entry: RoyaltyEntry {
            recipient: recipient.clone(),
            share,
            updated: Some(env.block.time),
        },
    };

    royalty_default.royalty_entry.validate()?;

    ROYALTY_DEFAULTS.save(deps.storage, collection.clone(), &royalty_default)?;

    response = response.add_event(Event::new("set-collection-royalty-default").add_attributes(
        vec![
            attr("collection", collection.to_string()),
            attr("recipient", recipient.to_string()),
            attr("share", share.to_string()),
            attr("updated", env.block.time.to_string()),
        ],
    ));

    Ok(response)
}

pub fn execute_update_collection_royalty_default(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    collection: Addr,
    recipient: Option<Addr>,
    share_delta: Option<Decimal>,
    decrement: Option<bool>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    only_collection_creator(deps.as_ref(), &info, &collection)?;

    let config = CONFIG.load(deps.storage)?;
    let mut response = Response::new();

    let mut royalty_default = ROYALTY_DEFAULTS
        .load(deps.storage, collection.clone())
        .map_err(|_| {
            ContractError::InvalidCollectionRoyalty("Collection royalty does not exist".to_string())
        })?;

    if let Some(updated) = royalty_default.royalty_entry.updated {
        ensure!(
            updated.plus_seconds(config.update_wait_period) <= env.block.time,
            ContractError::Unauthorized("Royalty entry cannot be updated yet".to_string())
        );
    }

    let mut event = Event::new("update-collection-royalty-default")
        .add_attribute("collection", collection.to_string());

    if let Some(recipient) = recipient {
        royalty_default.royalty_entry.recipient = recipient.clone();
        event = event.add_attribute("recipient", recipient.to_string());
    }

    if let Some(share_delta) = share_delta {
        royalty_default
            .royalty_entry
            .update_share(&config, share_delta, decrement)?;

        event = event.add_attribute("share", royalty_default.royalty_entry.share.to_string());
    }

    royalty_default.royalty_entry.updated = Some(env.block.time);
    royalty_default.royalty_entry.validate()?;
    ROYALTY_DEFAULTS.save(deps.storage, collection, &royalty_default)?;

    response = response.add_event(event);

    Ok(response)
}

pub fn execute_set_collection_royalty_protocol(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    collection: Addr,
    protocol: Addr,
    recipient: Addr,
    share: Decimal,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    only_collection_creator(deps.as_ref(), &info, &collection)?;

    let mut response = Response::new();

    let royalty_protocol_key: RoyaltyProtocolKey = (collection.clone(), protocol.clone());
    let royalty_protocol =
        ROYALTY_PROTOCOLS.may_load(deps.storage, royalty_protocol_key.clone())?;
    if royalty_protocol.is_some() {
        return Err(ContractError::InvalidCollectionRoyalty(
            "Collection royalty protocol already initialized".to_string(),
        ));
    }

    let royalty_entry = RoyaltyEntry {
        recipient: recipient.clone(),
        share,
        updated: Some(env.block.time),
    };
    royalty_entry.validate()?;
    ROYALTY_PROTOCOLS.save(
        deps.storage,
        royalty_protocol_key,
        &RoyaltyProtocol {
            collection: collection.clone(),
            protocol: protocol.clone(),
            royalty_entry,
        },
    )?;

    response = response.add_event(
        Event::new("set-collection-royalty-protocol").add_attributes(vec![
            attr("collection", collection.to_string()),
            attr("protocol", protocol.to_string()),
            attr("recipient", recipient.to_string()),
            attr("share", share.to_string()),
            attr("updated", env.block.time.to_string()),
        ]),
    );

    Ok(response)
}

#[allow(clippy::too_many_arguments)]
pub fn execute_update_collection_royalty_protocol(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    collection: Addr,
    protocol: Addr,
    recipient: Option<Addr>,
    share_delta: Option<Decimal>,
    decrement: Option<bool>,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    only_collection_creator(deps.as_ref(), &info, &collection)?;

    let config = CONFIG.load(deps.storage)?;
    let mut response = Response::new();

    let royalty_protocol_key: RoyaltyProtocolKey = (collection.clone(), protocol);
    let mut royalty_protocol = ROYALTY_PROTOCOLS
        .load(deps.storage, royalty_protocol_key.clone())
        .map_err(|_| {
            ContractError::InvalidCollectionRoyalty("Collection royalty does not exist".to_string())
        })?;

    if let Some(updated) = royalty_protocol.royalty_entry.updated {
        ensure!(
            updated.plus_seconds(config.update_wait_period) <= env.block.time,
            ContractError::Unauthorized("Royalty entry cannot be updated yet".to_string())
        );
    }

    let mut event = Event::new("update-collection-royalty-protocol")
        .add_attribute("collection", collection.to_string());

    if let Some(recipient) = recipient {
        royalty_protocol.royalty_entry.recipient = recipient.clone();
        event = event.add_attribute("recipient", recipient.to_string());
    }

    if let Some(share_delta) = share_delta {
        royalty_protocol
            .royalty_entry
            .update_share(&config, share_delta, decrement)?;
        event = event.add_attribute("share", royalty_protocol.royalty_entry.share.to_string());
    }

    royalty_protocol.royalty_entry.updated = Some(env.block.time);
    royalty_protocol.royalty_entry.validate()?;
    ROYALTY_PROTOCOLS.save(deps.storage, royalty_protocol_key, &royalty_protocol)?;

    response = response.add_event(event);

    Ok(response)
}
