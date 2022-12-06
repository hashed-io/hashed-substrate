#!/usr/bin/env bash


usage="./run-node.sh <chainspec> <data-path>"
if [ $# -ne 2 ]; then
    echo $usage
    exit 1
fi

node_args=(
    --base-path $2
    --ws-external
    --rpc-external
    --rpc-cors all
    --rpc-methods unsafe
    --validator
    --node-key ${NODEKEY}
    --chain $1
)

if [[ ! -z ${BOOTNODES} ]]; then
    node_args+=(--bootnodes ${BOOTNODES})
fi

echo "Inserting keys..."

echo "${node_args[@]}"

./target/release/hashed key insert --base-path $2 $chain_spec --scheme sr25519 --suri "${MNEMO}" --key-type aura

./target/release/hashed key insert --base-path $2 $chain_spec --scheme ed25519 --suri "${MNEMO}" --key-type gran

./target/release/hashed "${node_args[@]}"
