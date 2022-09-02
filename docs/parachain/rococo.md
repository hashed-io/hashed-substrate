# Connecting MD5 to Rococo

The MD5 Network is `ParaId = 4088` for now. We are using a temporary status. Once we have a Kusama parachain slot, we can apply for a Rococo permanent slot for testing. This is hard-coded in the `md5.rs` chain_spec currently.

# Create the Rococo spec (could also download one) 
```bash
cd ~/github.com/paritytech

git clone https://github.com/paritytech/polkadot
cd polkadot

cargo build --release

# Generate a raw chain spec
./target/release/polkadot build-spec --chain rococo --disable-default-bootnode --raw > ~/github.com/paritytech/polkadot/rococo-raw.json
```

# Build the Collator 
```bash

cd ~/github.com/hashed-io

git clone https://github.com/hashed-io/hashed-parachain
cd hashed-parachain

cargo build --release
```

# Build the `md5-spec-raw.json` file using defaults

```bash
# build raw spec 
./target/release/hashed-parachain build-spec --chain md5 --raw --disable-default-bootnode > md5-spec-raw.json
```

# Building genesis state and wasm files
```bash
./target/release/hashed-parachain export-genesis-state --chain resources/md5-spec-raw.json > md5-genesis-head
./target/release/hashed-parachain export-genesis-wasm --chain resources/md5-spec-raw.json > md5-wasm
```

# Start Collator 
```bash
./target/release/hashed-parachain \
    --alice \
    --collator \
    --force-authoring \
    --chain resources/md5-spec-raw.json \
    --base-path ~/chain-data/md5 \
    --port 40333 \
    --ws-port 8844 \
    -- \
    --execution wasm \
    --chain ~/github.com/paritytech/polkadot/rococo-raw.json \
    --port 30343 \
    --ws-port 9977
```

# Register the parachain

### In Process