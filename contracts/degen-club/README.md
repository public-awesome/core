# Stargaze Degen Club (SDC)

The Stargaze Degen Club is a loyalty program that rewards users for staking STARS in the form of reduced fees. The program is implemented as a set of NFT smart contracts. Users are assigned to a tier based on the amount of STARS they have staked. The tier determines the amount of fees they pay.

A user mints a Stargaze Loyalty NFT (LNFT) to join the Stargaze Degen Club. It contains metadata that includes the amount of STARS they have staked. The staked amount is calculated on mint, and subsequently updated every 24 hours via ABCI end blocker callbacks. LNFTs are non-transferable, and can only be minted by the user that staked the STARS.

A Stargaze Name is required to mint an LNFT. In the future, when account abstraction and sub-accounts are implemented, multiple accounts can be associated with a single LNFT. This is useful in the case, for example, when you have a cold wallet that does the majority of staking, and a hot wallet for use for minting and trading on Stargaze.

## Loyalty Collection (sg721-loyalty)

The Loyalty Collection is a cw721 contract that stores all the LNFTs.

### State

```rs
pub struct LoyaltyMetadata {
    pub staked_amount: [Coin],
    pub data: Option<String>,
    pub updated_at: u64,
}
```

An optional `data` field makes the metadata future-proof and open for extension.

When new LNFTs are minted, we have to keep track of them in a queue to be updated periodically in `end_block()`. We can use a [`Deque`](https://github.com/cosmWasm/cw-storage-plus#deque) for this. If encountering an LNFT that has been burned, we can remove it from the queue.

```rs
// String = `token_id`
const UPDATE_QUEUE: Deque<String> = Deque::new("uq");
```

Another alternative is to use a `Map` indexed by the block height, and process 24 hours later.

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

## Loyalty Minter (loyalty-minter)

The Loyalty Minter is a contract that allows users to mint LNFTs. It is initialized with a minimum stake amount required to mint an LNFT. The minimum stake amount is denominated in STARS. The Loyalty Minter is a singleton contract.

```rs
pub struct Instantiate {
    pub minimum_stake_amount: Coin,
    pub owner: String, // use cw-ownable
}
```

```rs
pub struct Config {
    pub minimum_stake_amount: Coin,
}
```

```rs
pub struct MintMsg<T> {
    pub token_id: String,           // Stargaze Name
    pub owner: String,
    pub token_uri: Option<String>,  // ignored
    pub extension: T,               // `LoyaltyMetadata`
}
```

The unique `token_id` for the collection is the Stargaze Name since a name is required. On each 24 hour update, the name owner is checked to see if it matches the associated LNFT owner. If it does not, the LNFT is burned. If a user changes their Stargaze Name, they must mint a new LNFT.

Since this minter is a singleton contract, it's config has to be updated via a governance proposal using a sudo message.

```rs
pub enum SudoMsg {
    UpdateConfig {
        minimum_stake_amount: Coin,
    },
}
```
