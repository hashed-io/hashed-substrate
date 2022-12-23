./target/release/hashed build-spec --chain=inovaSpec.json --raw --disable-default-bootnode > inovaSpecRaw.json




./target/release/hashed  --base-path /tmp/node01 --chain ./inovaSpecRaw.json --node-key-file ~/Downloads/node-key-1 --port 30333 --ws-port 9945 --rpc-port 9933 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --validator --rpc-methods Unsafe --name MyNode01
./target/release/hashed  --base-path /tmp/node02 --chain ./inovaSpecRaw.json --node-key-file ~/Downloads/node-key-2 --port 30334 --ws-port 9946 --rpc-port 9934 --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" --validator --rpc-methods Unsafe --name MyNode02 --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWRNsn75iQuPz4AAq6B64351z9u2LLSsdsxEjDy5vgJV9t

./target/release/hashed key insert --base-path /tmp/node01 --chain inovaSpecRaw.json --scheme Sr25519 --suri "adjust wife drop same pyramid immune tissue damage clean car kiss walk" --key-type aura
./target/release/hashed key insert --base-path /tmp/node01 --chain inovaSpecRaw.json --scheme Ed25519 --suri "adjust wife drop same pyramid immune tissue damage clean car kiss walk" --key-type gran

./target/release/hashed key insert --base-path /tmp/node02 --chain inovaSpecRaw.json --scheme Ed25519 --suri "attack cereal kingdom recall august blood destroy virtual push syrup seek labor" --key-type gran
./target/release/hashed key insert --base-path /tmp/node02 --chain inovaSpecRaw.json --scheme Sr25519 --suri "attack cereal kingdom recall august blood destroy virtual push syrup seek labor" --key-type aura



./target/release/hashed key insert --base-path /var/www/hashed/node-data $chain_spec --scheme sr25519 --suri "${MNEMO}" --key-type aura


./target/release/hashed key insert --base-path /var/www/hashed/node-data --chain inovaSpecRaw.json --scheme sr25519 --suri "${MNEMO}" --key-type aura
