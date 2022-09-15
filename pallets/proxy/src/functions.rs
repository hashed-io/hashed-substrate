use super::*;
use frame_support::{pallet_prelude::*};
use frame_support::traits::Time;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec; // vec primitive
use scale_info::prelude::vec; // vec![] macro

use pallet_rbac::types::*;
use crate::types::*;

impl<T: Config> Pallet<T> {
    // M A I N  F U N C T I O N S
    // --------------------------------------------------------------------------------------------
    
    pub fn do_initial_setup() -> DispatchResult{
        let pallet_id = Self::pallet_id();
        let super_roles = vec![ProxyRole::Administrator.to_vec()];
        //Admin rol & permissions
        let super_role_ids = T::Rbac::create_and_set_roles(pallet_id.clone(), super_roles)?;
        for super_role in super_role_ids{
            T::Rbac::create_and_set_permissions(pallet_id.clone(), super_role, ProxyPermission::administrator_permissions())?;
        }
        //
        Self::deposit_event(Event::ProxySetupCompleted);
        let global_scope = pallet_id.using_encoded(blake2_256);
        <GlobalScope<T>>::put(global_scope);

        Ok(())
    }
    
    pub fn do_create_project(
        admin: T::AccountId, 
        tittle: BoundedVec<u8, T::ProjectNameMaxLen>,
        description: BoundedVec<u8, T::ProjectDescMaxLen>,
        image: BoundedVec<u8, T::CIDMaxLen>,
        completition_date: u64,
        developer: Option<T::AccountId>,
        builder: Option<T::AccountId>,
        issuer: Option<T::AccountId>,
        regional_center: Option<T::AccountId>,
     ) -> DispatchResult {
        //TOREVIEW: admin only - check if validation is working
        let global_scope = <GlobalScope<T>>::try_get().map_err(|_| Error::<T>::NoneValue)?;
        Self::is_superuser(admin.clone(), &global_scope, ProxyRole::Administrator.id())?;

        //Add timestamp 
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        //Create project_id
        //TOREVIEW: We could use only name as project_id or use a method/storagemap to check if the name is already in use
        let project_id = (tittle.clone()).using_encoded(blake2_256);

        //ensure completition date is in the future
        ensure!(completition_date > timestamp, Error::<T>::CompletitionDateMustBeLater);
        
        //Create project data
        let project_data = Project{
            tittle,
            description,
            image,
            developer,
            builder,
            issuer,
            regional_center,
            creation_date: timestamp,
            completition_date,
            updated_date: timestamp,
        };

        //Insert project data
        // ensure that the project_id is not already in use
        ensure!(!Projects::<T>::contains_key(project_id), Error::<T>::ProjectIdAlreadyInUse);
        Projects::<T>::insert(project_id, project_data);

        // Event
        Self::deposit_event(Event::ProjectCreated(admin, project_id));

        Ok(())
    }

    pub fn do_add_user(admin: T::AccountId, user: T::AccountId, role: ProxyRole, related_projects: Option<BoundedVec<[u8;32], T::MaxProjectsPerUser>>, documents: Option<BoundedVec<u8, T::MaxDocuments>>, ) -> DispatchResult {
        //TODO: admin only

        // ensure if user is already registered
        ensure!(<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserAlreadyRegistered);

        let user_data = UserData::<T> {
            role,
            related_projects,
            documents,
        };

        //Insert user data
        <UsersInfo<T>>::insert(user.clone(), user_data);
        Self::deposit_event(Event::UserAdded(user));
        Ok(())
    }

    // H E L P E R S
    // --------------------------------------------------------------------------------------------
    /// Get the current timestamp in milliseconds
    fn get_timestamp_in_milliseconds() -> Option<u64> {
        let timestamp:u64 = T::Timestamp::now().into();

        Some(timestamp)
    }

    /// Get the pallet_id
    pub fn pallet_id()->IdOrVec{
        IdOrVec::Vec(
            Self::module_name().as_bytes().to_vec()
        )
    }

    fn is_authorized( authority: T::AccountId, project_id: &[u8;32], permission: ProxyPermission ) -> DispatchResult{
        T::Rbac::is_authorized(
            authority,
            Self::pallet_id(), 
            project_id,
            &permission.id(),
        )
    }

    fn is_superuser( authority: T::AccountId, scope_global: &[u8;32], rol_id: RoleId ) -> DispatchResult{
        T::Rbac::has_role(
            authority,
            Self::pallet_id(), 
            scope_global,
            vec![rol_id],
        )
    }




}