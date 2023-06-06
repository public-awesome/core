use cosmwasm_schema::write_api;
use sg_mutable_whitelist::{ExecuteMsg, QueryMsg};
use stargaze_mutable_whitelist::msg::InstantiateMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
