#!/usr/bin/env bash

echo "*** Initializing becnhmarking"

cargo build --package node-template --release --features runtime-benchmarks

./target/release/hashed benchmark pallet \                                                                                                                                                               1  6.99  100%
--chain dev \
--pallet pallet_template \
--extrinsic '*' \
--steps 20 \
--repeat 10 \
--output pallets/template/src/weights.rs
