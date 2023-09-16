#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdError, StdResult,
    Uint128,
};
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
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(info.sender.as_str()))?;

    COLLECTION.save(deps.storage, &deps.api.addr_validate(&msg.collection)?)?;

    TIERS.save(deps.storage, &msg.tiers)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateTiers { tiers } => execute_update_tiers(deps, info, tiers),
    }
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
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
        QueryMsg::Collection {} => {
            let collection = COLLECTION.load(deps.storage)?;
            Ok(to_binary(&collection)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Empty, Event, Uint128};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    fn program_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(super::execute, super::instantiate, super::query);
        Box::new(contract)
    }

    #[test]
    fn try_instantiate() {
        let mut app = App::default();
        let program_contract_code_id = app.store_code(program_contract());

        let creator = Addr::unchecked("creator");

        let init_msg = InstantiateMsg {
            collection: Addr::unchecked("collection").to_string(),
            tiers: vec![Uint128::new(5000000000), Uint128::new(10000000000)],
        };

        let response = app.instantiate_contract(
            program_contract_code_id,
            creator,
            &init_msg,
            &[],
            "program contract",
            None,
        );
        assert!(response.is_ok())
    }

    #[test]
    fn try_update_tiers() {
        let mut app = App::default();
        let program_contract_code_id = app.store_code(program_contract());

        let creator = Addr::unchecked("creator");

        let init_msg = InstantiateMsg {
            collection: Addr::unchecked("collection").to_string(),
            tiers: vec![Uint128::new(5000000000), Uint128::new(10000000000)],
        };

        let program_contract_address = app
            .instantiate_contract(
                program_contract_code_id,
                creator.clone(),
                &init_msg,
                &[],
                "program contract",
                None,
            )
            .unwrap();

        let tiers = vec![
            Uint128::new(1000000000),
            Uint128::new(2000000000),
            Uint128::new(3000000000),
        ];
        let update_msg = crate::msg::ExecuteMsg::UpdateTiers { tiers };

        let response = app.execute_contract(creator, program_contract_address, &update_msg, &[]);
        assert!(response.is_ok());
        assert!(response
            .unwrap()
            .has_event(&Event::new("wasm-update_tiers")));
    }
}
