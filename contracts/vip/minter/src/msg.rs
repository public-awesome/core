use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub collection_code_id: u64,
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
}

// #[allow(clippy::large_enum_variant)]
#[cw_serde]
pub enum SudoMsg {
    BeginBlock {}, // Is called by x/cron module BeginBlocker
    EndBlock {},   // Is called by x/cron module EndBlocker
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(String)]
    Collection {},
    #[returns(bool)]
    IsPaused {},
    #[returns(u64)]
    TokenUpdateHeight { token_id: u64 },
}
