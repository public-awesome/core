# Stargaze Core Contracts

## Stargaze Loyalty Program (SLP)

The Stargaze Loyalty Program is a program that rewards users for staking STARS in the form of reduced fees. The program is implemented as a set of NFT smart contracts. Users are assigned to a tier based on the amount of STARS they have staked. The tier determines the amount of fees they pay.

A user mints a Stargaze Loyalty NFT (LNFT) to join the Stargaze Loyalty Program. It contains metadata that includes the amount of STARS they have staked that is populated via an oracle.

A set of privileged operators as determined by governance are allowed to update the metadata of the LNFT. This is done via an oracle that reads the amount of STARS staked by the user and updates the LNFT metadata.

## Loyalty Collection (sg721-loyalty)

The Loyalty Collection is a contract that stores all the LNFTs, and allows operators to update the metadata of the LNFTs.

```rs
pub struct LoyaltyMetadata {
    pub staked_amount: [Coin],
    pub data: Option<String>,
    pub updated_at: u64,
}
```

An optional `data` field makes the metadata future-proof and open for extension.

### Messages

```rs
pub enum ExecuteMsg {
    UpdateMetadata {
        address: String,
        staked_amount: [Coin],
        data: Option<String>,
    },
}
```

```rs
pub enum SudoMsg {
    AddOperator { operator: String },
    RemoveOperator { operator: String },
}
```

### Queries

```rs
pub enum QueryMsg {
    Metadata { address: String },
    Operators {},
    TotalStakedAmount { owner: String },
}
```

Note that a user may have multiple wallets that stake. For example they may have a cold wallet that does the majority of staking, and a hot wallet for use for minting on Stargaze. To determine the tier of a user, we need to sum up the amount of STARS staked across all wallets. To associate wallets, a user can transfer their LNFT to their hot wallet. The `TotalStakedAmount` query returns the total amount of STARS staked by a user across all wallets.

## Loyalty Minter (loyalty-minter)

The Loyalty Minter is a contract that allows users to mint LNFTs.

```rs
pub struct MintMsg<T> {
    pub token_id: String,           // auto-incrementing ID
    pub owner: String,
    pub token_uri: Option<String>,  // ignored
    pub extension: T,               // `LoyaltyMetadata`
}
```
