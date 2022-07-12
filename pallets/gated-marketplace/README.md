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
      - [Apply to a marketplace](#apply-to-a-marketplace)
      - [Enroll () an applicant](#enroll--an-applicant)
    - [Polkadot-js api (javascript library)](#polkadot-js-api-javascript-library)
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
- **Application**: The process which a user can submit to a marketplace in order to get enrolled (or rejected). The user needs to upload the necesary documentation, while the marketplace administrator will review it and render a verdict about the users application. The documents will be encrypted and only accesible to the user, the marketplace administrator, and an optional custodian account.
- **Enroll**: When enrolled, the user's application becomes approved, gaining the ability to sell and buy assets.
- **Reject**: If the user gets rejected, its application becomes rejected and won't have access to the marketplace.
- **Custodian**: When submitting an application, the user can optionally specify a third account that will have access to the confidential documents. A custodian doesn't need to be an authority nor being part of the marketplace.

## Interface

### Dispachable functions
- `create_marketplace` creates an on-chain marketplace record, it takes a `label` and an account that will fulfill the role of `administrator`, the extrinsic origin will be set up automatically as the marketplace owner.
- `apply` starts the process to enter the specidied `marketplace`.
- `enroll` is only callable by the marketplace owner or administrator, as it finishes the application process. It takes a `marketplace` identification, and `account` or `application` identification to enroll or reject, and an `approved` boolean flag which approves the application if set to `true`.
- `add_authority` is only callable by the marketplace owner or administrator. As it name implies, adds a new user that will have special permission within the marketplace. It takes the `account` which will have the permissions, the type of `authority` it will have, and the `marketplace` identification in which the permissions will be enforced.
- `remove_authority` is only callable by the marketplace owner or administrator. Removes the authority enforcer from the marketplace. The marketplace owner cannot be removed and the administrator cannot remove itself.

### Getters
- `marketplaces`
- `marketplaces_by_authority` (double storage map)
- `authorities_by_marketplace` (double storage map)
- `applications` 
- `applications_by_account` (double storage map)
- `applicants_by_marketplace` (double storage map)
- `custodians` (double storage map)


## Usage

The following examples will be using these credentials and testing data:

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
```

### Polkadot-js CLI

#### Create a marketplace
```bash
# Administrator account and marketplace label
polkadot-js-api tx.gatedMarketplace.createMarketplace "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" "my marketplace" --seed "//Alice"
```
#### Get a marketplace

#### Apply to a marketplace

#### Enroll () an applicant


### Polkadot-js api (javascript library)

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
/// Admin can not remove itself
NegateRemoveAdminItself,
/// User has already been assigned with that role
CannotAddAuthority,
/// User not found
UserNotFound,
// Rol not found for the selected user
RolNotFoundForUser,
/// User is not admin	
UserIsNotAdmin,
/// User is not found for the query
UserNotFoundForThisQuery
```