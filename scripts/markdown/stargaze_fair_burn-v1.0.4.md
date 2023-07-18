## Store WASM Code

This proposal uploads the code for Stargaze Fair Burn v1.0.4

The source code is available at https://github.com/public-awesome/core/releases/tag/stargaze_fair_burn-v1.0.4

The Stargaze Fair Burn contract is is responsible for handling fees paid by other contracts. Fees can be paid in multiple denoms. The Fair Burn contract performs the following logic:

- If the funds transferred are in STARS, then a percentage of the funds are burned, and the remaining funds are sent either to the treasury, or a specified recipient address.
- If the funds transferred are not in STARS, then a percentage of the funds are sent to the treasury, and the remaining funds are sent either to the treasury (also), or a specified recipient address.
- The specified recipient address can be used as a developer payout address.

To integrate with the Stargaze Fair Burn contract please refer to the following documentation https://crates.io/crates/stargaze-fair-burn

### Compile Instructions

```sh
docker run --rm -v "$(pwd)":/code --platform linux/amd64 \
	--mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
	--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
	cosmwasm/workspace-optimizer:0.12.13
```

This results in the following SHA256 checksum:

```
886deb781278824762fc9d0d167e1e4fd31fd5d9aec34356d5d02889b0223457  stargaze_fair_burn.wasm
```

### Verify On-chain Contract

```sh
starsd q gov proposal $id --output json \\
| jq -r '.content.wasm_byte_code' \\
| base64 -d \\
| gzip -dc \\
| sha256sum

```

### Verify Local Contract

```
sha256sum artifacts/stargaze_fair_burn.wasm
```
