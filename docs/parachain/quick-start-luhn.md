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

git clone https://github.com/hashed-io/hashed-substrate
cd hashed-substrate

cargo build --release

./target/release/hashed-parachain build-spec --chain luhn --disable-default-bootnode > luhn-local-parachain.json
```

# Add the ParaID
Update `luhn-local-parachain.json` and change the parachain ID to 2000 in two places.

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
./target/release/hashed-parachain build-spec --chain luhn-local-parachain.json --raw --disable-default-bootnode > luhn-local-parachain-raw.json
```

# Building genesis state and wasm files
```bash
./target/release/hashed-parachain export-genesis-state --chain luhn-local-parachain-raw.json > luhn-genesis-head

./target/release/hashed-parachain export-genesis-wasm --chain luhn-local-parachain-raw.json > luhn-wasm-upgrade
```

# Building genesis state and wasm files
```bash
./target/release/hashed-parachain export-genesis-state --chain luhn > luhn-genesis-head

./target/release/hashed-parachain export-genesis-wasm --chain luhn > luhn-wasm
```

# Start Collator #1
```bash
./target/release/hashed-parachain \
    --collator \
    --force-authoring \
    --chain luhn \
    --base-path /tmp/parachain/luhn-local \
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


# Generate new WASM
```bash
./target/release/hashed-parachain export-genesis-wasm --chain hashed > hashed-wasm-upgrade
```

# Start Second Collator  
```bash
./target/release/hashed-parachain \
    --bob \
    --collator \
    --force-authoring \
    --chain hashed \
    --base-path /tmp/parachain/bob \
    --port 40334 \
    --ws-port 8845 \
    -- \
    --execution wasm \
    --chain ~/github.com/paritytech/polkadot/rococo-custom-2-raw.json \
    --port 30344 \
    --ws-port 9978

```

## Insert second collator key
```bash
./target/release/hashed key insert --scheme sr25519 --keystore-path /tmp/parachain/hashed-local/chains/hashed/keystore --key-type aura --suri ""
```

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

## Refresh the resources/chain specs

./target/release/hashed-parachain build-spec --chain luhn --disable-default-bootnode > resources/luhn-spec.json

./target/release/hashed-parachain build-spec --chain md5 --disable-default-bootnode > resources/md5-spec.json

./target/release/hashed-parachain build-spec --chain hashed --disable-default-bootnode > resources/hashed-spec.json