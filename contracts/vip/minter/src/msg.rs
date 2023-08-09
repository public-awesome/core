use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum ExecuteMsg {
    /// Mint a loyalty token for the given name
    Mint { name: String },
    /// Update the stake amount for the given name
    Update { name: String },
    /// So we can pause before migrating names, etc.
    Pause {},
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
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
