pub mod minter {
    use cosmwasm_schema::cw_serde;

    #[cw_serde]
    pub struct InstantiateMsg {
        pub vip_collection: String,
        pub name_collection: String,
    }
}

pub mod collection {
    use cosmwasm_schema::cw_serde;
    use cosmwasm_std::Uint128;

    // #[cw_serde]
    // pub struct InstantiateMsg {
    //     pub minter_code_id: u64,
    //     pub name_collection: String,
    // }

    #[cw_serde]
    pub enum ExecuteMsg {
        Mint {
            name: String,
            owner: String,
            staked_amount: Uint128,
            data: Option<String>,
        },
        UpdateToken {
            name: String,
            owner: String,
            staked_amount: Uint128,
            data: Option<String>,
        },
    }

    // #[cw_serde]
    // #[derive(QueryResponses)]
    // pub enum QueryMsg {
    //     #[returns(Metadata)]
    //     Metadata { address: String },
    //     #[returns(Uint128)]
    //     TotalStaked { owner: String },
    // }
}
