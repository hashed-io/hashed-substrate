# Gated marketplaces
Create marketplaces that require permissions to access.

- [Gated marketplaces](#gated-marketplaces)
  - [Overview](#overview)
    - [Terminology](#terminology)
  - [Interface](#interface)
    - [Dispachable functions](#dispachable-functions)
    - [Getters](#getters)
  - [Usage](#usage)
    - [Polkadot-js CLI](#polkadot-js-cli)
    - [Polkadot-js api (javascript library)](#polkadot-js-api-javascript-library)
  - [Events](#events)
  - [Errors](#errors)

## Overview

### Terminology

## Interface

### Dispachable functions

### Getters

## Usage

### Polkadot-js CLI

### Polkadot-js api (javascript library)

## Events

```rust
/// Marketplaces stored. [owner, admin, market_id]
MarketplaceStored(T::AccountId, T::AccountId, [u8;32]),
/// Application stored on the specified marketplace. [application_id, market_id]
ApplicationStored([u8;32], [u8;32]),
/// An applicant was accepted or rejected on the marketplace. [AccountOrApplication, market_id, status]
ApplicationProcessed(AccountOrApplication<T>,[u8;32], ApplicationStatus),
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
/// Applicaion doesnt exist
ApplicationNotFound,
/// The user has not applicated to that market before
ApplicantNotFound,
/// A marketplace with the same data exists already
MarketplaceAlreadyExists,
/// The user has already applied to the marketplace (or an identical application exist)
AlreadyApplied,
/// The specified marketplace does not exist
MarketplaceNotFound,
/// You need to be an owner or an admin of the marketplace
CannotEnroll,
```