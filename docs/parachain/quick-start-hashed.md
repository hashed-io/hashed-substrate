
# Building the Hashed Parachain Spec
```bash

cd ~/github.com/hashed-io

git clone https://github.com/hashed-io/hashed-substrate
cd hashed-substrate

cargo build --release

./target/release/hashed-parachain build-spec --chain hashed --disable-default-bootnode > resources/hashed-spec.json

```

# Build the Raw Spec File
```bash
# build raw spec 
./target/release/hashed-parachain build-spec --chain hashed --raw --disable-default-bootnode > hashed-parachain-raw.json
```

# Building genesis state and wasm files
```bash
./target/release/hashed-parachain export-genesis-state --chain hashed > hashed-genesis-head

./target/release/hashed-parachain export-genesis-wasm --chain hashed > hashed-wasm
```

# Start Collator #1
```bash
./target/release/hashed-parachain \
    --collator \
    --force-authoring \
    --chain hashed \
    --base-path /tmp/parachain/hashed-local \
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


## Insert second collator key
```bash
./target/release/hashed key insert --scheme sr25519 --keystore-path /tmp/parachain/hashed-local/chains/hashed/keystore --key-type aura --suri ""
```
