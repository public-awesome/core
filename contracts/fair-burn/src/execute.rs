use crate::{error::ContractError, helpers::calculate_payouts, msg::ExecuteMsg, state::CONFIG};

use cosmwasm_std::{ensure, Addr, BankMsg, Coin, DepsMut, Env, Event, MessageInfo};
use cw_utils::{maybe_addr, NativeBalance};
use sg_std::{create_fund_fairburn_pool_msg, Response, NATIVE_DENOM};
use std::collections::BTreeMap;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

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
