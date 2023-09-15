#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Empty};
use cw2::set_contract_version;
use cw721::Cw721Query;
use cw721_base::InstantiateMsg;
use cw721_base::state::TokenInfo;

use crate::error::ContractError;
use crate::{ExecuteMsg, QueryMsg, VipCollection};
use crate::state::Metadata;

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

    // This configures the collection with the minter as the owner, the only one that can mint
    VipCollection::default().instantiate(deps.branch(), env, info, msg)
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
        cw721_base::ExecuteMsg::Mint {
        token_id,
        owner,
        token_uri,
        extension,
        } => execute_mint(deps, env, info, token_id, owner, token_uri, extension),
        _ => VipCollection::default()
            .execute(deps, env, info, msg)
            .map_err(Into::into),
    }
}

pub fn execute_mint(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
    owner: String,
    token_uri: Option<String>,
    extension: Metadata,
) -> Result<Response, ContractError> {
    cw_ownable::assert_owner(deps.branch().storage, &info.sender).map_err(|_| ContractError::Unauthorized {})?;

    let token = TokenInfo {
        owner: deps.api.addr_validate(&owner)?,
        approvals: vec![],
        token_uri,
        extension,
    };
    if !VipCollection::default().tokens.has(deps.storage, &token_id) {
        VipCollection::default().increment_tokens(deps.storage)?;
    }
    VipCollection::default().tokens.update(deps.branch().storage, &token_id, |old| match old {
        Some(_) => Ok::<TokenInfo<Metadata>, ContractError>(token),
        None => {
            Ok(token)
        },
    }
        )?;

        Ok(Response::new()
           .add_attribute("action", "mint")
           .add_attribute("minter", info.sender)
           .add_attribute("owner", owner)
           .add_attribute("token_id", token_id))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    VipCollection::default().query(deps, env, msg)
}

#[cfg(test)]
mod tests {
    use crate::state::Metadata;
    use crate::ContractError;
    use cosmwasm_std::{Addr, Empty, Event, Timestamp, Uint128};
    use cw721_base::InstantiateMsg;
    use cw_multi_test::{App, AppResponse, Contract, ContractWrapper, Executor};

    fn collection_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    fn find_event<'a>(response: &'a AppResponse, event_type: &'a str) -> Option<&'a Event> {
        response.events.iter().find(|event| event.ty == event_type)
    }

    fn find_attribute(event: &Event, key: &str) -> Option<String> {
        event
            .attributes
            .iter()
            .find(|attr| attr.key == key)
            .map(|attr| attr.value.clone())
    }

    #[test]
    fn try_instantiate() {
        let mut app = App::default();
        let vip_collection_code_id = app.store_code(collection_contract());

        let creator = Addr::unchecked("creator");

        let init_msg = InstantiateMsg {
            name: "Stargaze VIP Collection".to_string(),
            symbol: "SGVIP".to_string(),
            minter: Addr::unchecked("minter").to_string(),
        };

        let response = app.instantiate_contract(
            vip_collection_code_id,
            creator,
            &init_msg,
            &[],
            "vip collection",
            None,
        );
        assert!(response.is_ok());
    }

    #[test]
    fn try_execute() {
        let mut app = App::default();
        let vip_collection_code_id = app.store_code(collection_contract());

        let creator = Addr::unchecked("creator");

        let init_msg = InstantiateMsg {
            name: "Stargaze VIP Collection".to_string(),
            symbol: "SGVIP".to_string(),
            minter: Addr::unchecked("minter").to_string(),
        };

        let collection_address = app
            .instantiate_contract(
                vip_collection_code_id,
                creator,
                &init_msg,
                &[],
                "vip collection",
                None,
            )
            .unwrap();

        let mint_msg: cw721_base::ExecuteMsg<Metadata, Empty> = cw721_base::ExecuteMsg::Mint {
            token_id: "1".to_string(),
            owner: Addr::unchecked("owner").to_string(),
            token_uri: None,
            extension: Metadata {
                staked_amount: Uint128::new(10000000000),
                data: None,
                updated_at: Timestamp::from_seconds(100),
            },
        };

        let response = app.execute_contract(
            Addr::unchecked("minter"),
            collection_address.clone(),
            &mint_msg,
            &[],
        );
        assert!(response.is_ok());
        let app_response = response.unwrap();
        let event = find_event(&app_response, "wasm").unwrap();
        let action = find_attribute(&event, "action").unwrap();
        assert_eq!(action, "mint");

        let transfer_msg: cw721_base::ExecuteMsg<Metadata, Empty> =
            cw721_base::ExecuteMsg::TransferNft {
                recipient: Addr::unchecked("recipient").to_string(),
                token_id: "1".to_string(),
            };

        let response = app
            .execute_contract(
                Addr::unchecked("owner"),
                collection_address,
                &transfer_msg,
                &[],
            )
            .map_err(|e| e.downcast::<ContractError>().unwrap())
            .unwrap_err();
        assert!(matches!(response, ContractError::Unauthorized {}));
    }
}
