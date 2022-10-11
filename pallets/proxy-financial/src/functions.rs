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
    
    // I N I T I A L 
    // --------------------------------------------------------------------------------------------
		
    pub fn do_initial_setup() -> DispatchResult{
        let pallet_id = Self::pallet_id();
        let global_scope = pallet_id.using_encoded(blake2_256);
        <GlobalScope<T>>::put(global_scope);

        //Admin rol & permissions
        let administrator_role_id = T::Rbac::create_and_set_roles(pallet_id.clone(), [ProxyRole::Administrator.to_vec()].to_vec())?;
        T::Rbac::create_and_set_permissions(pallet_id.clone(), administrator_role_id[0], ProxyPermission::administrator_permissions())?;
        
        //Developer rol & permissions
        let _developer_role_id = T::Rbac::create_and_set_roles(pallet_id.clone(), [ProxyRole::Developer.to_vec()].to_vec())?;
        //T::Rbac::create_and_set_permissions(pallet_id.clone(), developer_role_id[0], ProxyPermission::developer_permissions())?;

        // Investor rol & permissions
        let _investor_role_id = T::Rbac::create_and_set_roles(pallet_id.clone(), [ProxyRole::Investor.to_vec()].to_vec())?;
        //T::Rbac::create_and_set_permissions(pallet_id.clone(), investor_role_id[0], ProxyPermission::investor_permissions())?;

        // Issuer rol & permissions
        let _issuer_role_id = T::Rbac::create_and_set_roles(pallet_id.clone(), [ProxyRole::Issuer.to_vec()].to_vec())?;
        //T::Rbac::create_and_set_permissions(pallet_id.clone(), issuer_role_id[0], ProxyPermission::issuer_permissions())?;

        // Regional center rol & permissions
        let _regional_center_role_id = T::Rbac::create_and_set_roles(pallet_id.clone(), [ProxyRole::RegionalCenter.to_vec()].to_vec())?;
        //T::Rbac::create_and_set_permissions(pallet_id.clone(), regional_center_role_id[0], ProxyPermission::regional_center_permissions())?;
        
        // Create a global scope for the administrator role
        T::Rbac::create_scope(Self::pallet_id(), global_scope)?;

        Self::deposit_event(Event::ProxySetupCompleted);
        Ok(())
    }

    pub fn do_sudo_add_administrator(
        admin: T::AccountId, 
        name: FieldName,
    ) -> DispatchResult{
        let pallet_id = Self::pallet_id();
        let global_scope =  <GlobalScope<T>>::try_get().map_err(|_| Error::<T>::GlobalScopeNotSet)?;

        T::Rbac::assign_role_to_user(
            admin.clone(), 
            pallet_id.clone(), 
            &global_scope, 
            ProxyRole::Administrator.id())?;

        // create a administrator user account
        Self::sudo_register_admin(admin.clone(), name)?;
        
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
        
        // remove administrator user account
        Self::sudo_delete_admin(admin.clone())?;
        
        Self::deposit_event(Event::AdministratorRemoved(admin));
        Ok(())
    }
    

    // P R O J E C T S
    // --------------------------------------------------------------------------------------------
	
    /// Create a new project
    /// - only administrator can create a new project
    /// expenditures = (name, type, budget amount, naics code, jobs multiplier)
    /// users = (accountid, role)
    pub fn do_create_project(
        admin: T::AccountId, 
        title: FieldName,
        description: FieldDescription,
        image: CID,
        address: FieldName,
        project_type: ProjectType,
        completion_date: u64,
        expenditures: BoundedVec<(
            FieldName,
            ExpenditureType,
            u64,
            Option<u32>,
            Option<u32>,
        ), T::MaxRegistrationsAtTime>,
        users: Option<BoundedVec<(
            T::AccountId,
            ProxyRole
        ), T::MaxRegistrationsAtTime>>,
        ) -> DispatchResult {
        // Ensure admin permissions 
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Add timestamp 
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        //Create project_id
        //TOREVIEW: We could use only name as project_id or use a method/storagemap to check if the name is already in use
        let project_id = (title.clone()).using_encoded(blake2_256);

        //ensure completion_date is in the future
        ensure!(completion_date > timestamp, Error::<T>::CompletionDateMustBeLater);
        
        //Create project data
        let project_data = ProjectData::<T> {
            developer: Some(BoundedVec::<T::AccountId, T::MaxDevelopersPerProject>::default()),
            investor: Some(BoundedVec::<T::AccountId, T::MaxInvestorsPerProject>::default()),
            issuer: Some(BoundedVec::<T::AccountId, T::MaxIssuersPerProject>::default()),
            regional_center: Some(BoundedVec::<T::AccountId, T::MaxRegionalCenterPerProject>::default()),
            title,
            description,
            image,
            address,
            status: ProjectStatus::default(), 
            project_type,
            creation_date: timestamp,
            completion_date,
            updated_date: timestamp,
        };

        // create scope for project_id
        T::Rbac::create_scope(Self::pallet_id(), project_id)?;

        //Insert project data
        // ensure that the project_id is not already in use
        ensure!(!ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectIdAlreadyInUse);
        ProjectsInfo::<T>::insert(project_id, project_data);

        //Add expenditures
        Self::do_create_expenditure(admin.clone(), project_id, expenditures)?;

        // Add users
        if let Some(users) = users {
            Self::do_assign_user(admin.clone(), project_id, users)?;
        }

        //Initialize drawdowns
        Self::do_initialize_drawdowns(admin.clone(), project_id)?;

        // Event
        Self::deposit_event(Event::ProjectCreated(admin, project_id));

        Ok(())
    }

    pub fn do_edit_project(
        admin: T::AccountId,
        project_id: [u8;32], 
        title: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,	
        description: Option<BoundedVec<FieldDescription, T::MaxBoundedVecs>>,
        image: Option<BoundedVec<CID, T::MaxBoundedVecs>>,
        address: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>, 
        completion_date: Option<u64>,  
    ) -> DispatchResult {
        //ensure admin permissions             
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;
        
        //Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        // Ensure project is not completed
        Self::is_project_completed(project_id)?;

        //Get current timestamp
        let current_timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        //Mutate project data
        <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
            let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
            
            if let Some(title) = title {
                let mod_title = title.into_inner();
                project.title = mod_title[0].clone();
            }
            if let Some(description) = description {
                let mod_description = description.into_inner();
                project.description = mod_description[0].clone();
            }
            if let Some(image) = image {
                let mod_image = image.into_inner();
                project.image = mod_image[0].clone();
            }
            if let Some(address) = address {
                let mod_address = address.into_inner();
                project.address = mod_address[0].clone();
            }
            if let Some(completion_date) = completion_date {
                //ensure new completion_date date is in the future
                ensure!(completion_date > current_timestamp, Error::<T>::CompletionDateMustBeLater);
                project.completion_date = completion_date;
            }
            //TOREVIEW: Check if this is working
            project.updated_date = current_timestamp;

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
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Ensure project exists & get project data
        let project_data = ProjectsInfo::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;

        //Ensure project is not completed
        ensure!(project_data.status != ProjectStatus::Completed, Error::<T>::CannotDeleteCompletedProject);

        // Delete scope from rbac pallet
        T::Rbac::remove_scope(Self::pallet_id(), project_id)?;

        //TOREVIEW: check if this method is the best way to delete data from storage
        // we could use get method (<UsersByProject<T>>::get()) instead getter function
        // Delete project from ProjectsByUser storage
        let users_by_project = Self::users_by_project(project_id).iter().cloned().collect::<Vec<T::AccountId>>();
        for user in users_by_project {
            <ProjectsByUser<T>>::mutate(user, |projects| {
                projects.retain(|project| *project != project_id);
            });
        }

        // Delete from ProjectsInfo storagemap
        <ProjectsInfo<T>>::remove(project_id);

        // Delete from UsersByProject storagemap
        <UsersByProject<T>>::remove(project_id);

        //Event 
        Self::deposit_event(Event::ProjectDeleted(project_id));
        Ok(())
    }

    // U S E R S
    // --------------------------------------------------------------------------------------------
	//TODO: Create a custom type for users bounded vec	
    pub fn do_register_user(
        admin: T::AccountId,
        users: BoundedVec<(T::AccountId, FieldName, ProxyRole), T::MaxRegistrationsAtTime>,
    ) -> DispatchResult {
        //ensure admin permissions     
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Get current timestamp
        let current_timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        for user in users {
            // Ensure if user is already registered
            ensure!(!<UsersInfo<T>>::contains_key(user.0.clone()), Error::<T>::UserAlreadyRegistered);

            match user.2 {
                ProxyRole::Administrator => {
                    Self::do_sudo_add_administrator(user.0.clone(), user.1.clone())?;
                },
                _ => {
                    // Create user data
                    let user_data = UserData::<T> {
                        name: user.1.clone(),
                        role: user.2,
                        image: CID::default(),
                        date_registered: current_timestamp,
                        email: FieldName::default(),
                        documents: None,
                    };

                    //Insert user data
                    <UsersInfo<T>>::insert(user.0.clone(), user_data);
                    Self::deposit_event(Event::UserAdded(user.0));
                },
            }
        }

        Ok(())
    }

    pub fn do_assign_user(
        admin: T::AccountId,
        project_id: [u8;32], 
        users: BoundedVec<(T::AccountId, ProxyRole), T::MaxRegistrationsAtTime>,
    ) -> DispatchResult {
        //ensure admin permissions 
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        // Ensure project is not completed
        Self::is_project_completed(project_id)?;

        for user in users{
            // Basic validations prior to assign user
            Self::check_user_role(user.0.clone(), user.1)?;

            //Ensure user is not already assigned to the project
            ensure!(!<UsersByProject<T>>::get(project_id).contains(&user.0), Error::<T>::UserAlreadyAssignedToProject);
            ensure!(!<ProjectsByUser<T>>::get(user.0.clone()).contains(&project_id), Error::<T>::UserAlreadyAssignedToProject);

            // Ensure user is not assigened to the selected scope (project_id) with the selected role
            ensure!(!T::Rbac::has_role(user.0.clone(), Self::pallet_id(), &project_id, [user.1.id()].to_vec()).is_ok(), Error::<T>::UserAlreadyAssignedToProject);

            // Update project data depending on the role assigned
            Self::add_project_role(project_id, user.0.clone(), user.1)?;

            // Insert project to ProjectsByUser storagemap
            <ProjectsByUser<T>>::try_mutate::<_,_,DispatchError,_>(user.0.clone(), |projects| {
                projects.try_push(project_id).map_err(|_| Error::<T>::MaxProjectsPerUserReached)?;
                Ok(())
            })?;

            // Insert user to UsersByProject storagemap
            <UsersByProject<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |users| {
                users.try_push(user.0.clone()).map_err(|_| Error::<T>::MaxUsersPerProjectReached)?;
                Ok(())
            })?;

            // Insert user into scope rbac pallet
            T::Rbac::assign_role_to_user(user.0.clone(), Self::pallet_id(), &project_id, user.1.id())?;
    }

        //Event 
        Self::deposit_event(Event::UserAssignedToProject);
        Ok(())
    }

    pub fn do_unassign_user(
        admin: T::AccountId,
        user: T::AccountId,
        project_id: [u8;32], 
        role: ProxyRole, 
    ) -> DispatchResult {
        //ensure admin permissions 
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        // Ensure project is not completed
        Self::is_project_completed(project_id)?;

        //Ensure user is registered
        ensure!(<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserNotRegistered);

        //Ensure user is assigned to the project
        ensure!(<UsersByProject<T>>::get(project_id).contains(&user.clone()), Error::<T>::UserNotAssignedToProject);
        ensure!(<ProjectsByUser<T>>::get(user.clone()).contains(&project_id), Error::<T>::UserNotAssignedToProject);

        // Ensure user has the specified role assigned in the selected project
        ensure!(T::Rbac::has_role(user.clone(), Self::pallet_id(), &project_id, [role.id()].to_vec()).is_ok(), Error::<T>::UserDoesNotHaveRole);

        // Update project data depending on the role unassigned
        Self::remove_project_role(project_id, user.clone(), role)?;
        
        //HERE
        // Update user data depending on the role unassigned
        //Self::remove_user_role(user.clone())?;

        // Remove user from UsersByProject storagemap
        <UsersByProject<T>>::mutate(project_id, |users| {
            users.retain(|u| u != &user);
        });

        // Remove user from ProjectsByUser storagemap
        <ProjectsByUser<T>>::mutate(user.clone(), |projects| {
            projects.retain(|p| p != &project_id);
        });

        // Remove user from scope
        T::Rbac::remove_role_from_user(user.clone(), Self::pallet_id(), &project_id, role.id())?;

        Self::deposit_event(Event::UserUnassignedFromProject(user, project_id));
        Ok(())
    }

    pub fn do_update_user(
        admin: T::AccountId,
        user: T::AccountId, 
        name: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,
        image: Option<BoundedVec<CID, T::MaxBoundedVecs>>,
        email: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,
        documents: Option<Documents<T>>, 
    ) -> DispatchResult {
        //ensure admin permissions 
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Ensure user is registered
        ensure!(<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserNotRegistered);

        //Update user data
        <UsersInfo<T>>::try_mutate::<_,_,DispatchError,_>(user.clone(), |user_data| {
            let user_info = user_data.as_mut().ok_or(Error::<T>::UserNotRegistered)?;

            if let Some(name) = name {
                let mod_name = name.into_inner();
                user_info.name = mod_name[0].clone();
            }
            if let Some(image) = image {
                let mod_image = image.into_inner();
                user_info.image = mod_image[0].clone();
            }
            if let Some(email) = email {
                let mod_email = email.into_inner();
                user_info.email = mod_email[0].clone();
            }
            if let Some(documents) = documents {
                user_info.documents = Some(documents);
            }
            Ok(())
        })?;

        Self::deposit_event(Event::UserUpdated(user));

        Ok(())
    }

    pub fn do_delete_user(
        admin: T::AccountId,
        user: T::AccountId,
    ) -> DispatchResult {
        //ensure admin permissions 
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Ensure user is registered
        ensure!(<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserNotRegistered);
        
        //HERE
        //Prevent users from deleting an administator
        // if let Some(admin_role) = user_data.role{
        //     ensure!(admin_role != ProxyRole::Administrator, Error::<T>::CannotRemoveAdminRole);
        // }

        // Can not delete an user if it has assigned projects
        let projects_by_user = Self::projects_by_user(user.clone()).iter().cloned().collect::<Vec<[u8;32]>>();

        if projects_by_user.len() == 0 {
            // Remove user from UsersInfo storagemap
            <UsersInfo<T>>::remove(user.clone());

            // Remove user from UsersByProject storagemap
            //TODO: FIX THIS ITERATION
            for project_id in projects_by_user {
                <UsersByProject<T>>::mutate(project_id, |users| {
                    users.retain(|u| u != &user);
                });
            }

            // Remove user from ProjectsByUser storagemap
            <ProjectsByUser<T>>::remove(user.clone());

            Self::deposit_event(Event::UserDeleted(user));
            Ok(())
        
        } else {
            Err(Error::<T>::CannotDeleteUserWithAssignedProjects.into())
        }

    }

    // B U D G E T  E X P E N D I T U R E 
    // --------------------------------------------------------------------------------------------
    /// Create a new budget expenditure
    /// 
    /// # Arguments
    /// 
    /// * `admin` - The admin user that creates the budget expenditure
    /// * `project_id` - The project id where the budget expenditure will be created
    /// 
    /// Then we add the budget expenditure data
    /// * `name` - The name of the budget expenditure
    /// * `type` - The type of the budget expenditure
    /// * `budget amount` - The amount of the budget expenditure
    /// * `naics code` - The naics code of the budget expenditure
    /// * `jobs_multiplier` - The jobs multiplier of the budget expenditure
    pub fn do_create_expenditure(
        admin: T::AccountId,
        project_id: [u8;32], 
        expenditures: BoundedVec<(
            FieldName,
            ExpenditureType,
            u64,
            Option<u32>,
            Option<u32>,
        ), T::MaxRegistrationsAtTime>,
    ) -> DispatchResult {
        //ensure admin permissions 
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        // We use this way to validate because it's necessary to get the project type 
        // in order to generate the right expenditure types 
        //Ensure project exists & get project data
        let project_data = ProjectsInfo::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;

        // Get timestamp
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        // Ensure project is not completed
        ensure!(project_data.status != ProjectStatus::Completed, Error::<T>::ProjectIsAlreadyCompleted);

        for expenditure in expenditures {
            // Ensure expenditure name is not empty
            ensure!(!expenditure.0.is_empty(), Error::<T>::FieldNameCannotBeEmpty);

            // Create expenditure id
            let expenditure_id = (project_id, expenditure.0.clone(), expenditure.1, timestamp).using_encoded(blake2_256);

            // Match project type to validate expenditure type
            match project_data.project_type {
                ProjectType::Construction => {
                    // Ensure expenditure type is valid
                    ensure!(expenditure.1 == ExpenditureType::HardCost || expenditure.1 == ExpenditureType::SoftCost, Error::<T>::InvalidExpenditureType);
                },
                ProjectType::ConstructionOperation => {
                    // Ensure expenditure type is valid
                    ensure!(expenditure.1 != ExpenditureType::Others, Error::<T>::InvalidExpenditureType);
                },
                ProjectType::ConstructionBridge => {
                    // Ensure expenditure type is valid
                    ensure!(expenditure.1 != ExpenditureType::Operational, Error::<T>::InvalidExpenditureType);
                },
                ProjectType::Operation => {
                    // Ensure expenditure type is valid
                    ensure!(expenditure.1 == ExpenditureType::Operational, Error::<T>::InvalidExpenditureType);
                },
            }

            // Create expenditure data
            let expenditure_data = ExpenditureData {
                project_id,
                name: expenditure.0.clone(),
                expenditure_type: expenditure.1,
                expenditure_amount: expenditure.2,
                naics_code: expenditure.3,
                jobs_multiplier: expenditure.4,
            };  

            // Insert expenditure data into ExpendituresInfo
            // Ensure expenditure_id is unique
            ensure!(!<ExpendituresInfo<T>>::contains_key(expenditure_id), Error::<T>::ExpenditureAlreadyExists);
            <ExpendituresInfo<T>>::insert(expenditure_id, expenditure_data);

            //Insert expenditure_id into ExpendituresByProject
            <ExpendituresByProject<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |expenditures| {
                expenditures.try_push(expenditure_id).map_err(|_| Error::<T>::MaxExpendituresPerProjectReached)?;
                Ok(())
            })?;

            // Create a budget for the expenditure
            Self::do_create_budget(expenditure_id, 0, project_id)?;
        }

        Self::deposit_event(Event::ExpenditureCreated);
        Ok(())
    }

    pub fn do_edit_expenditure(
        admin: T::AccountId,
        project_id: [u8;32], 
        expenditure_id: [u8;32],
        name: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>, 
        expenditure_amount: Option<u64>,
        naics_code: Option<u32>,
        jobs_multiplier: Option<u32>,
    ) -> DispatchResult {
        //Ensure admin permissions, TODO: add developer permissions
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        // Ensure project is not completed
        Self::is_project_completed(project_id)?;

        // Ensure expenditure_id exists 
        ensure!(<ExpendituresInfo<T>>::contains_key(expenditure_id), Error::<T>::ExpenditureNotFound);

        // Mutate expenditure data
        <ExpendituresInfo<T>>::try_mutate::<_,_,DispatchError, _>(expenditure_id, |expenditure_data| {
            let expenditure = expenditure_data.as_mut().ok_or(Error::<T>::ExpenditureNotFound)?;

            // Ensure expenditure belongs to project
            ensure!(expenditure.project_id == project_id, Error::<T>::ExpenditureDoesNotBelongToProject);

            //TODO: ensure name is unique

            if let  Some(name) = name {
                let mod_name = name.into_inner();
                // Ensure name is not empty
                ensure!(mod_name[0].len() > 0, Error::<T>::FieldNameCannotBeEmpty);
                expenditure.name = mod_name[0].clone();
            }
            if let Some(expenditure_amount) = expenditure_amount {
                expenditure.expenditure_amount = expenditure_amount;
            }
            if let Some(naics_code) = naics_code {
                expenditure.naics_code = Some(naics_code);
            }
            if let Some(jobs_multiplier) = jobs_multiplier {
                expenditure.jobs_multiplier = Some(jobs_multiplier);
            }

            Ok(())
        })?;


        Self::deposit_event(Event::ExpenditureEdited(expenditure_id));
        Ok(())
    }

    pub fn do_delete_expenditure(
        admin: T::AccountId,
        project_id: [u8;32], 
        expenditure_id: [u8;32],
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        // Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        // Ensure project is not completed
        Self::is_project_completed(project_id)?;

        // Ensure expenditure_id exists 
        ensure!(<ExpendituresInfo<T>>::contains_key(expenditure_id), Error::<T>::ExpenditureNotFound);

        // Delete expenditure data
        <ExpendituresInfo<T>>::remove(expenditure_id);

        // Delete expenditure_id from ExpendituresByProject
        <ExpendituresByProject<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |expenditures| {
            expenditures.retain(|expenditure| expenditure != &expenditure_id);
            Ok(())
        })?;

        // Delete expenditure budget
        Self::do_delete_budget(project_id, expenditure_id)?;

        Self::deposit_event(Event::ExpenditureDeleted(expenditure_id));
        Ok(())
    }



    // B U D G E T S
    // --------------------------------------------------------------------------------------------
    // Buget functions are not exposed to the public. They are only used internally by the module.
    fn do_create_budget(
        expenditure_id: [u8;32],
        amount: u64,
        project_id: [u8;32],
    ) -> DispatchResult {
        // Ensure expenditure_id exists 
        ensure!(<ExpendituresInfo<T>>::contains_key(expenditure_id), Error::<T>::ExpenditureNotFound);

        // Get timestamp
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        // Create budget id
        let budget_id = (expenditure_id, timestamp).using_encoded(blake2_256);

        //TOREVIEW: Check if project_id exists.

        // Create budget data
        let budget_data = BudgetData {
            expenditure_id,
            balance: amount,
            created_date: timestamp,
            updated_date: timestamp,
        };

        // Insert budget data
        <BudgetsInfo<T>>::insert(budget_id, budget_data);

        // Insert budget id into BudgetsByProject
        <BudgetsByProject<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |budgets| {
            budgets.try_push(budget_id).map_err(|_| Error::<T>::MaxBudgetsPerProjectReached)?;
            Ok(())
        })?;

        //TOREVIEW: Check if this event is needed
        Self::deposit_event(Event::BudgetCreated(budget_id));
        Ok(())
    }

    //For now budgets are not editable.
    fn _do_edit_budget(
        admin: T::AccountId,
        budget_id: [u8;32],
        amount: u64,
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;
        
        //Ensure budget exists
        ensure!(<BudgetsInfo<T>>::contains_key(budget_id), Error::<T>::BudgetNotFound);

        // Get timestamp
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        // Mutate budget data
        <BudgetsInfo<T>>::try_mutate::<_,_,DispatchError,_>(budget_id, |budget_data| {
            let mod_budget_data = budget_data.as_mut().ok_or(Error::<T>::BudgetNotFound)?;
            // Update budget data
            mod_budget_data.balance = amount;
            mod_budget_data.updated_date = timestamp;
            Ok(())
        })?;

        Ok(())
    }

    fn do_delete_budget(
        project_id: [u8;32],
        expenditure_id: [u8;32],
    ) -> DispatchResult {
        // Get budget id
        let budget_id = Self::get_budget_id(project_id, expenditure_id)?;

        // Remove budget data
        <BudgetsInfo<T>>::remove(budget_id);

        // Delete budget_id from BudgetsByProject
        <BudgetsByProject<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |budgets| {
            budgets.retain(|budget| budget != &budget_id);
            Ok(())
        })?;
        
        Ok(())
    }

    fn get_budget_id(
        project_id: [u8;32],
        expenditure_id: [u8;32],
    ) -> Result<[u8;32], DispatchError> {
        // Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        // Get budgets by project (Id's)
        let budget_ids = Self::budgets_by_project(project_id).into_inner();

        // Check if the project has any budgets
        if budget_ids.len() == 0 {
            return Err(Error::<T>::ThereIsNoBudgetsForTheProject.into());
        }

        // Get budget id
        let budget_id: [u8;32] = budget_ids.iter().try_fold::<_,_,Result<[u8;32], DispatchError>>([0;32], |mut accumulator, &budget_id| {
            // Get individual budget data
            let budget_data = BudgetsInfo::<T>::get(budget_id).ok_or(Error::<T>::BudgetNotFound)?;

            // Check if budget belongs to expenditure
            if budget_data.expenditure_id == expenditure_id {
                accumulator = budget_id;
            }
            Ok(accumulator)
        })?;

        Ok(budget_id)
    }


    // D R A W D O W N S
    // --------------------------------------------------------------------------------------------
    // For now drawdowns functions are private, but in the future they may be public
    
    fn do_create_drawdown(
        admin: T::AccountId,
        project_id: [u8;32],
        drawdown_type: DrawdownType,
        drawdown_number: u32,
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        // Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        // Get timestamp
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        // Create drawdown id
        let drawdown_id = (project_id, drawdown_type, drawdown_number).using_encoded(blake2_256);

        // Create drawdown data
        let drawdown_data = DrawdownData::<T> {
            project_id,
            drawdown_number,
            drawdown_type,
            total_amount: 0,
            status: DrawdownStatus::default(),
            created_date: timestamp,
            close_date: 0,
            creator: Some(admin.clone()),
        };

        // Insert drawdown data
        // Ensure drawdown id is unique
        ensure!(!DrawdownsInfo::<T>::contains_key(drawdown_id), Error::<T>::DrawdownAlreadyExists);
        <DrawdownsInfo<T>>::insert(drawdown_id, drawdown_data);

        // Insert drawdown id into DrawdownsByProject
        <DrawdownsByProject<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |drawdowns| {
            drawdowns.try_push(drawdown_id).map_err(|_| Error::<T>::MaxDrawdownsPerProjectReached)?;
            Ok(())
        })?;

        //TOREVIEW: Check if an event is needed

        Ok(())
    }

//    update(const uint64_t &drawdown_id, const eosio::asset &total_amount, const bool &is_add_balance);
//    edit(const uint64_t &drawdown_id,

    /// TODO: Function to create initial drawdowns for a project
    fn do_initialize_drawdowns(
        admin: T::AccountId,
        project_id: [u8;32],
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        // Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        //Create a EB5 drawdown
        Self::do_create_drawdown(admin.clone(), project_id, DrawdownType::EB5, 1)?;

        //Create a Construction Loan drawdown
        Self::do_create_drawdown(admin.clone(), project_id, DrawdownType::ConstructionLoan, 1)?;

        //Create a Developer Equity drawdown
        Self::do_create_drawdown(admin.clone(), project_id, DrawdownType::DeveloperEquity, 1)?;

        Ok(())
    }


//    submit(const uint64_t &drawdown_id);
//    approve(const uint64_t &drawdown_id);
//    reject(const uint64_t &drawdown_id);


    // T R A N S A C T I O N S
    // --------------------------------------------------------------------------------------------
    // For now transactions functions are private, but in the future they may be public
    // TOREVIEW: Each transaction has an amount and it refers to a selected expenditure,
    // so each drawdown sums the amount of each transaction -> drawdown.total_amount = transaction.amount + transaction.amount + transaction.amount
    // when a drawdown is approved, the amount is transfered to every expenditure
    // using the storage map, transactions_by_drawdown, we can get the transactions for a specific drawdown

    fn do_create_transaction(
        admin: T::AccountId,
        project_id: [u8;32],
        drawdown_id: [u8;32],
        expenditure_id: [u8;32],
        amount: u64,
        description: FieldDescription,
        //TOREVIEW: Is mandatory to upload documents with every transaction? If not we can wrap this field in an Option
        documents: Option<Documents<T>>
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        // Ensure drawdown exists
        ensure!(DrawdownsInfo::<T>::contains_key(drawdown_id), Error::<T>::DrawdownNotFound);

        // Ensure amount is valid
        Self::is_amount_valid(amount)?;

        // Ensure documents is not empty
        if let Some(mod_documents) = documents.clone() {
            ensure!(mod_documents.len() > 0, Error::<T>::DocumentsIsEmpty);
        }

        // Get timestamp
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        // Create transaction id
        let transaction_id = (drawdown_id, timestamp).using_encoded(blake2_256);

        // Create transaction data
        let transaction_data = TransactionData::<T> {
            project_id,
            drawdown_id,
            expenditure_id,
            creator: admin.clone(),
            created_date: timestamp,
            updated_date: timestamp,
            closed_date: 0,
            description,
            amount,
            status: TransactionStatus::default(),
            documents,
        };

        // Insert transaction data
        // Ensure transaction id is unique
        ensure!(!TransactionsInfo::<T>::contains_key(transaction_id), Error::<T>::TransactionAlreadyExists);
        <TransactionsInfo<T>>::insert(transaction_id, transaction_data);

        // Insert transaction id into TransactionsByProject
        <TransactionsByProject<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |transactions| {
            transactions.try_push(transaction_id).map_err(|_| Error::<T>::MaxTransactionsPerProjectReached)?;
            Ok(())
        })?;

        // Insert transaction id into TransactionsByDrawdown
        <TransactionsByDrawdown<T>>::try_mutate::<_,_,_,DispatchError,_>(project_id, drawdown_id, |transactions| {
            transactions.try_push(transaction_id).map_err(|_| Error::<T>::MaxTransactionsPerDrawdownReached)?;
            Ok(())
        })?;

        // Insert transaction id into TransactionsByExpenditure
        <TransactionsByExpenditure<T>>::try_mutate::<_,_,_,DispatchError,_>(project_id, expenditure_id, |transactions| {
            transactions.try_push(transaction_id).map_err(|_| Error::<T>::MaxTransactionsPerExpenditureReached)?;
            Ok(())
        })?;

        //TOREVIEW: Check if this event is needed
        Self::deposit_event(Event::TransactionCreated(transaction_id));
        Ok(())

    }

    fn do_edit_transaction(
        admin: T::AccountId,
        transaction_id: [u8;32],
        amount: Option<u64>,
        description: Option<BoundedVec<FieldDescription, T::MaxBoundedVecs>>,
        documents: Option<Documents<T>>
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        // Ensure transaction exists
        ensure!(TransactionsInfo::<T>::contains_key(transaction_id), Error::<T>::TransactionNotFound);

        // Ensure amount is valid.
        if let Some(mod_amount) = amount.clone() {
            Self::is_amount_valid(mod_amount)?;
        }

        // Ensure documents is not empty
        if let Some(mod_documents) = documents.clone() {
            ensure!(mod_documents.len() > 0, Error::<T>::DocumentsIsEmpty);
        }

        // Get timestamp
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;
        
        // Ensure transaction is not completed
        Self::is_transaction_editable(transaction_id)?;

        // Try mutate transaction data
        <TransactionsInfo<T>>::try_mutate::<_,_,DispatchError,_>(transaction_id, |transaction_data| {
            let mod_transaction_data = transaction_data.as_mut().ok_or(Error::<T>::TransactionNotFound)?;  
            
            // Ensure project is not completed
            Self::is_project_completed(mod_transaction_data.project_id)?;

            // Ensure drawdown is not completed
            Self::is_drawdown_editable(mod_transaction_data.drawdown_id)?;

            // Ensure expenditure exists
            ensure!(ExpendituresInfo::<T>::contains_key(mod_transaction_data.expenditure_id), Error::<T>::ExpenditureNotFound);
            
            if let Some(amount) = amount.clone() {
                mod_transaction_data.amount = amount;
            }
            if let Some(description) = description.clone() {
                let mod_description = description.into_inner();
                mod_transaction_data.description = mod_description[0].clone();
            }
            if let Some(documents) = documents.clone() {
                mod_transaction_data.documents = Some(documents);
            }
            mod_transaction_data.updated_date = timestamp;
            Ok(())
        })?;
        
        //TOREVIEW: Check if this event is needed
        Self::deposit_event(Event::TransactionEdited(transaction_id));

        Ok(())
    }

    fn do_delete_transaction(
        admin: T::AccountId,
        transaction_id: [u8;32]
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        // Ensure transaction exists and get transaction data
        let transaction_data = TransactionsInfo::<T>::get(transaction_id).ok_or(Error::<T>::TransactionNotFound)?;

        // Ensure project is not completed
        Self::is_project_completed(transaction_data.project_id)?;

        // Ensure drawdown is not completed
        ensure!(Self::is_drawdown_editable(transaction_data.drawdown_id).is_ok(), Error::<T>::DrawdownIsAlreadyCompleted);

        // Ensure transaction is not completed
        ensure!(Self::is_transaction_editable(transaction_id).is_ok(), Error::<T>::TransactionIsAlreadyCompleted);

        // Remove transaction from TransactionsByProject
        <TransactionsByProject<T>>::try_mutate::<_,_,DispatchError,_>(transaction_data.project_id, |transactions| {
            transactions.retain(|transaction| transaction != &transaction_id);
            Ok(())
        })?;

        // Remove transaction from TransactionsByDrawdown
        <TransactionsByDrawdown<T>>::try_mutate::<_,_,_,DispatchError,_>(transaction_data.project_id, transaction_data.drawdown_id, |transactions| {
            transactions.retain(|transaction| transaction != &transaction_id);
            Ok(())
        })?;

        // Remove transaction from TransactionsByExpenditure
        <TransactionsByExpenditure<T>>::try_mutate::<_,_,_,DispatchError,_>(transaction_data.project_id, transaction_data.expenditure_id, |transactions| {
            transactions.retain(|transaction| transaction != &transaction_id);
            Ok(())
        })?;

        // Remove transaction from TransactionsInfo
        <TransactionsInfo<T>>::remove(transaction_id);

        //TOREVIEW: Check if this event is needed
        Self::deposit_event(Event::TransactionDeleted(transaction_id));

        Ok(())
    }


    //TODO: create a function to automatically create a drawdown when the project is created
    //TODO: create a function to automatically tracks the drawdown number of each type


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

    /// Get global scope
    pub fn get_global_scope() -> [u8;32] {
        let global_scope = <GlobalScope<T>>::try_get().map_err(|_| Error::<T>::NoneValue).unwrap();
        global_scope
    }


    fn _change_project_status(
        admin: T::AccountId,
        project_id: [u8;32], 
        status: ProjectStatus
    ) -> DispatchResult {
        //ensure admin permissions
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        // Check project status is not completed
        Self::is_project_completed(project_id)?;

        //Mutate project data
        <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
            let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
            project.status = status;
            Ok(())    
        })?;

        Ok(())
    }

    fn add_project_role(
        project_id: [u8;32],
        user: T::AccountId,
        role: ProxyRole,
    ) -> DispatchResult {

        match role {
            ProxyRole::Administrator => {
                return Err(Error::<T>::CannotRegisterAdminRole.into());
            },
            ProxyRole::Developer => {
                //TODO: Fix internal validations
                //TODO: move logic to a helper function to avoid boilerplate

                //Mutate project data
                <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
                    let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
                    match project.developer.as_mut() {
                        Some(developer) => {
                            //developer.iter().find(|&u| *u != user).ok_or(Error::<T>::UserAlreadyAssignedToProject)?;
                            developer.try_push(user.clone()).map_err(|_| Error::<T>::MaxDevelopersPerProjectReached)?;
                        },
                        None => {
                            let devs = project.developer.get_or_insert(BoundedVec::<T::AccountId, T::MaxDevelopersPerProject>::default());
                            devs.try_push(user.clone()).map_err(|_| Error::<T>::MaxDevelopersPerProjectReached)?;
                        }
                    }
                    Ok(())    
                })?;
            },
            ProxyRole::Investor => {
                //Mutate project data
                <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
                    let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
                    match project.investor.as_mut() {
                        Some(investor) => {
                            //investor.iter().find(|&u| *u == user).ok_or(Error::<T>::UserAlreadyAssignedToProject)?;
                            investor.try_push(user.clone()).map_err(|_| Error::<T>::MaxInvestorsPerProjectReached)?;
                        },
                        None => {
                            let investors = project.investor.get_or_insert(BoundedVec::<T::AccountId, T::MaxInvestorsPerProject>::default());
                            investors.try_push(user.clone()).map_err(|_| Error::<T>::MaxInvestorsPerProjectReached)?;
                        }
                    }
                    Ok(())    
                })?;
            },
            ProxyRole::Issuer => {
                //Mutate project data
                <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
                    let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
                    match project.issuer.as_mut() {
                        Some(issuer) => {
                            //issuer.iter().find(|&u| u != &user).ok_or(Error::<T>::UserAlreadyAssignedToProject)?;
                            issuer.try_push(user.clone()).map_err(|_| Error::<T>::MaxIssuersPerProjectReached)?;
                        },
                        None => {
                            let issuers = project.issuer.get_or_insert(BoundedVec::<T::AccountId, T::MaxIssuersPerProject>::default());
                            issuers.try_push(user.clone()).map_err(|_| Error::<T>::MaxIssuersPerProjectReached)?;
                        }
                    }
                    Ok(())    
                })?;
            },
            ProxyRole::RegionalCenter => {
                //Mutate project data
                <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
                    let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
                    match project.regional_center.as_mut() {
                        Some(regional_center) => {
                            //regional_center.iter().find(|&u| u != &user).ok_or(Error::<T>::UserAlreadyAssignedToProject)?;
                            regional_center.try_push(user.clone()).map_err(|_| Error::<T>::MaxRegionalCenterPerProjectReached)?;
                        },
                        None => {
                            let regional_centers = project.regional_center.get_or_insert(BoundedVec::<T::AccountId, T::MaxRegionalCenterPerProject>::default());
                            regional_centers.try_push(user.clone()).map_err(|_| Error::<T>::MaxRegionalCenterPerProjectReached)?;
                        }
                    }
                    Ok(())    
                })?;
            },
        }

        Ok(())
    }

    pub fn remove_project_role(
        project_id: [u8;32],
        user: T::AccountId,
        role: ProxyRole,
    ) -> DispatchResult {

        match role {
            ProxyRole::Administrator => {
                return Err(Error::<T>::CannotRemoveAdminRole.into());
            },
            ProxyRole::Developer => {
                //TODO: Fix internal validations
                //TODO: move logic to a helper function to avoid boilerplate
                //Mutate project data
                <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
                    let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
                    match project.developer.as_mut() {
                        Some(developer) => {
                            //developer.clone().iter().find(|&u| *u == user).ok_or(Error::<T>::UserNotAssignedToProject)?;
                            developer.retain(|u| *u != user);
                        },
                        None => {
                            return Err(Error::<T>::UserNotAssignedToProject.into());
                        }
                    }
                    Ok(())    
                })?;
            },
            ProxyRole::Investor => {
                //Mutate project data
                <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
                    let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
                    match project.investor.as_mut() {
                        Some(investor) => {
                            //investor.iter().find(|&u| *u == user).ok_or(Error::<T>::UserNotAssignedToProject)?;
                            investor.retain(|u| *u != user);
                        },
                        None => {
                            return Err(Error::<T>::UserNotAssignedToProject.into());
                        }
                    }
                    Ok(())    
                })?;
            },
            ProxyRole::Issuer => {
                //Mutate project data
                <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
                    let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
                    match project.issuer.as_mut() {
                        Some(issuer) => {
                            //issuer.iter().find(|&u| *u == user).ok_or(Error::<T>::UserNotAssignedToProject)?;
                            issuer.retain(|u| *u != user);
                        },
                        None => {
                            return Err(Error::<T>::UserNotAssignedToProject.into());
                        }
                    }
                    Ok(())    
                })?;
            },
            ProxyRole::RegionalCenter => {
                //Mutate project data
                <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
                    let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
                    match project.regional_center.as_mut() {
                        Some(regional_center) => {
                            //regional_center.iter().find(|&u| *u == user).ok_or(Error::<T>::UserNotAssignedToProject)?;
                            regional_center.retain(|u| *u != user);
                        },
                        None => {
                            return Err(Error::<T>::UserNotAssignedToProject.into());
                        }
                    }
                    Ok(())    
                })?;
            },
        }
        Ok(())
    }
    
        
    /// This functions performs the following checks:
    /// 
    /// 1. Checks if the user is registered in the system
    /// 2. Checks if the user has the required role from UsersInfo storage
    /// 3. Checks if the user is trying to assign an admin role
    fn check_user_role(
        user: T::AccountId,
        role: ProxyRole,
    ) -> DispatchResult {
        //  Ensure user is registered & get user data
        let user_data = UsersInfo::<T>::get(user.clone()).ok_or(Error::<T>::UserNotRegistered)?;

        // Check if the user role trying to be assigned matches the actual user role from UsersInfo storage
        if user_data.role != role {
            return Err(Error::<T>::UserCannotHaveMoreThanOneRole.into());
        }   

        // Can't assign an admin to a project, admins exists globally 
        if role == ProxyRole::Administrator {
            return Err(Error::<T>::CannotAddAdminRole.into());
        }

        Ok(())
    }

            
    //HERE
    // fn remove_user_role(
    //     user: T::AccountId,
    // ) -> DispatchResult {
    //     // Get user account data
    //     let user_data = UsersInfo::<T>::get(user.clone()).ok_or(Error::<T>::UserNotRegistered)?;

    //     // Check if user already has a role
    //     match user_data.role {
    //         Some(_user_role) => {
    //             //Check how many projects the user has assigned
    //             let projects_by_user = Self::projects_by_user(user.clone()).iter().cloned().collect::<Vec<[u8;32]>>();
                
    //             match projects_by_user.len() {
    //                 1 => {
    //                     // Update user data
    //                     <UsersInfo<T>>::try_mutate::<_,_,DispatchError,_>(user.clone(), |user_data| {
    //                         let user_data = user_data.as_mut().ok_or(Error::<T>::UserNotRegistered)?;
    //                         user_data.role = None;
    //                         Ok(())
    //                     })?;
    //                     //TOREVIEW: Remove ? operator and final Ok(())
    //                     Ok(())
    //                 },
    //                 _ => {
    //                     return Ok(())
    //                 }
    //             }
    //         },
    //         None => {
    //             return Ok(())
    //         }
    //     }
    // }

    fn is_project_completed(
        project_id: [u8;32],
    ) -> DispatchResult {
        // Get project data
        let project_data = ProjectsInfo::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;

        // Ensure project is completed
        ensure!(project_data.status != ProjectStatus::Completed, Error::<T>::ProjectIsAlreadyCompleted);

        Ok(())
    }

    #[allow(dead_code)]
    fn is_drawdown_editable(
        drawdown_id: [u8;32],
    ) -> DispatchResult {
        // Get drawdown data
        let drawdown_data = DrawdownsInfo::<T>::get(drawdown_id).ok_or(Error::<T>::DrawdownNotFound)?;

        // Ensure transaction is in draft or rejected status
        // Match drawdown status
        match drawdown_data.status {
            DrawdownStatus::Draft => {
                return Ok(())
            },
            DrawdownStatus::Rejected => {
                return Ok(())
            },
            _ => {
                return Err(Error::<T>::CannotEditDrawdown.into());
            }
        }
    }

    #[allow(dead_code)]
    fn is_transaction_editable(
        transaction_id: [u8;32],
    ) -> DispatchResult {
        // Get transaction data
        let transaction_data = TransactionsInfo::<T>::get(transaction_id).ok_or(Error::<T>::TransactionNotFound)?;

        // Ensure transaction is in draft or rejected status
        // Match transaction status
        match transaction_data.status {
            TransactionStatus::Draft => {
                return Ok(())
            },
            TransactionStatus::Rejected => {
                return Ok(())
            },
            _ => {
                return Err(Error::<T>::CannotEditTransaction.into());
            }
        }
    }

    //TODO: remove macro when used 
    #[allow(dead_code)]
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

    fn sudo_register_admin(
        admin: T::AccountId,
        name: FieldName,
    ) -> DispatchResult{
        // check if user is already registered
        ensure!(!<UsersInfo<T>>::contains_key(admin.clone()), Error::<T>::UserAlreadyRegistered);
        
        //Get current timestamp
        let current_timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        let user_data = UserData::<T> {
            name,
            role: ProxyRole::Administrator,
            image: CID::default(),
            date_registered: current_timestamp,
            email: FieldName::default(),
            documents: None,
        };

        //Insert user data
        <UsersInfo<T>>::insert(admin.clone(), user_data);
        Ok(())
    }

    fn sudo_delete_admin( admin: T::AccountId ) -> DispatchResult{
        // check if user is already registered
        ensure!(<UsersInfo<T>>::contains_key(admin.clone()), Error::<T>::UserNotRegistered);
        
        //Remove user from UsersInfo storage map
        <UsersInfo<T>>::remove(admin.clone());

        Ok(())
    }

    #[allow(dead_code)]
    fn is_amount_valid(amount: u64,) -> DispatchResult {
        let minimun_amount: u64 = 0;
        ensure!(amount >= minimun_amount, Error::<T>::InvalidAmount);
        Ok(())
    }



}