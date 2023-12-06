CODE_ID=105
MSG=$(cat <<EOF
{
  "config": {
    "update_wait_period": 86400,
    "max_share_delta": "0.01"
  }
}
EOF
)

MULTISIG="stars1jwgchjqltama8z0v0smpagmnpjkc8sw8r03xzq"
ADMIN="stars1jwgchjqltama8z0v0smpagmnpjkc8sw8r03xzq"
CHAIN_ID="stargaze-1"
NODE="https://rpc.stargaze-apis.com:443"

starsd tx wasm instantiate $CODE_ID  "$MSG"  \
  --label "stargaze-royalty-registry" \
  --from "$MULTISIG" \
  --admin "$ADMIN" \
  --chain-id "$CHAIN_ID" \
  --node "$NODE" \
  --gas-prices 1ustars \
  --gas 5000000 \
  --generate-only \
  > ./unsigned_instantiate_stargaze_royalty_registry.json
