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

- `set_complete_identity` handles the identity `info` and `xpub` insertion on the pallet's storage and the identity pallet.
- `remove_xpub_from_identity` eliminates the user's xpub. It doesn't require any parameters.
- `set_psbt` WIP. Inserts 

### Getters
- `xpubs`
- `xpubs_by_owner`

## Usage

### Polkadot-js CLI

#### Insert an identity with a xpub

```bash
polkadot-js-api tx.nbvStorage.setCompleteIdentity '{                      ─╯
    "display": {
        "Raw": "Steve Harvey"
    },
    "web": {
        "Raw": "https://steveharvey.com"
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
}' <xpubToInsert> --seed "//Alice"

```

#### Query a specific stored xpub

```bash
polkadot-js-api query.nbvStorage.xpubs <xpubHash>
```

#### Query accounts xpub

```bash
polkadot-js-api query.nbvStorage.xpubsByOwner <accountId>
```

#### Remove user's xpub
```bash
polkadot-js-api tx.nbvStorage.removeXpubFromIdentity --seed "//Alice"
```
### Polkadot-js api (javascript library)


## Assumptions

Below are assumptions that must be held when using this module.  If any of
them are violated, the behavior of this module is undefined.

- The pallet will insert an additional identity field named `xpub`. In order to avoid conflicts and malfunctioning, it is highly advised to refrain naming another additional field like that when setting an account's identity.


License: Apache-2.0