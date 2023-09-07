#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, Event};
use sg_std::Response;

use crate::{msg::SudoMsg, state::CONFIG, ContractError};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::BeginBlock {} => sudo_begin_block(deps, env),
        SudoMsg::EndBlock {} => sudo_end_block(deps, env),
        SudoMsg::UpdateConfig {
            vip_collection,
            update_interval,
        } => sudo_execute_update_config(deps, vip_collection, update_interval),
        // SudoMsg::UpdateParams {
        //     fair_burn,
        //     trading_fee_percent,
        //     min_bid_increment_percent,
        // } => sudo_update_params(
        //     deps,
        //     env,
        //     fair_burn,
        //     trading_fee_percent,
        //     min_bid_increment_percent,
        // ),
    }
}

pub fn sudo_begin_block(_deps: DepsMut, _env: Env) -> Result<Response, ContractError> {
    Ok(Response::new())
}

pub fn sudo_end_block(_deps: DepsMut, _env: Env) -> Result<Response, ContractError> {
    /*let Config {
        vip_collection,
        update_interval,
        ..
    } = CONFIG.load(deps.storage)?;
    let names = NAME_QUEUE.load(deps.storage, env.block.height)?;

    let mint_msgs = names
        .iter()
        .map(|name| {
            let name = name.clone();
            let owner = associated_address(deps.as_ref(), name.clone())?;
            let mint_msg = mint(
                deps.as_ref(),
                owner,
                env.block.time,
                name,
                vip_collection.clone(),
            )?;
            Ok(mint_msg)
        })
        .collect::<Result<Vec<_>, ContractError>>()?;

    NAME_QUEUE.remove(deps.storage, env.block.height);
    NAME_QUEUE.save(deps.storage, env.block.height + update_interval, &names)?;

    Ok(Response::new().add_messages(mint_msgs))*/
    Ok(Response::new())
}

pub fn sudo_execute_update_config(
    deps: DepsMut,
    vip_collection: Option<String>,
    update_interval: Option<u64>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if let Some(vip_collection) = vip_collection {
        config.vip_collection = deps.api.addr_validate(&vip_collection)?;
    }
    if let Some(update_interval) = update_interval {
        // TODO: define a min and max for update_interval (and update the error)
        if update_interval < 1 {
            return Err(ContractError::InvalidUpdateInterval {});
        }
        config.update_interval = update_interval;
    }
    CONFIG.save(deps.storage, &config)?;
    let event = Event::new("sudo_update_config")
        .add_attribute("vip_collection", config.vip_collection)
        .add_attribute("update_interval", config.update_interval.to_string());
    Ok(Response::new().add_event(event))
}
