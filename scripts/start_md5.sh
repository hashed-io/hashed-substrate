#!/usr/bin/env bash

cargo build --release

./target/release/hashed key insert --base-path ./hashed-chaos-data --chain ./md5Spec.json --scheme sr25519 --suri "${MNEMO}" --key-type aura

./target/release/hashed key insert --base-path ./hashed-chaos-data --chain ./md5Spec.json --scheme ed25519 --suri "${MNEMO}" --key-type gran

./target/release/hashed --base-path ./hashed-chaos-data/ --chain md5Spec.json --ws-external --rpc-external --rpc-cors all --rpc-methods unsafe --validator --node-key ${NODEKEY}
