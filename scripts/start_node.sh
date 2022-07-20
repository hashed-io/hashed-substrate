#!/usr/bin/env bash

cargo build --release

node_args=(
    --base-path ./hashed-chaos-data/ 
    --ws-external 
    --rpc-external 
    --rpc-cors all 
    --rpc-methods unsafe 
    --validator 
    --node-key ${NODEKEY}
)

if [[ ${ISMAINNET} = true ]]; then
    echo "Mainnet deployment detected"
    chain_spec="--chain chaosSpec.json"
    node_args+=(--bootnodes /ip4/206.221.189.10/tcp/30335/p2p/12D3KooWQxwQyQ3BaCs5tweoTmHNWHbpHePZt6P9SscBps1FWsUc)
    node_args+=(--public-addr ${PUBLICADDR})
else
echo "MD5 deployment detected"
    chain_spec="--chain md5Spec.json"
fi
node_args+=($chain_spec)

echo "Inserting keys..."

./target/release/hashed key insert --base-path ./hashed-chaos-data $chain_spec --scheme sr25519 --suri "${MNEMO}" --key-type aura

./target/release/hashed key insert --base-path ./hashed-chaos-data $chain_spec --scheme ed25519 --suri "${MNEMO}" --key-type gran


./target/release/hashed "${node_args[@]}"