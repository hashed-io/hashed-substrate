# Quick Start 
This is a quick guide on connecting the parachain to a local testnet relay chain. 

# Launch the Relay Chain
```bash
cd ~/github.com/paritytech

git clone https://github.com/paritytech/polkadot
cd polkadot

cargo build --release

# Generate a raw chain spec
./target/release/polkadot build-spec --chain rococo-local --disable-default-bootnode --raw > ~/github.com/paritytech/polkadot/rococo-custom-2-raw.json

# Alice
./target/release/polkadot --alice --validator --base-path /tmp/relay/alice --chain ~/github.com/paritytech/polkadot/rococo-custom-2-raw.json --port 30333 --ws-port 9944

# Bob (In a separate terminal)
./target/release/polkadot --bob --validator --base-path /tmp/relay/bob --chain ~/github.com/paritytech/polkadot/rococo-custom-2-raw.json --port 30334 --ws-port 9945
```

# Reserve the Para ID 
Go to https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/parachains/parathreads

and Click `+ParaID`

# Launch the Parachain
```bash

cd ~/github.com/hashed-io

git clone https://github.com/hashed-io/hashed-parachain
cd hashed-parachain

cargo build --release

./target/release/hashed-parachain build-spec --chain md5 --disable-default-bootnode > md5-local-parachain.json
```

# Add the ParaID
Update `md5-local-parachain.json` and change the parachain ID to 2000 in two places.

```json
// --snip--
  "para_id": 2000,
// --snip--
  "parachainInfo": {
      "parachainId": 2000 
  },
// --snip--
```

# Build the Raw Spec File
```bash
# build raw spec 
./target/release/hashed-parachain build-spec --chain md5-local-parachain.json --raw --disable-default-bootnode > md5-local-parachain-raw.json
```

# Building genesis state and wasm files
```bash
./target/release/hashed-parachain export-genesis-state --chain md5-local-parachain-raw.json > md5-genesis-head

./target/release/hashed-parachain export-genesis-wasm --chain md5-local-parachain-raw.json > md5-wasm
```

# Start Collator 
```bash
./target/release/hashed-parachain \
    --alice \
    --collator \
    --force-authoring \
    --chain md5-local-parachain-raw.json \
    --base-path /tmp/parachain/alice \
    --port 40333 \
    --ws-port 8844 \
    -- \
    --execution wasm \
    --chain ~/github.com/paritytech/polkadot/rococo-custom-2-raw.json \
    --port 30343 \
    --ws-port 9977

```

## Sudo Register the parachain
![image](https://user-images.githubusercontent.com/2915325/99548884-1be13580-2987-11eb-9a8b-20be658d34f9.png)


### Purging the Chains
```bash
# Purge a chain
./target/release/hashed-parachain \
    purge-chain \
    --base-path /tmp/parachain/alice \
    --chain ~/github.com/hashed-io/hashed-parachain/md5-local-parachain-raw.json

# Purge relay chain
./target/release/polkadot purge-chain --base-path /tmp/relay/alice --chain ~/github.com/paritytech/polkadot/rococo-custom-2-raw.json 

# Sometimes I use this:
rm -rf /tmp/relay && rm -rf /tmp/parachain
```
