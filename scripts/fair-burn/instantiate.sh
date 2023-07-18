FAIR_BURN_ELGAFAR_CODE_ID=2566
MSG=$(cat <<EOF
{
  "fee_bps": 5000
}

EOF
)

starsd tx wasm instantiate $FAIR_BURN_ELGAFAR_CODE_ID  "$MSG"  --label "stargaze-fair-burn" --admin "stars1vs6ezyqu2mk6x9k5uwvzjx0thsvdhq90u3vvw0" \
  --from testnet-1 --gas-prices 0.1ustars --gas-adjustment 1.7 --gas auto \
  -b block -o json | jq .

