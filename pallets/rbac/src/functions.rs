use super::*;
use frame_support::{pallet_prelude::*};
use frame_system::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use frame_support::sp_std::borrow::ToOwned;
use sp_runtime::sp_std::vec::Vec;

use crate::types::*;
// TODO: make vec to manage pallet errors here
impl<T: Config> RoleBasedAccessControl<T::AccountId> for Pallet<T>{
    /*---- Basic Insertion of individual storage maps ---*/
    fn create_scope(pallet_id: u64, scope_id: [u8;32])-> DispatchResult{
        let pallet_id: u64 = pallet_id.try_into().unwrap();
        <Scopes<T>>::try_mutate(pallet_id, |scopes|{
            ensure!(!scopes.contains(&scope_id), Error::<T>::ScopeAlreadyExists);
            scopes.try_push(scope_id).map_err(|_| Error::<T>::ExceedMaxScopesPerPallet)?;
            Ok(())
        })
    }

    fn remove_scope(pallet_id: u64, scope_id: [u8;32]) -> DispatchResult{
        // WIP
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
        // remove on users by scope
        let mut scope_users = <UsersByScope<T>>::iter_prefix((pallet_id, scope_id)).map(
            |(_role, users)|users).flatten().collect::<Vec<_>>();
        // remove duplicate users
        scope_users.sort();     scope_users.dedup();
        // remove scope users
        
        Ok(())
    }
    /// Inserts roles and links them to the pallet
    fn create_and_set_roles(pallet_id: u64, roles: Vec<Vec<u8>>) -> 
        Result<BoundedVec<[u8;32], T::MaxRolesPerPallet>, DispatchError>{
        let mut role_ids= Vec::<[u8;32]>::new();
        for role in roles{
            role_ids.push( Self::create_role(role.to_owned())? );
        }
        Self::set_multiple_pallet_roles(pallet_id, role_ids.clone())?;
        let bounded_ids = Self::bound(role_ids, Error::<T>::ExceedMaxRolesPerPallet)?;
        Ok(bounded_ids)
    }

    fn create_role(role: Vec<u8>)-> Result<[u8;32], DispatchError>{
        let role_id = role.using_encoded(blake2_256);
        // no "get_or_insert" method found
        let b_role = Self::bound::<_,T::RoleMaxLen>(role, Error::<T>::ExceedMaxRolesPerUser)?;
        ensure!(role_id == b_role.using_encoded(blake2_256), Error::<T>::NoneValue);
        if !<Roles<T>>::contains_key(role_id) {<Roles<T>>::insert(role_id, b_role)};
        Ok(role_id)
    }

    fn set_role_to_pallet(pallet_id: u64, role_id: [u8;32] )-> DispatchResult{
        ensure!(<Roles<T>>::contains_key(role_id), Error::<T>::RoleNotFound);

        <PalletRoles<T>>::try_mutate(pallet_id, |roles|{
            ensure!(!roles.contains(&role_id), Error::<T>::DuplicateRole );
            roles.try_push(role_id).map_err(|_| Error::<T>::ExceedMaxRolesPerPallet)
        })?;
        Ok(())
    }

    fn set_multiple_pallet_roles(pallet_id: u64, roles: Vec<[u8;32]>)->DispatchResult{
        // checks for duplicates:
        let pallet_roles = <PalletRoles<T>>::get(&pallet_id);
        for id in roles.clone(){
            ensure!(!pallet_roles.contains(&id), Error::<T>::DuplicateRole );
        }
        <PalletRoles<T>>::try_mutate(pallet_id, |pallet_roles|{
            pallet_roles.try_extend(roles.into_iter())
        }).map_err(|_| Error::<T>::ExceedMaxRolesPerPallet)?;

        Ok(())
    }

    fn create_and_set_permissions(pallet_id: u64, role_id: [u8;32], permissions: Vec<Vec<u8>>)->
        Result<BoundedVec<[u8;32], Self::MaxPermissionsPerRole>, DispatchError> {
        // TODO: Test this functionality
        let mut permission_ids = Vec::<[u8;32]>::new();
        for permision in permissions{
            permission_ids.push( Self::create_permission(pallet_id, permision.to_owned())? );
        }
        Self::set_multiple_permisions_to_role(pallet_id, role_id, permission_ids.clone())?;
        let b_permissions =  Self::bound(permission_ids, Error::<T>::ExceedMaxPermissionsPerRole)?;
        Ok(b_permissions)
    }

    fn create_permission(pallet_id: u64, permission: Vec<u8>) -> Result<[u8;32], DispatchError>{
        let permission_id = permission.using_encoded(blake2_256);
        //let b_permission= BoundedVec::<u8, Self::PermissionMaxLen>::try_from(permission);
        let b_permission = Self::bound::
            <_,T::PermissionMaxLen>(permission, Error::<T>::ExceedPermissionMaxLen)?;
        // Testing: a boundedvec id should be equal to a vec id because they have the same data
        ensure!(permission_id == b_permission.using_encoded(blake2_256), Error::<T>::NoneValue);

        log::info!("Is permission_id equal: {}",permission_id == b_permission.using_encoded(blake2_256));
        if !<Permissions<T>>::contains_key(pallet_id, permission_id){
            <Permissions<T>>::insert(pallet_id, permission_id, b_permission);
        }
        Ok(permission_id)
    }

    fn set_permission_to_role( pallet_id: u64, role_id: [u8;32], permission_id: [u8;32] ) -> DispatchResult{
        ensure!(<Permissions<T>>::contains_key(pallet_id, permission_id), Error::<T>::PermissionNotFound);
        Self::is_role_linked_to_pallet(pallet_id, &role_id)?;

        <PermissionsByRole<T>>::try_mutate(pallet_id, role_id, | role_permissions|{
            ensure!(role_permissions.contains(&permission_id), Error::<T>::DuplicatePermission);
            role_permissions.try_push(permission_id).map_err(|_| Error::<T>::ExceedMaxPermissionsPerRole)
        })?;
        Ok(())
    }

    fn set_multiple_permisions_to_role(  pallet_id: u64, role_id: [u8;32], permissions: Vec<[u8;32]> )-> DispatchResult{
        // checks for duplicates:
        let role_permissions = <PermissionsByRole<T>>::get(&pallet_id, role_id);
        for id in permissions.clone(){
            ensure!(!role_permissions.contains(&id), Error::<T>::DuplicateRole );
        }
        <PermissionsByRole<T>>::try_mutate(pallet_id, role_id,  |role_permissions|{
            role_permissions.try_extend(permissions.into_iter())
        }).map_err(|_| Error::<T>::ExceedMaxPermissionsPerRole)?;

        Ok(())
    }

    fn assign_role_to_user(user: T::AccountId, pallet_id: u64, scope_id: &[u8;32], role_id: [u8;32]) -> DispatchResult{
        Self::scope_exists(pallet_id, scope_id)?;
        Self::is_role_linked_to_pallet(pallet_id, &role_id)?;

        <Users<T>>::try_mutate((&user, pallet_id, scope_id), | roles |{
            ensure!(!roles.contains(&role_id), Error::<T>::DuplicateRole);
            roles.try_push(role_id).map_err(|_| Error::<T>::ExceedMaxRolesPerUser)
        })?;

        <UsersByScope<T>>::try_mutate((pallet_id, scope_id, role_id), | users|{
            ensure!(!users.contains(&user), Error::<T>::UserAlreadyHasRole);
            users.try_push(user).map_err(|_| Error::<T>::ExceedMaxUsersPerRole)
        })?;
        Ok(())
    }

    fn remove_role_from_user(user: T::AccountId, pallet_id: u64, scope_id: &[u8;32], role_id: [u8;32]) -> DispatchResult{
        <Users<T>>::try_mutate_exists::<_,(),DispatchError,_>((user.clone(), pallet_id, scope_id), |user_roles_option|{
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

    /*---- Helper functions ----*/

    /// Authorize by role, not permissions
    fn is_user_authorized(user: T::AccountId, pallet_id: u64, scope_id: &[u8;32], role_id : &[u8;32] ) -> DispatchResult{
        // get id, whether is given directly or by its string in boundedvec format
        //let role_id = Self::get_role_id(role)?;
        Self::scope_exists(pallet_id, scope_id)?;
        Self::is_role_linked_to_pallet(pallet_id, role_id)?;
        // Perform confirmation on both maps
        // TODO: test if a role that doesnt exists cause any errors
        let users = <UsersByScope<T>>::get( (pallet_id, scope_id, role_id) );
        ensure!(users.contains(&user), Error::<T>::NotAuthorized);
        let roles = <Users<T>>::get((user, pallet_id, scope_id));
        // Not likely to happen but just in case:
        ensure!(roles.contains(&role_id), Error::<T>::NotAuthorized );
        Ok(())
    }

    fn has_role(user: T::AccountId, pallet_id: u64, scope_id: &[u8;32], role_ids: Vec<[u8;32]>)->DispatchResult {
        Self::scope_exists(pallet_id, scope_id)?;

        let user_roles = <Users<T>>::get((user, pallet_id, scope_id));
        ensure!(
            user_roles.iter().any(|r| role_ids.contains(r) ),
            Error::<T>::NotAuthorized
        );
        Ok(())
    }
    /// Also checks if pallet is stored. Need this function to expose the check to other pallets
    fn scope_exists(pallet_id: u64, scope_id:&[u8;32]) -> DispatchResult{
        ensure!(<Scopes<T>>::get(pallet_id).contains(&scope_id), Error::<T>::ScopeNotFound);
        Ok(())
    }

    fn is_role_linked_to_pallet(pallet_id: u64, role_id: &[u8;32])-> DispatchResult{
        // The role exists, now  check if the role is assigned to that pallet
        <PalletRoles<T>>::get(pallet_id).iter().find(|pallet_role| *pallet_role==role_id )
            .ok_or(Error::<T>::RoleNotLinkedToPallet)?;
        Ok(())
    }

    fn get_role_id(id_or_role: IdOrString<Self::RoleMaxLen>)->Result<[u8;32], DispatchError>{
        let role_id = match id_or_role{
            IdOrString::Id(id)=>id,
            IdOrString::String(role_str)=> role_str.using_encoded(blake2_256),
        };
        ensure!(<Roles<T>>::contains_key(role_id), Error::<T>::RoleNotFound);
        Ok(role_id)
    }

    fn get_permission(pallet_id: u64 ,id_or_permission: IdOrString<T::PermissionMaxLen>)->Result<[u8;32], DispatchError>{
        let permission_id = match id_or_permission{
            IdOrString::Id(id)=>id,
            IdOrString::String(permission_str)=> permission_str.using_encoded(blake2_256),
        };
        ensure!(<Permissions<T>>::contains_key(pallet_id, permission_id), Error::<T>::PermissionNotFound);
        Ok(permission_id)
    }

    fn has_unique_elements(vec: Vec<u8>) -> bool{
        let mut filtered_vec = vec.clone();
        filtered_vec.sort();
        filtered_vec.dedup();
        vec.len() == filtered_vec.len()
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
}