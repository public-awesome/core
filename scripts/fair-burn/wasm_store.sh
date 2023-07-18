set -eux

CONTRACT=artifacts/stargaze_fair_burn.wasm

TITLE="Stargaze Fair Burn v1.0.4" 
DESCRIPTION=$(cat scripts/markdown/stargaze_fair_burn-v1.0.4.md | jq -Rsa | tr -d '"')
SOURCE="https://github.com/public-awesome/core/releases/tag/stargaze_fair_burn-v1.0.4"
BUILDER="cosmwasm/workspace-optimizer:0.12.13"
HASH="886deb781278824762fc9d0d167e1e4fd31fd5d9aec34356d5d02889b0223457"

FROM="hot-wallet"
DEPOSIT="10000000000ustars"

RUN_AS="stars19mmkdpvem2xvrddt8nukf5kfpjwfslrsu7ugt5"
ANY_OF_ADDRS="stars19mmkdpvem2xvrddt8nukf5kfpjwfslrsu7ugt5,stars1r5ecq7zn6hwh5e68e79ume8rp9ht7kjz352drk"

CHAIN_ID="elgafar-1"
NODE="https://rpc.elgafar-1.stargaze-apis.com:443"

starsd tx gov submit-proposal wasm-store "$CONTRACT" \
 --title "$TITLE" \
 --description "$DESCRIPTION" \
 --code-source-url "$SOURCE" \
 --builder "$BUILDER" \
 --code-hash "$HASH" \
 --from "$FROM" \
 --deposit "$DEPOSIT" \
 --run-as "$RUN_AS" \
 --instantiate-anyof-addresses "$ANY_OF_ADDRS" \
 --chain-id "$CHAIN_ID" \
 --node "$NODE" \
 --gas-prices 1ustars \
 --gas auto \
 --gas-adjustment 1.5
