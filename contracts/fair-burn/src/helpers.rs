use cosmwasm_std::{coin, to_json_binary, Addr, Coin, Decimal, Uint128, WasmMsg};
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

/// Invoke `append_fair_burn_msg` to pay the fair burn contract and distribute funds.
///
/// # Arguments
///
/// * `fair_burn_addr` - The address of the fair burn contract.
/// * `funds` - A vector of [cosmwasm_std::Coin] to be distributed.
/// * `recipient` - A recipient address that recieve excess funds (optional).
/// * `response` - The [cosmwasm_std::Response] object used to append the message.
///
/// # Returns
///
/// * `Response` - The [cosmwasm_std::Response] with the appended message.
///
pub fn append_fair_burn_msg(
    fair_burn_addr: &Addr,
    funds: Vec<Coin>,
    recipient: Option<&Addr>,
    response: Response,
) -> Response {
    response.add_message(WasmMsg::Execute {
        contract_addr: fair_burn_addr.to_string(),
        msg: to_json_binary(&ExecuteMsg::FairBurn {
            recipient: recipient.map(|r| r.to_string()),
        })
        .unwrap(),
        funds,
    })
}

pub fn bps_to_decimal(bps: u64) -> Decimal {
    Decimal::percent(bps) / Uint128::from(100u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_fair_burn_msg() {
        let fair_burn_addr = Addr::unchecked("fair-burn");
        let funds = vec![coin(100, "uusd")];
        let recipient = Some(Addr::unchecked("recipient"));
        let response = Response::default();

        let response = append_fair_burn_msg(&fair_burn_addr, funds, recipient.as_ref(), response);
        assert_eq!(response.messages.len(), 1);
    }
}
