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
    ) -> DispatchResult{
        let pallet_id = Self::pallet_id();
        let global_scope =  <GlobalScope<T>>::try_get().map_err(|_| Error::<T>::GlobalScopeNotSet)?;

        T::Rbac::assign_role_to_user(
            admin.clone(), 
            pallet_id.clone(), 
            &global_scope, 
            ProxyRole::Administrator.id())?;

        // create a administrator user account
        Self::sudo_register_admin(admin.clone())?;
        
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
		
    pub fn do_create_project(
        admin: T::AccountId, 
        tittle: FieldName,
        description: FieldDescription,
        image: CID,
        adress: FieldName,
        project_type: ProjectType,
        completition_date: u64,
        ) -> DispatchResult {
        //ensure admin permissions 
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Add timestamp 
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        //Create project_id
        //TOREVIEW: We could use only name as project_id or use a method/storagemap to check if the name is already in use
        let project_id = (tittle.clone()).using_encoded(blake2_256);

        //ensure completition date is in the future
        ensure!(completition_date > timestamp, Error::<T>::CompletitionDateMustBeLater);
        
        //Create project data
        let project_data = ProjectData::<T> {
            developer: Some(BoundedVec::<T::AccountId, T::MaxDevelopersPerProject>::default()),
            investor: Some(BoundedVec::<T::AccountId, T::MaxInvestorsPerProject>::default()),
            issuer: Some(BoundedVec::<T::AccountId, T::MaxIssuersPerProject>::default()),
            regional_center: Some(BoundedVec::<T::AccountId, T::MaxRegionalCenterPerProject>::default()),
            tittle,
            description,
            image,
            adress,
            status: ProjectStatus::default(), 
            project_type,
            creation_date: timestamp,
            completition_date,
            updated_date: timestamp,
        };

        // create scope for project_id
        T::Rbac::create_scope(Self::pallet_id(), project_id)?;

        //Insert project data
        // ensure that the project_id is not already in use
        ensure!(!ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectIdAlreadyInUse);
        ProjectsInfo::<T>::insert(project_id, project_data);

        // Match project type, call default expednitures
        match project_type {
            ProjectType::Construction => {
                //Generate its related expenditures
                Self::do_generate_hard_cost_defaults(admin.clone(), project_id)?;
                Self::do_generate_soft_cost_defaults(admin.clone(), project_id)?;
            },
            ProjectType::ConstructionOperation => {
                //Generate its related expenditures
                Self::do_generate_hard_cost_defaults(admin.clone(), project_id)?;
                Self::do_generate_soft_cost_defaults(admin.clone(), project_id)?;
                Self::do_generate_operational_defaults(admin.clone(), project_id)?;
            },
            ProjectType::ConstructionBridge => {
                //Generate its related expenditures
                Self::do_generate_hard_cost_defaults(admin.clone(), project_id)?;
                Self::do_generate_soft_cost_defaults(admin.clone(), project_id)?;
                Self::do_generate_others_defaults(admin.clone(), project_id)?;
            },
            ProjectType::Operation => {
                //Generate its related expenditures
                Self::do_generate_operational_defaults(admin.clone(), project_id)?;
            },
        }

        //TODO: Generate drawdowns
        //Self::do_generate_drawdowns(admin.clone(), project_id)?;

        // Event
        Self::deposit_event(Event::ProjectCreated(admin, project_id));

        Ok(())
    }

    pub fn do_edit_project(
        admin: T::AccountId,
        project_id: [u8;32], 
        tittle: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,	
        description: Option<BoundedVec<FieldDescription, T::MaxBoundedVecs>>,
        image: Option<BoundedVec<CID, T::MaxBoundedVecs>>,
        adress: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>, 
        completition_date: Option<u64>,  
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
            
            if let Some(tittle) = tittle {
                let mod_tittle = tittle.into_inner();
                project.tittle = mod_tittle[0].clone();
            }
            if let Some(description) = description {
                let mod_description = description.into_inner();
                project.description = mod_description[0].clone();
            }
            if let Some(image) = image {
                let mod_image = image.into_inner();
                project.image = mod_image[0].clone();
            }
            if let Some(adress) = adress {
                let mod_adress = adress.into_inner();
                project.adress = mod_adress[0].clone();
            }
            if let Some(completition_date) = completition_date {
                //ensure new completition date is in the future
                ensure!(completition_date > current_timestamp, Error::<T>::CompletitionDateMustBeLater);
                project.completition_date = completition_date;
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
		
    pub fn do_register_user(admin: T::AccountId, user: T::AccountId, name: FieldName, image: CID, email: FieldName, documents: Option<Documents<T>>, ) -> DispatchResult {
        //ensure admin permissions     
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        // check if user is already registered
        ensure!(!<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserAlreadyRegistered);

        //Get current timestamp
        let current_timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        let user_data = UserData::<T> {
            name,
            role: None,
            image,
            date_registered: current_timestamp,
            email,
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
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        // Ensure project is not completed
        Self::is_project_completed(project_id)?;

        //Ensure user is registered
        ensure!(<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserNotRegistered);

        //Ensure user is not already assigned to the project
        ensure!(!<UsersByProject<T>>::get(project_id).contains(&user.clone()), Error::<T>::UserAlreadyAssignedToProject);
        ensure!(!<ProjectsByUser<T>>::get(user.clone()).contains(&project_id), Error::<T>::UserAlreadyAssignedToProject);

        // Ensure user is not assigened to the selected scope (project_id) with the selected role
        ensure!(!T::Rbac::has_role(user.clone(), Self::pallet_id(), &project_id, [role.id()].to_vec()).is_ok(), Error::<T>::UserAlreadyHasRole);

        // Update project data depending on the role assigned
        Self::add_project_role(project_id, user.clone(), role)?;

        //Update user data depending on the role assigned
        Self::add_user_role(user.clone(), role)?;

        //TOREVIEW: this storage map will be removed?
        // Insert project to ProjectsByUser storagemap
        <ProjectsByUser<T>>::try_mutate::<_,_,DispatchError,_>(user.clone(), |projects| {
            projects.try_push(project_id).map_err(|_| Error::<T>::MaxProjectsPerUserReached)?;
            Ok(())
        })?;

        //TOREVIEW: this storage map will be removed?
        // Insert user to UsersByProject storagemap
        <UsersByProject<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |users| {
            users.try_push(user.clone()).map_err(|_| Error::<T>::MaxUsersPerProjectReached)?;
            Ok(())
        })?;

        // Insert user into scope rbac pallet
        T::Rbac::assign_role_to_user(user.clone(), Self::pallet_id(), &project_id, role.id())?;

        //Event 
        Self::deposit_event(Event::UserAssignedToProject(user, project_id));
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

        // Ensure user has roles assigned to the project
        // TODO: catch error and return custom error
        //ensure!(T::Rbac::has_role(user.clone(), Self::pallet_id(), &project_id, [role.id()].to_vec()).is_ok(), Error::<T>::UserDoesNotHaveRole);
        T::Rbac::has_role(user.clone(), Self::pallet_id(), &project_id, [role.id()].to_vec())?;

        // Update project data depending on the role unassigned
        Self::remove_project_role(project_id, user.clone(), role)?;

        // Update user data depending on the role unassigned
        Self::remove_user_role(user.clone())?;

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

        //Ensure user is registered & get user data
        let user_data = UsersInfo::<T>::get(user.clone()).ok_or(Error::<T>::UserNotRegistered)?;
        
        //Prevent users from deleting an administator
        if let Some(admin_role) = user_data.role{
            ensure!(admin_role != ProxyRole::Administrator, Error::<T>::CannotRemoveAdminRole);
        }

        // Can not delete an user if it has assigned projects
        let projects_by_user = Self::projects_by_user(user.clone()).iter().cloned().collect::<Vec<[u8;32]>>();

        if projects_by_user.len() == 0 {
            // Remove user from UsersInfo storagemap
            <UsersInfo<T>>::remove(user.clone());

            // Remove user from UsersByProject storagemap
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
    pub fn do_create_expenditure(
        admin: T::AccountId,
        project_id: [u8;32], 
        name: FieldName,
        expenditure_type: ExpenditureType,
        budget_amount: Option<u64>,
        naics_code: Option<u32>,
        jobs_multiplier: Option<u32>,
    ) -> DispatchResult {
        //ensure admin permissions 
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        //Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        // Ensure project is not completed
        Self::is_project_completed(project_id)?;

        //TOREVIEW: ensure field name is not empty
        ensure!(name.len() > 0, Error::<T>::FieldNameCannotBeEmpty);

        //Create expenditure_id
        let expenditure_id = (project_id, name.clone()).using_encoded(blake2_256);

        //TODO: check budget amount if valid

        // Create expenditure data
        let expenditure_data = ExpenditureData {
            name,
            project_id,
            expenditure_type,
            balance: 0,
            naics_code,
            jobs_multiplier,
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
        match budget_amount {
            Some(amount) => {
                Self::do_create_budget(admin, expenditure_id, amount, project_id)?;
            },
            None => {
                Self::do_create_budget(admin, expenditure_id, 0, project_id)?;
            },
        }

        Self::deposit_event(Event::ExpenditureCreated(expenditure_id));
        Ok(())
    }

    pub fn do_edit_expenditure(
        admin: T::AccountId,
        project_id: [u8;32], 
        expenditure_id: [u8;32],
        name: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>, 
        budget_amount: Option<u64>,
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
            if let Some(budget_amount) = budget_amount {
                //get budget id
                let budget_id = Self::get_budget_id(project_id, expenditure_id)?;
                // Edit budget amount
                Self::do_edit_budget(admin.clone(), budget_id, budget_amount)?;
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
        Self::do_delete_budget(admin, project_id, expenditure_id)?;

        Self::deposit_event(Event::ExpenditureDeleted(expenditure_id));
        Ok(())
    }



    // B U D G E T S
    // --------------------------------------------------------------------------------------------
    // Buget functions are not exposed to the public. They are only used internally by the module.
    fn do_create_budget(
        admin: T::AccountId,
        expenditure_id: [u8;32],
        amount: u64,
        project_id: [u8;32],
    ) -> DispatchResult {
        //TODO: ensure admin & developer permissions
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        // Ensure expenditure_id exists 
        ensure!(<ExpendituresInfo<T>>::contains_key(expenditure_id), Error::<T>::ExpenditureNotFound);

        //TODO: balance check

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

    fn do_edit_budget(
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

        //TOREVIEW: Check if an event is needed

        Ok(())
    }

    fn do_delete_budget(
        admin: T::AccountId,
        project_id: [u8;32],
        expenditure_id: [u8;32],
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        // Get budget id
        let budget_id = Self::get_budget_id(project_id, expenditure_id)?;

        // Remove budget data
        <BudgetsInfo<T>>::remove(budget_id);

        //TOREVIEW: Check budget id is deleted
        // Delete budget_id from BudgetsByProject
        <BudgetsByProject<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |budgets| {
            budgets.retain(|budget| budget != &budget_id);
            Ok(())
        })?;
        
        //TOREVIEW: Check if an event is needed

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
            amount,
            description,
            created_date: timestamp,
            updated_date: timestamp,
            documents,
        };

        //TODO: Update drawdown with this transaction id

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

    // update()
    // edit()
    // delete()


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
    

    fn add_user_role(
        user: T::AccountId,
        role: ProxyRole,
    ) -> DispatchResult {
        // Get user account data
        let user_data = UsersInfo::<T>::get(user.clone()).ok_or(Error::<T>::UserNotRegistered)?;

        // Check if user already has a role
        match user_data.role {
            Some(user_role) => {
                //TODO: Ccheck what role is the user trying to add
                if user_role == role {
                    return Ok(())
                } else {
                    return Err(Error::<T>::UserCannotHaveMoreThanOneRole.into());
                }
            },
            None => {
                match role {
                    ProxyRole::Administrator => {
                        return Err(Error::<T>::CannotAddAdminRole.into());
                    },
                    _ => {
                        // Update user data
                        <UsersInfo<T>>::try_mutate::<_,_,DispatchError,_>(user.clone(), |user_data| {
                            let user_data = user_data.as_mut().ok_or(Error::<T>::UserNotRegistered)?;
                            user_data.role = Some(role);
                            Ok(())
                        })?;
                        //TOREVIEW: Remove ? operator and final Ok(())
                        Ok(())
                    },
                }
            }
        }
    }

    fn remove_user_role(
        user: T::AccountId,
    ) -> DispatchResult {
        // Get user account data
        let user_data = UsersInfo::<T>::get(user.clone()).ok_or(Error::<T>::UserNotRegistered)?;

        // Check if user already has a role
        match user_data.role {
            Some(_user_role) => {
                //Check how many projects the user has assigned
                let projects_by_user = Self::projects_by_user(user.clone()).iter().cloned().collect::<Vec<[u8;32]>>();
                
                match projects_by_user.len() {
                    1 => {
                        // Update user data
                        <UsersInfo<T>>::try_mutate::<_,_,DispatchError,_>(user.clone(), |user_data| {
                            let user_data = user_data.as_mut().ok_or(Error::<T>::UserNotRegistered)?;
                            user_data.role = None;
                            Ok(())
                        })?;
                        //TOREVIEW: Remove ? operator and final Ok(())
                        Ok(())
                    },
                    _ => {
                        return Ok(())
                    }
                }
            },
            None => {
                return Ok(())
            }
        }
    }

    fn is_project_completed(
        project_id: [u8;32],
    ) -> DispatchResult {
        // Get project data
        let project_data = ProjectsInfo::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;

        // Ensure project is completed
        ensure!(project_data.status != ProjectStatus::Completed, Error::<T>::CannotEditCompletedProject);

        Ok(())
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

    fn sudo_register_admin( admin: T::AccountId ) -> DispatchResult{
        // check if user is already registered
        ensure!(!<UsersInfo<T>>::contains_key(admin.clone()), Error::<T>::UserAlreadyRegistered);
        
        //Get current timestamp
        let current_timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        let user_data = UserData::<T> {
            name: FieldName::default(),
            role: Some(ProxyRole::Administrator),
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


    /// Generate Hard Cost default expenditures
    fn do_generate_hard_cost_defaults(
        admin: T::AccountId,
        project_id: [u8;32],
    ) -> DispatchResult{
        //ensure admin permissions
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        // Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        let hard_cost_expenditures = vec![
            "Construction".as_bytes().to_vec(),
            "Furniture, Fixtures & Allowance".as_bytes().to_vec(),
            "Hard Cost contingency & Allowance".as_bytes().to_vec(),
        ];

        //Create default expenditures
        for name in hard_cost_expenditures {
            Self::do_create_expenditure(
                admin.clone(), 
                project_id, 
                FieldName::try_from(name).map_err(|_| Error::<T>::NameTooLong)?,
                ExpenditureType::HardCost,
                None,
                None,
                None,
            )?;
        }

        Ok(())
    }

    /// Generate Soft Cost default expenditures
    fn do_generate_soft_cost_defaults(
        admin: T::AccountId,
        project_id: [u8;32],
    ) -> DispatchResult{
        //ensure admin permissions
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        // Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);
        
        let soft_cost_expenditures = vec![
            "Architect & Design".as_bytes().to_vec(),
            "Building Permits & Impact Fees".as_bytes().to_vec(),
            "Developer Reimbursable".as_bytes().to_vec(),
            "Builder Risk Insurance".as_bytes().to_vec(),
            "Environment / Soils / Survey".as_bytes().to_vec(),
            "Testing & Inspections".as_bytes().to_vec(),
            "Legal & Professional".as_bytes().to_vec(),
            "Real Estate Taxes & Owner's Liability Insurance".as_bytes().to_vec(),
            "Pre - Development Fee".as_bytes().to_vec(),
            "Equity Management Fee".as_bytes().to_vec(),
            "Bank Origination Fee".as_bytes().to_vec(),
            "Lender Debt Placement Fee".as_bytes().to_vec(),
            "Title, Appraisal, Feasibility, Plan Review & Closing".as_bytes().to_vec(),
            "Interest Carry during Construction".as_bytes().to_vec(),
            "Ops Stabilization & Interest Carry Reserve".as_bytes().to_vec(),
            "Sales & Marketing".as_bytes().to_vec(),
            "Pre - Opening Expenses".as_bytes().to_vec(),
            "Contingency".as_bytes().to_vec(),
        ];

        //Create default expenditures
        for name in soft_cost_expenditures {
            Self::do_create_expenditure(
                admin.clone(), 
                project_id, 
                FieldName::try_from(name).map_err(|_| Error::<T>::NameTooLong)?,
                ExpenditureType::SoftCost,
                None,
                None,
                None,
            )?;
        }

        Ok(())
    }

    /// Generate Operational default expenditures
    fn do_generate_operational_defaults(
        admin: T::AccountId,
        project_id: [u8;32],
    ) -> DispatchResult{
        //ensure admin permissions
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        // Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        let operational_expenditures = vec![
            "Operational default 1".as_bytes().to_vec(),
            "Operational default 2".as_bytes().to_vec(),
            "Operational default 3".as_bytes().to_vec(),
        ];

        //Create default expenditures
        for name in operational_expenditures {
            Self::do_create_expenditure(
                admin.clone(), 
                project_id, 
                FieldName::try_from(name).map_err(|_| Error::<T>::NameTooLong)?,
                ExpenditureType::Operational,
                None,
                None,
                None,
            )?;
        }

        Ok(())
    }

    /// Generate Other Costs default expenditures
    fn do_generate_others_defaults(
        admin: T::AccountId,
        project_id: [u8;32],
    ) -> DispatchResult{
        //ensure admin permissions
        Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        // Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        let hard_cost_expenditures = vec![
            "Others default 1".as_bytes().to_vec(),
            "Others default 2".as_bytes().to_vec(),
            "Others default 3".as_bytes().to_vec(),
        ];

        //Create default expenditures
        for name in hard_cost_expenditures {
            Self::do_create_expenditure(
                admin.clone(), 
                project_id, 
                FieldName::try_from(name).map_err(|_| Error::<T>::NameTooLong)?,
                ExpenditureType::Others,
                None,
                None,
                None,
            )?;
        }

        Ok(())
    }

    fn is_amount_valid(amount: u64,) -> DispatchResult {
        let minimun_amount: u64 = 0;
        ensure!(amount >= minimun_amount, Error::<T>::InvalidAmount);
        Ok(())
    }



}