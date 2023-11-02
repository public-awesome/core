set -eux

CONTRACT=artifacts/stargaze_vip_collection.wasm

TITLE="Stargaze Loyalty Program Collection Contract" 
SOURCE="https://github.com/public-awesome/core/releases/tag/..."
MARKDOWN="scripts/markdown/..."
DESCRIPTION=$(cat "$MARKDOWN" | base64 | tr -d '\n')
BUILDER="cosmwasm/workspace-optimizer:0.14.0"
HASH="..."

FROM="stars19mmkdpvem2xvrddt8nukf5kfpjwfslrsu7ugt5"
DEPOSIT="10000000000ustars"

RUN_AS="hot-wallet"
ANY_OF_ADDRS="stars19mmkdpvem2xvrddt8nukf5kfpjwfslrsu7ugt5,stars1r5ecq7zn6hwh5e68e79ume8rp9ht7kjz352drk"

CHAIN_ID="stargaze-1"
NODE="https://rpc.stargaze-apis.com:443"

starsd tx gov submit-proposal wasm-store "$CONTRACT" \
 --title "$TITLE" \
 --description "$(echo "$DESCRIPTION" | base64 --decode)" \
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