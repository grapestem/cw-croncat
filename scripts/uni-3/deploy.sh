#!/bin/bash
set -e

cd "$(dirname "$0")"
. ./init-vars.sh
cd ../../..

docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  --platform linux/amd64 \
  cosmwasm/workspace-optimizer:0.12.6

if [ "$#" -eq 3 ]
then
    OWNER=$1
    AGENT=$2
    USER=$3
else 
    . ./contract/cw-croncat/scripts/testnet_create_addresses.sh
fi

RES=$(junod tx wasm store artifacts/cw_croncat.wasm --from $OWNER $TXFLAG -y --output json -b block)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')

# Instantiate
INIT='{"denom":"ujunox"}'
junod tx wasm instantiate $CODE_ID "$INIT" --from $OWNER --label "croncat" $TXFLAG -y --no-admin
CONTRACT=$(junod query wasm list-contract-by-code $CODE_ID $NODE --output json | jq -r '.contracts[-1]')

# Now we can register an agent, create tasks and execute a task
# Register an agent
REGISTER_AGENT='{"register_agent":{}}'
junod tx wasm execute $CONTRACT "$REGISTER_AGENT" --from $AGENT $TXFLAG -y

# Create a task
STAKE='{"create_task":{"task":{"interval":"Immediate","boundary":null,"cw20_coins":[],"stop_on_fail":false,"actions":[{"msg":{"staking":{"delegate":{"validator":"juno14vhcdsyf83ngsrrqc92kmw8q9xakqjm0ff2dpn","amount":{"denom":"ujunox","amount":"10000"}}}},"gas_limit":150000}],"rules":null}}}'
junod tx wasm execute $CONTRACT "$STAKE" --amount 10000ujunox --from $USER $TXFLAG -y

# proxy_call
sleep 10      # is needed to make sure this call in the next block 
PROXY_CALL='{"proxy_call":{}}'
junod tx wasm execute $CONTRACT "$PROXY_CALL" --from $AGENT $TXFLAG -y

echo "CONTRACT CODEID - $CODE_ID"
echo "CONTRACT $CONTRACT"
echo "OWNER $OWNER"
echo "AGENT $AGENT"
echo "USER $USER"
