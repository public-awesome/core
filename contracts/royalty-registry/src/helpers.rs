use crate::{
    msg::{ExecuteMsg, QueryMsg, RoyaltyPaymentResponse},
    state::RoyaltyEntry,
    ContractError,
};

use cosmwasm_std::{ensure, to_binary, Addr, Deps, MessageInfo, QuerierWrapper, WasmMsg};
use sg721_base::msg::{CollectionInfoResponse, QueryMsg as Sg721QueryMsg};
use sg_std::Response;

/// Ensures that the sender is the collection contract admin. If a collection contract admin does not exist,
/// then the sender must be the collection contract creator.
pub fn only_collection_creator(
    deps: Deps,
    info: &MessageInfo,
    collection: &Addr,
) -> Result<(), ContractError> {
    let collection_info: CollectionInfoResponse = deps
        .querier
        .query_wasm_smart(collection, &Sg721QueryMsg::CollectionInfo {})?;

    ensure!(
        info.sender == collection_info.creator,
        ContractError::Unauthorized("Only collection owner can execute this action".to_string())
    );

    Ok(())
}

/// Invoke `fetch_royalty_entry` to fetch the royalties for a given NFT sale
/// with an optional protocol address.
///
/// # Arguments
///
/// * `deps` - [cosmwasm_std::Deps]
/// * `royalty_registry` - The address of the royalty registry.
/// * `collection` - The address of the collection contract to fetch royalties for.
/// * `protocol` - The address of the protocol looking to pay royalties (optional).
///
/// # Returns
///
/// * `RoyaltyEntry` - The [RoyaltyEntry] for the given collection and protocol (if any).
///
pub fn fetch_royalty_entry(
    querier: &QuerierWrapper,
    royalty_registry: &Addr,
    collection: &Addr,
    protocol: Option<&Addr>,
) -> Result<Option<RoyaltyEntry>, ContractError> {
    let royalty_payment_response = querier.query_wasm_smart::<RoyaltyPaymentResponse>(
        royalty_registry,
        &QueryMsg::RoyaltyPayment {
            collection: collection.to_string(),
            protocol: protocol.map(|p| p.to_string()),
        },
    )?;

    if let Some(royalty_protocol) = royalty_payment_response.royalty_protocol {
        return Ok(Some(royalty_protocol.royalty_entry));
    }

    if let Some(royalty_default) = royalty_payment_response.royalty_default {
        return Ok(Some(royalty_default.royalty_entry));
    }

    Ok(None)
}

/// Invoke `fetch_or_set_royalties` to fetch the royalties for a given NFT sale
/// with an optional protocol address. If royalties are not found on the royalty registry
/// then the collection contract's royalties are used, and the collection contract's royalties
/// are set on the royalty registry.
///
/// # Arguments
///
/// * `deps` - [cosmwasm_std::Deps]
/// * `royalty_registry` - The address of the royalty registry.
/// * `collection` - The address of the collection contract to fetch royalties for.
/// * `protocol` - The address of the protocol looking to pay royalties (optional).
/// * `response` - The [cosmwasm_std::Response] object used to append the message.
///
/// # Returns
///
/// * `RoyaltyEntry` - The [RoyaltyEntry] for the given collection and protocol (if any).
/// * `Response` - The [cosmwasm_std::Response] with the appended message.
///
pub fn fetch_or_set_royalties(
    deps: Deps,
    royalty_registry: &Addr,
    collection: &Addr,
    protocol: Option<&Addr>,
    mut response: Response,
) -> Result<(Option<RoyaltyEntry>, Response), ContractError> {
    let royalty_entry = fetch_royalty_entry(&deps.querier, royalty_registry, collection, protocol)?;
    if let Some(royalty_entry) = royalty_entry {
        return Ok((Some(royalty_entry), response));
    }

    let collection_info: CollectionInfoResponse = deps
        .querier
        .query_wasm_smart(collection, &Sg721QueryMsg::CollectionInfo {})?;

    if let Some(royalty_info_response) = collection_info.royalty_info {
        let royalty_entry = RoyaltyEntry {
            recipient: deps
                .api
                .addr_validate(&royalty_info_response.payment_address)?,
            share: royalty_info_response.share,
            updated: None,
        };

        response = response.add_message(WasmMsg::Execute {
            contract_addr: royalty_registry.to_string(),
            msg: to_binary(&ExecuteMsg::InitializeCollectionRoyalty {
                collection: collection.to_string(),
            })
            .unwrap(),
            funds: vec![],
        });

        return Ok((Some(royalty_entry), response));
    }

    Ok((None, response))
}
