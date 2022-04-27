# How to add a node to the permissioned hashed chain


## Prerrequisites
- [Install subkey](https://docs.substrate.io/v3/tools/subkey/#installation)
- Have `sudo` access to the machine in which the node will run.
- Download and compile the latest main branch of the project.
- Have a mnemonic that is specified in the aura/grandpa authorities list (at the current time, it will be provided by us).
- Have a directory route in mind where the node storage will be set, those directories will be created on later steps (we recommend a subdirectory within the hashed-substrate project: `./hashed-substrate/hashed-chaos-data/`). This route will be refferred as the `base path`.

Additionally, most of the provided commands are executed on the project directory:

```bash
cd hashed-substrate/
```

## Generate node key
A node key must be generated in order to identify the nodes that are connected to the network. 
```
subkey generate-node-key
```
It will output two rows of information looking something like this:
```bash
12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2 # this is PeerId.
c12b6d18942f5ee8528c8e2baf4e147b5c5c18710926ea492d09cbd9f6c9f82a # This is node-key.
```
- The `PeerId` is what the other nodes see, while the `node-key` should remain private, it will be specified on the command that runs the node.

- It is recommended to store both outputs on a safe place.

- Please share the generated `PeerId` with a Hashed team member, as it will be used for creating a link between your node and the provided validator account.


## Insert the provided mnemonic to the node's local storage

Next, the provided validator mnemonic has to be inserted in the node storage (aka. `base path`), replace the `--suri` option content with the provided mnemonic:

```bash
sudo ./target/release/hashed key insert --base-path hashed-chaos-data --chain ./chaos2.json --scheme sr25519 --suri "your mnemonic goes here" --key-type aura
```
- Please note that the `--base-path` option specifies the suggested directory where the node will store all the chain data, and it should be change accordingly in case another one is desired.
- The command is executed on the project's directory.
- The latest chain spec is `chaos2.json`, as specified in the command.

If the process is successful, the specified `base path` should be created and the key inserted:

```bash
ls hashed-chaos-data/chains/chaos/keystore/
# The command should output a file named something like "6175726...", the name might be different
```

## Starting the node

Now all that is left is to boot up the node, replacing the `--node-key` content with the generated node-key:
```bash
sudo ./target/release/hashed --base-path hashed-chaos-data --chain chaos2.json --node-key=<your node key> --rpc-external --rpc-cors all --rpc-methods=unsafe --no-mdns --validator --bootnodes /ip4/206.221.189.10/tcp/30335/p2p/12D3KooWQxwQyQ3BaCs5tweoTmHNWHbpHePZt6P9SscBps1FWsUc --offchain-worker always
```

## References
[Permissioned network tutorial](https://docs.substrate.io/tutorials/v3/permissioned-network/)