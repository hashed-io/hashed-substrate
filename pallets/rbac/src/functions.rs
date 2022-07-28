use super::*;
use frame_support::{pallet_prelude::*};
use frame_system::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use frame_support::sp_std::borrow::ToOwned;
use sp_runtime::sp_std::vec::Vec;

use crate::types::*;

impl<T: Config> RoleBasedAccessControl<T::AccountId> for Pallet<T>{
    /*---- Basic Insertion of individual storage maps ---*/
    fn create_scope(pallet_id: u32, scope_id: [u8;32])-> DispatchResult{
        <Scopes<T>>::try_mutate(pallet_id, |scopes|{
            ensure!(!scopes.contains(&scope_id), Error::<T>::ScopeAlreadyExists);
            scopes.try_push(scope_id).map_err(|_| Error::<T>::ExceedMaxScopesPerPallet)?;
            Ok(())
        })
    }

    /// Inserts roles and links them to the pallet
    fn create_and_set_roles(pallet_id: u32, roles: BoundedVec<BoundedVec<u8,ConstU32<100> >, T::MaxRolesPerPallet>) -> 
        Result<BoundedVec<[u8;32], T::MaxRolesPerPallet>, DispatchError>{
        // TODO: check for duplicates
        //ensure!(Self::has_unique_elements(roles.to_vec()), Error::<T>::DuplicateRole);
        
        let role_ids: Vec<[u8;32]> = roles.iter().map(|r|{
            Self::create_role(r.to_owned())
        }).collect();
        let bounded_ids = BoundedVec::try_from(role_ids).map_err(|_| Error::<T>::ExceedMaxRolesPerPallet)?;
        Self::set_multiple_pallet_roles(pallet_id, bounded_ids.clone())?;
        Ok(bounded_ids)
    }

    fn create_role(role: BoundedVec<u8, ConstU32<100>>)-> [u8;32]{
        let role_id = role.using_encoded(blake2_256);
        // insert is infalible in this case
        <Roles<T>>::insert(role_id, role);
        role_id
    }

    fn set_role_to_pallet(pallet_id: u32, role_id: [u8;32] )-> DispatchResult{
        ensure!(<Roles<T>>::contains_key(role_id), Error::<T>::RoleNotFound);

        <PalletRoles<T>>::try_mutate(pallet_id, |roles|{
            ensure!(!roles.contains(&role_id), Error::<T>::DuplicateRole );
            roles.try_push(role_id).map_err(|_| Error::<T>::ExceedMaxRolesPerPallet)
        })?;
        Ok(())
    }

    fn set_multiple_pallet_roles(pallet_id: u32, roles: BoundedVec<[u8;32], T::MaxRolesPerPallet>)->DispatchResult{
        for id in roles.clone(){
            ensure!(!<PalletRoles<T>>::get(&pallet_id).contains(&id), Error::<T>::DuplicateRole );
        }
        <PalletRoles<T>>::try_mutate(pallet_id, |pallet_roles|{
            pallet_roles.try_extend(roles.into_iter())
        }).map_err(|_| Error::<T>::ExceedMaxRolesPerPallet)?;

        Ok(())
    }

    fn create_permission(pallet_id: u32, permission: BoundedVec<u8, T::PermissionMaxLen>) -> [u8;32]{
        let permission_id = permission.using_encoded(blake2_256);
        <Permissions<T>>::insert(pallet_id, permission_id, permission);
        permission_id
    }

    fn set_permission_to_role( pallet_id: u32, role: [u8;32], permission: [u8;32] ) -> DispatchResult{
        //check for duplicates

        // try pushing 

        Ok(())
    }

    fn assign_role_to_user(user: T::AccountId, pallet_id: u32, scope_id: [u8;32], role_id: [u8;32]) -> DispatchResult{
        Self::scope_exists(&pallet_id, &scope_id)?;
        Self::is_role_linked_to_pallet(&pallet_id, &role_id)?;
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

    /*---- Helper functions ----*/

    /// Authorize by role, not permissions
    fn is_user_authorized(user: T::AccountId, pallet_id: u32, scope_id: [u8;32], role: IdOrString<ConstU32<100>> ) -> DispatchResult{
        // get id, whether is given directly or by its string in boundedvec format
        let role_id = Self::get_role_id(role)?;
        Self::scope_exists(&pallet_id, &scope_id)?;
        Self::is_role_linked_to_pallet(&pallet_id, &role_id)?;
        // Perform confirmation on both maps
        // TODO: test if a role that doesnt exists cause any errors
        let users = <UsersByScope<T>>::get( (pallet_id, scope_id, role_id) );
        ensure!(users.contains(&user), Error::<T>::NotAuthorized);
        let roles = <Users<T>>::get((user, pallet_id, scope_id));
        // Not likely to happen but just in case:
        ensure!(roles.contains(&role_id), Error::<T>::NotAuthorized );
        Ok(())
    }
    /// Also checks if pallet is stored
    fn scope_exists(pallet_id: &u32, scope_id:&[u8;32]) -> DispatchResult{
        ensure!(<Scopes<T>>::get(pallet_id).contains(&scope_id), Error::<T>::ScopeNotFound);
        Ok(())
    }

    fn is_role_linked_to_pallet(pallet_id: &u32, role_id: &[u8;32])-> DispatchResult{
        // The role exists, now  check if the role is assigned to that pallet
        <PalletRoles<T>>::get(pallet_id).iter().find(|pallet_role| *pallet_role==role_id )
            .ok_or(Error::<T>::RoleNotLinkedToPallet)?;
        Ok(())
    }

    fn get_role_id(id_or_role: IdOrString<ConstU32<100>>)->Result<[u8;32], DispatchError>{
        let role_id = match id_or_role{
            IdOrString::Id(id)=>id,
            IdOrString::String(role_str)=> role_str.using_encoded(blake2_256),
        };
        ensure!(<Roles<T>>::contains_key(role_id), Error::<T>::RoleNotFound);
        Ok(role_id)
    }

    fn get_permission(pallet_id: u32 ,id_or_permission: IdOrString<T::PermissionMaxLen>)->Result<[u8;32], DispatchError>{
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

    type PermissionMaxLen = T::PermissionMaxLen;

}