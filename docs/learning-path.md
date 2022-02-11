### Substrate Training Path
#### Substrate Tutorial #1
The full binary executable is compiled from source and includes a default setup with Alice, Bob and other keys preconfigured with funds and sudo rights. This tutorial is the simplest and most informative way to ensure you have your environment ready to work on Substrate projects.
https://docs.substrate.io/tutorials/v3/create-your-first-substrate-chain/

#### Build the `hypha-/hashed-substrate` project 
Hashed project
```bash
git clone https://github.com/hashed-io/hashed-substrate
cd hashed-substrate
cargo build --release
```
Hypha project
```bash
git clone https://github.com/hypha-dao/hypha-substrate
cd hypha-substrate
cargo build --release
```

Connect the Front End template from the tutorial step above to the running `hypha/hashed` node. 

The only difference between the tutorial and the `hypha/hashed` node, in terms of setup instructions, are that the executable name will be `hypha/hashed` instead of `node-template`. That'll make sense after the tutorial. The `hypha/hashed` node will have more functionality than the `node-template`.

#### Set Identity 
Use the pallet explorer on either the Front End template or https://polkadot.js.org to explore your node. 

The instructions here should work on your local node using Alice and Bob: https://wiki.polkadot.network/docs/learn-identity

#### Interact with Identity Pallet - CLI
Pre-requisites: `polkadot-js-api`:  https://github.com/polkadot-js/tools/

You can read the notes and copy/paste the commands from:
https://github.com/hashed-io/hashed-substrate/blob/main/docs/identity.md

#### Interact with Uniques/NFT Pallet - CLI
You can read the notes and copy/paste the commands from:
https://github.com/hashed-io/hashed-substrate/blob/main/docs/uniques.md

#### Training - Deeper Dive
Now that you have an idea for the environment, dive deeper into both the Rust training and Substrate training. As opposed to do them consecutively, I recommend starting both of the trainings and switch back and forth between the two as you progress.

1. Rustlings 
    - Good for interactive learners
    - Use watch mode and just follow the instructions
    - [Link to Repo](https://github.com/rust-lang/rustlings)
2. [Parity Substrate Tutorials](https://docs.substrate.io/tutorials/v3/)
    - No particular order 
    - Some may be out-dated; don't get stuck on a versioning issue, just skip ahead.

#### Tools and Tips
- [`polkadot-js-api`](https://github.com/polkadot-js/tools/)-
- [Awesome Substrate](https://github.com/substrate-developer-hub/awesome-substrate)
- Spend time learning about the [keys types and related commands](https://docs.substrate.io/v3/tools/subkey/)
