use super::*;
use frame_support::{pallet_prelude::*, BoundedBTreeSet};
use frame_system::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec;

use crate::types::*;

impl<T: Config> Pallet<T> {

    pub fn insert_initial_config(pallet_id: u32, roles: BoundedVec<IdOrRole, T::MaxRolesPerPallet>){
        todo!()
    }
    /*---- Basic Insertion of individual storage maps ---*/
    pub fn create_scope(pallet_id: u32, scope_id: [u8;32])-> DispatchResult{
        <Scopes<T>>::try_mutate(pallet_id, |scopes_option|{
            let scopes =scopes_option.as_mut().ok_or(Error::<T>::ScopeNotFound)?;
            scopes.try_push(scope_id).map_err(|_| Error::<T>::ExceedMaxScopesPerPallet)?;
            Ok(())
        })
    }

    pub fn create_role(role: BoundedVec<u8, ConstU32<100>>)-> [u8;32]{
        let role_id = role.using_encoded(blake2_256);
        // insert is infalible in this case
        <Roles<T>>::insert(role_id, role);
        role_id
    }

    pub fn set_pallet_role(pallet_id: u32, id_or_role: IdOrRole)-> DispatchResult{
        // get_role checks if that role exists
        let role_id = Self::get_role(id_or_role)?;

        <PalletRoles<T>>::try_mutate(pallet_id, |roles|{
            ensure!(!roles.contains(&role_id), Error::<T>::DuplicateRole );
            roles.try_push(role_id).map_err(|_| Error::<T>::ExceedMaxRolesPerPallet)
        })?;
        Ok(())
    }

    pub fn set_multiple_pallet_roles(pallet_id: u32, roles: BoundedVec<IdOrRole, T::MaxRolesPerPallet>)->DispatchResult{
        let mut role_ids = Vec::<[u8;32]>::new();
        for id_or_role in roles{
            let id = Self::get_role(id_or_role)?;
            ensure!(!<PalletRoles<T>>::get(&pallet_id).contains(&id), Error::<T>::DuplicateRole );
            role_ids.push(id)
        }
        <PalletRoles<T>>::try_mutate(pallet_id, |roles|{
            roles.try_append(&mut role_ids)
                .map_err(|_| Error::<T>::ExceedMaxRolesPerPallet)
        })?;

        Ok(())
    }

    pub fn create_permission(pallet_id: u32, scope_id:[u8;32] ,id_or_role: IdOrRole, permission: BoundedVec<u8, T::PermissionMaxLen>) -> DispatchResult{
        let role_id = Self::get_role(id_or_role)?;
        Self::is_role_in_scope(&pallet_id, &scope_id, &role_id)?;
        <Permissions<T>>::try_mutate((pallet_id, scope_id,role_id), |ps|{
            ensure!(!ps.contains(&permission), Error::<T>::DuplicatePermission);
            ps.try_push(permission).map_err(|_| Error::<T>::ExceedMaxPermissionsPerRole)
        })?;
        Ok(())
    }

    pub fn assign_role_to_user(user: T::AccountId, pallet_id: u32, scope_id: [u8;32], role_id: [u8;32]) -> DispatchResult{
        Self::is_role_in_scope(&pallet_id, &scope_id, &role_id)?;
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

    /// Authorize by rolE, not permissions
    pub fn is_user_authorized(user: T::AccountId, pallet_id: u32, scope_id: [u8;32], role: IdOrRole ) -> DispatchResult{
        // get id, whether is given directly or by its string in boundedvec format
        let role_id = Self::get_role(role)?;
        Self::is_role_in_scope(&pallet_id, &scope_id, &role_id)?;
        // Perform confirmation on both maps
        // TODO: test if a role that doesnt exists cause any errors
        let users = <UsersByScope<T>>::get( (pallet_id, scope_id, role_id) );
        ensure!(users.contains(&user), Error::<T>::NotAuthorized);
        let roles = <Users<T>>::get((user, pallet_id, scope_id));
        // Not likelly to happen but just in case:
        ensure!(roles.contains(&role_id), Error::<T>::NotAuthorized );
        Ok(())
    }
    /// Also checks if pallet is stored
    fn scope_exists(pallet_id: &u32, scope_id:&[u8;32]) -> DispatchResult{
        let p = <Scopes<T>>::get(pallet_id).ok_or(Error::<T>::PalletNotFound)?;
        ensure!(p.contains(&scope_id), Error::<T>::ScopeNotFound);
        Ok(())
    }

    fn is_role_in_scope(pallet_id: &u32, scope_id:&[u8;32], role_id: &[u8;32])-> DispatchResult{
        Self::scope_exists(pallet_id, scope_id)?;
        //ensure!(<Roles<T>>::contains_key(role_id), Error::<T>::RoleNotFound);
        // The role exists, now  check if the role is assigned to that pallet
        <PalletRoles<T>>::get(pallet_id).iter().find(|pallet_role| *pallet_role==role_id )
            .ok_or(Error::<T>::RoleNotLinkedToPallet)?;
        Ok(())
    }

    fn get_role(id_or_role: IdOrRole)->Result<[u8;32], DispatchError>{
        let role_id = match id_or_role{
            IdOrRole::Id(id)=>id,
            IdOrRole::Role(role_str)=> role_str.using_encoded(blake2_256),
        };
        ensure!(<Roles<T>>::contains_key(role_id), Error::<T>::RoleNotFound);
        Ok(role_id)
    }
}