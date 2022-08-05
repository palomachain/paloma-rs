# Wormhole Contract Deployment

This readme describes the steps for building, verifying, and deploying smart contracts for Wormhole.

**WARNING**: *This process is only Linux host compatible at this time.*

## Build and Deploy Contracts on Testnet

Contracts can be built in this repository with just

```console
cargo wasm
```

Which is an alias for `cargo build --release --target wasm32-unknown-unknown`.
It is helpful to optimize the output wasm. 
Upon completion, the compiled `wormhole.wasm` and `pyth-bridge.wasm` files can
be found in `target/wasm32-unknown-unknown/release/`. Both of these contracts
should be uploaded with the command

```bash
PUBKEY=<your pubkey>
for contract in wormhole.wasm pyth-bridge.wasm; do
  palomad tx wasm store \
    "$contract" \
    --from $PUBKEY \
    --output json \
    --gas auto \
    --fees 5000ugrain \
    --chain-id paloma-testnet-6 \
    -y -b block
done
```

We'll want to keep track of the addresses of these contracts for later.
Instantiate the wormhole contract:

```bash
# These values are obtained from https://github.com/certusone/wormhole-examples/blob/main/receiver/evm/.env.testnet
# INIT_SIGNERS=["0x13947Bd48b18E53fdAeEe77F3473391aC727C638"] has been translated to base64.
JSON="$(cat <<EOT
{
  "gov_chain": 1,
  "gov_address": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQ=",
  "guardian_set_expirity": 86400,
  "initial_guardian_set": {
    "addresses": [ { "bytes": "E5R71IsY5T/a7ud/NHM5Gscnxjg=" } ],
    "expiration_time": 0
  }
}
EOT
)"

exec palomad tx wasm instantiate \
6 "$JSON" \
--from "$PUBKEY" \
--fees 400ugrain \
--label "wormhole" \
--chain-id paloma-testnet-6 \
--gas auto \
-y --no-admin -b block
```

This is instantiated on `paloma-testnet-6` as `WORMHOLE=paloma12fykm2xhg5ces2vmf4q2aem8c958exv3v0wmvrspa8zucrdwjedsqk2609`.
Now we can instantiate the pyth bridge contract:

```bash
JSON="$(cat <<EOT
{
  "pyth_emitter": "80YZWsAvN9YNTbj/pu90yxvjVQBHVDpKnums9NeGl7A=",
  "pyth_emitter_chain": 1,
  "wormhole_contract": "$WORMHOLE"
}
EOT
)"

exec palomad tx wasm instantiate \
7 "$JSON" \
--from "$PUBKEY" \
--fees 400ugrain \
--label "wormhole" \
--chain-id paloma-testnet-6 \
--gas auto \
-y --no-admin -b block
```

With the bridge contract instantiated we can submit pyth VAA's to the feed and query their value.
Another contract could base decisions on this feed.

```bash
ALGO_USD="0x08f781a893bc9340140c5f89c8a96f438bcfae4d1474cc0f688e3a52892c7318"
BTC_USD="0xf9c0172ba10dfa4d19088d94f5bf61d3b54d5bd7483a322a982e1373ee8ea31b"
ETH_USD="0xca80ba6dc32e08d06f1aa886011eed1d77c77be9eb761cc10d72b7d0a2fd57a6"
LUNA_USD="0x677dbbf4f68b5cb996a40dfae338b87d5efb2e12a9b2686d1ca16d69b3d7f204"
USDC_USD="0x41f3625971ca2ed2263e78573fe5ce23e13d2558ed3f2e47ab0f84fb9e7ae722"

VAA="$(curl "https://prices.testnet.pyth.network/api/latest_vaas?ids[]=${ALGO_USD}&ids[]=${BTC_USD}&ids[]=${ETH_USD}&ids[]=${USDC_USD}" | jq .[0])"

JSON="$(cat <<EOT
{
  "submit_vaa": {
    "data": ${VAA}
  }
}
EOT
)"

# Submit the VAA
palomad tx wasm execute \
"$WORMHOLE" \
"$JSON" \
--chain-id paloma-testnet-6 \
--from "$PUBKEY" \
--fees 400ugrain \
--gas auto \
-y -b block

JSON="$(cat <<EOT
{
  "price_feed": {
    "id": "${ETH_USD:2}"
  }
}
EOT
)"

palomad q wasm contract-state smart \
"$WORMHOLE" \
"$JSON" \
--chain-id paloma-testnet-6
```


