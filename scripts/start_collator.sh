#!/usr/bin/env bash

#cargo build --release

collator_args=(
    --collator 
    --base-path ./collator-data/ 
    --force-authoring 
    --port 40333 
    --ws-port 9946 
    --ws-external 
    --rpc-external 
    --rpc-cors all 
    --rpc-methods unsafe
)

relay_args=(
    --execution wasm 
    --base-path ./relay-data/ 
    --chain /var/www/hashed-substrate/resources/polkadot.json 
    --port 30333
    --ws-port 9944 
    --ws-external 
    --rpc-external 
    --rpc-cors all 
    --rpc-methods unsafe 
)

chain_spec="--chain hashed"
collator_args+=($chain_spec)

#/target/release/hashed key insert --base-path ./collator-data $chain_spec --scheme sr25519 --suri "${MNEMO}" --key-type aura

/var/www/hashed-substrate/target/release/hashed-parachain "${collator_args[@]}" -- "${relay_args[@]}"