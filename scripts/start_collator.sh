#!/usr/bin/env bash


usage="./start_collator.sh [hashed|luhn] <base-data-path>"
if [ $# -ne 2 ]; then
    echo $usage
    exit 1
fi

if [[ ($1 != 'hashed' && $1 != 'luhn') ]]; then
    echo $usage
    exit 1
fi

#cargo build --release

collator_args=(
    --collator
    --base-path $2/collator-data/
    --force-authoring
    --port 40333
    --ws-port 9946
    --ws-external
    --rpc-external
    --rpc-cors all
    --rpc-methods unsafe
    --chain $1
)

relay_args=(
    --execution wasm
    --base-path $2/relay-data/
    --chain /var/www/hashed-substrate/resources/polkadot.json
    --port 30333
    --ws-port 9944
    --ws-external
    --rpc-external
    --rpc-cors all
    --rpc-methods unsafe
    --wasm-execution Compiled
    --pruning 10000
)

collator_args+=($chain_spec)

#/target/release/hashed key insert --base-path ./collator-data $chain_spec --scheme sr25519 --suri "${MNEMO}" --key-type aura
# echo "${collator_args[@]}" -- "${relay_args[@]}"
/var/www/hashed-substrate/target/release/hashed-parachain "${collator_args[@]}" -- "${relay_args[@]}"
