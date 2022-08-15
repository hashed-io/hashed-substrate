use super::*;
use frame_support::{pallet_prelude::*};
//use frame_system::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use frame_support::sp_std::borrow::ToOwned;
use sp_runtime::sp_std::vec::Vec;

use crate::types::*;

impl<T: Config> RoleBasedAccessControl<T::AccountId> for Pallet<T>{
    /*---- Basic Insertion of individual storage maps ---*/
    /// Scope creation
    /// 
    /// Creates a scope within a external pallet using the pallet index.
    /// ### Parameters:
    /// - `pallet_id`: The unique pallet identifier.
    /// - `scope_id`: The newly generated scope identifier.
    fn create_scope(pallet_id: u64, scope_id: ScopeId)-> DispatchResult{
        let pallet_id: u64 = pallet_id.try_into().unwrap();
        <Scopes<T>>::try_mutate(pallet_id, |scopes|{
            ensure!(!scopes.contains(&scope_id), Error::<T>::ScopeAlreadyExists);
            scopes.try_push(scope_id).map_err(|_| Error::<T>::ExceedMaxScopesPerPallet)?;
            Ok(())
        })
    }

    /// Scope removal
    /// 
    /// Removes a scope within a external pallet using the pallet index.
    /// Executing this function will delete all registered role users.
    /// ### Parameters:
    /// - `pallet_id`: The unique pallet identifier.
    /// - `scope_id`: The scope identifier to remove.
    fn remove_scope(pallet_id: u64, scope_id: ScopeId) -> DispatchResult{
        // remove on scopes
        <Scopes<T>>::try_mutate_exists::<_,(),DispatchError,_>(pallet_id, |scopes_option|{
            let scopes = scopes_option.as_mut().ok_or(Error::<T>::ScopeNotFound)?;
            let s_pos = scopes.iter().position(|&s| s==scope_id).ok_or(Error::<T>::ScopeNotFound)?;
            scopes.remove(s_pos);
            if scopes.is_empty(){
                scopes_option.clone_from(&None);
            }
            Ok(())
        })?;
        let mut scope_users = <UsersByScope<T>>::iter_prefix((pallet_id, scope_id)).map(
            |(_role, users)|users).flatten().collect::<Vec<_>>();
        // exclude duplicate users
        scope_users.sort();     scope_users.dedup();
        // remove on RolesByUser
        scope_users.iter().for_each(|user|{
            <RolesByUser<T>>::remove((user, pallet_id, scope_id));
        });
        // remove on users by scope
        <UsersByScope<T>>::remove_prefix((pallet_id, scope_id), None);
        
        Ok(())
    }

     /// External pallet storage removal
    /// 
    /// Removes all storage associated to a external pallet.
    /// 
    /// Executing this function will delete all role lists and permissions linked
    /// to that pallet.
    /// ### Parameters:
    /// - `pallet_id`: The unique pallet identifier.
    fn remove_pallet_storage(pallet_id: u64) -> DispatchResult{
        //remove all scopes
        let scopes = <Scopes<T>>::get(pallet_id);
        for scope in scopes{
            Self::remove_scope(pallet_id, scope)?;
        }
        // remove all roles
        let pallet_roles = <PalletRoles<T>>::take(pallet_id);
        //check if there's other pallet that uses the roles, if not, remove them
        let all_pallet_roles = <PalletRoles<T>>::iter().map(| p| p.1.to_vec())
            .collect::<Vec<Vec<[u8; 32]>>>();
        let flatten_all_pallet_roles = all_pallet_roles.iter().flatten().collect::<Vec<&[u8;32]>>();
        let filtered_roles = pallet_roles.iter().filter(|pallet_role| !flatten_all_pallet_roles.contains(pallet_role));
        filtered_roles.for_each(|role|{
            <Roles<T>>::remove(role);
        });
        //remove all permissions
        <PermissionsByRole<T>>::remove_prefix(pallet_id, None);
        <Permissions<T>>::remove_prefix(pallet_id, None);
        Ok(())
    }

    /// Role creation and coupling with pallet.
    /// 
    /// Creates the specified roles if needed and adds them to the pallet.
    /// Recommended first step to enable RBAC on a external pallet.
    /// ### Parameters:
    /// - `pallet_id`: The unique pallet identifier.
    /// - `roles`: A list of roles to create, encoded in bytes.
    fn create_and_set_roles(pallet_id: u64, roles: Vec<Vec<u8>>) -> 
        Result<BoundedVec<RoleId, T::MaxRolesPerPallet>, DispatchError>{
        let mut role_ids= Vec::<[u8;32]>::new();
        for role in roles{
            role_ids.push( Self::create_role(role.to_owned())? );
        }
        Self::set_multiple_pallet_roles(pallet_id, role_ids.clone())?;
        let bounded_ids = Self::bound(role_ids, Error::<T>::ExceedMaxRolesPerPallet)?;
        Ok(bounded_ids)
    }

    /// Role creation.
    /// 
    /// Creates a role and returns its identifier, if its already created,
    /// the function will return the preexisting one.
    /// ### Parameters:
    /// - `role`: A role to create, encoded in bytes.
    fn create_role(role: Vec<u8>)-> Result<RoleId, DispatchError>{
        let role_id = role.using_encoded(blake2_256);
        // no "get_or_insert" method found
        let b_role = Self::bound::<_,T::RoleMaxLen>(role, Error::<T>::ExceedRoleMaxLen)?;
        ensure!(role_id == b_role.using_encoded(blake2_256), Error::<T>::NoneValue);
        if !<Roles<T>>::contains_key(role_id) {<Roles<T>>::insert(role_id, b_role)};
        Ok(role_id)
    }

    /// Role coupling with pallet.
    /// 
    /// Assigns a previously created role to a pallet.
    /// 
    /// ### Parameters:
    /// - `pallet_id`: The unique pallet identifier.
    /// - `role_id`: The unique role identifier. 
    fn set_role_to_pallet(pallet_id: u64, role_id: RoleId )-> DispatchResult{
        ensure!(<Roles<T>>::contains_key(role_id), Error::<T>::RoleNotFound);

        <PalletRoles<T>>::try_mutate(pallet_id, |roles|{
            ensure!(!roles.contains(&role_id), Error::<T>::RoleAlreadyLinkedToPallet );
            roles.try_push(role_id).map_err(|_| Error::<T>::ExceedMaxRolesPerPallet)
        })?;
        Ok(())
    }

    /// Multiple role coupling with pallet.
    /// 
    /// Assigns multiple, previously created roles to a pallet.
    /// ### Parameters:
    /// - `pallet_id`: The unique pallet identifier.
    /// - `roles`: A list of unique role identifiers. 
    fn set_multiple_pallet_roles(pallet_id: u64, roles: Vec<RoleId>)->DispatchResult{
        // checks for duplicates:
        ensure!(Self::has_unique_elements(roles.clone()), Error::<T>::DuplicateRole);
        let pallet_roles = <PalletRoles<T>>::get(&pallet_id);
        for id in roles.clone(){
            ensure!(!pallet_roles.contains(&id), Error::<T>::RoleAlreadyLinkedToPallet );
        }
        <PalletRoles<T>>::try_mutate(pallet_id, |pallet_roles|{
            pallet_roles.try_extend(roles.into_iter())
        }).map_err(|_| Error::<T>::ExceedMaxRolesPerPallet)?;

        Ok(())
    }

    /// Role assignation to a user
    /// 
    /// Assigns a role to a user in a scope context.
    /// ### Parameters:
    /// - `user`: The account which the role will be granted.
    /// - `pallet_id`: The unique pallet identifier.
    /// - `scope_id`: The scope in which the role will be granted.
    /// - `role_id`: The role identifier to grant for the user.
    fn assign_role_to_user(user: T::AccountId, pallet_id: u64, scope_id: &ScopeId, role_id: RoleId) -> DispatchResult{
        Self::scope_exists(pallet_id, scope_id)?;
        Self::is_role_linked_to_pallet(pallet_id, &role_id)?;

        <RolesByUser<T>>::try_mutate((&user, pallet_id, scope_id), | roles |{
            ensure!(!roles.contains(&role_id), Error::<T>::DuplicateRole);
            roles.try_push(role_id).map_err(|_| Error::<T>::ExceedMaxRolesPerUser)
        })?;

        <UsersByScope<T>>::try_mutate((pallet_id, scope_id, role_id), | users|{
            ensure!(!users.contains(&user), Error::<T>::UserAlreadyHasRole);
            users.try_push(user).map_err(|_| Error::<T>::ExceedMaxUsersPerRole)
        })?;
        Ok(())
    }

    /// Role removal from the user.
    /// 
    /// Removes the specified role from a user in a scope context. the user will no longer
    /// be able to enforce the removed role and its permissions.
    /// ### Parameters:
    /// - `user`: The account which the role will be removed.
    /// - `pallet_id`: The unique pallet identifier.
    /// - `scope_id`: The scope in which the role will be removed.
    /// - `role_id`: The role identifier to remove from the user.
    fn remove_role_from_user(user: T::AccountId, pallet_id: u64, scope_id: &ScopeId, role_id: RoleId) -> DispatchResult{
        <RolesByUser<T>>::try_mutate_exists::<_,(),DispatchError,_>((user.clone(), pallet_id, scope_id), |user_roles_option|{
            let user_roles = user_roles_option.as_mut().ok_or(Error::<T>::UserHasNoRoles)?;
            let r_pos = user_roles.iter().position(|&r| r==role_id).ok_or(Error::<T>::RoleNotFound)?;
            user_roles.remove(r_pos);
            if user_roles.is_empty(){
                user_roles_option.clone_from(&None)
            }
            Ok(())
        })?;
        <UsersByScope<T>>::try_mutate_exists::<_,(),DispatchError,_>((pallet_id, scope_id, role_id), |auth_users_option|{
            let auth_users = auth_users_option.as_mut().ok_or(Error::<T>::RoleHasNoUsers)?;
            let u_pos = auth_users.iter().position(|u| *u==user).ok_or(Error::<T>::UserNotFound)?;
            auth_users.remove(u_pos);
            if auth_users.is_empty(){
                auth_users_option.clone_from(&None);
            }
            Ok(())
        })?;
        Ok(())
    }

    /// Permission creation and coupling with a role.
    /// 
    /// Creates the specified permissions if needed and assigns them to a role.
    /// ### Parameters:
    /// - `pallet_id`: The unique pallet identifier.
    /// - `role_id`: The role identifier to which the permissions will
    /// be linked to.
    /// - `permissions`: A list of permissions to create and link, 
    /// encoded in bytes.
    fn create_and_set_permissions(pallet_id: u64, role_id: RoleId, permissions: Vec<Vec<u8>>)->
        Result<BoundedVec<PermissionId, Self::MaxPermissionsPerRole>, DispatchError> {
        ensure!(Self::has_unique_elements(permissions.clone()), Error::<T>::DuplicatePermission);
        Self::is_role_linked_to_pallet(pallet_id, &role_id )?;
        let mut permission_ids = Vec::<[u8;32]>::new();
        for permission in permissions{
            permission_ids.push( Self::create_permission(pallet_id, permission.to_owned())? );
        }
        Self::set_multiple_permissions_to_role(pallet_id, role_id, permission_ids.clone())?;
        let b_permissions =  Self::bound(permission_ids, Error::<T>::ExceedMaxPermissionsPerRole)?;
        Ok(b_permissions)
    }

    /// Permission creation
    /// 
    /// Creates the specified permission in the specified pallet..
    /// ### Parameters:
    /// - `pallet_id`: The unique pallet identifier.
    /// - `permission`: The permission to insert, encoded in bytes.
    fn create_permission(pallet_id: u64, permission: Vec<u8>) -> Result<PermissionId, DispatchError>{
        let permission_id = permission.using_encoded(blake2_256);

        let b_permission = Self::bound::
            <_,T::PermissionMaxLen>(permission, Error::<T>::ExceedPermissionMaxLen)?;

        if !<Permissions<T>>::contains_key(pallet_id, permission_id){
            <Permissions<T>>::insert(pallet_id, permission_id, b_permission);
        }
        Ok(permission_id)
    }

    /// Permission linking to role.
    /// 
    /// Assigns a previously created permission to a role.
    /// ### Parameters:
    /// - `pallet_id`: The unique pallet identifier.
    /// - `role_id`: The role identifier to which the permission will be added.
    /// - `permission_id`: The permission to assign to the role.
    fn set_permission_to_role( pallet_id: u64, role_id: RoleId, permission_id: PermissionId ) -> DispatchResult{
        ensure!(<Permissions<T>>::contains_key(pallet_id, permission_id), Error::<T>::PermissionNotFound);
        Self::is_role_linked_to_pallet(pallet_id, &role_id)?;

        <PermissionsByRole<T>>::try_mutate(pallet_id, role_id, | role_permissions|{
            ensure!(!role_permissions.contains(&permission_id), Error::<T>::DuplicatePermission);
            role_permissions.try_push(permission_id).map_err(|_| Error::<T>::ExceedMaxPermissionsPerRole)
        })?;
        Ok(())
    }

    /// Multiple permissions assignation to a role
    /// 
    /// Assigns multiple, previously created permissions 
    /// to a role in a pallet context.
    /// ### Parameters:
    /// - `pallet_id`: The unique pallet identifier.
    /// - `role_id`: The role identifier to which the permissions will be added.
    /// - `permissions`: A list of permission identifiers to assign to the role.
    fn set_multiple_permissions_to_role(  pallet_id: u64, role_id: RoleId, permissions: Vec<PermissionId> )-> DispatchResult{
        // checks for duplicates:
        ensure!(Self::has_unique_elements(permissions.clone()), Error::<T>::DuplicatePermission);
        Self::is_role_linked_to_pallet(pallet_id, &role_id )?;
        let role_permissions = <PermissionsByRole<T>>::get(&pallet_id, role_id);
        for id in permissions.clone(){
            ensure!(!role_permissions.contains(&id), Error::<T>::PermissionAlreadyLinkedToRole );
        }
        <PermissionsByRole<T>>::try_mutate(pallet_id, role_id,  |role_permissions|{
            role_permissions.try_extend(permissions.into_iter())
        }).map_err(|_| Error::<T>::ExceedMaxPermissionsPerRole)?;
        Ok(())
    }

    /*---- Helper functions ----*/

    /// Authorization function
    /// 
    /// Checks if the user has a role that includes the specified permission.
    /// ### Parameters:
    /// - `user`: The account to validate.
    /// - `pallet_id`: The unique pallet identifier.
    /// - `scope_id`: The scope context in which the permission will be validated.
    /// - `permission_id`: The permission the user must have.
    fn is_authorized(user: T::AccountId, pallet_id: u64, scope_id: &ScopeId, permission_id: &PermissionId) -> DispatchResult{
        Self::scope_exists(pallet_id, scope_id)?;
        Self::permission_exists(pallet_id, permission_id)?;

        // get roles the user has in this scope
        let user_roles = <RolesByUser<T>>::get((user, pallet_id, scope_id));
        // determine if one of the roles has the requested permission
        let has_permission = user_roles.iter().any(|r_id| <PermissionsByRole<T>>::get(pallet_id, r_id).contains(permission_id));
        ensure!(has_permission, Error::<T>::NotAuthorized);
        Ok(())
    }

    /// User role validation function
    /// 
    /// Checks if the user has at least one of the specified roles.
    /// ### Parameters:
    /// - `user`: The account to validate.
    /// - `pallet_id`: The unique pallet identifier.
    /// - `scope_id`: The scope context in which the permission will be validated.
    /// - `role_ids`: A list of roles to validate.
    fn has_role(user: T::AccountId, pallet_id: u64, scope_id: &ScopeId, role_ids: Vec<RoleId>)->DispatchResult {
        Self::scope_exists(pallet_id, scope_id)?;

        let user_roles = <RolesByUser<T>>::get((user, pallet_id, scope_id));
        ensure!(
            user_roles.iter().any(|r| role_ids.contains(r) ),
            Error::<T>::NotAuthorized
        );
        Ok(())
    }
    
    /// Scope validation
    /// 
    /// Checks if the scope exists in that pallet.
    /// ### Parameters:
    /// - `pallet_id`: The unique pallet identifier.
    /// - `scope_id`: The scope to validate.
    fn scope_exists(pallet_id: u64, scope_id:&ScopeId) -> DispatchResult{
        ensure!(<Scopes<T>>::get(pallet_id).contains(&scope_id), Error::<T>::ScopeNotFound);
        Ok(())
    }

    /// Permission validation.
    /// 
    /// Checks if the permission exists in a pallet context. 
    /// ### Parameters:
    /// - `pallet_id`: The unique pallet identifier.
    /// - `permission_id`: The permission to validate.
    fn permission_exists(pallet_id: u64, permission_id: &PermissionId)->DispatchResult{
        ensure!(<Permissions<T>>::contains_key(pallet_id, permission_id), Error::<T>::PermissionNotFound);
        Ok(()) 
    }

    /// Role validation
    /// 
    /// Checks if the role is linked to the pallet.
    /// ### Parameters:
    /// - `pallet_id`: The unique pallet identifier.
    /// - `role_id`: The role to validate
    fn is_role_linked_to_pallet(pallet_id: u64, role_id: &RoleId)-> DispatchResult{
        // The role exists, now  check if the role is assigned to that pallet
        <PalletRoles<T>>::get(pallet_id).iter().find(|pallet_role| *pallet_role==role_id )
            .ok_or(Error::<T>::RoleNotLinkedToPallet)?;
        Ok(())
    }

    /// Permission linking validation
    /// 
    /// Checks if the permission is linked to the role in the pallet context.
    /// ### Parameters:
    /// - `pallet_id`: The unique pallet identifier.
    /// - `role_id`: The role which should have the permission.
    /// - `permission_id`: The permission which the role should have.
    fn is_permission_linked_to_role(pallet_id: u64, role_id: &RoleId, permission_id: &PermissionId)-> DispatchResult{
        let role_permissions = <PermissionsByRole<T>>::get(pallet_id, role_id);
        ensure!(role_permissions.contains(permission_id), Error::<T>::PermissionNotLinkedToRole );
        Ok(())
    }

    /// Role list length
    /// 
    /// Returns the number of user that have the specified role in a scope context.
    /// ### Parameters:
    /// - `pallet_id`: The unique pallet identifier.
    /// - `scope_id`: The scope in which the users will be retrieved.
    /// - `role_id`: The role in which the number of users will be counted.
    fn get_role_users_len(pallet_id: u64, scope_id:&ScopeId, role_id: &RoleId) -> usize{
        <UsersByScope<T>>::get((pallet_id, scope_id, role_id)).len()
    }

    type MaxRolesPerPallet = T::MaxRolesPerPallet;

    type MaxPermissionsPerRole = T::MaxPermissionsPerRole;

    type PermissionMaxLen = T::PermissionMaxLen;

    type RoleMaxLen =  T::RoleMaxLen;

}

impl<T: Config> Pallet<T>{
    fn bound<E,Len: Get<u32>>(vec: Vec<E>, err : Error<T> )->Result<BoundedVec<E, Len>, Error<T>>{
        BoundedVec::<E,Len>::try_from(vec).map_err(|_| err)
    }

    fn has_unique_elements<E: Ord + Clone>(vec: Vec<E>) -> bool{
        let mut filtered_vec = vec.clone();
        filtered_vec.sort();
        filtered_vec.dedup();
        vec.len() == filtered_vec.len()
    }
}