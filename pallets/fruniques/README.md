# Fruniques Pallet

### **FR**actional **Uniques**

## Description

This pallet is being developed **tightly coupled** to both [`pallet_assets`](https://paritytech.github.io/substrate/latest/pallet_assets/) and [`pallet_uniques`](https://paritytech.github.io/substrate/latest/pallet_uniques/index.html). These are the default [Statemine](https://github.com/paritytech/cumulus/tree/master/polkadot-parachains/statemine) pallets for fungible and non-fungible tokens.

A Frunique is a type of Non-Fungible Token (NFT)

Fruniques allow token holders to lock any set of fungible and/or non-fungible tokens into a new Frunique.

Any Frunique may be transformed to become 1..n new Fruniques or a fungible token.

The source/parent asset(s) can be unlocked if and only if all of its child fruniques are held by the same account.

![funiques-composability](http://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/hashed-io/hashed-substrate/main/docs/fruniques-composability.iuml)

## TOC

1. [How to install](#how-to-install)
2. [How to run the project](#how-to-run-the-project)
3. [Usage](#usage)
4. [Run the Tests](#run-the-tests)
5. [Motivation](#motivation)

## How to Install

To run this project you need to follow the install section on [substrate documentation](https://docs.substrate.io/install/)

## How to run the project

To initialize the project make sure that you are running a local node of the solo-chain,
then you need to sign as Sudo the extrinsic `initial_setup()` and the sudo account id as the freezer, this is required to by pass some of the uniques permissions.

Then you can create a collection with the extrinsic `create_collection()`, and passing the name of the collection.

Onces you have a collection it's possible to invite others to create (spawn) NFTs with the `invite()` extrinsic.

To create NFTs you need to call the `spawn()` extrinsic, this extrinsic takes cares of the NFT division.

To create a Root NFT the optional `parent_info_call` argument needs to be a `None`, but feel free to pass as many attributes as you like, as this method is in charge of minting the NFT and adding the metadata and attributes to it.

To create a fraction of a given NFT, you need to call the same extrinsic but passing the `parent_info_call` argument where you specify the amount of the parent you want to fractionalize.

## Usage

### Install polkadot-js tools

```bash
git clone https://github.com/polkadot-js/tools/
cd tools
yarn install
```

### Create Frunique

```bash
yarn run:api tx.fruniques.create 1 1 1 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 1 100 --seed "//Alice"
```

### Check NFT

```bash
yarn run:api query.uniques.class 1
yarn run:api query.uniques.asset 1 1
yarn run:api query.uniques.account 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 1 1
```

### Check Fungible

```bash
yarn run:api query.assets.asset 1
yarn run:api query.assets.account 1 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

## Run the Tests

To run the test you simply run the following command

```bash
cargo test --package pallet-fruniques
```

## Motivation

### NFT <--> Fungible

This pallet provides functionality that allows NFT holders to mint a fungible token backed by the NFT.

The non-fungible token is created and minted using the Statemine `pallet_uniques`.

The fungible token is created and minted using the Statemine `pallet_assets`.

The NFT/Unique can be unlocked and released if and only if a single origin holds all of the corresponding fungible token.

![basket-of-fungibles](http://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/hashed-io/hashed-substrate/main/docs/fungible-basket-frunique.iuml)

### NFT <--> NFTs

An NFT as a trie of NFTs.

### Use cases

#### Tax credit marketplace

A credit is a single NFT, with an `amount`, state of redemption, expiration year, and other metadata. However, that owner can sell less than the `amount`, in which case the newly created credit NFT has all of the same associated data. The sum of the children `amount` values must be equal to the parent.

To support this, we'll create a `AggregatedFrunique` type that enforces the aggregation rules.

#### Cannabis compliance

For the NY state cannabis compliance program, all yield from all plants must be tracked. This aligns to a very similar data structure as above. Each mother plant is an NFT, each clone as an NFT, each package of flower an NFT, etc. Auditing a specific item is fairly easy via traversing all of its ancestors and descendants through to the harvest and dispensary.

## Code Contributors

Made with ❤️ by:

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center"><a href="https://github.com/3yekn"><img src="https://avatars.githubusercontent.com/u/32852271?s=100&v=4" width="100px;" alt="Max Gravitt"/><br /><sub><b>Max Gravitt</b></sub></a><br /></td>
      <td align="center"><a href="https://github.com/sebastianmontero"><img src="https://avatars.githubusercontent.com/u/13155714?100&v=4" width="100px;" alt="sebastianmontero"/><br /><sub><b>sebastianmontero</b></sub></a><br /></td>
      <td align="center"><a href="https://github.com/amatsonkali"><img src="https://avatars.githubusercontent.com/u/43050815?s=100&v=4" width="100px;" alt="Abel Yáñez"/><br /><sub><b>Abel Yáñez</b></sub></a><br /></td>
      <td align="center"><a href="https://github.com/tlacloc"><img src="https://avatars.githubusercontent.com/u/17482176?s=100&v=4" width="100px;" alt="Tlalocman"/><br /><sub><b>Tlalocman</b></sub></a><br /></td>
      <td align="center"><a href="https://github.com/didiermis"><img src="https://avatars.githubusercontent.com/u/62629075?s=100&v=4" width="100px;" alt="Didier Mis"/><br /><sub><b>Didier Mis</b></sub></a><br /></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

> This is WIP - just being spec'd out

## Principles

- Align to the `Statemint` pallets and maintain compatibility. This project imports the two pallets directly from `4.0.0-dev` into the runtime. It will also support cross-chain communication via `XCM`.

- Fruniques are designed to be fully composable. NFTs can be reserved to a fungible, some of those fungibles used in a basket to create another NFT, that NFT then divided into 5 child NFTs, and so on.

## Road Map

- Cross-chain messaging and parachain implementation
- Allow a basket of fungible tokens to be reserved and minted as an NFT. The basket may later be unlocked when the NFT is burned.

## Similar Projects

### [Fractional Art](https://fractional.art/)

### [Detailed Process Explainer with screenshots](https://medium.com/fractional-art/how-to-use-fractional-to-fractionalize-nfts-84da1a465b6d)

License: MIT
