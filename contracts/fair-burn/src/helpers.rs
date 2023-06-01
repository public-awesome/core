use crate::msg::ExecuteMsg;

use cosmwasm_std::{to_binary, Addr, Coin, WasmMsg};
use sg_std::Response;

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
