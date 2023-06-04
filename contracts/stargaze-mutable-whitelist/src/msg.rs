use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    /// Set to `true` for Cosmos and Stargaze addresses, `false` for Ethereum and others.
    pub bech32: bool,
    /// The address that can add and remove addresses from the whitelist.
    pub owner: String,
}
