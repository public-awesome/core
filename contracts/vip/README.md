# Stargaze VIP Program (SVP)

The Stargaze VIP Program is a program that rewards users for staking STARS in the form of reduced fees. The program is implemented as a set of NFT smart contracts. Users are assigned to a tier based on the amount of STARS they have staked. The tier determines the amount of fees they pay.

A user mints a Stargaze VIP NFT (vNFT) to join the Stargaze VIP Program. It contains metadata that includes the amount of STARS they have staked that is periodically updated via end blocker. vNFTs are non-transferrable.

A Stargaze Name is required to mint a vNFT.

## VIP Collection (sg721-vip)

The VIP Collection is a contract that stores all the vNFTs, and periodically updates the metadata of each vNFT.

vNFTs are indexed by Stargaze Names, using the name for the `token_id`. If a name is transferred or burned, the associated vNFT is burned. This happens via a hook in the Stargaze Name collection contract.

```rs
pub struct VipMetadata {
    pub staked_amount: [Coin],
    pub data: Option<String>,
    pub updated_at: Timestamp,
}
```

The stake weight metadata can only be updated at specific intervals. The `updated_at` field is used to determine when the metadata can be updated. The `updated_at` field is set to the current block time when the vNFT is minted. The metadata can be updated when the current block time is greater than the `updated_at` field plus the `update_period` field in the config.

```rs
pub struct Config {
    pub update_period: Duration,
}
```

### Messages

```rs
pub struct InstantiateMsg {
    pub update_period: Duration,
}
```

This updates the metadata of a vNFT immediately instead of waiting for the end blocker.

```rs
pub enum ExecuteMsg {
    UpdateMetadata {
        address: String,
        data: Option<String>,
    },
}
```

```rs
pub enum SudoMsg{
    BeginBlock {}, // Is called by x/cron module BeginBlocker
    EndBlock {},   // Is called by x/cron module EndBlocker
    UpdateConfig {
        config: Config,
    },
}
```

### Queries

```rs
pub enum QueryMsg {
    Config {},
    Metadata { address: String },
    TotalStakedAmount { name: String },
}
```

Note that a user may have multiple wallets that stake. For example they may have a cold wallet that does the majority of staking, and a hot wallet for use for minting on Stargaze. To determine the tier of a user, we need to sum up the amount of STARS staked across all wallets. To associate wallets, a user can link their accounts in their Stargaze Name. The `TotalStakedAmount` query returns the total amount of STARS staked by a user across all wallets.

#### TODO

- [ ] Come up with a consistent way to link multiple accounts to one name

## vNFT Minter (vip-minter)

The vNFT Minter is a contract that allows users to mint vNFTs.

```rs
pub struct MintMsg<T> {
    pub name: String,          
    pub owner: String,
    pub token_uri: Option<String>,  // ignored
    pub extension: T,               // `VipMetadata`
}
```