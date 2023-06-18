# Stargaze Core Contracts

## Stargaze Loyalty Program (SLP)

The Stargaze Loyalty Program is a program that rewards users for staking STARS in the form of reduced fees. The program is implemented as a set of NFT smart contracts. Users are assigned to a tier based on the amount of STARS they have staked. The tier determines the amount of fees they pay.

A user mints a Stargaze Loyalty NFT (LNFT) to join the Stargaze Loyalty Program. It contains metadata that includes the amount of STARS they have staked. The staked amount is calculated on mint, and subsequently updated every 24 hours via ABCI end blocker callbacks. LNFTs are non-transferable, and can only be minted by the user that staked the STARS.

A Stargaze Name is required to mint an LNFT. In the future, when account abstraction and sub-accounts are implemented, multiple accounts can be associated with a single LNFT. This is useful in the case, for example, when you have a cold wallet that does the majority of staking, and a hot wallet for use for minting and trading on Stargaze.

## Loyalty Collection (sg721-loyalty)

The Loyalty Collection is a contract that stores all the LNFTs.

```rs
pub struct LoyaltyMetadata {
    pub loyalty_address: Addr,
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

### Queries

```rs
pub enum QueryMsg {
    Metadata { address: String },
    TotalStakedAmount { owner: String },
}
```

Note that a user may have multiple wallets that stake. For example they may have a cold wallet that does the majority of staking, and a hot wallet for use for minting on Stargaze. To determine the tier of a user, we need to sum up the amount of STARS staked across all wallets. To associate wallets, a user can transfer their LNFT to their hot wallet. The `TotalStakedAmount` query returns the total amount of STARS staked by a user across all wallets.

## Loyalty Minter (loyalty-minter)

The Loyalty Minter is a contract that allows users to mint LNFTs. It is initialized with a minimum stake amount required to mint an LNFT. The minimum stake amount is denominated in STARS.

```rs
pub struct Instantiate {
    pub minimum_stake_amount: Coin,
}
```

```rs
pub struct MintMsg<T> {
    pub token_id: String,           // auto-incrementing ID
    pub owner: String,
    pub token_uri: Option<String>,  // ignored
    pub extension: T,               // `LoyaltyMetadata`
}
```
