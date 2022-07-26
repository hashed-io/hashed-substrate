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
      - [Create a marketplace](#create-a-marketplace)
      - [Get a marketplace](#get-a-marketplace)
      - [Get what permissions does an account have on a marketplace](#get-what-permissions-does-an-account-have-on-a-marketplace)
      - [Get all the accounts that have a certain permission on a marketplace](#get-all-the-accounts-that-have-a-certain-permission-on-a-marketplace)
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
    - [Polkadot-js api (javascript library)](#polkadot-js-api-javascript-library)
      - [Create a marketplace](#create-a-marketplace-1)
      - [Get a marketplace](#get-a-marketplace-1)
      - [Get what permissions does an account have on a marketplace](#get-what-permissions-does-an-account-have-on-a-marketplace-1)
      - [Get all the accounts that have a certain permission on a marketplace](#get-all-the-accounts-that-have-a-certain-permission-on-a-marketplace-1)
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
  - [Events](#events)
  - [Errors](#errors)

## Overview

This module allows to: 
- Create a marketplace within on-chain storage.
- Apply to a specific marketplace, uploading documents that will get encrypted in the process.
- Enroll or reject applicants to your marketplace.
- Add or remove users as supported authorities to your marketplace, like administrators and/or asset appraisers
- WIP: Assign a rating to assets as an Appraiser.

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

## Interface

### Dispachable functions
- `create_marketplace` creates an on-chain marketplace record, it takes a `label` and an account that will fulfill the role of `administrator`, the extrinsic origin will be set up automatically as the marketplace owner.
- `apply` starts the process to enter the specidied `marketplace`.
- `reapply` allows the applicant to apply again for the selected marketplace.
- `enroll` is only callable by the marketplace owner or administrator, as it finishes the application process. It takes a `marketplace` identification, and `account` or `application` identification to enroll or reject, and an `approved` boolean flag which approves the application if set to `true`. Owner/admin can add a feedback regarding the user's application.
- `add_authority` is only callable by the marketplace owner or administrator. As it name implies, adds a new user that will have special permission within the marketplace. It takes the `account` which will have the permissions, the type of `authority` it will have, and the `marketplace` identification in which the permissions will be enforced.
- `remove_authority` is only callable by the marketplace owner or administrator. Removes the authority enforcer from the marketplace. The marketplace owner cannot be removed and the administrator cannot remove itself.
- `update_label_marketplace`  is only callable by the marketplace owner or administrator. Changes the marketplace label. If the new label already exists, the old name won't be changed.
- `remove_marketplace`  is only callable by the marketplace owner or administrator. This action allows the user to remove a marketplace as well as all the information related to this marketplace.


### Getters
- `marketplaces`
- `marketplaces_by_authority` (double storage map)
- `authorities_by_marketplace` (double storage map)
- `applications` 
- `applications_by_account` (double storage map)
- `applicants_by_marketplace` (double storage map)
- `custodians` (double storage map)


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

#### Get what permissions does an account have on a marketplace
```bash
# account_id, marketplace_id
polkadot-js-api query.gatedMarketplace.marketplacesByAuthority "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95"
```
```bash
# Output should look like this:
{
  "marketplacesByAuthority": [
    "Owner"
  ]
}
```

#### Get all the accounts that have a certain permission on a marketplace 
```bash
# marketplace_id, type of authoriry (it can be "Owner", "Admin" or "Appraiser")
polkadot-js-api query.gatedMarketplace.authoritiesByMarketplace "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" "Admin"
```
```bash
# Output should look like this:
{
  "authoritiesByMarketplace": [
    "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
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
# market_id, accountOrApplicationEnumerator, approve boolean
polkadot-js-api tx.gatedMarketplace.enroll "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" '{"Account":"5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"}' true --seed "//Bob"
```

#### Enroll an applicant (by its application id)
```bash
# It can be called by the marketplace owner (Alice) or administrator (Bob)
# market_id, accountOrApplicationEnumerator, approve boolean
polkadot-js-api tx.gatedMarketplace.enroll "0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95" '{"Application":"0x9ab75a44b507c0030296dd3660bd77d606807cf3415c3409b88c2cad36fd5483"}' true --seed "//Bob"
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

### Polkadot-js api (javascript library)
While most of the data flow is almost identical to its CLI counter part, the javascript library is much more versatile regarding queries. The API setup will be ommited.

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

#### Get what permissions does an account have on a marketplace
```js
const marketplacesByAuth = await api.query.gatedMarketplace.marketplacesByAuthority("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95");
  console.log(marketplacesByAuth.toHuman() );
```
```bash
# Output should look like this:
[ 'Owner' ]
```

```js
// get all users permissions on all marketplaces
const allMarketplacesByAuth = await api.query.gatedMarketplace.marketplacesByAuthority.entries();
allMarketplacesByAuth.forEach(([key, exposure]) => {
  console.log('Authority account and marketplace_id:', key.args.map((k) => k.toHuman()));
  console.log('     type of authority:', exposure.toHuman(),"\n");
});
```

```bash
Authority account and marketplace_id: [
  '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
  '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95'
]
     type of authority: [ 'Admin' ]

Authority account and marketplace_id: [
  '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
  '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95'
]
     type of authority: [ 'Owner' ]
```

#### Get all the accounts that have a certain permission on a marketplace 
```js
//marketplace_id, type of authoriry (it can be "Owner", "Admin" or "Appraiser")
const authoritiesByMarketplace = await api.query.gatedMarketplace.authoritiesByMarketplace("0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95","Admin");
  console.log(authoritiesByMarketplace.toHuman());
```
```bash
# Output should look like this:
[ '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty' ]
```

```js
// get  all the accounts in a marketplace
const authoritiesByMarketplace = await api.query.gatedMarketplace.authoritiesByMarketplace.entries();
authoritiesByMarketplace.forEach(([key, exposure]) => {
    console.log('marketplace_id and type of authority:', key.args.map((k) => k.toHuman()));
    console.log('     accounts that have the role within the marketplace:', exposure.toHuman(),"\n");
  });
```
```bash
# Expected output:
marketplace_id and type of authority: [
  '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95',
  'Admin'
]
     accounts that have the role within the marketplace: [ '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty' ]

marketplace_id and type of authority: [
  '0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95',
  'Owner'
]
     accounts that have the role within the marketplace: [ '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY' ]
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
# market_id, accountOrApplicationEnumerator, approve boolean
const enroll = await api.tx.gatedMarketplace.enroll("0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95", {"Account":charlie.address}, true).signAndSend(alice);

```

#### Enroll an applicant (by its application id)
```bash
# It can be called by the marketplace owner (Alice) or administrator (Bob)
# market_id, accountOrApplicationEnumerator, approve boolean
const enroll = await api.tx.gatedMarketplace.enroll("0xace33a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b95", {"Application":"0x9ab75a44b507c0030296dd3660bd77d606807cf3415c3409b88c2cad36fd5483"}, true).signAndSend(alice);

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

## Events

```rust
/// Marketplaces stored. [owner, admin, market_id]
MarketplaceStored(T::AccountId, T::AccountId, [u8;32]),
/// Application stored on the specified marketplace. [application_id, market_id]
ApplicationStored([u8;32], [u8;32]),
/// An applicant was accepted or rejected on the marketplace. [AccountOrApplication, market_id, status]
ApplicationProcessed(AccountOrApplication<T>,[u8;32], ApplicationStatus),
/// Add a new authority to the selected marketplace
AuthorityAdded(T::AccountId, MarketplaceAuthority),
/// Remove the selected authority from the selected marketplace
AuthorityRemoved(T::AccountId, MarketplaceAuthority),
/// The label of the selected marketplace has been updated. [market_id]
MarketplaceLabelUpdated([u8;32]),
/// The selected marketplace has been removed. [market_id]
MarketplaceRemoved([u8;32])
```

## Errors

```rust
/// Work In Progress
NotYetImplemented,
/// Error names should be descriptive.
NoneValue,
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
CannotEnroll,
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
```