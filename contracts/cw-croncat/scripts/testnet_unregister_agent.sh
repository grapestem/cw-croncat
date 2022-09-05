#!/bin/bash

cd "$(dirname "$0")"
. ./testnet_init_vars.sh

if [ -z "$1"]
then
    echo "Must provide contracts address"
    exit 1
elif [ -z "$2" ]
then
    echo "Must provide agent address"
    exit 1
else
    CONTRACT="$1"
    AGENT="$2"
fi

UNREGISTER_AGENT='{"unregister_agent":{}}'
junod tx wasm execute $CONTRACT "$UNREGISTER_AGENT" --from $AGENT $TXFLAG -y