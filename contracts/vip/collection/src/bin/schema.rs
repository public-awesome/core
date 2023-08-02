use cosmwasm_schema::write_api;

use cw721_base::InstantiateMsg;
use stargaze_vip_collection::msg::{ExecuteMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
