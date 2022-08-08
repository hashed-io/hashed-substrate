# Role-Based Access Control (RBAC)
Restrict access to custom pallets by coupling this pallet. Create roles and assign permissions.

- [Role-Based Access Control (RBAC)](#role-based-access-control-rbac)
  - [Overview](#overview)
  - [Terminology](#terminology)
  - [Interface](#interface)
    - [Helper functions](#helper-functions)
    - [Getters](#getters)

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
- **Pallet index**: a unique number that serves as an identifier, as it is assigned automatically to a pallet when its instantiated in the runtime.

## Interface

### Helper functions
This module is intended to use with conjunction of a pallet which loosely couples it, due to that, the pallet doesn't expose any extrinsic. However, the implementation of `RoleBasedAccessControl` has numerous helper functions that allow a flexible roles management.

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
- `set_multiple_permissions_to_role` assigns multiple, previously created permission to a role.

### Getters
- 