use stargaze_fair_burn::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};

use cosmwasm_schema::write_api;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
        sudo: SudoMsg,
    }
}
