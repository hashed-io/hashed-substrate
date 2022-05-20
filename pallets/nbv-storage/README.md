# NBV Storage
A storage module for Native Bitcoin Vaults on substrate.

- [NBV Storage](#nbv-storage)
  - [Overview](#overview)
    - [Terminology](#terminology)
  - [Interface](#interface)
    - [Dispachable functions](#dispachable-functions)
      - [Offchain worker dispatchable functions](#offchain-worker-dispatchable-functions)
    - [Getters](#getters)
  - [Usage](#usage)
    - [Polkadot-js CLI](#polkadot-js-cli)
      - [Insert an xpub](#insert-an-xpub)
      - [Query a specific stored xpub](#query-a-specific-stored-xpub)
      - [Query accounts hash that links to the xpub](#query-accounts-hash-that-links-to-the-xpub)
      - [Remove user's xpub](#remove-users-xpub)
      - [Insert Vault](#insert-vault)
      - [Query user's vaults (id)](#query-users-vaults-id)
      - [Query vault details](#query-vault-details)
      - [Remove Vault](#remove-vault)
    - [Polkadot-js api (javascript library)](#polkadot-js-api-javascript-library)
      - [Insert an xpub](#insert-an-xpub-1)
      - [Query stored xpubs](#query-stored-xpubs)
      - [Query accounts hash that links to the xpub](#query-accounts-hash-that-links-to-the-xpub-1)
      - [Remove user's xpub](#remove-users-xpub-1)
      - [Insert Vault](#insert-vault-1)
      - [Query vaults ids by signer](#query-vaults-ids-by-signer)
      - [Query vaults details by id](#query-vaults-details-by-id)
      - [Remove vault](#remove-vault-1)
  - [Events](#events)
  - [Errors](#errors)
  - [Assumptions](#assumptions)

## Overview

This module provides functionality for data management regarding the Native Bitcoin Vaults storage: 
- Insert, update and remove an extended public key.
- Query the stored, extended public keys by individual accounts (owners).
- Create Bitcoin Vaults: Specify vault users to participate in your vault and threshold in order to get a transaction aproved. The output descriptors will get generated automatically by an offchain worker (requests `bdk-services`).
- Sign Vault PSBT's: You can approve and sign a proposal in a vault you participate, if the number of signs gets equal or greater than the threshold, the Transaction will be executed.


### Terminology

- **Extended Public Key**: Also known as xpub, is a key mainly used for deriving child public keys. They were proposed on the Bitcoin Improvement Proposal number 32 and have a maximum length of 112 characters.
- **Vault**: A vault is a BIP-174 multisignature Bitcoin wallet.
- **Partially Signed Bitcoin Transaction**:

## Interface

### Dispachable functions

- `set_xpub` handles the identity `info` and `xpub` insertion on the pallet's storage. The xpub 
- `remove_xpub` eliminates the user's xpub. It doesn't require any parameters but it will fire an error if a user's xpub is not found or if its part of a vault.
- `create_vault` takes a vault `description`, a set or `BoundedVec` of accounts as well as the signature `threshold`.
- `remove_vault` receives `vault_id` - remove vault and any open proposals or PSBTs from storage.
- `propose` WIP
- `generate_new_address` WIP

#### Offchain worker dispatchable functions

- `ocw_insert_descriptors` is only an extrinsic that is meant to be called by the pallet's offchain worker, as it makes the output descriptors insertion.

### Getters
- `xpubs`
- `xpubs_by_owner`
- `vaults`
- `vaults_by_signer`
- `psbts`

## Usage

The following examples will be using the following credentials and testing data:

```bash
# Alice's mnemonic
"//Alice"

# Alice's public address 
"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

# Dummy xpub
"xpub6AHA9hZDN11k2ijHMeS5QqHx2KP9aMBRhTDqANMnwVtdyw2TDYRmF8PjpvwUFcL1Et8Hj59S3gTSMcUQ5gAqTz3Wd8EsMTmF3DChhqPQBnU"

# The resulting hash from the previous xpub should look something like this
"0x9ee1b23c479e03288d3d1d791abc580439598f70e7607c1de108c4bb6a9b5b6f"
```

### Polkadot-js CLI

#### Insert an xpub
Note that the identity data structure is identical to the original setIdentity extrinsic from the identity pallet and additional fields can be specified.
The xpub to store is sent on the second parameter and the pallet will handle the link between xpub and hash. 
```bash
polkadot-js-api tx.nbvStorage.setXpub "xpub6AHA9hZDN11k2ijHMeS5QqHx2KP9aMBRhTDqANMnwVtdyw2TDYRmF8PjpvwUFcL1Et8Hj59S3gTSMcUQ5gAqTz3Wd8EsMTmF3DChhqPQBnU" --seed "//Alice"

```

#### Query a specific stored xpub
If successful, the previous extrinsic returns a `XPubStored` event, which will contain the hash that links the identity and the xpub itself.
```bash
# Note that the "0x9ee..." hash was returned from the previous tx. 
polkadot-js-api query.nbvStorage.xpubs "0x9ee1b23c479e03288d3d1d791abc580439598f70e7607c1de108c4bb6a9b5b6f"
```

#### Query accounts hash that links to the xpub
The hash can also be retrieved by specifying the owner account.
```bash
# This tx should return the previous hash "0x9ee..."
polkadot-js-api query.nbvStorage.xpubsByOwner "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
```

#### Remove user's xpub
The account's xpub can be removed by submiting this extrinsic. 
```bash
polkadot-js-api tx.nbvStorage.removeXpub --seed "//Alice"
```

#### Insert Vault
In order to create a vault, refer to the following extrinsic structure:
```bash
# All users need to have an xpub
polkadot-js-api tx.nbvStorage.createVault 1 "description" '["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"]' --seed "//Alice"
```

#### Query user's vaults (id)

```bash
polkadot-js-api query.nbvStorage.vaultsBySigner 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

#### Query vault details

```bash
polkadot-js-api query.nbvStorage.vaults 0xdc08dcf7b4e6525bdd894433ffe45644262079dec2cdd8d5293e6b78c10edbcf
```

#### Remove Vault
Only the vault's owner can remove it, a `vault_id` needs to be provided:
```bash
polkadot-js-api tx.nbvStorage.removeVault "0xdc08dcf7b4e6525bdd894433ffe45644262079dec2cdd8d5293e6b78c10edbcf" --seed "//Alice"
```

### Polkadot-js api (javascript library)
While most of the data flow is almost identical to its CLI counter part, the javascript library is much more versatile regarding queries. The API setup will be ommited.


#### Insert an xpub


```js
const setXpub = api.tx.nbvStorage.setXpub(
     "xpub6AHA9hZDN11k2ijHMeS5QqHx2KP9aMBRhTDqANMnwVtdyw2TDYRmF8PjpvwUFcL1Et8Hj59S3gTSMcUQ5gAqTz3Wd8EsMTmF3DChhqPQBnU");
const identityResult = await setCompleteIdentity.signAndSend(alice);
console.log('Extrinsic sent with hash', identityResult.toHex());
```
#### Query stored xpubs

Query an xpub with specific hash;
```js
const specificXpub = await api.query.nbvStorage.xpubs("0x9ee1b23c479e03288d3d1d791abc580439598f70e7607c1de108c4bb6a9b5b6f");
console.log(specificXpub.toHuman());
```

Query and print all the stored xpubs:
```js
const xpubs = await api.query.nbvStorage.xpubs.entries();
xpubs.forEach(([key, value]) => {
console.log(
    "Xpub hash:",
    key.args.map((k) => k.toHuman())
);
console.log("     Xpub value:", value.toHuman());
});
```

#### Query accounts hash that links to the xpub

Query an accounts hashed xpub.
```js
const xpubsByOwner = await api.query.nbvStorage.xpubsByOwner(alice.address);
console.log(xpubsByOwner.toHuman() );
```

Query and print all the xpub hashes, classified by account 
```js
const allXpubsByOwner = await api.query.nbvStorage.xpubsByOwner.entries();
allXpubsByOwner.forEach(([key, value]) => {
        console.log(
        "Account:",
        key.args.map((k) => k.toHuman())
    );
    console.log("     Xpub Hash value:", value.toHuman());
});

```
#### Remove user's xpub

```js
const removeAccountXpub = await api.tx.nbvStorage.removeXpub().signAndSend(alice);
console.log('Tx sent with hash', removeAccountXpub.toHex());
```

#### Insert Vault
```js
const insertVault = await api.tx.nbvStorage.createVault(1, "descripcion", ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"]).signAndSend(alice);
console.log('Tx sent with hash', insertVault.toHuman());
```

#### Query vaults ids by signer

```js
const vaultsBySigner = await api.query.nbvStorage.vaultsBySigner(alice.address);
console.log(vaultsBySigner.toHuman());
```

#### Query vaults details by id

```js
const vaultDetails = await api.query.nbvStorage.vaults("0x39c7ffa1b10d9d75fe20eb55e07788c23c06238b6e25e719a8e58d0bdf6bcd21");
console.log(vaultDetails.toHuman());
```

#### Remove vault

```js
const removeVault = await api.tx.nbvStorage.removeVault("0xdc08dcf7b4e6525bdd894433ffe45644262079dec2cdd8d5293e6b78c10edbcf").signAndSend(alice);
console.log('Tx sent with hash', removeVault.toHex());
```
## Events
```rust
/// Xpub and hash stored
XPubStored([u8; 32], T::AccountId),
/// Removed Xpub previously linked to the account
XPubRemoved(T::AccountId),
/// The PBST was succesfully inserted and linked to the account
PSBTStored(T::AccountId),
/// The vault was succesfully inserted and linked to the account as owner
VaultStored([u8; 32], T::AccountId),
/// The vault was succesfully removed by its owner
VaultRemoved([u8; 32],T::AccountId),
/// An offchain worker inserted a vault's descriptor 
DescriptorsStored([u8;32]),
```
## Errors
```rust
/// Work in progress!
NotYetImplemented,
/// Xpub shouldn't be empty
NoneValue,
// The xpub has already been uploaded and taken by an account
XPubAlreadyTaken,
/// The Account doesn't have an xpub
XPubNotFound,
/// The user already has an xpub, try to remove it first
UserAlreadyHasXpub,
/// The Xpub cant be removed/changed because a vault needs it
XpubLinkedToVault,
/// The generated Hashes aren't the same
HashingError,
/// Found Invalid name on an additional field
InvalidAdditionalField,
/// The vault threshold cannot be greater than the number of vault participants
InvalidVaultThreshold,
/// A defined cosigner reached its vault limit
SignerVaultLimit,
/// Vault not found
VaultNotFound,
/// A vault needs at least 1 cosigner
NotEnoughCosigners,
/// Only the owner of this vault can do this transaction
VaultOwnerPermissionsNeeded,
/// Vault members cannot be duplicate
DuplicateVaultMembers,
```

## Assumptions

Below are assumptions that must be held when using this module.  If any of
them are violated, the behavior of this module is undefined.

- The pallet will contact the remote endpoint `bdk-services` to generate descriptors, proposals, and next adressesss.


License: Apache-2.0