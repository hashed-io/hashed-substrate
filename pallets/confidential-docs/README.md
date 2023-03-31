# Confidential documents
Provides the backend services and metadata storage for the confidential docs solution

- [Confidential documents](#confidential-documents)
  - [Overview](#overview)
  - [Interface](#interface)
    - [Dispachable functions](#dispachable-functions)
    - [Getters](#getters)
  - [Usage](#usage)
    - [Polkadot-js api (javascript library)](#polkadot-js-api-javascript-library)
      - [Create a vault](#create-a-vault)
      - [Get a vault](#get-a-vault)
      - [Get a public key](#get-a-public-key)
      - [Create an owned confidential document](#create-an-owned-confidential-document)
      - [Get an owned confidential document by CID](#get-an-owned-confidential-document-by-cid)
      - [Remove an owned confidential document](#remove-an-owned-confidential-document)
      - [Share a confidential document](#share-a-confidential-document)
      - [Get a shared confidential document by CID](#get-a-shared-confidential-document-by-cid)
      - [Update a shared confidential document's metadata](#update-a-shared-confidential-documents-metadata)
      - [Remove a shared confidential document](#remove-a-shared-confidential-document)
      - [Create a group](#create-a-group)
      - [Add a group member](#add-a-group-member)
      - [Remove a group member](#remove-a-group-member)
      - [Get a group](#get-a-group)
      - [Get a group member](#get-a-group-member)
      - [Get groups a member belongs to](#get-groups-a-member-belongs-to)
## Overview

This module allows a user to:
- Create their vault. The vault stores the cipher private key used to cipher the user documents. The way the user vault is ciphered depends on the login method used by the user.
- Create on owned confidential document that only the user has access to
- Update the metadata of an owned confidential document
- Share a confidential document with another user

## Interface

### Dispachable functions
- `set_vault` Creates/Updates the calling user's vault and sets their public cipher key
- `set_owned_document` Creates a new owned document or updates an existing owned document's metadata
- `remove_owned_document` Removes an owned document
- `share_document` Creates a shared document
- `update_shared_document_metadata` Updates share document metadata
- `remove_shared_document` Removes a shared document
- `create_group` Creates a group that enables the sharing of documents with multiple users
- `add_group_member` Adds a member to a group only the owner and admins can add members
- `remove_group_member` Removes a member from a group the owner can remove any member, an admin only users added by them selfs

### Getters
- `vaults`
- `public_keys`
- `owned_docs`
- `owned_docs_by_owner`
- `shared_docs`
- `shared_docs_by_to`
- `shared_docs_by_from`
- `groups`
- `group_members`
- `member_groups`

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
```

### Polkadot-js api (javascript library)

#### Create a vault
```js
const response = await api.tx.confidentialDocs.setVault(userId, publicKey, cid).signAndSend(alice);
```

#### Get a vault
```js
const vault = await api.query.confidentialDocs.vaults(userId);
  console.log(vault.toHuman());
```
```bash
# Output should look like this:
{
  cid: 'QmeHEb5TF4zkP2H6Mg5TcrvDs5egPCJgWFBB7YZaLmK7jr',
  owner: '5FSuxe2q7qCYKie8yqmM56U4ovD1YtBb3DoPzGKjwZ98vxua'
}
```

#### Get a public key
```js
const publicKey = await api.query.confidentialDocs.publicKeys(address);
  console.log(markets.toHuman());
```
```bash
# Output should look like this:
'0xabe44a53e2c1a5c7fa2f920338136d0ddc3aba23eacaf708e3871bc856a34b75'
```

#### Create an owned confidential document
```js
const response = await api.tx.confidentialDocs.setOwnedDocument({
     "cid": "QmeHEb5TF4zkP2H6Mg5TcrvDs5egPCJgWFBB7YZaLmK7jr",
     "name": "name",
     "description": "desc",
     "owner": "5FSuxe2q7qCYKie8yqmM56U4ovD1YtBb3DoPzGKjwZ98vxua"
    }).signAndSend(alice);
```

#### Get an owned confidential document by CID
```js
const ownedDoc = await api.query.confidentialDocs.ownedDocs(cid);
  console.log(ownedDoc.toHuman());
```
```bash
# Output should look like this:
{
  "cid": "QmeHEb5TF4zkP2H6Mg5TcrvDs5egPCJgWFBB7YZaLmK7jr",
  "name": "name",
  "description": "desc",
  "owner": "5FSuxe2q7qCYKie8yqmM56U4ovD1YtBb3DoPzGKjwZ98vxua"
}
```

#### Remove an owned confidential document
```js
const response = await api.tx.confidentialDocs.removeOwnedDocument("QmeHEb5TF4zkP2H6Mg5TcrvDs5egPCJgWFBB7YZaLmK7jr").signAndSend(alice);
```

#### Share a confidential document
```js
const response = await api.tx.confidentialDocs.shareDocument({
     "cid": "QmeHEb5TF4zkP2H6Mg5TcrvDs5egPCJgWFBB7YZaLmK7jr",
     "name": "name",
     "description": "desc",
     "to": "5FSuxe2q7qCYKie8yqmM56U4ovD1YtBb3DoPzGKjwZ98vxua",
     "from": "5FWtfhKTuGKm9yWqzApwTfiUL4UPWukJzEcCTGYDiYHsdKaG"
    }).signAndSend(alice);
```

#### Get a shared confidential document by CID
```js
const sharedDoc = await api.query.confidentialDocs.sharedDocs(cid);
  console.log(sharedDoc.toHuman());
```
```bash
# Output should look like this:
{
     "cid": "QmeHEb5TF4zkP2H6Mg5TcrvDs5egPCJgWFBB7YZaLmK7jr",
     "name": "name",
     "description": "desc",
     "to": "5FSuxe2q7qCYKie8yqmM56U4ovD1YtBb3DoPzGKjwZ98vxua",
     "from": "5FWtfhKTuGKm9yWqzApwTfiUL4UPWukJzEcCTGYDiYHsdKaG"
}
```

#### Update a shared confidential document's metadata
```js
const response = await api.tx.confidentialDocs.updateSharedDocumentMetadata({
     "cid": "QmeHEb5TF4zkP2H6Mg5TcrvDs5egPCJgWFBB7YZaLmK7jr",
     "name": "name",
     "description": "desc"
    }).signAndSend(alice);
```

#### Remove a shared confidential document
```js
const response = await api.tx.confidentialDocs.removeSharedDocument("QmeHEb5TF4zkP2H6Mg5TcrvDs5egPCJgWFBB7YZaLmK7jr").signAndSend(alice);
```

#### Create a group
```js
const response = await api.tx.confidentialDocs.createGroup(group, name, public_key, cid).signAndSend(alice);
```

#### Add a group member
```js
const response = await api.tx.confidentialDocs.addGroupMember({
     "cid": "QmeHEb5TF4zkP2H6Mg5TcrvDs5egPCJgWFBB7YZaLmK7jr",
     "group": "5FSuxe2q7qCYKie8yqmM56U4ovD1YtBb3DoPzGKjwZ98vxua",
     "member": "5FWtfhKTuGKm9yWqzApwTfiUL4UPWukJzEcCTGYDiYHsdKaG",
     "authorizer": "5FWtfhKTuGKm9yWqzApwTfiUL4UPWukJzEcCTGYDiYHsdKaG",
     "role": "Member"
}).signAndSend(alice);
```

#### Remove a group member
```js
const response = await api.tx.confidentialDocs.removeGroupMember(group, member).signAndSend(alice);
```

#### Get a group
```js
const group = await api.query.confidentialDocs.groups(group);
  console.log(group.toHuman());
```
```bash
# Output should look like this:
{
     "name": "name",
     "group": "5FSuxe2q7qCYKie8yqmM56U4ovD1YtBb3DoPzGKjwZ98vxua",
     "creator": "5FWtfhKTuGKm9yWqzApwTfiUL4UPWukJzEcCTGYDiYHsdKaG"
}
```

#### Get a group member
```js
const groupMember = await api.query.confidentialDocs.groupMembers(member);
  console.log(groupMember.toHuman());
```
```bash
# Output should look like this:
{
     "cid": "QmeHEb5TF4zkP2H6Mg5TcrvDs5egPCJgWFBB7YZaLmK7jr",
     "group": "5FSuxe2q7qCYKie8yqmM56U4ovD1YtBb3DoPzGKjwZ98vxua",
     "member": "5FWtfhKTuGKm9yWqzApwTfiUL4UPWukJzEcCTGYDiYHsdKaG",
     "authorizer": "5FWtfhKTuGKm9yWqzApwTfiUL4UPWukJzEcCTGYDiYHsdKaG",
     "role": "Member"
}
```

#### Get groups a member belongs to
```js
const memberGroups = await api.query.confidentialDocs.memberGroups(member);
  console.log(memberGroups.toHuman());
```
```bash
# Output should look like this:
[
  "5FSuxe2q7qCYKie8yqmM56U4ovD1YtBb3DoPzGKjwZ98vxua",
  "5FWtfhKTuGKm9yWqzApwTfiUL4UPWukJzEcCTGYDiYHsdKaG"
]
```
