#!/usr/bin/env bash

echo "*** Initializing becnhmarking"

cargo build --package hashed-runtime --release --features runtime-benchmarks

./target/release/hashed benchmark pallet \
--chain dev \
--pallet pallet_template \
--extrinsic '*' \
--steps 20 \
--repeat 10 \
--output pallets/pallet-template/src/weights.rs
