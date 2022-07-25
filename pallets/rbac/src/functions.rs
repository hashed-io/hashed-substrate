use super::*;
use frame_support::{pallet_prelude::*, BoundedBTreeSet};
use frame_system::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
//use sp_runtime::sp_std::vec::Vec;
use crate::types::*;

impl<T: Config> Pallet<T> {

    /*---- Basic CRUD of individual storage maps ---*/
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

    pub fn set_pallet_role(pallet_id: u32, role_id: [u8;32])-> DispatchResult{
        ensure!(<Roles<T>>::contains_key(role_id), Error::<T>::RoleNotFound);
        <PalletRoles<T>>::try_mutate(pallet_id, |roles|{
            ensure!(!roles.contains(&role_id), Error::<T>::DuplicateRole );
            roles.try_push(role_id).map_err(|_| Error::<T>::ExceedMaxRolesPerPallet)
        })?;
        Ok(())
    }

    pub fn create_permission(pallet_id: u32, scope_id:[u8;32] ,role_id: [u8;32], permission: BoundedVec<u8, T::PermissionMaxLen>) -> DispatchResult{
        let p = <Scopes<T>>::get(pallet_id).ok_or(Error::<T>::PalletNotFound)?;
        ensure!(p.contains(&scope_id), Error::<T>::ScopeNotFound);
        ensure!(<Roles<T>>::contains_key(role_id), Error::<T>::RoleNotFound);
        <Permissions<T>>::try_mutate((pallet_id, scope_id,role_id), |ps|{
            ensure!(!ps.contains(&permission), Error::<T>::DuplicatePermission);
            ps.try_push(permission).map_err(|_| Error::<T>::ExceedMaxPermissionsPerRole)
        })?;
        Ok(())
    }
}