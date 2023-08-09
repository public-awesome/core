use cosmwasm_schema::write_api;

use sg_vip::minter::InstantiateMsg;
use stargaze_vip_minter::msg::{ExecuteMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
