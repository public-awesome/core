use crate::{fetch_or_set_royalties, ContractError};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use cw_multi_test::{Contract, ContractWrapper, Executor};
use cw_utils::maybe_addr;
use test_suite::common_setup::contract_boxes::App;

#[cw_serde]
pub enum TestExecuteMsg {
    TestFetchOrSetRoyalties {
        royalty_registry: String,
        collection: String,
        protocol: Option<String>,
    },
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: TestExecuteMsg,
) -> Result<Response, ContractError> {
    let api = deps.api;

    match msg {
        TestExecuteMsg::TestFetchOrSetRoyalties {
            royalty_registry,
            collection,
            protocol,
        } => {
            let (_royalty_entry, response) = fetch_or_set_royalties(
                deps.as_ref(),
                &api.addr_validate(&royalty_registry)?,
                &api.addr_validate(&collection)?,
                maybe_addr(api, protocol)?.as_ref(),
                Response::new(),
            )?;
            Ok(response)
        }
    }
}

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

pub fn query(_deps: Deps, _env: Env, _msg: Empty) -> StdResult<Binary> {
    to_json_binary(&_msg)
}

pub fn contract_dummy() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

pub fn setup_dummy_contract(router: &mut App, creator: Addr) -> Addr {
    let dummy_contract_id = router.store_code(contract_dummy());
    router
        .instantiate_contract(
            dummy_contract_id,
            creator,
            &Empty {},
            &[],
            "dummy-contract",
            None,
        )
        .unwrap()
}
