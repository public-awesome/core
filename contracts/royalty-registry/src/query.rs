use crate::{
    msg::{QueryMsg, RoyaltyPaymentResponse},
    state::{
        Config, RoyaltyDefault, RoyaltyProtocol, RoyaltyProtocolKey, CONFIG, ROYALTY_DEFAULTS,
        ROYALTY_PROTOCOLS,
    },
};

use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, Env, StdResult};
use cw_utils::maybe_addr;
use sg_index_query::{QueryOptions, QueryOptionsInternal};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let api = deps.api;

    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::CollectionRoyaltyDefault { collection } => to_json_binary(
            &query_collection_royalty_default(deps, api.addr_validate(&collection)?)?,
        ),
        QueryMsg::CollectionRoyaltyProtocol {
            collection,
            protocol,
        } => to_json_binary(&query_collection_royalty_protocol(
            deps,
            api.addr_validate(&collection)?,
            api.addr_validate(&protocol)?,
        )?),
        QueryMsg::RoyaltyProtocolByCollection {
            collection,
            query_options,
        } => to_json_binary(&query_royalty_protocol_by_collection(
            deps,
            api.addr_validate(&collection)?,
            query_options.unwrap_or_default(),
        )?),
        QueryMsg::RoyaltyPayment {
            collection,
            protocol,
        } => to_json_binary(&query_royalty_payment(
            deps,
            api.addr_validate(&collection)?,
            maybe_addr(api, protocol)?,
        )?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}

pub fn query_collection_royalty_default(
    deps: Deps,
    collection: Addr,
) -> StdResult<Option<RoyaltyDefault>> {
    let royalty_default = ROYALTY_DEFAULTS.may_load(deps.storage, collection)?;
    Ok(royalty_default)
}

pub fn query_collection_royalty_protocol(
    deps: Deps,
    collection: Addr,
    protocol: Addr,
) -> StdResult<Option<RoyaltyProtocol>> {
    let royalty_protocol_key: RoyaltyProtocolKey = (collection, protocol);
    let royalty_protocol = ROYALTY_PROTOCOLS.may_load(deps.storage, royalty_protocol_key)?;
    Ok(royalty_protocol)
}

pub fn query_royalty_protocol_by_collection(
    deps: Deps,
    collection: Addr,
    query_options: QueryOptions<String>,
) -> StdResult<Vec<RoyaltyProtocol>> {
    let QueryOptionsInternal {
        limit,
        order,
        min,
        max,
    } = query_options.unpack(
        &Box::new(|sa: &String| Addr::unchecked(sa.clone())),
        None,
        None,
    );

    let royalty_protocols: Vec<RoyaltyProtocol> = ROYALTY_PROTOCOLS
        .prefix(collection)
        .range(deps.storage, min, max, order)
        .take(limit)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<_>>()?;

    Ok(royalty_protocols)
}

pub fn query_royalty_payment(
    deps: Deps,
    collection: Addr,
    protocol: Option<Addr>,
) -> StdResult<RoyaltyPaymentResponse> {
    let royalty_default = ROYALTY_DEFAULTS.may_load(deps.storage, collection.clone())?;

    let mut royalty_protocol = None;
    if let Some(protocol_val) = &protocol {
        let royalty_protocol_key: RoyaltyProtocolKey = (collection, protocol_val.clone());
        royalty_protocol = ROYALTY_PROTOCOLS.may_load(deps.storage, royalty_protocol_key)?;
    }

    Ok(RoyaltyPaymentResponse {
        royalty_default,
        royalty_protocol,
    })
}
