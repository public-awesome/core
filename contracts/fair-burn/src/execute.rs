use crate::{error::ContractError, helpers::calculate_payouts, msg::ExecuteMsg, state::CONFIG};

use cosmwasm_std::{coin, ensure, Addr, BankMsg, Coin, DepsMut, Env, Event, MessageInfo};
use cw_utils::{maybe_addr, NativeBalance};
use sg_std::{create_fund_fairburn_pool_msg, Response, NATIVE_DENOM};

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

    let mut recipient_funds: Vec<Coin> = vec![];
    let mut fee_manager_funds: Vec<Coin> = vec![];
    let mut funds_normalized_vec = funds_normalized.into_vec();

    while let Some(funds) = funds_normalized_vec.pop() {
        let (protocol_coin, dist_coin) = calculate_payouts(&funds, &config);

        match funds.denom.as_str() {
            NATIVE_DENOM => {
                // For STARS, we burn a percentage of the funds and the rest is
                // distributed to the recipient or the fairburn pool.
                let mut event = Event::new("fair-burn")
                    .add_attribute("burn_amount", protocol_coin.amount.to_string());

                response = response.add_message(BankMsg::Burn {
                    amount: vec![protocol_coin],
                });

                if let Some(dist_coin) = dist_coin {
                    if recipient.is_some() {
                        recipient_funds.push(dist_coin);
                    } else {
                        event = event.add_attribute("dist_amount", dist_coin.amount.to_string());

                        response =
                            response.add_message(create_fund_fairburn_pool_msg(vec![dist_coin]));
                    }
                }

                response = response.add_event(event);
            }
            _ => {
                if recipient.is_some() {
                    fee_manager_funds.push(protocol_coin);
                    if let Some(dist_coin) = dist_coin {
                        recipient_funds.push(dist_coin);
                    }
                } else {
                    let fee_manager_coin = coin(
                        protocol_coin.amount.u128() + dist_coin.map_or(0u128, |c| c.amount.u128()),
                        protocol_coin.denom,
                    );
                    fee_manager_funds.push(fee_manager_coin);
                }
            }
        }
    }

    if !fee_manager_funds.is_empty() {
        response = response.add_message(BankMsg::Send {
            to_address: config.fee_manager.to_string(),
            amount: fee_manager_funds,
        })
    }

    if !recipient_funds.is_empty() {
        response = response.add_message(BankMsg::Send {
            to_address: recipient.unwrap().to_string(),
            amount: recipient_funds,
        })
    }

    Ok(response)
}
