pub mod minter {
    use cosmwasm_schema::cw_serde;

    #[cw_serde]
    pub struct InstantiateMsg {
        pub collection: String,
    }
}
