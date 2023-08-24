use crate::state::{Config, RoyaltyDefault, RoyaltyProtocol};

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;
use sg_index_query::QueryOptions;

#[cw_serde]
pub struct InstantiateMsg {
    pub config: Config,
}

#[cw_serde]
pub enum ExecuteMsg {
    InitializeCollectionRoyalty {
        collection: String,
    },
    SetCollectionRoyaltyDefault {
        collection: String,
        recipient: String,
        share: Decimal,
    },
    UpdateCollectionRoyaltyDefault {
        collection: String,
        recipient: Option<String>,
        share_delta: Option<Decimal>,
        decrement: Option<bool>,
    },
    SetCollectionRoyaltyProtocol {
        collection: String,
        protocol: String,
        recipient: String,
        share: Decimal,
    },
    UpdateCollectionRoyaltyProtocol {
        collection: String,
        protocol: String,
        recipient: Option<String>,
        share_delta: Option<Decimal>,
        decrement: Option<bool>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
    #[returns(Option<RoyaltyDefault>)]
    CollectionRoyaltyDefault { collection: String },
    #[returns(Option<RoyaltyProtocol>)]
    CollectionRoyaltyProtocol {
        collection: String,
        protocol: String,
    },
    #[returns(Vec<RoyaltyProtocol>)]
    RoyaltyProtocolByCollection {
        collection: String,
        query_options: Option<QueryOptions<String>>,
    },
    #[returns(RoyaltyPaymentResponse)]
    RoyaltyPayment {
        collection: String,
        protocol: Option<String>,
    },
}

#[cw_serde]
pub struct RoyaltyPaymentResponse {
    pub royalty_default: Option<RoyaltyDefault>,
    pub royalty_protocol: Option<RoyaltyProtocol>,
}

#[cw_serde]
pub enum SudoMsg {
    UpdateConfig { config: Config },
}
