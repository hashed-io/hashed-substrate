# Bitcoin Vaults
A storage module for Native Bitcoin Vaults on substrate.

- [Bitcoin Vaults](#bitcoin-vaults)
  - [Overview](#overview)
    - [Terminology](#terminology)
  - [Interface](#interface)
    - [Dispachable functions](#dispachable-functions)
    - [Offchain worker dispatchable functions](#offchain-worker-dispatchable-functions)
    - [Getters](#getters)
  - [Usage](#usage)
    - [Polkadot-js CLI](#polkadot-js-cli)
      - [Enabling Offchain Worker](#enabling-offchain-worker)
      - [Insert an xpub](#insert-an-xpub)
      - [Query a specific stored xpub](#query-a-specific-stored-xpub)
      - [Query accounts hash that links to the xpub](#query-accounts-hash-that-links-to-the-xpub)
      - [Remove user's xpub](#remove-users-xpub)
      - [Insert Vault](#insert-vault)
      - [Query user's vaults (id)](#query-users-vaults-id)
      - [Query vault details](#query-vault-details)
      - [Remove Vault](#remove-vault)
      - [Propose](#propose)
      - [Query vault's proposals](#query-vaults-proposals)
      - [Query proposals details](#query-proposals-details)
      - [Remove a proposal](#remove-a-proposal)
      - [Sign a proposal](#sign-a-proposal)
      - [Finalize (and posibly broadcast) a PSBT](#finalize-and-posibly-broadcast-a-psbt)
      - [Broadcast a PSBT](#broadcast-a-psbt)
    - [Polkadot-js api (javascript library)](#polkadot-js-api-javascript-library)
      - [Enabling Offchain Worker](#enabling-offchain-worker-1)
      - [Insert an xpub](#insert-an-xpub-1)
      - [Query stored xpubs](#query-stored-xpubs)
      - [Query accounts hash that links to the xpub](#query-accounts-hash-that-links-to-the-xpub-1)
      - [Remove user's xpub](#remove-users-xpub-1)
      - [Insert Vault](#insert-vault-1)
      - [Query vaults ids by signer](#query-vaults-ids-by-signer)
      - [Query vaults details by id](#query-vaults-details-by-id)
      - [Remove vault](#remove-vault-1)
      - [Propose](#propose-1)
      - [Query vault's proposals](#query-vaults-proposals-1)
      - [Query proposals details](#query-proposals-details-1)
      - [Remove a proposal](#remove-a-proposal-1)
      - [Sign a proposal](#sign-a-proposal-1)
      - [Finalize (and posibly broadcast) a PSBT](#finalize-and-posibly-broadcast-a-psbt-1)
      - [Broadcast a PSBT](#broadcast-a-psbt-1)
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
- `remove_vault` receives `vault_id` - remove vault and any proposals or PSBTs from storage.
- `propose` Propose an expense in a specified vault, takes a `vault_id`, `recipient_address` to which the `amount_in_sats` will be sent, and a `description`. You need to participate on the vault.
- `remove_proposal` removes the specified proposal by taking `proposal_id`. You need to be the user who proposed it.
- `save_psbt` takes a `proposal_id` and collects a signature payload in pure bytes for it, these types of signatures are necessary to fullfill the vault's `threshold`. If successful, this process cannot be undone.
- `finalize_psbt` generates a `tx_id` by taking a `proposal_id`, a `broadcast` boolean flag must be specified to determine if the transaction will be automatically transmited to the blockchain or not. 
- `broadcast` publishes the transaction if it wasn't on the previous step.

### Offchain worker dispatchable functions

- `ocw_insert_descriptors` is only an extrinsic that is meant to be called by the pallet's offchain worker, as it makes the output descriptors insertion.
- `ocw_insert_psbts` is meant to be called by the pallet's offchain worker, it performs the psbt proposal insertion.
- `ocw_finalize_psbts` inserts the generated `tx_id` for the transaction, which can be inspected with a block explorer.

### Getters
- `xpubs`
- `xpubs_by_owner`
- `vaults`
- `vaults_by_signer`
- `proposals`
- `proposals_by_vault`
- `DefaultURL` (for bdk services)

## Usage

The following examples will be using the following credentials and testing data:

```bash
# Alice's mnemonic
"//Alice"

# Alice's public address 
"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

# Dummy xpub
"Zpub74kbYv5LXvBaJRcbSiihEEwuDiBSDztjtpSVmt6C6nB3ntbcEy4pLP3cJGVWsKbYKaAynfCwXnkuVncPGQ9Y4XwWJDWrDMUwTztdxBe7GcM"

# The resulting hash from the previous xpub should look something like this
"0x9ee1b23c479e03288d3d1d791abc580439598f70e7607c1de108c4bb6a9b5b6f"
```

### Polkadot-js CLI

#### Enabling Offchain Worker
In order to enable vault-related features, an account needs to be linked to the offchain worker. This process needs to be done just once, preferably by one of the chain administrators:  

```bash
# key type (constant to bdks), suri, public key in hex
polkadot-js-api rpc.author.insertKey 'bdks' '//Alice' '0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d'
```

#### Insert an xpub
Note that the identity data structure is identical to the original setIdentity extrinsic from the identity pallet and additional fields can be specified.
The xpub to store is sent on the second parameter and the pallet will handle the link between xpub and hash.
```bash
polkadot-js-api tx.bitcoinVaults.setXpub "Zpub74kbYv5LXvBaJRcbSiihEEwuDiBSDztjtpSVmt6C6nB3ntbcEy4pLP3cJGVWsKbYKaAynfCwXnkuVncPGQ9Y4XwWJDWrDMUwTztdxBe7GcM" --seed "//Alice"

```

#### Query a specific stored xpub
If successful, the previous extrinsic returns a `XPubStored` event, which will contain the hash that links the identity and the xpub itself.
```bash
# Note that the "0x9ee..." hash was returned from the previous tx. 
polkadot-js-api query.bitcoinVaults.xpubs "0x9ee1b23c479e03288d3d1d791abc580439598f70e7607c1de108c4bb6a9b5b6f"
```

#### Query accounts hash that links to the xpub
The hash can also be retrieved by specifying the owner account.
```bash
# This tx should return the previous hash "0x9ee..."
polkadot-js-api query.bitcoinVaults.xpubsByOwner "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
```

#### Remove user's xpub
The account's xpub can be removed by submiting this extrinsic. 
```bash
polkadot-js-api tx.bitcoinVaults.removeXpub --seed "//Alice"
```

#### Insert Vault
In order to create a vault, refer to the following extrinsic structure:
```bash
# All users need to have an xpub
polkadot-js-api tx.bitcoinVaults.createVault 1 "description" true '["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"]' --seed "//Alice"
```

#### Query user's vaults (id)

```bash
polkadot-js-api query.bitcoinVaults.vaultsBySigner 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

#### Query vault details

```bash
polkadot-js-api query.bitcoinVaults.vaults 0xdc08dcf7b4e6525bdd894433ffe45644262079dec2cdd8d5293e6b78c10edbcf
```

#### Remove Vault
Only the vault's owner can remove it, a `vault_id` needs to be provided. This will remove all the vault proposals:
```bash
polkadot-js-api tx.bitcoinVaults.removeVault "0xdc08dcf7b4e6525bdd894433ffe45644262079dec2cdd8d5293e6b78c10edbcf" --seed "//Alice"
```
#### Propose
```bash
# Parameters in order: vault_id, recipient address, amount in satoshis, and description:
polkadot-js-api tx.bitcoinVaults.propose 0xdc08dcf7b4e6525bdd894433ffe45644262079dec2cdd8d5293e6b78c10edbcf Zpub75bKLk9fCjgfELzLr2XS5TEcCXXGrci4EDwAcppFNBDwpNy53JhJS8cbRjdv39noPDKSfzK7EPC1Ciyfb7jRwY7DmiuYJ6WDr2nEL6yTkHi 1000 "lorem ipsum" --seed "//Alice"
```

#### Query vault's proposals
```bash
polkadot-js-api query.bitcoinVaults.proposalsByVault 0x739829829f1a2891918f626a79bd830c0e46609f6db013a4f557746c014c374e
```

#### Query proposals details
```bash
polkadot-js-api query.bitcoinVaults.proposals 0x8426160f6705e480825a5bdccb2e465ad097d8a0a09981467348f884682d5675
```

#### Remove a proposal
```bash
polkadot-js-api tx.bitcoinVaults.removeProposal 0x8426160f6705e480825a5bdccb2e465ad097d8a0a09981467348f884682d5675 --seed "//Alice"
```

#### Sign a proposal
```bash
polkadot-js-api tx.bitcoinVaults.savePsbt 0x8426160f6705e480825a5bdccb2e465ad097d8a0a09981467348f884682d5675 "<generated_psbt>" --seed "//Alice"
```

#### Finalize (and posibly broadcast) a PSBT
The second parameter is a boolean flag, if set to true, the transaction will be automatically broadcasted.
```bash
polkadot-js-api tx.bitcoinVaults.finalizePsbt 0x8426160f6705e480825a5bdccb2e465ad097d8a0a09981467348f884682d5675 <true/false> --seed "//Alice"
```

#### Broadcast a PSBT
This extrinsic is needed in case the PSBT is finalized but not broadcasted. 

```bash
polkadot-js-api tx.bitcoinVaults.broadcastPsbt 0x8426160f6705e480825a5bdccb2e465ad097d8a0a09981467348f884682d5675 --seed "//Alice"
```

### Polkadot-js api (javascript library)
While most of the data flow is almost identical to its CLI counter part, the javascript library is much more versatile regarding queries. The API setup will be omitted.

#### Enabling Offchain Worker
In order to enable vault-related features, an account needs to be linked to the offchain worker. This process needs to be done just once, preferably by one of the chain administrators:  

```js
# key type (constant to bdks), suri, public key in hex (no method was found for parsing an address to hex)
const setKey = api.rpc.author.insertKey("bdks", "//Alice", "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d")
```

#### Insert an xpub

```js
const setXpub = api.tx.bitcoinVaults.setXpub(
     "Zpub75bKLk9fCjgfELzLr2XS5TEcCXXGrci4EDwAcppFNBDwpNy53JhJS8cbRjdv39noPDKSfzK7EPC1Ciyfb7jRwY7DmiuYJ6WDr2nEL6yTkHi");
const identityResult = await setCompleteIdentity.signAndSend(alice);
console.log('Extrinsic sent with hash', identityResult.toHex());
```
#### Query stored xpubs

Query an xpub with specific hash;
```js
const specificXpub = await api.query.bitcoinVaults.xpubs("0x9ee1b23c479e03288d3d1d791abc580439598f70e7607c1de108c4bb6a9b5b6f");
console.log(specificXpub.toHuman());
```

Query and print all the stored xpubs:
```js
const xpubs = await api.query.bitcoinVaults.xpubs.entries();
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
const xpubsByOwner = await api.query.bitcoinVaults.xpubsByOwner(alice.address);
console.log(xpubsByOwner.toHuman() );
```

Query and print all the xpub hashes, classified by account 
```js
const allXpubsByOwner = await api.query.bitcoinVaults.xpubsByOwner.entries();
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
const removeAccountXpub = await api.tx.bitcoinVaults.removeXpub().signAndSend(alice);
console.log('Tx sent with hash', removeAccountXpub.toHex());
```

#### Insert Vault
```js
const insertVault = await api.tx.bitcoinVaults.createVault(1, "descripcion", ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"]).signAndSend(alice);
console.log('Tx sent with hash', insertVault.toHuman());
```

#### Query vaults ids by signer

```js
const vaultsBySigner = await api.query.bitcoinVaults.vaultsBySigner(alice.address);
console.log(vaultsBySigner.toHuman());
```

#### Query vaults details by id

```js
const vaultDetails = await api.query.bitcoinVaults.vaults("0x39c7ffa1b10d9d75fe20eb55e07788c23c06238b6e25e719a8e58d0bdf6bcd21");
console.log(vaultDetails.toHuman());
```

#### Remove vault

```js
const removeVault = await api.tx.bitcoinVaults.removeVault("0xdc08dcf7b4e6525bdd894433ffe45644262079dec2cdd8d5293e6b78c10edbcf").signAndSend(alice);
console.log('Tx sent with hash', removeVault.toHex());
```

#### Propose
```js
// Parameters in order: vault_id, recipient address, amount in satoshis, and description:
const propose = await api.tx.bitcoinVaults
    .propose("0x739829829f1a2891918f626a79bd830c0e46609f6db013a4f557746c014c374e", "Zpub75bKLk9fCjgfELzLr2XS5TEcCXXGrci4EDwAcppFNBDwpNy53JhJS8cbRjdv39noPDKSfzK7EPC1Ciyfb7jRwY7DmiuYJ6WDr2nEL6yTkHi", 1000, "lorem ipsum").signAndSend(alice);
console.log('Tx sent with hash', propose.toHuman());
```

#### Query vault's proposals
```js
const vaultsProposals = await api.query.bitcoinVaults.proposalsByVault("0x739829829f1a2891918f626a79bd830c0e46609f6db013a4f557746c014c374e");
console.log(vaultsProposals.toHuman() );
```

#### Query proposals details
```js
const proposal = await api.query.bitcoinVaults.proposals("0x8426160f6705e480825a5bdccb2e465ad097d8a0a09981467348f884682d5675");
console.log(proposal.toHuman());
```

#### Remove a proposal
```js
const removeProposal = await api.tx.bitcoinVaults.removeProposal("0x8426160f6705e480825a5bdccb2e465ad097d8a0a09981467348f884682d5675").signAndSend(alice);
console.log(removeProposal.toHuman());
```

#### Sign a proposal

```js
const savePSBT = await api.tx.bitcoinVaults.savePsbt("0x8426160f6705e480825a5bdccb2e465ad097d8a0a09981467348f884682d5675").signAndSend(alice);
console.log(savePSBT.toHuman());
```

#### Finalize (and posibly broadcast) a PSBT
```js
const finalizePsbt = await api.tx.bitcoinVaults.finalizePsbt("0x8426160f6705e480825a5bdccb2e465ad097d8a0a09981467348f884682d5675").signAndSend(alice);
console.log(finalizePsbt.toHuman());
```

#### Broadcast a PSBT 
```js
const broadcastPsbt = await api.tx.bitcoinVaults.broadcastPsbt("0x8426160f6705e480825a5bdccb2e465ad097d8a0a09981467348f884682d5675").signAndSend(alice);
console.log(broadcastPsbt.toHuman());
```

## Events

```rust
/// Xpub and hash stored
XPubStored([u8; 32], T::AccountId),
/// Removed Xpub previously linked to the account
XPubRemoved(T::AccountId),
/// The PBST was succesfully inserted by an OCW
PSBTStored([u8;32]),
/// The vault was succesfully inserted and linked to the account as owner
VaultStored([u8; 32], T::AccountId),
/// The vault was succesfully removed by its owner
VaultRemoved([u8; 32],T::AccountId),
/// An offchain worker inserted a vault's descriptor 
DescriptorsStored([u8;32]),
/// A proposal has been inserted. 
ProposalStored([u8;32],T::AccountId),
/// A proposal has been removed.
ProposalRemoved([u8;32],T::AccountId),
/// A proposal tx has been inserted by an OCW
ProposalTxIdStored([u8;32])
/// A proof of reserve was stored for the vault
ProofOfReserveStored([u8; 32]),
/// A psbt was stored for the vaults Proof of reserve
ProofPSBTStored([u8; 32]),
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
/// Too many cosigners
ExceedMaxCosignersPerVault,
/// Vault not found
VaultNotFound,
/// A vault needs at least 1 cosigner
NotEnoughCosigners,
/// Only the owner of this vault can do this transaction
VaultOwnerPermissionsNeeded,
/// Vault members cannot be duplicate
DuplicateVaultMembers,
/// The account must participate in the vault to make a proposal or sign
SignerPermissionsNeeded,
/// The vault has too many proposals 
ExceedMaxProposalsPerVault,
/// Proposal not found (id)
ProposalNotFound,
/// The account must be the proposer to remove it
ProposerPermissionsNeeded,
/// An identical proposal exists in storage 
AlreadyProposed,
/// The proposal was already signed by the user
AlreadySigned,
/// The proposal is already finalized or broadcasted
PendingProposalRequired,
/// The proposal signatures need to surpass the vault's threshold 
NotEnoughSignatures,
/// The proposal has structural failures
InvalidProposal,
/// This vault cant take proposals due to structural failures
InvalidVault,
/// The proof of reserve was not found
ProofNotFound
```

## Assumptions

Below are assumptions that must be held when using this module.  If any of
them are violated, the behavior of this module is undefined.

- The pallet relies on the remote endpoint `bdk-services` to generate descriptors, proposals, and next adressess.


License: Apache-2.0