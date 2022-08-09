# Role-Based Access Control (RBAC)
Restrict access to custom pallets by coupling this pallet. Create roles and assign permissions.

- [Role-Based Access Control (RBAC)](#role-based-access-control-rbac)
  - [Overview](#overview)
  - [Terminology](#terminology)
  - [Interface](#interface)
    - [Helper functions](#helper-functions)
    - [Getters](#getters)
    - [Constants](#constants)
  - [Usage](#usage)
    - [Loosely coupling RBAC with another pallet](#loosely-coupling-rbac-with-another-pallet)
    - [Querying with Polkadot-js CLI](#querying-with-polkadot-js-cli)
      - [Get pallet scopes](#get-pallet-scopes)
      - [Get role by its id](#get-role-by-its-id)
      - [Get all role ids linked to a pallet](#get-all-role-ids-linked-to-a-pallet)
      - [Get a permission by pallet and id](#get-a-permission-by-pallet-and-id)
      - [Get permissions linked to a role within a pallet](#get-permissions-linked-to-a-role-within-a-pallet)
      - [Get which roles the user has in a pallet scope](#get-which-roles-the-user-has-in-a-pallet-scope)
      - [Get which users have the role in a pallet scope](#get-which-users-have-the-role-in-a-pallet-scope)
    - [Querying with Polkadot-js API (js library)](#querying-with-polkadot-js-api-js-library)

## Overview
This module allows to
- Define roles grouping them by the runtime pallet index.
- Assign permissions to roles.
- Create scopes, each of them will have an independent list of users.
- Assign roles to users within defined scopes.
- Ask if a user has certain permission, the pallet will search which roles the user has and will determine if its authorized.
- Remove roles from users, scopes and the entire storage assigned to an external pallet.


## Terminology
- **Scope**: A group of users with one or more roles, scopes are delimited and categorized by the pallet that created it.
- **Role**: A group of permissions, the RBAC pallet has a global list of roles to avoid data redundancy, however, only the selected roles will be assigned (or created if they don't exist) to the pallet.  
- **Permission**: The bottom level filter, permissions are stored and categorized by pallet, and it is highly recommended each restricted extrinsic have its own permission.
- **Pallet index**: a unique number that serves as an identifier, as it is assigned automatically to a pallet when its instantiated in the runtime. The term is interchangeable with pallet id. 

## Interface

### Helper functions
This module is intended to be used in conjunction with a pallet which loosely couples it, due to that, the pallet doesn't expose any extrinsic. However, the implementation of the `RoleBasedAccessControl` trait has numerous helper functions that allow a flexible roles management.

- `create_scope` inserts a scope within a external pallet context using its index.
- `remove_scope` deletes all role lists linked to that scope.   
- `remove_pallet_storage` deletes all role lists and permissions associated with the pallet.
- `create_and_set_roles` is the recommended first step for setting up the role access for the pallet, as it takes the pallet index and a list of roles to be created (and assigned) in encoded string format.
- `create_role` inserts a role in the global role list and return a generated `role_id`, if its already in the list, it won't perform the id generation and will return the previously stored one instead. It is important to mention that this function won't assign the role to any pallet.
- `set_role_to_pallet` assigns a previously created role to a pallet.
- `set_multiple_pallet_roles` assigns multiple, previously created roles to a pallet.
- `assign_role_to_user` assigns a role to a user in a scope context. The role needs to be previously created and assigned to that pallet. After this function is executed, the specified user will have additional capabilities according to the role. 
- `remove_role_from_user` removes a specified role from a user in a scope context. After this function is executed, the user will no longer be able to enforce the removed role and its permissions.
- `create_and_set_permissions` a good second step for enabling role access to the coupled pallet, as it creates and assigns a list of permissions to a role in a pallet context.
- `create_permission` inserts a permission in a pallet context, after this function is executed, the permission is not yet assigned to any role. 
- `set_permission_to_role` assigns a previously created permission to a role in a pallet context.
- `set_multiple_permissions_to_role` assigns multiple, previously created permissions to a role in a pallet context.
-  `is_authorized` is the suggested authorization mechanism, as it takes the pallet index, scope and the requested permission to be enforced. This function will search the users permissions and will validate if there's a role that has the permission enabled.
 - `has_role` a secondary authorization mechanism that takes the pallet index, scope, and a set of roles that the user tentatively has. This method is specially useful when its unclear which roles the user has and any of the specified roles will suffice the authorization.
 - `scope_exists` a validation function used internally by other methods, ensure the requested scope is registered in the specified pallet.
 - `permission_exists` is a validation function used internally, as it provides, as it confirms if the permission is stored in the specified pallet.
 - `is_role_linked_to_pallet` validates if a role is registered in the pallet. This method doesn't validates if the role has been previously created and assumes it is.
 - `is_permission_linked_to_role` ensures the specified permission is linked to the role in a pallet context. This method assumes both the role and permission exists.
 - `get_role_users_len` returns the number of users that have the specified role, useful when implementing restrictions on the number of users that can have that role.

### Getters
- `scopes` 
- `roles`
- `pallet_roles`
- `permissions` (storage double map)
- `permissions_by_role` (storage double map)
- `roles_by_user` (storage N map with 3 keys)
- `users_by_scope` (storage N map with 3 keys)


### Constants
- `MaxScopesPerPallet: Get<u32>`
- `MaxRolesPerPallet: Get<u32>`
- `RoleMaxLen: Get<u32>`
- `PermissionMaxLen: Get<u32>`
- `MaxPermissionsPerRole: Get<u32>`
- `MaxRolesPerUser: Get<u32>`
- `MaxUsersPerRole: Get<u32>`

## Usage

### Loosely coupling RBAC with another pallet
Once the RBAC pallet is imported and configured in the runtime, the first step is to import the `RoleBasedAccessControl` trait from the rbac types into the custom pallet, and declare a type within the pallet configuration:
```rust
use pallet_rbac::types::RoleBasedAccessControl;
	
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		// ...
		type Rbac : RoleBasedAccessControl<Self::AccountId>;
	}
```

Then the RBAC pallet can safely be imported as a parameter within another pallet, for example, `gated_marketplaces`:

```rust
impl pallet_gated_marketplace::Config for Runtime {
	type Event = Event;
  // ...
	type Rbac = RBAC;
}
```

Now all the previously mentioned functions are accessible within the custom pallet:
```rust
let create_scope_result : DispatchResult = T::Rbac::create_scope(pallet_id,marketplace_id);
```

### Querying with Polkadot-js CLI
As previously stated, this pallet doesn't expose any extrinsics, but rather expose a collection of helper functions that are accessible by any custom pallet that couples it. Therefore, the following section assumes theres a basic RBAC configuration stored on chain.

#### Get pallet scopes
```bash
# pallet_id
polkadot-js-api query.rbac.scopes 20
```
```bash
# Expected output
{
  "scopes": [
    "0x112a94197eb935a48b13ac5e6d37d316a143dd3dcf725c9d9d27d64dbba62890"
  ]
}
```

#### Get role by its id
```bash
# role_id
polkadot-js-api query.rbac.roles 0x08aef7203969e2467b33b14965dfab62e11b085610c798b3cac150b1d7ea033b
```
```bash
# Expected output:
{
  "roles": "Owner"
}
```

#### Get all role ids linked to a pallet

```bash
# pallet_id
polkadot-js-api query.rbac.palletRoles 20
```
```bash
# Expected output
{
  "palletRoles": [
    "0x08aef7203969e2467b33b14965dfab62e11b085610c798b3cac150b1d7ea033b",
    "0xc1237f9841c265fb722178da01a1e088c25fb892d6b7cd9634a20ac84bb3ee01",
    "0xae9e025522f868c39b41b8a5ba513335a2a229690bd44c71c998d5a9ad38162b"
  ]
}
```

#### Get a permission by pallet and id
```bash
# pallet_id, permission_id
polkadot-js-api query.rbac.permissions 20 0xdd2f4fc1f525a38ab2f18b2ef4ff4559ddc344d04aa2ceaec1f5d0c6b4f67674
```
```bash
# Expected output
{
  "permissions": "Enroll"
}
```

#### Get permissions linked to a role within a pallet
```bash
# pallet_id, permission_id
polkadot-js-api query.rbac.permissionsByRole 20 0x08aef7203969e2467b33b14965dfab62e11b085610c798b3cac150b1d7ea033b
```
```bash
# Expected output
{
  "permissionsByRole": [
    "0xdd2f4fc1f525a38ab2f18b2ef4ff4559ddc344d04aa2ceaec1f5d0c6b4f67674",
    "0x2c40feed7853568ca1cb5f852636359f8cc8dc82108191397cb7b8ad90a1d0a1",
    "0x78dcd6644c3f21fd1872659dcb32c58af797c5c06963fb2ea0937b8d24479815",
    "0xbe1f77a2f9266a2dbaa4858ec7aa3933da37346e96a7968c99870d15552d51a5",
    "0x599314a6cceabfd08491d4847fe78ad0e932340ff1877704376890aa6ddb045c"
  ]
}
```

#### Get which roles the user has in a pallet scope
```bash
# account_id, pallet_id, scope_id
polkadot-js-api query.rbac.rolesByUser 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 20 0x112a94197eb935a48b13ac5e6d37d316a143dd3dcf725c9d9d27d64dbba62890
```
```bash
# Expected output
{
  "rolesByUser": [
    "0x08aef7203969e2467b33b14965dfab62e11b085610c798b3cac150b1d7ea033b"
  ]
}
```

#### Get which users have the role in a pallet scope
```bash
# pallet_id, scope_id, role_id
polkadot-js-api query.rbac.usersByScope 20 0x112a94197eb935a48b13ac5e6d37d316a143dd3dcf725c9d9d27d64dbba62890 0x08aef7203969e2467b33b14965dfab62e11b085610c798b3cac150b1d7ea033b
```
```bash
# Expected output
{
  "usersByScope": [
    "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
  ]
}
```

### Querying with Polkadot-js API (js library)