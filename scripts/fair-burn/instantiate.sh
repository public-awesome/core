CODE_ID=74
MSG=$(cat <<EOF
{
  "fee_bps": 5000
}
EOF
)

FROM="hot-wallet"
CHAIN_ID="stargaze-1"
NODE="https://rpc.stargaze-apis.com:443"

starsd tx wasm instantiate $CODE_ID  "$MSG"  \
  --label "stargaze-fair-burn" \
  --from "$FROM" \
  --chain-id "$CHAIN_ID" \
  --node "$NODE" \
  --gas-prices 1ustars \
  --gas-adjustment 1.7 \
  --gas auto \
  --no-admin \
  -b block \
  -o json | jq .
