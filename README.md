## Hashed Chain

### [Open Hashed Chain on polkadot.js.org](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fn1.hashed.systems#/explorer)

## Features
### Standard Pallets
#### [Identity](https://wiki.polkadot.network/docs/learn-identity)
Hashed Chain provides a naming system that allows participants to add information, such as social media accounts, web domains, email addresses, etc. to their on-chain account and subsequently ask for verification of this information by registrars.

#### [Indices](https://wiki.polkadot.network/docs/learn-accounts#indices)
An index is a short and easy-to-remember version of an address. Claiming an index requires a deposit that is released when the index is cleared.

#### [Social Recovery]()
The Recovery pallet is an M-of-N social recovery tool for users to gain access to their accounts if the private key or other authentication mechanism is lost. Through this pallet, a user is able to make calls on-behalf-of another account which they have recovered. The recovery process is protected by trusted "friends" whom the original account owner chooses. A threshold (M) out of N friends are needed to give another account access to the recoverable account.

#### [Uniques (NFTs)](https://github.com/paritytech/substrate/tree/master/frame/uniques)
A simple, secure module for dealing with non-fungible assets.

#### [Treasury](https://wiki.polkadot.network/docs/learn-treasury)
The Treasury pallet provides a "pot" of funds that can be managed by stakeholders in the system and a structure for making spending proposals from this pot.

> Roadmap: Integrate support for native multisig BTC in the treasury via PSBT and output descriptors

#### [Society](https://wiki.polkadot.network/docs/maintain-guides-society-kusama)
The Society module is an economic game which incentivizes users to participate and maintain a membership society.

#### [Bounties](https://wiki.polkadot.network/docs/learn-treasury#bounties-spending) 
Bounties Spending proposals aim to delegate the curation activity of spending proposals to experts called Curators: They can be defined as addresses with agency over a portion of the Treasury with the goal of fixing a bug or vulnerability, developing a strategy, or monitoring a set of tasks related to a specific topic: all for the benefit of the whole ecosystem.

## Custom Pallets
#### [Fruniques (FRactional UNIQUES)](https://github.com/hashed-io/hashed-substrate/tree/main/pallets/fruniques)
A Frunique is a type of Non-Fungible Token (NFT). Fruniques allow token holders to lock any set of fungible and/or non-fungible tokens into a new NFT backed by the tokens. The source/parent asset(s) can be unlocked if and only if all of its child fruniques are held by the same account. Any Frunique may be transformed to become 1..n new Fruniques or a fungible token.

Fruniques are compatible with the `Uniques` pallet referenced above.

## Quick Start

### Rust Setup
First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

```bash
git clone https://github.com/hashed-io/hashed-substrate.git
cd hashed-substrate
cargo build --release 

./target/release/hashed --chain ./hashed-chaos-spec-raw.json --name MyNode --validator --ws-external --rpc-external --rpc-cors all --rpc-methods=unsafe --bootnodes /ip4/206.221.189.10/tcp/30333/p2p/12D3KooWL7R8De1mPmCj3zA2pMEJXzbDrJVeEJf2SudV21EK9LxU
```

![hashed-chain-arch](http://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/hashed-io/hashed-substrate/main/docs/hashed-chain-arch.iuml)

## Starting a node
### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev --tmp
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/hashed -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with non-persistent state:

```bash
./target/release/hashed --dev
```

Purge the development chain's state:

```bash
./target/release/hashed purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/hashed -ldebug --dev
```

> Development chain means that the state of our chain will be in a tmp folder while the nodes are
> running. Also, **alice** account will be authority and sudo account as declared in the
> [genesis state](https://github.com/substrate-developer-hub/substrate-hashed/blob/main/node/src/chain_spec.rs#L49).
> At the same time the following accounts will be pre-funded:
> - Alice
> - Bob 
> - Alice//stash
> - Bob//stash

In case of being interested in maintaining the chain' state between runs a base path must be added
so the db can be stored in the provided folder instead of a temporal one. We could use this folder 
to store different chain databases, as a different folder will be created per different chain that
is ran. The following commands shows how to use a newly created folder as our db base path.

```bash
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/hashed --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```


### Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local node template.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to our
[Start a Private Network tutorial](https://docs.substrate.io/tutorials/v3/private-network).

## Template Structure

A Substrate project such as this consists of a number of components that are spread across a few
directories.

### Node

A blockchain node is an application that allows users to participate in a blockchain network.
Substrate-based blockchain nodes expose a number of capabilities:

- Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
  nodes in the network to communicate with one another.
- Consensus: Blockchains must have a way to come to
  [consensus](https://docs.substrate.io/v3/advanced/consensus) on the state of the
  network. Substrate makes it possible to supply custom consensus engines and also ships with
  several consensus mechanisms that have been built on top of
  [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
- RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory - take special note of the following:

- [`chain_spec.rs`](./node/src/chain_spec.rs): A
  [chain specification](https://docs.substrate.io/v3/runtime/chain-specs) is a
  source code file that defines a Substrate chain's initial (genesis) state. Chain specifications
  are useful for development and testing, and critical when architecting the launch of a
  production chain. Take note of the `development_config` and `testnet_genesis` functions, which
  are used to define the genesis state for the local development chain configuration. These
  functions identify some
  [well-known accounts](https://docs.substrate.io/v3/tools/subkey#well-known-keys)
  and use them to configure the blockchain's initial state.
- [`service.rs`](./node/src/service.rs): This file defines the node implementation. Take note of
  the libraries that this file imports and the names of the functions it invokes. In particular,
  there are references to consensus-related topics, such as the
  [longest chain rule](https://docs.substrate.io/v3/advanced/consensus#longest-chain-rule),
  the [Aura](https://docs.substrate.io/v3/advanced/consensus#aura) block authoring
  mechanism and the
  [GRANDPA](https://docs.substrate.io/v3/advanced/consensus#grandpa) finality
  gadget.

After the node has been [built](#build), refer to the embedded documentation to learn more about the
capabilities and configuration parameters that it exposes:

```shell
./target/release/hashed --help
```

### Runtime

In Substrate, the terms
"[runtime](https://docs.substrate.io/v3/getting-started/glossary#runtime)" and
"[state transition function](https://docs.substrate.io/v3/getting-started/glossary#state-transition-function-stf)"
are analogous - they refer to the core logic of the blockchain that is responsible for validating
blocks and executing the state changes they define. The Substrate project in this repository uses
the [FRAME](https://docs.substrate.io/v3/runtime/frame) framework to construct a
blockchain runtime. FRAME allows runtime developers to declare domain-specific logic in modules
called "pallets". At the heart of FRAME is a helpful
[macro language](https://docs.substrate.io/v3/runtime/macros) that makes it easy to
create pallets and flexibly compose them to create blockchains that can address
[a variety of needs](https://www.substrate.io/substrate-users/).

Review the [FRAME runtime implementation](./runtime/src/lib.rs) included in this template and note
the following:

- This file configures several pallets to include in the runtime. Each pallet configuration is
  defined by a code block that begins with `impl $PALLET_NAME::Config for Runtime`.
- The pallets are composed into a single runtime by way of the
  [`construct_runtime!`](https://crates.parity.io/frame_support/macro.construct_runtime.html)
  macro, which is part of the core
  [FRAME Support](https://docs.substrate.io/v3/runtime/frame#support-crate)
  library.

### Pallets

The runtime in this project is constructed using many FRAME pallets that ship with the
[core Substrate repository](https://github.com/paritytech/substrate/tree/master/frame) and a
template pallet that is [defined in the `pallets`](./pallets/template/src/lib.rs) directory.

A FRAME pallet is compromised of a number of blockchain primitives:

- Storage: FRAME defines a rich set of powerful
  [storage abstractions](https://docs.substrate.io/v3/runtime/storage) that makes
  it easy to use Substrate's efficient key-value database to manage the evolving state of a
  blockchain.
- Dispatchables: FRAME pallets define special types of functions that can be invoked (dispatched)
  from outside of the runtime in order to update its state.
- Events: Substrate uses [events and errors](https://docs.substrate.io/v3/runtime/events-and-errors)
  to notify users of important changes in the runtime.
- Errors: When a dispatchable fails, it returns an error.
- Config: The `Config` configuration interface is used to define the types and parameters upon
  which a FRAME pallet depends.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command
(`cargo build --release && ./target/release/hashed --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/hashed --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/hashed purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```
