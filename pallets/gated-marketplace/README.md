# Gated marketplaces
Create marketplaces that require previous authorization before placing sell and buy orders.

- [Gated marketplaces](#gated-marketplaces)
  - [Overview](#overview)
    - [Terminology](#terminology)
  - [Interface](#interface)
    - [Dispachable functions](#dispachable-functions)
    - [Getters](#getters)
  - [Usage](#usage)
    - [Polkadot-js CLI](#polkadot-js-cli)
      - [Submit initial role setup (needs sudo](#submit-initial-role-setup-needs-sudo)
      - [Create a marketplace](#create-a-marketplace)
      - [Get a marketplace](#get-a-marketplace)
      - [Get what roles does an account have on a marketplace](#get-what-roles-does-an-account-have-on-a-marketplace)
      - [Get all the accounts that have a certain role on a marketplace](#get-all-the-accounts-that-have-a-certain-role-on-a-marketplace)
      - [Apply to a marketplace (without custodian)](#apply-to-a-marketplace-without-custodian)
      - [Apply to a marketplace (with custodian)](#apply-to-a-marketplace-with-custodian)
      - [Get an application](#get-an-application)
      - [Get the users application id on a marketplace](#get-the-users-application-id-on-a-marketplace)
      - [Get marketplace applicants by status](#get-marketplace-applicants-by-status)
      - [Get which applications the user guards as a custodian](#get-which-applications-the-user-guards-as-a-custodian)
      - [Enroll an applicant (by its account)](#enroll-an-applicant-by-its-account)
      - [Enroll an applicant (by its application id)](#enroll-an-applicant-by-its-application-id)
      - [Add authority user to marketplace](#add-authority-user-to-marketplace)
      - [Remove authority user to marketplace](#remove-authority-user-to-marketplace)
      - [Put an asset on sale](#put-an-asset-on-sale)
      - [Put a buy offer](#put-a-buy-offer)
      - [Get offer details](#get-offer-details)
      - [Get offers by item](#get-offers-by-item)
      - [Get offers by account](#get-offers-by-account)
      - [Get offers by marketplace](#get-offers-by-marketplace)
      - [Duplicate offer](#duplicate-offer)
      - [Remove offer](#remove-offer)
      - [Take sell offer - direct purchase](#take-sell-offer---direct-purchase)
      - [Take buy offer](#take-buy-offer)
    - [Polkadot-js api (javascript library)](#polkadot-js-api-javascript-library)
      - [Submit initial role setup (needs sudo)](#submit-initial-role-setup-needs-sudo-1)
      - [Create a marketplace](#create-a-marketplace-1)
      - [Get a marketplace](#get-a-marketplace-1)
      - [Get what roles does an account have on a marketplace](#get-what-roles-does-an-account-have-on-a-marketplace-1)
      - [Get all the accounts that have a certain permission on a marketplace](#get-all-the-accounts-that-have-a-certain-permission-on-a-marketplace)
      - [Apply to a marketplace (without custodian)](#apply-to-a-marketplace-without-custodian-1)
      - [Apply to a marketplace (with custodian)](#apply-to-a-marketplace-with-custodian-1)
      - [Get an application](#get-an-application-1)
      - [Get the users application id on a marketplace](#get-the-users-application-id-on-a-marketplace-1)
      - [Get marketplace applicants by status](#get-marketplace-applicants-by-status-1)
      - [Get which applications the user guards as a custodian](#get-which-applications-the-user-guards-as-a-custodian-1)
      - [Enroll an applicant (by its account)](#enroll-an-applicant-by-its-account-1)
      - [Enroll an applicant (by its application id)](#enroll-an-applicant-by-its-application-id-1)
      - [Add authority user to marketplace](#add-authority-user-to-marketplace-1)
      - [Remove authority user to marketplace](#remove-authority-user-to-marketplace-1)
      - [Put an asset on sale](#put-an-asset-on-sale-1)
      - [Put a buy offer](#put-a-buy-offer-1)
      - [Get offer details](#get-offer-details-1)
      - [Get offers by item](#get-offers-by-item-1)
      - [Get offers by account](#get-offers-by-account-1)
      - [Get offers by marketplace](#get-offers-by-marketplace-1)
      - [Duplicate offer in another marketplace](#duplicate-offer-in-another-marketplace)
      - [Remove offer](#remove-offer-1)
      - [Take sell offer - direct purchase](#take-sell-offer---direct-purchase-1)
      - [Take buy offer](#take-buy-offer-1)
  - [Events](#events)
  - [Errors](#errors)

## Overview

This module allows to: 
- Create a marketplace within on-chain storage.
- Apply to a specific marketplace, uploading documents that will get encrypted in the process.
- Enroll or reject applicants to your marketplace.
- Add or remove users as supported authorities to your marketplace, like administrators and/or asset appraisers
- WIP: Assign a rating to assets as an Appraiser.
- Create sell or buy orders. Users can bid on the item if they don't like the sale price.  

### Terminology
- **Authority**: Refers to any user that has special faculties within the marketplace, like enroll new users or grade assets.
- **Owner**: The authority that created the marketplace, it has the capacity of adding and removing new authorities and enroll or reject applicants. The owner cannot be removed from the marketplace.
- **Administrator**: Like the owner role, it can add or remove new authorities and enroll applicants, with the only difference being that an administrator can be removed from the marketplace.
- **Appraiser**: An authority that can place a rating to the enlisted assets.
- **Redemption Specialist**: This authority is responsible for transforming the on-chain asset into the IRL asset.
- **Application**: The process which a user can submit to a marketplace in order to get enrolled (or rejected). The user needs to upload the necesary documentation, while the marketplace administrator will review it and render a verdict about the users application. The documents will be encrypted and only accesible to the user, the marketplace administrator, and an optional custodian account.
- **Enroll**: When enrolled, the user's application becomes approved, gaining the ability to sell and buy assets.
- **Reject**: If the user gets rejected, its application becomes rejected and won't have access to the marketplace.
- **Custodian**: When submitting an application, the user can optionally specify a third account that will have access to the confidential documents. A custodian doesn't need to be an authority nor being part of the marketplace.
- **Sell order**: The owner of the item creates sales offer fot the item.
- **Buy order**: Users from the marketplace can bid for the item.

## Interface

### Dispachable functions

- `initial_setup` enables all the permission related functionality using the `RBAC` pallet, it can only be called by the sudo account or a majority of the Council (60%). It is essential to call this extrinsic before using other extrinsics.
- `create_marketplace` creates an on-chain marketplace record, it takes a `label` and an account that will fulfill the role of `administrator`, the extrinsic origin will be set up automatically as the marketplace owner.
- `apply` starts the process to enter the specified `marketplace`.
- `reapply` allows the applicant to apply again for the selected marketplace.
- `enroll` is only callable by the marketplace owner or administrator, as it finishes the application process. It takes a `marketplace` identification, and `account` or `application` identification to enroll or reject, and an `approved` boolean flag which approves the application if set to `true`. Owner/admin can add a feedback regarding the user's application.
- `add_authority` is only callable by the marketplace owner or administrator. As it name implies, adds a new user that will have special permission within the marketplace. It takes the `account` which will have the permissions, the type of `authority` it will have, and the `marketplace` identification in which the permissions will be enforced.
- `remove_authority` is only callable by the marketplace owner or administrator. Removes the authority enforcer from the marketplace. The marketplace owner cannot be removed and the administrator cannot remove itself.
- `update_label_marketplace`  is only callable by the marketplace owner or administrator. Changes the marketplace label. If the new label already exists, the old name won't be changed.
- `remove_marketplace`  is only callable by the marketplace owner or administrator. This action allows the user to remove a marketplace as well as all the information related to this marketplace.
- `enlist_sell_offer` is only callable by the owner of the item. It allows the user to sell an item in the selected marketplace. 
- `take_sell_offer` any user interested to buy the item can call this extrinsic. User must have enough balance to buy it. When the transaction is completed, the item ownership is transferred to the buyer. 
- `duplicate_offer` allows the owner of the item to duplicate an sell order in any marketplace. 
- `remove_offer` is only callable by the creator of the offer, it deletes any offer type from all the storages.
- `enlist_buy_offer` is callable by any market participant, the owner of the item can't create buy orders for their own items.  User must have the enough balance to call it. 
- `take_buy_offer` is only callable by the owner of the item. If the owner accepts the offer, the buyer must have enough balance to finalize the transaction. 


### Getters
|Name| Type |
|--|--|
|`marketplaces`| storage map|
|`applications`| storage map|
|`applications_by_account`|double storage map|
|`applicants_by_marketplace`|double storage map|
|`custodians`|double storage map|
|`offers_info` |storage map|
|`offers_by_item`|double storage map|
|`offers_by_account`|storage map|
|`offers_by_marketplace`|storage map|


## Usage

The following examples will be using these prefunded accounts and testing data:

```bash
# Alice's mnemonic seed
"//Alice"
# Alice's public address 
"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

# Bob's mnemonic seed
"//Bob"
# Bob's public address
"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"

# Charlie's mnemonic seed
"//Charlie"
# Charlie's public address
"5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"

# Dave's mnemonic seed
"//Dave"
# Dave's public address
"5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
```

### Polkadot-js CLI

#### Submit initial role setup (needs sudo
```bash
polkadot-js-api tx.gatedMarketplace.initialSetup --sudo --seed "//Alice"
```

#### Create a marketplace
```bash
# Administrator account and marketplace label
polkadot-js-api tx.gatedMarketplace.createMarketplace "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" "my marketplace" --seed "//Alice"
```
#### Get a marketplace
```bash
# marketplace_id
polkadot-js-api query.gatedMarketplace.marketplaces "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95"
```
```bash
# Output should look like this: 
{
  "marketplaces": {
    "label": "my marketplace"
  }
}
```

#### Get what roles does an account have on a marketplace
```bash
# account_id, pallet_id, marketplace_id
polkadot-js-api query.rbac.rolesByUser "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" 20 "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95"
```
```bash
# Output should look like this:
{
  "rolesByUser": [
    "0xc1237f9841c265fb722178da01a1e088c25fb892d6b7cd9634a20ac84bb3ee01"
  ]
}
```

#### Get all the accounts that have a certain role on a marketplace 
```bash
# pallet_id, marketplace_id, role_id
polkadot-js-api query.rbac.usersByScope 20 "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" "0x08aef7203969e2467b33b14965dfab62e11b085610c798b3cac150b1d7ea033b"
```
```bash
# Output should look like this:
{
  "usersByScope": [
    "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
  ]
}
```

#### Apply to a marketplace (without custodian)
```bash
# marketplace_id, relevant information [names,cids], and optionally, [custodian, [custodian cids]]
polkadot-js-api tx.gatedMarketplace.apply "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" '[["file1","cid1"]]' null --seed "//Charlie"
```

#### Apply to a marketplace (with custodian)
```bash
# marketplace_id, relevant information [names,cids], and optionally, [custodian, [custodian cids]]
polkadot-js-api tx.gatedMarketplace.apply "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" '[["file1","cid1"]]' '["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",["cid_custodian1"]]' --seed "//Charlie"
```

#### Get an application
```bash
# application_id
polkadot-js-api query.gatedMarketplace.applications "0x9ab75a44b507c0030296dd3660bd77d606807cf3415c3409b88c2cad36fd5483"
```
```bash
# Expected output:
{
  "applications": {
    "status": "Pending",
    "fields": [
      {
        "displayName": "file1",
        "cid": "cid1",
        "custodianCid": "cid_custodian1"
      }
    ],
    "feedback:"[]
  }
}
```

#### Get the users application id on a marketplace 
```bash
# account_id, marketplace_id
polkadot-js-api query.gatedMarketplace.applicationsByAccount "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y" "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95"
```
```bash
# Expected output:
{
  "applicationsByAccount": "0x9ab75a44b507c0030296dd3660bd77d606807cf3415c3409b88c2cad36fd5483"
}
```

#### Get marketplace applicants by status 
```bash
# marketplace_id, applicationStatus (it can be "Pending", "Approved" or "Rejected")
polkadot-js-api query.gatedMarketplace.applicantsByMarketplace "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" "Pending"
```
```bash
# Expected output:
{
  "applicantsByMarketplace": [
    "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"
  ]
}
```

#### Get which applications the user guards as a custodian 
```bash
# account_id (custodian), marketplace_id
polkadot-js-api query.gatedMarketplace.custodians "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy" "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95"
```
```bash
# Expected output
{
  "custodians": [
    "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"
  ]
}
```

#### Enroll an applicant (by its account)
```bash
# It can only be called by the marketplace owner (Alice) or administrator (Bob)
# market_id, accountOrApplicationEnumerator, feedback, approve boolean
polkadot-js-api tx.gatedMarketplace.enroll "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" '{"Account":"5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"}' "feedback" true --seed "//Bob"
```

#### Enroll an applicant (by its application id)
```bash
# It can be called by the marketplace owner (Alice) or administrator (Bob)
# market_id, accountOrApplicationEnumerator, approve boolean
polkadot-js-api tx.gatedMarketplace.enroll "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" '{"Application":"0x9ab75a44b507c0030296dd3660bd77d606807cf3415c3409b88c2cad36fd5483"}' "feedback" true --seed "//Bob"
```

#### Add authority user to marketplace
```bash
# It can only be called by the marketplace owner (Alice) or administrator (Bob)
# account_id, MarketplaceAuthority (it can be "Owner", "Admin" or "Appraiser")
polkadot-js-api tx.gatedMarketplace.addAuthority "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy" "Appraiser" "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" --seed "//Alice"
```

#### Remove authority user to marketplace
```bash
# It can only be called by the marketplace owner (Alice) or administrator (Bob)
# account_id, MarketplaceAuthority (it can be "Owner", "Admin" or "Appraiser")
polkadot-js-api tx.gatedMarketplace.removeAuthority "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy" "Appraiser" "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" --seed "//Alice"
```

#### Put an asset on sale
```bash
# marketplace_id, collection_id, item_id, sell price
polkadot-js-api tx.gatedMarketplace.enlistSellOffer "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" 0 0 10000 --seed "//Charlie"
```

#### Put a buy offer
```bash
# marketplace_id, collection_id, item_id, buy price
polkadot-js-api tx.gatedMarketplace.enlistBuyOffer "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" 0 0 10001 --seed "//Dave"
```

#### Get offer details
```bash
polkadot-js-api query.gatedMarketplace.offersInfo "0x9abbb3e227dedf26a4a64705ffb924ef8d48dc47de981f4db799790ae2239e6b"
```

```bash
# Output should look like this
{
  "offersInfo": {
    "marketplaceId": "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95",
    "collectionId": "0",
    "itemId": "0",
    "creator": "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
    "price": "10,000",
    "status": "Open",
    "creationDate": "1,660,778,892,000",
    "expirationDate": "1,661,383,692,000",
    "offerType": "SellOrder",
    "buyer": null
  }
}
```

#### Get offers by item
```bash
# collection_id, item_id
polkadot-js-api query.gatedMarketplace.offersByItem 0 0
```

```bash
# Output should look similar
{
  "offersByItem": [
    "0x4508b428b15e1a0a0138d36efebe3382739726beca8d67239e02a56c19d378eb",
    "0x66fcfadc174a596d8f8dc1b067038ed0056c5c3127d6996bc54fa05148caccf0"
  ]
}
```

#### Get offers by account
```bash
# account_id
polkadot-js-api query.gatedMarketplace.offersByAccount 5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y
```

```bash
{
  "offersByAccount": [
    "0x4508b428b15e1a0a0138d36efebe3382739726beca8d67239e02a56c19d378eb"
  ]
}
```


#### Get offers by marketplace
```bash
# marketplace_id
polkadot-js-api query.gatedMarketplace.offersByMarketplace 0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95
```

```bash
# Output should llok like this
{
  "offersByMarketplace": [
    "0x4508b428b15e1a0a0138d36efebe3382739726beca8d67239e02a56c19d378eb",
    "0x66fcfadc174a596d8f8dc1b067038ed0056c5c3127d6996bc54fa05148caccf0"
  ]
}
```

#### Duplicate offer

```bash
polkadot-js-api tx.gatedMarketplace.duplicateOffer "0x65c7f4fa353a2212c2db497a8a1ad073453aad2030be7f756cba42a2f976dc82" "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" 0 0 10002 --seed "//Charlie"
```

#### Remove offer

```bash
# offer_id, marketplace_id, collection_id, item_id 
polkadot-js-api tx.gatedMarketplace.removeOffer "0x8cb8cc124e19fc58eaf9c6dbd0953a7fd955769e6d3983ce2ea83d64d742a62e" "0xa1c17609528fe2630b3be72d6ac8eafc5e0ef95ce78ddad70e83e5fa77ac7342" 0 0 --seed "//Charlie"
```

#### Take sell offer - direct purchase
```bash
polkadot-js-api tx.gatedMarketplace.takeSellOffer "0x65c7f4fa353a2212c2db497a8a1ad073453aad2030be7f756cba42a2f976dc82" "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" 0 0 --seed "//Dave"
```

#### Take buy offer
```bash
# offer_id, marketplace_id, collection_id, item_id
polkadot-js-api tx.gatedMarketplace.takeBuyOffer "0x66fcfadc174a596d8f8dc1b067038ed0056c5c3127d6996bc54fa05148caccf0" "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" 0 0 --seed "//Charlie"
```

### Polkadot-js api (javascript library)
While most of the data flow is almost identical to its CLI counter part, the javascript library is much more versatile regarding queries.


#### Submit initial role setup (needs sudo)
```js
const initial_set_up = await api.tx.sudo.sudo(api.tx.gatedMarketplace.initialSetup()).signAndSend(alice);
```

#### Create a marketplace
```js
# Administrator account and marketplace label
const createMarketplace = await api.tx.gatedMarketplace.createMarketplace(bob.address, "my marketplace").signAndSend(alice);
```
#### Get a marketplace
```js
// marketplace_id
const markets = await api.query.gatedMarketplace.marketplaces("0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95");
  console.log(markets.toHuman());
```
```bash
# Output should look like this: 
{ label: 'my marketplace' }
```
```js
// get all marketplaces
const marketplaces = await api.query.gatedMarketplace.marketplaces.entries();
  marketplaces.forEach(([key, exposure]) => {
    console.log('key marketplace_id:', key.args.map((k) => k.toHuman()));
    console.log('     marketplace:', exposure.toHuman(),"\n");
  });
```
```bash
# Output:
key marketplace_id: [
  '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95'
]
     marketplace: { label: 'my marketplace' }
```

#### Get what roles does an account have on a marketplace
```js
// account_id, pallet_id, scope_id
const rolesByUser = await api.query.rbac.rolesByUser(alice.address,20, "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95");
  console.log(rolesByUser.toHuman());
```
```bash
# Output should look like this:
['0x08aef7203969e2467b33b14965dfab62e11b085610c798b3cac150b1d7ea033b']
```

```js
// get all users permissions on all marketplaces
const all_roles_by_user = await api.query.rbac.rolesByUser.entries();
all_roles_by_user.forEach(([key, exposure]) => {
  console.log('account_id, pallet_id, scope_id:', key.args.map((k) => k.toHuman()));
  console.log('     role_ids', exposure.toHuman());
});
```

```bash
account_id, pallet_id, scope_id: [
  '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
  '20',
  '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95'
]
     role_ids [
  '0xc1237f9841c265fb722178da01a1e088c25fb892d6b7cd9634a20ac84bb3ee01'
]
account_id, pallet_id, scope_id: [
  '5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y',
  '20',
  '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95'
]
     role_ids [
  '0xae9e025522f868c39b41b8a5ba513335a2a229690bd44c71c998d5a9ad38162b'
]
account_id, pallet_id, scope_id: [
  '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
  '20',
  '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95'
]
     role_ids [
  '0x08aef7203969e2467b33b14965dfab62e11b085610c798b3cac150b1d7ea033b'
]
```

#### Get all the accounts that have a certain permission on a marketplace 
```js
//pallet_id, marketplace_id, type of authoriry (it can be "Owner", "Admin" or "Appraiser")
const usersByScope = await api.query.rbac.usersByScope(20, "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95", "0x08aef7203969e2467b33b14965dfab62e11b085610c798b3cac150b1d7ea033b");
console.log(usersByScope.toHuman());
```
```bash
# Output should look like this:
[ '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty' ]
```

```js
// get  all the accounts in a marketplace
const scope_users_by_role = await api.query.rbac.usersByScope.entries(20, "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95");
  scope_users_by_role.forEach(([key, exposure]) => {
    console.log('pallet_id, scope_id, role_id:', key.args.map((k) => k.toHuman()));
    console.log('     account_id', exposure.toHuman());
  });
```
```bash
# Expected output:
pallet_id, scope_id, role_id: [
  '20',
  '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95',
  '0xae9e025522f868c39b41b8a5ba513335a2a229690bd44c71c998d5a9ad38162b'
]
     account_id [ '5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y' ]
pallet_id, scope_id, role_id: [
  '20',
  '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95',
  '0x08aef7203969e2467b33b14965dfab62e11b085610c798b3cac150b1d7ea033b'
]
     account_id [ '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY' ]
pallet_id, scope_id, role_id: [
  '20',
  '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95',
  '0xc1237f9841c265fb722178da01a1e088c25fb892d6b7cd9634a20ac84bb3ee01'
]
     account_id [ '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty' ]
```

#### Apply to a marketplace (without custodian)
```js
// marketplace_id, relevant information [names,cids], and optionally, [custodian, [custodian cids]]
const apply = await api.tx.gatedMarketplace.apply("0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95",[["file1","cid1"]], null).signAndSend(charlie);
```

#### Apply to a marketplace (with custodian)
```js
// marketplace_id, relevant information [names,cids], and optionally, [custodian, [custodian cids]]
# marketplace_id, relevant information [names,cids], and optionally, [custodian, [custodian cids]]
const apply = await api.tx.gatedMarketplace.apply("0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95",[["file1","cid1"]], [dave.address,["cid_custodian1"]]).signAndSend(charlie);
```

#### Get an application
```js
// application_id
const application = await api.query.gatedMarketplace.applications("0x9ab75a44b507c0030296dd3660bd77d606807cf3415c3409b88c2cad36fd5483");
console.log(application.toHuman());
```
```bash
# Expected output:
{
  status: 'Pending',
  fields: [
    {
      displayName: 'file1',
      cid: 'cid1',
      custodianCid: 'cid_custodian1'
    }
  ],
  feedback:[]
}
```

```js
// get all applications
const applications = await api.query.gatedMarketplace.applications.entries();
applications.forEach(([key, exposure]) => {
  console.log('application_id:', key.args.map((k) => k.toHuman()));
  console.log('     application details:', exposure.toHuman());
});
```

```bash
# Output:
application_id: [
  '0x9ab75a44b507c0030296dd3660bd77d606807cf3415c3409b88c2cad36fd5483'
]
     application details: {
  status: 'Pending',
  fields: [
    {
      displayName: 'file1',
      cid: 'cid1',
      custodianCid: 'cid_custodian1'
    }
  ],
  feedback:[]
}
```

#### Get the users application id on a marketplace 
```js
# account_id, marketplace_id
const applicationsByAccount = await api.query.gatedMarketplace.applicationsByAccount(charlie.address, "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95");
console.log(applicationsByAccount.toHuman());

```
```bash
# Expected output:
0x9ab75a44b507c0030296dd3660bd77d606807cf3415c3409b88c2cad36fd5483
```

```js
// get all applications of all users
const applicationsByAccount = await api.query.gatedMarketplace.applicationsByAccount.entries();
applicationsByAccount.forEach(([key, exposure]) => {
  console.log(' account_id and marketplace_id:', key.args.map((k) => k.toHuman()));
  console.log('     application_id:', exposure.toHuman());
});
```

```bash
# Output:
 account_id and marketplace_id: [
  '5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y',
  '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95'
]
     application_id: 0x9ab75a44b507c0030296dd3660bd77d606807cf3415c3409b88c2cad36fd5483
```

#### Get marketplace applicants by status 
```js
# marketplace_id, applicationStatus (it can be "Pending", "Approved" or "Rejected")
const applicantsByMarketplace = await api.query.gatedMarketplace.applicantsByMarketplace("0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95","Pending");
  console.log(applicantsByMarketplace.toHuman());
```
```bash
# Expected output:
[ '5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y' ]
```

```js
// get all applicants for all marketplaces, grouped by status
const applicantsByMarketplace = await api.query.gatedMarketplace.applicantsByMarketplace.entries();
applicantsByMarketplace.forEach(([key, exposure]) => {
  console.log(' marketplace_id and status:', key.args.map((k) => k.toHuman()));
  console.log('     applicants:', exposure.toHuman());
});
```
```bash
 marketplace_id and status: [
  '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95',
  'Pending'
]
     applicants: [ '5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y' ]
```

#### Get which applications the user guards as a custodian 
```js
// account_id (custodian), marketplace_id
const custodians = await api.query.gatedMarketplace.custodians(dave.address, "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95");
console.log(custodians.toHuman());
```
```bash
# Expected output
[ '5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y' ]
```

```js
// get all applications by custodian grouped by marketplace
const custodians = await api.query.gatedMarketplace.custodians.entries();
custodians.forEach(([key, exposure]) => {
  console.log(' custodian and marketplace_id:', key.args.map((k) => k.toHuman()));
  console.log('     marketplace applicants:', exposure.toHuman());
});
```
```bash
# Expected output
 custodian and marketplace_id: [
  '5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy',
  '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95'
]
    marketplace applicants: [ '5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y' ]
```

#### Enroll an applicant (by its account)
```bash
# It can only be called by the marketplace owner (Alice) or administrator (Bob)
# market_id, accountOrApplicationEnumerator, feedback, approve boolean
const enroll = await api.tx.gatedMarketplace.enroll("0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95", {"Account":charlie.address}, "feedback", true).signAndSend(alice);

```

#### Enroll an applicant (by its application id)
```bash
# It can be called by the marketplace owner (Alice) or administrator (Bob)
# market_id, accountOrApplicationEnumerator, feedback, approve boolean
const enroll = await api.tx.gatedMarketplace.enroll("0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95", {"Application":"0x9ab75a44b507c0030296dd3660bd77d606807cf3415c3409b88c2cad36fd5483"}, "feedback", true).signAndSend(alice);

```

#### Add authority user to marketplace
```js
// It can only be called by the marketplace owner (Alice) or administrator (Bob)
// account_id, MarketplaceAuthority (it can be "Owner", "Admin" or "Appraiser")
const addAuthority = await api.tx.gatedMarketplace.addAuthority(dave.address, "Appraiser", "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" ).signAndSend(alice);

```

#### Remove authority user to marketplace
```js
// It can only be called by the marketplace owner (Alice) or administrator (Bob)
// account_id, MarketplaceAuthority (it can be "Owner", "Admin" or "Appraiser")
const removeAuthority = await api.tx.gatedMarketplace.removeAuthority(dave.address, "Appraiser", "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" ).signAndSend(alice);

```

#### Put an asset on sale
```js
// marketplace_id, collection_id, item_id, sell price
const sell = await api.tx.gatedMarketplace.enlistSellOffer("0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95",0,0,10000).signAndSend(charlie);
```

#### Put a buy offer
```js
// marketplace_id, collection_id, item_id, buy price
const buy = await api.tx.gatedMarketplace.enlistBuyOffer("0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95", 0,0, 10001).signAndSend(dave);
```

#### Get offer details
```js
const offer_info = await api.query.gatedMarketplace.offersInfo("0x9abbb3e227dedf26a4a64705ffb924ef8d48dc47de981f4db799790ae2239e6b");
  console.log(offer_info.toHuman());
```

```bash
# Output should look like this
{
  marketplaceId: '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95',
  collectionId: '0',
  itemId: '0',
  creator: '5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y',
  price: '10,000',
  status: 'Open',
  creationDate: '1,660,778,892,000',
  expirationDate: '1,661,383,692,000',
  offerType: 'SellOrder',
  buyer: null
}
```

```js
// Get details of all offers 
const all_offers =  await api.query.gatedMarketplace.offersInfo.entries()
all_offers.forEach(([key, exposure]) => {
  console.log('offer_id:', key.args.map((k) => k.toHuman()));
  console.log('offer details:', exposure.toHuman());
});
```

```bash
# Output should look like this
offer_id: [
  '0x9abbb3e227dedf26a4a64705ffb924ef8d48dc47de981f4db799790ae2239e6b'
]
offer details: {
  marketplaceId: '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95',
  collectionId: '0',
  itemId: '0',
  creator: '5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y',
  price: '10,000',
  status: 'Open',
  creationDate: '1,660,778,892,000',
  expirationDate: '1,661,383,692,000',
  offerType: 'SellOrder',
  buyer: null
}
# ...
```

#### Get offers by item
```js
const offers_by_item = await api.query.gatedMarketplace.offersByItem(0,0);
console.log(offers_by_item.toHuman());
```

```bash
[
  '0x4508b428b15e1a0a0138d36efebe3382739726beca8d67239e02a56c19d378eb',
  '0x66fcfadc174a596d8f8dc1b067038ed0056c5c3127d6996bc54fa05148caccf0'
]
```

```js
// Get all offers in the whole collection
// collection_id, could get omitted to get all offers from all assets, grouped by collection.
const offers_by_collection = await api.query.gatedMarketplace.offersByItem.entries(0);
offers_by_collection.forEach(([key, exposure]) => {
  console.log('collection_id, item_id:', key.args.map((k) => k.toHuman()));
  console.log('offer_ids:', exposure.toHuman());
});
```

```bash
# Output should look like this
collection_id, item_id: [ '0', '0' ]
offer_ids: [
  '0x4508b428b15e1a0a0138d36efebe3382739726beca8d67239e02a56c19d378eb',
  '0x66fcfadc174a596d8f8dc1b067038ed0056c5c3127d6996bc54fa05148caccf0'
]
```

#### Get offers by account
```js
// account_id
const offers_by_account = await api.query.gatedMarketplace.offersByAccount(charlie.address);
console.log(offers_by_account.toHuman());
```

```bash
['0x4508b428b15e1a0a0138d36efebe3382739726beca8d67239e02a56c19d378eb']
```

```js
const all_offers_by_account = await api.query.gatedMarketplace.offersByAccount.entries();
all_offers_by_account.forEach(([key, exposure]) => {
  console.log('account_id:', key.args.map((k) => k.toHuman()));
  console.log('offer_ids:', exposure.toHuman());
});
```

```bash
account_id: [ '5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy' ]
offer_ids: [
  '0x66fcfadc174a596d8f8dc1b067038ed0056c5c3127d6996bc54fa05148caccf0'
]
account_id: [ '5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y' ]
offer_ids: [
  '0x4508b428b15e1a0a0138d36efebe3382739726beca8d67239e02a56c19d378eb'
]
```


#### Get offers by marketplace

```js
// marketplace_id
const offers_by_market = await api.query.gatedMarketplace.offersByMarketplace("0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95")
  console.log(offers_by_market.toHuman());
```

```bash
# output should look like this
[
  '0x4508b428b15e1a0a0138d36efebe3382739726beca8d67239e02a56c19d378eb',
  '0x66fcfadc174a596d8f8dc1b067038ed0056c5c3127d6996bc54fa05148caccf0'
]
```

```js
// All offers by marketplace
const all_offers_by_market = await api.query.gatedMarketplace.offersByMarketplace.entries();
all_offers_by_market.forEach(([key, exposure]) => {
  console.log('marketplace_id:', key.args.map((k) => k.toHuman()));
  console.log('offer_ids:', exposure.toHuman());
});
```

```bash
# output should look like this
marketplace_id: [
  '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95'
]
offer_ids: [
  '0x4508b428b15e1a0a0138d36efebe3382739726beca8d67239e02a56c19d378eb',
  '0x66fcfadc174a596d8f8dc1b067038ed0056c5c3127d6996bc54fa05148caccf0'
]
```

#### Duplicate offer in another marketplace
```js
/// offer_id, marketplace_id, collection_id, item_id, price on that marketplace
const duplicate_offer = await api.tx.gatedMarketplace.duplicateOffer("0x65c7f4fa353a2212c2db497a8a1ad073453aad2030be7f756cba42a2f976dc82","0xa1c17609528fe2630b3be72d6ac8eafc5e0ef95ce78ddad70e83e5fa77ac7342", 0, 0, 10002).signAndSend(charlie) 
```

#### Remove offer
```js
/// offer_id, marketplace_id, collection_id, item_id
const remove_offer = await api.tx.gatedMarketplace.removeOffer("0x8cb8cc124e19fc58eaf9c6dbd0953a7fd955769e6d3983ce2ea83d64d742a62e", "0xa1c17609528fe2630b3be72d6ac8eafc5e0ef95ce78ddad70e83e5fa77ac7342", 0, 0).signAndSend(charlie)
```

#### Take sell offer - direct purchase
```js
/// offer_id, marketplace_id, collection_id, item_id
const take_sell_offer = await api.tx.gatedMarketplace.takeSellOffer("0x65c7f4fa353a2212c2db497a8a1ad073453aad2030be7f756cba42a2f976dc82", "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95", 0, 0).signAndSend(dave)
```

#### Take buy offer

```js
/// offer_id, marketplace_id, collection_id, item_id
const take_buy_offer = await api.tx.gatedMarketplace.takeBuyOffer("0x66fcfadc174a596d8f8dc1b067038ed0056c5c3127d6996bc54fa05148caccf0", "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95", 0, 0).signAndSend(charlie)
```

## Events

```rust
/// Marketplaces stored. [owner, admin, market_id]
1. MarketplaceStored(T::AccountId, T::AccountId, [u8;32])

/// Application stored on the specified marketplace. [application_id, market_id]
2. ApplicationStored([u8;32], [u8;32])

/// An applicant was accepted or rejected on the marketplace. [AccountOrApplication, market_id, status]
3. ApplicationProcessed(AccountOrApplication<T>,[u8;32], ApplicationStatus)

/// Add a new authority to the selected marketplace
4. AuthorityAdded(T::AccountId, MarketplaceAuthority)

/// Remove the selected authority from the selected marketplace
5. AuthorityRemoved(T::AccountId, MarketplaceAuthority)

/// The label of the selected marketplace has been updated. [market_id]
6. MarketplaceLabelUpdated([u8;32])

/// The selected marketplace has been removed. [market_id]
7. MarketplaceRemoved([u8;32])

/// Offer stored. [collection_id, item_id]
8. OfferStored(T::CollectionId, T::ItemId)

/// Offer was accepted [offer_id, account]
9. OfferWasAccepted([u8;32], T::AccountId)

/// Offer was duplicated. [new_offer_id, new_marketplace_id]
10. OfferDuplicated([u8;32], [u8;32])
```

## Errors

```rust
///Limit bounded vector exceeded
LimitExceeded,
/// The account supervises too many marketplaces
ExceedMaxMarketsPerAuth,
/// The account has too many roles in that marketplace 
ExceedMaxRolesPerAuth,
/// Too many applicants for this market! try again later
ExceedMaxApplicants,
/// This custodian has too many applications for this market, try with another one
ExceedMaxApplicationsPerCustodian,
/// Applicaion doesnt exist
ApplicationNotFound,
/// The user has not applicated to that market before
ApplicantNotFound,
/// The user cannot be custodian of its own application
ApplicantCannotBeCustodian,
/// A marketplace with the same data exists already
MarketplaceAlreadyExists,
/// The user has already applied to the marketplace (or an identical application exist)
AlreadyApplied,
/// The specified marketplace does not exist
MarketplaceNotFound,
/// You need to be an owner or an admin of the marketplace
NotOwnerOrAdmin,
/// There was no change regarding the application status
AlreadyEnrolled,
/// There cannot be more than one owner per marketplace
OnlyOneOwnerIsAllowed,
/// Cannot remove the owner of the marketplace
CantRemoveOwner,
/// Admin can not remove itself from the marketplace
AdminCannotRemoveItself,
/// User not found
UserNotFound,
/// Owner not found
OwnerNotFound,
// Rol not found for the selected user
AuthorityNotFoundForUser,
/// Admis cannot be deleted between them, only the owner can
CannotDeleteAdmin,
/// Application ID not found
ApplicationIdNotFound,
/// Application status is still pending, user cannot apply/reapply
ApplicationStatusStillPending,
/// The application has already been approved, application status is approved
ApplicationHasAlreadyBeenApproved,
/// Collection not found
CollectionNotFound,
/// User who calls the function is not the owner of the collection
NotOwner,
/// Offer already exists
OfferAlreadyExists,
/// Offer not found
OfferNotFound,
/// Offer is not available at the moment
OfferIsNotAvailable,
/// Owner cannnot buy its own offer
CannotTakeOffer,
/// User cannot remove the offer from the marketplace
CannotRemoveOffer,
/// Error related to the timestamp
TimestampError,
/// User does not have enough balance to buy the offer
NotEnoughBalance,
/// User cannot delete the offer because is closed
CannotDeleteOffer,
/// There was a problem storing the offer
OfferStorageError,
/// Price must be greater than zero
PriceMustBeGreaterThanZero,
/// User cannot create buy offers for their own items
CannotCreateOffer,
/// This items is not available for sale
ItemNotForSale,
```