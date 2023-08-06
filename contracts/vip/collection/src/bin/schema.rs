use cosmwasm_schema::write_api;

use cw721_base::InstantiateMsg;
use sg_vip::collection::ExecuteMsg;
use stargaze_vip_collection::msg::QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
