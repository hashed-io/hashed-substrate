# NBV Storage
A storage module for Native Bitcoin Vaults on substrate.

## Overview

This module provides functionality for data management regarding the Native Bitcoin Vaults storage: 
- Insert, update and remove an extended public key, both within the pallet and an identity additional field.
- Query the stored, extended public keys by individual accounts (owners).
- Inserting Partially Signed Bitcoin Transactions on-chain.

To use it in your runtime, you need to implement the NBVStorage runtime config, in addition to the `pallet_identity::Config`.

### Terminology

- **Extended Public Key**: Also known as xpub, is a key mainly used for deriving child public keys. They were proposed on the Bitcoin Improvement Proposal number 32 and have a maximum length of 112 characters.
- **Partially Signed Bitcoin Transaction**:

## Interface

### Dispachable functions

- `set_complete_identity` handles the identity `info` and `xpub` insertion on the pallet's storage and the identity pallet. The xpub itself is too long for inserting it on a identity field, so a blake2 256 hash is inserted on the identity, while the xpub itself is stored on this pallet.
- `remove_xpub_from_identity` eliminates the user's xpub. It doesn't require any parameters.
- `set_psbt` WIP. 

### Getters
- `xpubs`
- `xpubs_by_owner`
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

#### **Insert an identity with a xpub**
Note that the identity data structure is identical to the original setIdentity extrinsic from the identity pallet and additional fields can be specified.
The xpub to store is sent on the second parameter and the pallet will handle the link between xpub and hash. 
```bash
polkadot-js-api tx.nbvStorage.setCompleteIdentity '{
    "display": {
        "Raw": "Your name goes here"
    },
    "web": {
        "Raw": "https://github.com/hashed-io"
    },
    "riot": {
        "Raw": "@surveysays:matrix.org"
    },
    "email": {
        "Raw": "notrealsteve@email.com"
    }, 
    "additional": [[{
            "Raw": "ApprovedForMarketplace"
        },{
            "Raw": "1"
          }
        ]
    ]
}' "xpub6AHA9hZDN11k2ijHMeS5QqHx2KP9aMBRhTDqANMnwVtdyw2TDYRmF8PjpvwUFcL1Et8Hj59S3gTSMcUQ5gAqTz3Wd8EsMTmF3DChhqPQBnU" --seed "//Alice"

```

#### **Query a specific stored xpub**
If successful, the previous extrinsic returns a `XPubStored` event, which will contain the hash that links the identity and the xpub itself.
```bash
# Note that the "0x9ee..." hash was returned from the previous tx. 
polkadot-js-api query.nbvStorage.xpubs "0x9ee1b23c479e03288d3d1d791abc580439598f70e7607c1de108c4bb6a9b5b6f"
```

#### **Query accounts hash that links to the xpub**
The hash can also be retrieved by specifying the owner account.
```bash
# This tx should return the previous hash "0x9ee..."
polkadot-js-api query.nbvStorage.xpubsByOwner "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
```

#### **Remove user's xpub**
The account's xpub can be removed by submiting this extrinsic. 
```bash
polkadot-js-api tx.nbvStorage.removeXpubFromIdentity --seed "//Alice"
```
### Polkadot-js api (javascript library)
While most of the data flow is almost identical to its CLI counter part, the javascript library is much more versatile regarding queries. The API setup will be ommited.


#### **Insert an identity with a xpub**


```js
const setCompleteIdentity = api.tx.nbvStorage.setCompleteIdentity({
    "display": {
        "Raw": "Your name goes here"
    },
    "web": {
        "Raw": "https://github.com/hashed-io"
    },
    "riot": {
        "Raw": "@surveysays:matrix.org"
    },
    "email": {
        "Raw": "notrealsteve@email.com"
    }, 
    "additional": [[{
            "Raw": "ApprovedForMarketplace"
        },{
            "Raw": "1"
          }
        ]
    ]
}, "xpub6AHA9hZDN11k2ijHMeS5QqHx2KP9aMBRhTDqANMnwVtdyw2TDYRmF8PjpvwUFcL1Et8Hj59S3gTSMcUQ5gAqTz3Wd8EsMTmF3DChhqPQBnU");
const identityResult = await setCompleteIdentity.signAndSend(alice);
console.log('Extrinsic sent with hash', identityResult.toHex());
```
#### **Query stored xpubs**

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

#### **Query accounts hash that links to the xpub**

Query an accounts hashed xpub.
```hash
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
#### **Remove user's xpub**

```js
const removeAccountXpub = await api.tx.nbvStorage.removeXpubFromIdentity().signAndSend(alice);
console.log('Tx sent with hash', removeAccountXpub.toHex());
```
## Assumptions

Below are assumptions that must be held when using this module.  If any of
them are violated, the behavior of this module is undefined.

- The pallet will insert an additional identity field named `xpub`. In order to avoid conflicts and malfunctioning, it is highly advised to refrain naming another additional field like that when setting an account's identity.


License: Apache-2.0