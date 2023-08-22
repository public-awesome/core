CODE_ID=2753
MSG=$(cat <<EOF
{
  "config": {
    "update_wait_period": 30,
    "max_share_delta": "0.02"
  }
}
EOF
)

FROM="hot-wallet"
ADMIN="stars19mmkdpvem2xvrddt8nukf5kfpjwfslrsu7ugt5"
CHAIN_ID="elgafar-1"
NODE="https://rpc.elgafar-1.stargaze-apis.com:443"

starsd tx wasm instantiate $CODE_ID  "$MSG"  \
  --label "stargaze-fair-burn" \
  --from "$FROM" \
  --chain-id "$CHAIN_ID" \
  --node "$NODE" \
  --gas-prices 1ustars \
  --gas-adjustment 1.7 \
  --gas auto \
  --admin "$ADMIN" \
  -b block \
  -o json | jq .