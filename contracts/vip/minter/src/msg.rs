use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub collection_code_id: u64,
    pub update_interval: u64, // in blocks
}

#[cw_serde]
pub struct ConfigResponse {
    pub vip_collection: String,
    pub update_interval: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Mint a loyalty token for the given name
    Mint {},
    /// Update the stake amount for the given name
    Update { token_id: u64 },
    /// So we can pause before migrating names, etc.
    Pause {},
    /// To resume paused operations
    Resume {},
    /// Update the minter config params
    UpdateConfig {
        vip_collection: Option<String>,
        update_interval: Option<u64>,
    },
}

// #[allow(clippy::large_enum_variant)]
#[cw_serde]
pub enum SudoMsg {
    BeginBlock {}, // Is called by x/cron module BeginBlocker
    EndBlock {},   // Is called by x/cron module EndBlocker
    // UpdateParams {
    //     // fair_burn: Option<String>,
    //     // trading_fee_percent: Option<Decimal>,
    //     // min_bid_increment_percent: Option<Decimal>,
    // },
    UpdateConfig {
        vip_collection: Option<String>,
        update_interval: Option<u64>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(bool)]
    IsPaused {},
    #[returns(u64)]
    TokenUpdateHeight { token_id: u64 },
}
