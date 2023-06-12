use cosmwasm_std::{coin, to_binary, Addr, Coin, Uint128, WasmMsg};
use sg_std::Response;

use crate::{msg::ExecuteMsg, state::Config};

pub fn calculate_payouts(funds: &Coin, config: &Config) -> (Coin, Option<Coin>) {
    let denom = funds.denom.clone();

    let protocol_amount = funds.amount.mul_ceil(config.fee_percent);
    let protocol_coin = coin(protocol_amount.u128(), &denom);

    let dist_coin = match funds.amount - protocol_amount {
        amount if amount > Uint128::zero() => Some(coin(amount.u128(), denom)),
        _ => None,
    };

    (protocol_coin, dist_coin)
}

pub fn append_fair_burn_msg(
    fair_burn_addr: &Addr,
    funds: Vec<Coin>,
    recipient: Option<&Addr>,
    response: Response,
) -> Response {
    response.add_message(WasmMsg::Execute {
        contract_addr: fair_burn_addr.to_string(),
        msg: to_binary(&ExecuteMsg::FairBurn {
            recipient: recipient.map(|r| r.to_string()),
        })
        .unwrap(),
        funds,
    })
}
