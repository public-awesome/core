pub mod minter {
    use cosmwasm_schema::cw_serde;

    #[cw_serde]
    pub struct InstantiateMsg {
        pub vip_collection: String,
        pub name_collection: String,
        pub update_interval: u64, // in blocks
    }
}
