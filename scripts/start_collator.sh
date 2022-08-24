#!/usr/bin/env bash

cargo build --release


collator_args=(
    --collator 
    --base-path ./collator-data/ 
    --force-authoring 
    --port 40333 
    --ws-port 8844 
    --ws-external 
    --rpc-external 
    --rpc-cors all 
    --rpc-methods unsafe
    --node-key ${NODEKEY}
)

relay_args=(
    --execution wasm 
    --base-path ./relay-data/ 

    --chain ./resources/rococo-raw.json 
    --port 30343 
    --ws-port 9977 
    --ws-external 
    --rpc-external 
    --rpc-cors all 
    --rpc-methods unsafe 
    # --node-key ${NODEKEY}
)

chain_spec="--chain resources/md-spec-raw.json"
collator_args+=($chain_spec)

echo "Inserting keys..."

./target/release/hashed key insert --base-path ./collator-data $chain_spec --scheme sr25519 --suri "${MNEMO}" --key-type aura

./target/release/hashed-parachain "${collator_args[@]}" -- "${relay_args[@]}"

./target/release/hashed "${node_args[@]}"