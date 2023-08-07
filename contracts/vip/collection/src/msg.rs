use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CustomMsg, Uint128};

use crate::state::Metadata;

#[cw_serde]
pub struct InstantiateMsg {
    pub minter_code_id: u64,
    pub name_collection: String,
}

// cw721_base::ExecuteMsg::Mint {
//     token_id,
//     owner,
//     token_uri,
//     extension,
// } => todo!(),

#[cw_serde]
pub enum ExecuteExt {
    UpdateToken {
        token_id: String,
        owner: String,
        token_uri: Option<String>,
        extension: Metadata,
    },
}
impl CustomMsg for ExecuteExt {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Metadata)]
    Metadata { address: String },
    #[returns(Uint128)]
    TotalStaked { owner: String },
}
