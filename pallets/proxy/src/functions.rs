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
        let global_scope = pallet_id.using_encoded(blake2_256);
        <GlobalScope<T>>::put(global_scope);

        //Admin rol & permissions
        let administrator_role_id = T::Rbac::create_and_set_roles(pallet_id.clone(), [ProxyRole::Administrator.to_vec()].to_vec())?;
        T::Rbac::create_and_set_permissions(pallet_id.clone(), administrator_role_id[0], ProxyPermission::administrator_permissions())?;
        // Create a global scope for the administrator role
        T::Rbac::create_scope(Self::pallet_id(), global_scope)?;

        Self::deposit_event(Event::ProxySetupCompleted);
        Ok(())
    }

    pub fn do_sudo_add_administrator(
        admin: T::AccountId, 
    ) -> DispatchResult{
        let pallet_id = Self::pallet_id();
        let global_scope =  <GlobalScope<T>>::try_get().map_err(|_| Error::<T>::GlobalScopeNotSet)?;

        T::Rbac::assign_role_to_user(
            admin.clone(), 
            pallet_id.clone(), 
            &global_scope, 
            ProxyRole::Administrator.id())?;
        
        Self::deposit_event(Event::AdministratorAssigned(admin));
        Ok(())
    }

    pub fn do_sudo_remove_administrator(
        admin: T::AccountId, 
    ) -> DispatchResult{
        let pallet_id = Self::pallet_id();
        let global_scope =  <GlobalScope<T>>::try_get().map_err(|_| Error::<T>::GlobalScopeNotSet)?;

        T::Rbac::remove_role_from_user(
            admin.clone(), 
            pallet_id.clone(), 
            &global_scope, 
            ProxyRole::Administrator.id())?;
        
        Self::deposit_event(Event::AdministratorRemoved(admin));
        Ok(())
    }
    
    pub fn do_create_project(
        admin: T::AccountId, 
        tittle: BoundedVec<u8, T::ProjectNameMaxLen>,
        description: BoundedVec<u8, T::ProjectDescMaxLen>,
        image: BoundedVec<u8, T::CIDMaxLen>,
        completition_date: u64,
        ) -> DispatchResult {
        //ensure admin permissions 
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
        let project_data = ProjectData::<T> {
            developer: None,
            investor: None,
            issuer: None,
            regional_center: None,
            tittle,
            description,
            image,
            status: ProjectStatus::default(), 
            creation_date: timestamp,
            completition_date,
            updated_date: timestamp,
        };

        //Insert project data
        // ensure that the project_id is not already in use
        ensure!(!ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectIdAlreadyInUse);
        ProjectsInfo::<T>::insert(project_id, project_data);

        // Event
        Self::deposit_event(Event::ProjectCreated(admin, project_id));

        Ok(())
    }

    pub fn do_edit_project(
        admin: T::AccountId,
        project_id: [u8;32], 
        tittle: Option<BoundedVec<u8, T::ProjectNameMaxLen>>,
        description: Option<BoundedVec<u8, T::ProjectDescMaxLen>>, 
        image:  Option<BoundedVec<u8, T::CIDMaxLen>>, 
        creation_date: Option<u64>, 
        completition_date: Option<u64>,  
    ) -> DispatchResult {
        //ensure admin permissions             
        let global_scope = <GlobalScope<T>>::try_get().map_err(|_| Error::<T>::NoneValue)?;
        Self::is_superuser(admin.clone(), &global_scope, ProxyRole::Administrator.id())?;
        
        //Ensure project exists & get project data
        let project_data = ProjectsInfo::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;

        // Ensure project is not completed
        ensure!(project_data.status != ProjectStatus::Completed, Error::<T>::CannotEditCompletedProject);

        //Get current timestamp
        let current_timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        //Mutate project data
        <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
            let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
            
            if let Some(tittle) = tittle {
                project.tittle = tittle;
            }
            if let Some(description) = description {
                project.description = description;
            }
            if let Some(image) = image {
                project.image = image;
            }
            if let Some(creation_date) = creation_date {
                //ensure new creation date is in the past
                ensure!(creation_date < current_timestamp, Error::<T>::CreationDateMustBeInThePast);
                project.creation_date = creation_date;
            }
            if let Some(completition_date) = completition_date {
                //ensure new completition date is in the future
                ensure!(completition_date > current_timestamp, Error::<T>::CompletitionDateMustBeLater);
                project.completition_date = completition_date;
            }

            Ok(())    
        })?;

        // Event
        Self::deposit_event(Event::ProjectEdited(project_id));
        Ok(())
    } 

    pub fn do_delete_project(
        admin: T::AccountId,
        project_id: [u8;32], 
    ) -> DispatchResult {
        //ensure admin permissions 
        let global_scope = <GlobalScope<T>>::try_get().map_err(|_| Error::<T>::NoneValue)?;
        Self::is_superuser(admin.clone(), &global_scope, ProxyRole::Administrator.id())?;

        //Ensure project exists & get project data
        let project_data = ProjectsInfo::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;

        //Ensure project is not completed
        ensure!(project_data.status != ProjectStatus::Completed, Error::<T>::CannotDeleteCompletedProject);


        //TODO - Delete project scope from rbac pallet & any extra data

        
        // Delete from ProjectsInfo storagemap
        ProjectsInfo::<T>::remove(project_id);

        // Delete from UsersByProject storagemap
        UsersByProject::<T>::remove(project_id);

        //Event 
        Self::deposit_event(Event::ProjectDeleted(project_id));
        Ok(())
    }


    pub fn do_register_user(admin: T::AccountId, user: T::AccountId, documents: Option<BoundedVec<u8, T::MaxDocuments>>, ) -> DispatchResult {
        //ensure admin permissions     
        let global_scope = <GlobalScope<T>>::try_get().map_err(|_| Error::<T>::NoneValue)?;
        Self::is_superuser(admin.clone(), &global_scope, ProxyRole::Administrator.id())?;

        // check if user is already registered
        ensure!(<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserAlreadyRegistered);

        let user_data = UserData::<T> {
            related_projects: None,
            documents,
        };

        //Insert user data
        <UsersInfo<T>>::insert(user.clone(), user_data);
        Self::deposit_event(Event::UserAdded(user));
        Ok(())
    }

    pub fn do_assign_user(
        admin: T::AccountId,
        user: T::AccountId,
        project_id: [u8;32], 
        role: ProxyRole, 
    ) -> DispatchResult {
        //ensure admin permissions 
        let global_scope = <GlobalScope<T>>::try_get().map_err(|_| Error::<T>::NoneValue)?;
        Self::is_superuser(admin.clone(), &global_scope, ProxyRole::Administrator.id())?;

        //Ensure project exists & get project data
        let project_data = ProjectsInfo::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;

        //Ensure project is not completed
        ensure!(project_data.status != ProjectStatus::Completed, Error::<T>::CannotEditCompletedProject);

        //Ensure user is registered
        ensure!(<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserNotRegistered);

        //Ensure user is not already assigned to the project
        ensure!(Self::is_user_assigned_to_project(project_id, user.clone()), Error::<T>::UserAlreadyAssignedToProject);

        //TODO: Ensure user is not assigened to that scope rbac pallet

        //Insert user 
        <UsersByProject<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |users| {
            let users = users.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
            users.try_push(user.clone());
            Ok(())
        }).map_err(|_| Error::<T>::UserAlreadyAssignedToProject)?;

        // Inser user intoscope rbac pallet

        //Event 
        Self::deposit_event(Event::UserAssignedToProject(user, project_id));
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


    pub fn change_project_status(project_id: [u8;32], status: ProjectStatus) -> DispatchResult {
        //ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        //Mutate project data
        <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
            let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
            project.status = status;
            Ok(())    
        })?;

        Ok(())
    }

    fn is_user_assigned_to_project(project_id: [u8;32], user: T::AccountId) -> bool {
        let project_data = match  <UsersByProject<T>>::try_get(project_id) {
            Ok(project_data) => project_data,
            Err(_) => return false,
        };

        project_data.iter().any(|user_id| user_id == &user)
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