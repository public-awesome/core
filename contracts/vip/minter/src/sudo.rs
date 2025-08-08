#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env};
use sg_std::Response;

use crate::{msg::SudoMsg, ContractError};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::BeginBlock {} => sudo_begin_block(deps, env),
        SudoMsg::EndBlock {} => sudo_end_block(deps, env),
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
