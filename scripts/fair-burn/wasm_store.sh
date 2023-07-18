set -eux

CONTRACT=artifacts/stargaze_fair_burn.wasm

TITLE="Stargaze Fair Burn v1.0.4" 
SOURCE="https://github.com/public-awesome/core/releases/tag/stargaze_fair_burn-v1.0.4"
MARKDOWN="scripts/markdown/stargaze_fair_burn-v1.0.4.md"
DESCRIPTION=$(cat "$MARKDOWN" | base64 | tr -d '\n')
BUILDER="cosmwasm/workspace-optimizer:0.12.13"
HASH="bf1497f4303d20c1db5f1af2ccec7b367e150c84c5e86f6a2798a1c4cc0d52c9"

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