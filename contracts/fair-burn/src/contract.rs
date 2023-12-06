use crate::{
    error::ContractError,
    helpers::{bps_to_decimal, calculate_payouts},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg},
    state::{Config, CONFIG},
};
use cosmwasm_std::{
    ensure, to_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Empty, Env, Event, MessageInfo,
    StdResult,
};
use cw2::{get_contract_version, set_contract_version};
use cw_utils::{maybe_addr, NativeBalance};
use sg_std::{create_fund_fairburn_pool_msg, Response, NATIVE_DENOM};
use std::collections::BTreeMap;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        fee_percent: bps_to_decimal(msg.fee_bps),
    };
    config.save(deps.storage)?;

    let event = Event::new("instantiate")
        .add_attribute("action", "instantiate")
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("contract_version", CONTRACT_VERSION)
        .add_attribute("fee_percent", config.fee_percent.to_string());

    Ok(Response::new().add_event(event))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let api = deps.api;

    match msg {
        ExecuteMsg::FairBurn { recipient } => {
            execute_fair_burn(deps, info, maybe_addr(api, recipient)?)
        }
    }
}

pub fn execute_fair_burn(
    deps: DepsMut,
    info: MessageInfo,
    recipient: Option<Addr>,
) -> Result<Response, ContractError> {
    let mut funds_normalized = NativeBalance(info.funds);
    funds_normalized.normalize();

    ensure!(!funds_normalized.is_empty(), ContractError::ZeroFunds);

    let mut response = Response::new();

    let config = CONFIG.load(deps.storage)?;

    let mut payout_map: BTreeMap<String, Vec<Coin>> = BTreeMap::new();

    let fair_burn_pool_key = "fair-burn-pool".to_string();

    for funds in funds_normalized.into_vec() {
        if funds.denom == NATIVE_DENOM {
            let mut event = Event::new("fair-burn");

            let (burn_coin, dist_coin) = calculate_payouts(&funds, &config);

            event = event.add_attribute("burn_amount", burn_coin.amount.to_string());
            response = response.add_message(BankMsg::Burn {
                amount: vec![burn_coin],
            });

            if let Some(dist_coin) = dist_coin {
                match &recipient {
                    Some(recipient) => {
                        payout_map
                            .entry(recipient.to_string())
                            .or_insert(vec![])
                            .push(dist_coin.clone());
                    }
                    None => {
                        event = event.add_attribute("dist_amount", dist_coin.amount.to_string());
                        response =
                            response.add_message(create_fund_fairburn_pool_msg(vec![dist_coin]));
                    }
                }
            }

            response = response.add_event(event);
        } else {
            let (fee_coin, dist_coin) = match recipient {
                Some(_) => calculate_payouts(&funds, &config),
                None => (funds, None),
            };

            payout_map
                .entry(fair_burn_pool_key.clone())
                .or_insert(vec![])
                .push(fee_coin.clone());

            if let Some(dist_coin) = dist_coin {
                let recipient = recipient.as_ref().unwrap().to_string();
                payout_map
                    .entry(recipient)
                    .or_insert(vec![])
                    .push(dist_coin.clone());
            }
        }
    }

    for (entry_key, funds) in payout_map {
        match entry_key {
            k if k == fair_burn_pool_key => {
                let mut event = Event::new("fund-fair-burn-pool");
                for (idx, c) in funds.iter().enumerate() {
                    event = event.add_attribute(format!("coin_{idx}"), c.to_string());
                }
                response = response
                    .add_event(event)
                    .add_message(create_fund_fairburn_pool_msg(funds));
            }
            k => {
                response = response.add_message(BankMsg::Send {
                    to_address: k.to_string(),
                    amount: funds,
                });
            }
        }
    }

    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ContractVersion {} => to_binary(&get_contract_version(deps.storage)?),
        QueryMsg::Config {} => to_binary(&CONFIG.load(deps.storage)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::UpdateConfig { fee_bps } => sudo_update_config(deps, fee_bps),
    }
}

pub fn sudo_update_config(deps: DepsMut, fee_bps: Option<u64>) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    let mut event = Event::new("sudo-update-config");

    if let Some(fee_bps) = fee_bps {
        config.fee_percent = bps_to_decimal(fee_bps);
        event = event.add_attribute("fee_percent", config.fee_percent.to_string());
    }

    config.save(deps.storage)?;

    Ok(Response::new().add_event(event))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new())
}

#[cfg(test)]
mod tests {
    use crate::{
        msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg},
        state::Config,
    };

    use cosmwasm_std::{
        coin, coins, to_binary, Addr, Coin, Decimal, Event, StdResult, Uint128, WasmMsg,
    };
    use cw_multi_test::{
        AppResponse, BankSudo, Contract, ContractWrapper, Executor, SudoMsg as CwSudoMsg, WasmSudo,
    };
    use sg_multi_test::StargazeApp;
    use sg_std::{StargazeMsgWrapper, NATIVE_DENOM};

    const INITIAL_BALANCE: u128 = 5_000_000_000;

    fn contract() -> Box<dyn Contract<StargazeMsgWrapper>> {
        let contract = ContractWrapper::new(super::execute, super::instantiate, super::query)
            .with_sudo(super::sudo);
        Box::new(contract)
    }

    fn fund_account(app: &mut StargazeApp, addr: &Addr, balances: Vec<Coin>) -> StdResult<()> {
        app.sudo(CwSudoMsg::Bank({
            BankSudo::Mint {
                to_address: addr.to_string(),
                amount: balances,
            }
        }))
        .unwrap();

        Ok(())
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
        let mut app = StargazeApp::default();
        let fair_burn_id = app.store_code(contract());

        let creator = Addr::unchecked("creator");

        let init_msg = InstantiateMsg { fee_bps: 500 };
        let msg = WasmMsg::Instantiate {
            admin: None,
            code_id: fair_burn_id,
            msg: to_binary(&init_msg).unwrap(),
            funds: vec![],
            label: "FairBurn".to_string(),
        };
        let response = app.execute(creator, msg.into());

        assert!(response.is_ok());
        assert!(response.unwrap().has_event(&Event::new("instantiate")));
    }

    #[test]
    fn try_sudo_update() {
        let mut app = StargazeApp::default();
        let fair_burn_id = app.store_code(contract());

        let creator = Addr::unchecked("creator");

        let fee_bps = 5000;
        let init_msg = InstantiateMsg { fee_bps };
        let fair_burn = app
            .instantiate_contract(fair_burn_id, creator, &init_msg, &[], "FairBurn", None)
            .unwrap();

        let query_msg = QueryMsg::Config {};
        let response = app
            .wrap()
            .query_wasm_smart::<Config>(fair_burn.clone(), &query_msg)
            .unwrap();
        assert_eq!(
            response.fee_percent,
            Decimal::percent(fee_bps) / Uint128::from(100u64)
        );

        let new_fee_bps = 4000;
        let sudo_msg = SudoMsg::UpdateConfig {
            fee_bps: Some(new_fee_bps),
        };
        let response = app.sudo(CwSudoMsg::Wasm(WasmSudo {
            contract_addr: fair_burn.clone(),
            msg: to_binary(&sudo_msg).unwrap(),
        }));
        assert!(response.is_ok());

        let query_msg = QueryMsg::Config {};
        let response = app
            .wrap()
            .query_wasm_smart::<Config>(fair_burn, &query_msg)
            .unwrap();
        assert_eq!(
            response.fee_percent,
            Decimal::percent(new_fee_bps) / Uint128::from(100u64)
        );
    }

    #[test]
    fn try_execute_fair_burn() {
        let mut app = StargazeApp::default();
        let fair_burn_id = app.store_code(contract());

        let creator = Addr::unchecked("creator");

        let init_msg = InstantiateMsg { fee_bps: 5000 };
        let fair_burn = app
            .instantiate_contract(fair_burn_id, creator, &init_msg, &[], "FairBurn", None)
            .unwrap();

        let burner: Addr = Addr::unchecked("burner0");
        fund_account(&mut app, &burner, coins(INITIAL_BALANCE, NATIVE_DENOM)).unwrap();
        let alt_denom = "uatom";
        fund_account(&mut app, &burner, coins(INITIAL_BALANCE, alt_denom)).unwrap();
        let recipient = Addr::unchecked("recipient0");

        // Burning with no funds fails
        let response = app.execute_contract(
            burner.clone(),
            fair_burn.clone(),
            &ExecuteMsg::FairBurn { recipient: None },
            &[],
        );
        assert!(response.is_err());

        // Burning 0 STARS fails
        let response = app.execute_contract(
            burner.clone(),
            fair_burn.clone(),
            &ExecuteMsg::FairBurn { recipient: None },
            &[coin(0, NATIVE_DENOM)],
        );
        assert!(response.is_err());

        // Burning 1 STARS succeeds
        let response = app
            .execute_contract(
                burner.clone(),
                fair_burn.clone(),
                &ExecuteMsg::FairBurn { recipient: None },
                &[coin(1, NATIVE_DENOM)],
            )
            .unwrap();
        let event = find_event(&response, "wasm-fair-burn").unwrap();
        let burn_amount = find_attribute(event, "burn_amount").unwrap();
        assert_eq!(burn_amount, "1");

        // Burning 2 STARS with duplicate denoms in message succeeds
        let response = app
            .execute_contract(
                burner.clone(),
                fair_burn.clone(),
                &ExecuteMsg::FairBurn { recipient: None },
                &[coin(1, NATIVE_DENOM), coin(1, NATIVE_DENOM)],
            )
            .unwrap();
        let event = find_event(&response, "wasm-fair-burn").unwrap();
        let burn_amount = find_attribute(event, "burn_amount").unwrap();
        assert_eq!(burn_amount, "1");

        // Fees are calculated correctly
        let response = app
            .execute_contract(
                burner.clone(),
                fair_burn.clone(),
                &ExecuteMsg::FairBurn { recipient: None },
                &[coin(11, NATIVE_DENOM)],
            )
            .unwrap();
        let event = find_event(&response, "wasm-fair-burn").unwrap();
        let burn_amount = find_attribute(event, "burn_amount").unwrap();
        assert_eq!(burn_amount, "6");
        let dist_amount = find_attribute(event, "dist_amount").unwrap();
        assert_eq!(dist_amount, "5");

        // Can handle multiple denoms
        let response = app
            .execute_contract(
                burner.clone(),
                fair_burn.clone(),
                &ExecuteMsg::FairBurn { recipient: None },
                &[coin(11, NATIVE_DENOM), coin(11, alt_denom)],
            )
            .unwrap();
        let event = find_event(&response, "wasm-fair-burn").unwrap();
        let burn_amount = find_attribute(event, "burn_amount").unwrap();
        assert_eq!(burn_amount, "6");
        let dist_amount = find_attribute(event, "dist_amount").unwrap();
        assert_eq!(dist_amount, "5");
        let event = find_event(&response, "wasm-fund-fair-burn-pool").unwrap();
        let fair_burn_coin = find_attribute(event, "coin_0").unwrap();
        assert_eq!(fair_burn_coin, format!("11{alt_denom}"));

        // Can handle recipient address on native denom
        let response = app
            .execute_contract(
                burner.clone(),
                fair_burn.clone(),
                &ExecuteMsg::FairBurn {
                    recipient: Some(recipient.to_string()),
                },
                &[coin(11, NATIVE_DENOM)],
            )
            .unwrap();
        let event = find_event(&response, "wasm-fair-burn").unwrap();
        let burn_amount = find_attribute(event, "burn_amount").unwrap();
        assert_eq!(burn_amount, "6");
        let event = find_event(&response, "transfer").unwrap();
        let recipient_address = find_attribute(event, "recipient").unwrap();
        assert_eq!(recipient_address, recipient.to_string());
        let recipient_coin = find_attribute(event, "amount").unwrap();
        assert_eq!(recipient_coin, format!("5{NATIVE_DENOM}"));

        // Can handle recipient address on alt denom
        let response = app
            .execute_contract(
                burner.clone(),
                fair_burn,
                &ExecuteMsg::FairBurn {
                    recipient: Some(recipient.to_string()),
                },
                &[coin(11, alt_denom)],
            )
            .unwrap();
        let event = find_event(&response, "wasm-fund-fair-burn-pool").unwrap();
        let fund_pool_coin = find_attribute(event, "coin_0").unwrap();
        assert_eq!(fund_pool_coin, format!("6{alt_denom}"));
        let event = find_event(&response, "transfer").unwrap();
        let recipient_address = find_attribute(event, "recipient").unwrap();
        assert_eq!(recipient_address, recipient.to_string());
        let recipient_coin = find_attribute(event, "amount").unwrap();
        assert_eq!(recipient_coin, format!("5{alt_denom}"));
    }
}
