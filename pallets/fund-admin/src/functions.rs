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

    // I N I T I A L   S E T U P
    // --------------------------------------------------------------------------------------------

    pub fn do_initial_setup() -> DispatchResult{
        // Create a global scope for the administrator role
        let pallet_id = Self::pallet_id();
        let global_scope = pallet_id.using_encoded(blake2_256);
        <GlobalScope<T>>::put(global_scope);
        T::Rbac::create_scope(Self::pallet_id(), global_scope)?;

        //Admin rol & permissions
        let administrator_role_id = T::Rbac::create_and_set_roles(pallet_id.clone(), [ProxyRole::Administrator.to_vec()].to_vec())?;
        T::Rbac::create_and_set_permissions(pallet_id.clone(), administrator_role_id[0], ProxyPermission::administrator_permissions())?;

        //Builder rol & permissions
        let builder_role_id = T::Rbac::create_and_set_roles(pallet_id.clone(), [ProxyRole::Builder.to_vec()].to_vec())?;
        T::Rbac::create_and_set_permissions(pallet_id.clone(), builder_role_id[0], ProxyPermission::builder_permissions())?;

        // Investor rol & permissions
        let investor_role_id = T::Rbac::create_and_set_roles(pallet_id.clone(), [ProxyRole::Investor.to_vec()].to_vec())?;
        T::Rbac::create_and_set_permissions(pallet_id.clone(), investor_role_id[0], ProxyPermission::investor_permissions())?;

        // Issuer rol & permissions
        let issuer_role_id = T::Rbac::create_and_set_roles(pallet_id.clone(), [ProxyRole::Issuer.to_vec()].to_vec())?;
        T::Rbac::create_and_set_permissions(pallet_id.clone(), issuer_role_id[0], ProxyPermission::issuer_permissions())?;

        // Regional center rol & permissions
        let regional_center_role_id = T::Rbac::create_and_set_roles(pallet_id.clone(), [ProxyRole::RegionalCenter.to_vec()].to_vec())?;
        T::Rbac::create_and_set_permissions(pallet_id.clone(), regional_center_role_id[0], ProxyPermission::regional_center_permissions())?;

        Self::deposit_event(Event::ProxySetupCompleted);
        Ok(())
    }

    pub fn do_sudo_add_administrator(
        admin: T::AccountId,
        name: FieldName,
    ) -> DispatchResult{
        // create a administrator user account & register it in the rbac pallet
        Self::sudo_register_admin(admin.clone(), name)?;

        Self::deposit_event(Event::AdministratorAssigned(admin));
        Ok(())
    }

    pub fn do_sudo_remove_administrator(
        admin: T::AccountId,
    ) -> DispatchResult{
        // remove administrator user account & remove it from the rbac pallet
        Self::sudo_delete_admin(admin.clone())?;

        Self::deposit_event(Event::AdministratorRemoved(admin));
        Ok(())
    }


    // P R O J E C T S
    // --------------------------------------------------------------------------------------------

    /// Create a new project
    /// - only administrator can create a new project
    /// Expenditures: (name, type, amount, naics code, jobs multiplier, CUDAction, expenditure_id)
    /// users = (accountid, role)
    pub fn do_create_project(
        admin: T::AccountId,
        title: FieldName,
        description: FieldDescription,
        image: Option<CID>,
        address: FieldName,
        banks: Option<BoundedVec<(BankName, BankAddress), T::MaxBanksPerProject>>,
        creation_date: CreationDate,
        completion_date: CompletionDate,
        expenditures: BoundedVec<(
            Option<FieldName>,
            Option<ExpenditureType>,
            Option<ExpenditureAmount>,
            Option<NAICSCode>,
            Option<JobsMultiplier>,
            CUDAction,
            Option<ExpenditureId>,
        ), T::MaxRegistrationsAtTime>,
        job_eligibles: Option<BoundedVec<(
            Option<FieldName>, // name
            Option<JobEligibleAmount>, // amount
            Option<NAICSCode>, // naics code
            Option<JobsMultiplier>, // jobs multiplier
            CUDAction, // action
            Option<JobEligibleId>, // job_eligible_id
        ), T::MaxRegistrationsAtTime>>,
        users: Option<BoundedVec<(
            T::AccountId,
            ProxyRole,
            AssignAction,
        ), T::MaxRegistrationsAtTime>>,
        ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_authorized(admin.clone(), &Self::get_global_scope(), ProxyPermission::CreateProject)?;

        //Add timestamp
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        //Create project_id
        //TOREVIEW: We could use only name as project_id or use a method/storagemap to check if the name is already in use
        let project_id: ProjectId = (title.clone(), timestamp).using_encoded(blake2_256);

        //ensure completion_date is in the future
        ensure!(completion_date > creation_date, Error::<T>::CompletionDateMustBeLater);

        //Create project data
        let project_data = ProjectData::<T> {
            builder: Some(BoundedVec::<T::AccountId, T::MaxBuildersPerProject>::default()),
            investor: Some(BoundedVec::<T::AccountId, T::MaxInvestorsPerProject>::default()),
            issuer: Some(BoundedVec::<T::AccountId, T::MaxIssuersPerProject>::default()),
            regional_center: Some(BoundedVec::<T::AccountId, T::MaxRegionalCenterPerProject>::default()),
            title,
            description,
            image,
            address,
            status: ProjectStatus::default(),
            inflation_rate: None,
            banks,
            registration_date: timestamp,
            creation_date,
            completion_date,
            updated_date: timestamp,
			construction_loan_drawdown_status: None,
			developer_equity_drawdown_status: None,
			eb5_drawdown_status: None,
        };

        // create scope for project_id
        T::Rbac::create_scope(Self::pallet_id(), project_id)?;

        //Insert project data
        // ensure that the project_id is not already in use
        ensure!(!ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectIdAlreadyInUse);
        ProjectsInfo::<T>::insert(project_id, project_data);

        //Add expenditures
        Self::do_execute_expenditures(admin.clone(), project_id, expenditures)?;

        //Add job_eligibles
        if let Some(mod_job_eligibles) = job_eligibles {
            Self::do_execute_job_eligibles(admin.clone(), project_id, mod_job_eligibles)?;
        }

        // Add users
        if let Some(mod_users) = users {
            Self::do_execute_assign_users(admin.clone(), project_id, mod_users)?;
        }

        //Initialize drawdowns
        Self::do_initialize_drawdowns(admin.clone(), project_id)?;

        // Event
        Self::deposit_event(Event::ProjectCreated(admin, project_id));

        Ok(())
    }

    pub fn do_edit_project(
        admin: T::AccountId,
        project_id: ProjectId,
        title: Option<FieldName>,
        description: Option<FieldDescription>,
        image: Option<CID>,
        address: Option<FieldName>,
        banks: Option<BoundedVec<(BankName, BankAddress), T::MaxBanksPerProject>>,
        creation_date: Option<CreationDate>,
        completion_date: Option<CompletionDate>,
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_authorized(admin.clone(), &Self::get_global_scope(), ProxyPermission::EditProject)?;

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
                project.title = title;
            }
            if let Some(description) = description {
                project.description = description;
            }
            if let Some(image) = image {
                project.image = Some(image);
            }
            if let Some(address) = address {
                project.address = address;
            }
            if let Some(banks) = banks {
                project.banks = Some(banks);
            }
            if let Some(creation_date) = creation_date {
                project.creation_date = creation_date;
            }
            if let Some(completion_date) = completion_date {
                //ensure new completion_date date is in the future
                //ensure!(completion_date > current_timestamp, Error::<T>::CompletionDateMustBeLater);
                project.completion_date = completion_date;
            }
            // Update modified date
            project.updated_date = current_timestamp;

            Ok(())
        })?;

        //Ensure completion_date is later than creation_date
        Self::is_project_completion_date_later(project_id)?;

        // Event
        Self::deposit_event(Event::ProjectEdited(project_id));
        Ok(())
    }

    pub fn do_delete_project(
        admin: T::AccountId,
        project_id: ProjectId,
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_authorized(admin.clone(), &Self::get_global_scope(), ProxyPermission::DeleteProject)?;

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

        // Delete expenditures from ExpendituresInfo storagemap
        let expenditures_by_project = Self::expenditures_by_project(project_id).iter().cloned().collect::<Vec<[u8;32]>>();
        for expenditure_id in expenditures_by_project {
            <ExpendituresInfo<T>>::remove(expenditure_id);
        }

        // Deletes all expenditures from ExpendituresByProject storagemap
        <ExpendituresByProject<T>>::remove(project_id);

        let drawdowns_by_project = Self::drawdowns_by_project(project_id).iter().cloned().collect::<Vec<[u8;32]>>();
        for drawdown_id in drawdowns_by_project {
            // Delete transactions from TransactionsInfo storagemap
            let transactions_by_drawdown = Self::transactions_by_drawdown(project_id, drawdown_id).iter().cloned().collect::<Vec<[u8;32]>>();
            for transaction_id in transactions_by_drawdown {
                <TransactionsInfo<T>>::remove(transaction_id);
            }

            // Deletes all transactions from TransactionsByDrawdown storagemap
            <TransactionsByDrawdown<T>>::remove(project_id, drawdown_id);

            // Delete drawdown from DrawdownsInfo storagemap
            <DrawdownsInfo<T>>::remove(drawdown_id);

        }

        // Deletes all drawdowns from DrawdownsByProject storagemap
        <DrawdownsByProject<T>>::remove(project_id);

        //Event
        Self::deposit_event(Event::ProjectDeleted(project_id));
        Ok(())
    }

    pub fn do_execute_assign_users(
        admin: T::AccountId,
        project_id: ProjectId,
        users: BoundedVec<(
            T::AccountId,
            ProxyRole,
            AssignAction,
        ), T::MaxRegistrationsAtTime>,
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_authorized(admin.clone(), &Self::get_global_scope(), ProxyPermission::AssignUser)?;

        // Ensure project exists & is not completed
        Self::is_project_completed(project_id)?;

        //Assign users
        for user in users {
            match user.2 {
                AssignAction::Assign => {
                    Self::do_assign_user(project_id, user.0, user.1)?;
                },
                AssignAction::Unassign => {
                    Self::do_unassign_user(project_id, user.0, user.1)?;
                },
            }

        }

        // Event
        Self::deposit_event(Event::UsersAssignationExecuted(project_id));
        Ok(())
    }

    fn do_assign_user(
        project_id: ProjectId,
        user: T::AccountId,
        role: ProxyRole,
    ) -> DispatchResult {
        // Basic validations prior to assign user
        Self::check_user_role(user.clone(), role)?;

        // Ensure user is not already assigned to the project
        ensure!(!<UsersByProject<T>>::get(project_id).contains(&user), Error::<T>::UserAlreadyAssignedToProject);
        ensure!(!<ProjectsByUser<T>>::get(user.clone()).contains(&project_id), Error::<T>::UserAlreadyAssignedToProject);

        // Ensure user is not assigened to the selected scope (project_id) with the selected role
        ensure!(!T::Rbac::has_role(user.clone(), Self::pallet_id(), &project_id, [role.id()].to_vec()).is_ok(), Error::<T>::UserAlreadyAssignedToProject);

        // Update project data depending on the role assigned
        Self::add_project_role(project_id, user.clone(), role)?;

        // Insert project to ProjectsByUser storagemap
        <ProjectsByUser<T>>::try_mutate::<_,_,DispatchError,_>(user.clone(), |projects| {
            projects.try_push(project_id).map_err(|_| Error::<T>::MaxProjectsPerUserReached)?;
            Ok(())
        })?;

        // Insert user to UsersByProject storagemap
        <UsersByProject<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |users| {
            users.try_push(user.clone()).map_err(|_| Error::<T>::MaxUsersPerProjectReached)?;
            Ok(())
        })?;

        // Insert user into scope rbac pallet
        T::Rbac::assign_role_to_user(user.clone(), Self::pallet_id(), &project_id, role.id())?;

        //Event
        Self::deposit_event(Event::UsersAssignationCompleted(project_id));
        Ok(())
    }

    fn do_unassign_user(
        project_id: ProjectId,
        user: T::AccountId,
        role: ProxyRole,
    ) -> DispatchResult {
        //Ensure user is registered
        ensure!(<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserNotRegistered);

        //Ensure user is assigned to the project
        ensure!(<UsersByProject<T>>::get(project_id).contains(&user.clone()), Error::<T>::UserNotAssignedToProject);
        ensure!(<ProjectsByUser<T>>::get(user.clone()).contains(&project_id), Error::<T>::UserNotAssignedToProject);

        // Ensure user has the specified role assigned in the selected project
        ensure!(T::Rbac::has_role(user.clone(), Self::pallet_id(), &project_id, [role.id()].to_vec()).is_ok(), Error::<T>::UserDoesNotHaveRole);

        // Update project data depending on the role unassigned
        Self::remove_project_role(project_id, user.clone(), role)?;

        // Remove user from UsersByProject storagemap
        <UsersByProject<T>>::mutate(project_id, |users| {
            users.retain(|u| u != &user);
        });

        // Remove user from ProjectsByUser storagemap
        <ProjectsByUser<T>>::mutate(user.clone(), |projects| {
            projects.retain(|p| p != &project_id);
        });

        // Remove user from the scope rbac pallet
        T::Rbac::remove_role_from_user(user.clone(), Self::pallet_id(), &project_id, role.id())?;

        Self::deposit_event(Event::UsersUnassignationCompleted(project_id));
        Ok(())
    }


    // U S E R S
    // --------------------------------------------------------------------------------------------
    pub fn do_execute_users(
        admin: T::AccountId,
        users: BoundedVec<(
            T::AccountId, // 0:account id
            Option<FieldName>, // name
            Option<ProxyRole>, // 2:role
            CUDAction, // 3:action
        ), T::MaxRegistrationsAtTime>,
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_authorized(admin.clone(), &Self::get_global_scope(), ProxyPermission::RegisterUser)?;

        for user in users{
            match user.3 {
                CUDAction::Create => {
                    // Create user only needs: account id, name and role
                    Self::do_create_user(
                        user.0.clone(),
                        user.1.clone().ok_or(Error::<T>::UserNameRequired)?,
                        user.2.clone().ok_or(Error::<T>::UserRoleRequired)?,
                    )?;
                },
                CUDAction::Update => {
                    // Update user only needs: account id, name and role
                    Self::do_update_user(
                        user.0.clone(),
                        user.1.clone(),
                        user.2.clone()
                    )?;
                },
                CUDAction::Delete => {
                    // Ensure admin cannot delete themselves
                    ensure!(user.0 != admin, Error::<T>::AdministratorsCannotDeleteThemselves,
                    );

                    Self::do_delete_user(
                        user.0.clone()
                    )?;
                },
            }
        }

        // Event
        Self::deposit_event(Event::UsersExecuted);
        Ok(())
    }


    fn do_create_user(
        user: T::AccountId,
        name: FieldName,
        role: ProxyRole,
    ) -> DispatchResult {
        //Get current timestamp
        let current_timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        // Ensure user is not registered
        ensure!(!<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserAlreadyRegistered);

        //Ensure name is not empty
        ensure!(!name.is_empty(), Error::<T>::UserNameRequired);

        match role {
            ProxyRole::Administrator => {
                Self::do_sudo_add_administrator(user.clone(), name.clone())?;
            },
            _ => {
                // Create user data
                let user_data = UserData::<T> {
                    name: name.clone(),
                    role,
                    image: CID::default(),
                    date_registered: current_timestamp,
                    email: FieldName::default(),
                    documents: None,
                };

                // Insert user data
                <UsersInfo<T>>::insert(user.clone(), user_data);
                Self::deposit_event(Event::UserAdded(user.clone()));
            },
        }

        Ok(())
    }

    fn do_update_user(
        user: T::AccountId,
        name: Option<FieldName>, // name
        role: Option<ProxyRole>,
    ) -> DispatchResult {
        // Ensure user is registered
        ensure!(<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserNotRegistered);

        // Update user data
        <UsersInfo<T>>::try_mutate::<_,_,DispatchError,_>(user.clone(), |user_data| {
            let user_info = user_data.as_mut().ok_or(Error::<T>::UserNotRegistered)?;

            if let Some(mod_name) = name {
                user_info.name = mod_name.clone();
            }
            if let Some(mod_role) = role {
                // If user has assigned projects cannot update role
                ensure!(<ProjectsByUser<T>>::get(user.clone()).is_empty(), Error::<T>::UserHasAssignedProjectsCannotUpdateRole);

                user_info.role = mod_role;
            }
            Ok(())
        })?;

        Self::deposit_event(Event::UserUpdated(user));
        Ok(())
    }

    fn do_delete_user(
        user: T::AccountId,
    ) -> DispatchResult {
        //Ensure user is registered & get user data
        let user_data = <UsersInfo<T>>::get(user.clone()).ok_or(Error::<T>::UserNotRegistered)?;

        match user_data.role {
            ProxyRole::Administrator => {
                Self::do_sudo_remove_administrator(user.clone())?;
            },
            _ => {
                // Can not delete a user if the user has assigned projects
                let projects_by_user = <ProjectsByUser<T>>::get(user.clone());
                ensure!(projects_by_user.is_empty(), Error::<T>::UserHasAssignedProjectsCannotDelete);

                // Remove user from UsersInfo storage map
                <UsersInfo<T>>::remove(user.clone());

                // Remove user from ProjectsByUser storage map
                <ProjectsByUser<T>>::remove(user.clone());

                // Remove user from UsersByProject storage map
                for project in projects_by_user {
                    <UsersByProject<T>>::try_mutate::<_,_,DispatchError,_>(project, |users| {
                        users.retain(|u| u != &user);
                        Ok(())
                    })?;
                }

                Self::deposit_event(Event::UserDeleted(user.clone()));
            },
        }

        Ok(())

    }

    pub fn do_edit_user(
        user: T::AccountId,
        name: Option<FieldName>,
        image: Option<CID>,
        email: Option<FieldName>,
        documents: Option<Documents<T>>,
    ) -> DispatchResult {
        //Ensure user is registered
        ensure!(<UsersInfo<T>>::contains_key(user.clone()), Error::<T>::UserNotRegistered);

        //Update user data
        <UsersInfo<T>>::try_mutate::<_,_,DispatchError,_>(user.clone(), |user_data| {
            let user_info = user_data.as_mut().ok_or(Error::<T>::UserNotRegistered)?;

            if let Some(name) = name {
                user_info.name = name.clone();
            }
            if let Some(image) = image {
                user_info.image = image.clone();
            }
            if let Some(email) = email {
                user_info.email = email.clone();
            }
            if let Some(documents) = documents {
                // Ensure user is an investor
                ensure!(user_info.role == ProxyRole::Investor, Error::<T>::UserIsNotAnInvestor);
                user_info.documents = Some(documents);
            }
            Ok(())
        })?;

        Self::deposit_event(Event::UserUpdated(user));

        Ok(())
    }

    // B U D G E T  E X P E N D I T U R E
    // --------------------------------------------------------------------------------------------

    // Expenditures: (name, type, amount, naics code, jobs multiplier, CUDAction, expenditure_id)
    pub fn do_execute_expenditures(
        admin: T::AccountId,
        project_id: ProjectId,
        expenditures: BoundedVec<(
            Option<FieldName>, // 0: name
            Option<ExpenditureType>, // 1: type
            Option<ExpenditureAmount>, // 2: amount
            Option<NAICSCode>, // 3: naics code
            Option<JobsMultiplier>, // 4: jobs multiplier
            CUDAction, // 5: CUDAction
            Option<ExpenditureId>, // 6: expenditure_id
        ), T::MaxRegistrationsAtTime>,
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_authorized(admin.clone(), &Self::get_global_scope(), ProxyPermission::Expenditures)?;

        // Ensure project exists
        ensure!(<ProjectsInfo<T>>::contains_key(project_id), Error::<T>::ProjectNotFound);

        // Ensure expenditures are not empty
        ensure!(!expenditures.is_empty(), Error::<T>::EmptyExpenditures);

        for expenditure in expenditures {
            match expenditure.5 {
                CUDAction::Create => {
                    // Create expenditure only needs: name, type, amount, naics code, jobs multiplier
                    Self::do_create_expenditure(
                        project_id,
                        expenditure.0.ok_or(Error::<T>::ExpenditureNameRequired)?,
                        expenditure.1.ok_or(Error::<T>::ExpenditureTypeRequired)?,
                        expenditure.2.ok_or(Error::<T>::ExpenditureAmountRequired)?,
                        expenditure.3,
                        expenditure.4,
                    )?;
                },
                CUDAction::Update => {
                    // Update expenditure only needs: expenditure_id, name, amount, naics code, jobs multiplier
                    Self::do_update_expenditure(
                        project_id,
                        expenditure.6.ok_or(Error::<T>::ExpenditureIdRequired)?,
                        expenditure.0,
                        expenditure.2,
                        expenditure.3,
                        expenditure.4,
                    )?;
                },
                CUDAction::Delete => {
                    // Delete expenditure only needs: expenditure_id
                    Self::do_delete_expenditure(
                        expenditure.6.ok_or(Error::<T>::ExpenditureIdRequired)?,
                    )?;
                },
            }
        }



        Ok(())
    }


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
    fn do_create_expenditure(
        project_id: [u8;32],
        name: FieldName,
        expenditure_type: ExpenditureType,
        expenditure_amount: ExpenditureAmount,
        naics_code: Option<NAICSCode>,
        jobs_multiplier: Option<JobsMultiplier>,
    ) -> DispatchResult {
        // Ensure project exists & is not completed
        Self::is_project_completed(project_id)?;

        // Get timestamp
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        //Ensure expenditure name is not empty
        ensure!(!name.is_empty(), Error::<T>::EmptyExpenditureName);

        // Create expenditure id
        let expenditure_id: ExpenditureId = (project_id, name.clone(), expenditure_type, timestamp).using_encoded(blake2_256);

        // Create expenditure data
        let expenditure_data = ExpenditureData {
            project_id,
            name,
            expenditure_type,
            expenditure_amount,
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

        Self::deposit_event(Event::ExpenditureCreated);
        Ok(())
    }

    fn do_update_expenditure(
        project_id: ProjectId,
        expenditure_id: ExpenditureId,
        name: Option<FieldName>,
        expenditure_amount: Option<ExpenditureAmount>,
        naics_code: Option<NAICSCode>,
        jobs_multiplier: Option<JobsMultiplier>,
    ) -> DispatchResult {
        // Ensure project exists & is not completed
        Self::is_project_completed(project_id)?;

        // Ensure expenditure_id exists
        ensure!(<ExpendituresInfo<T>>::contains_key(expenditure_id), Error::<T>::ExpenditureNotFound);

        // Mutate expenditure data
        <ExpendituresInfo<T>>::try_mutate::<_,_,DispatchError, _>(expenditure_id, |expenditure_data| {
            let expenditure = expenditure_data.as_mut().ok_or(Error::<T>::ExpenditureNotFound)?;

            // Ensure expenditure belongs to the project
            ensure!(expenditure.project_id == project_id, Error::<T>::ExpenditureDoesNotBelongToProject);

            if let Some(mod_name) = name {
                expenditure.name = mod_name;
            }
            if let Some(mod_expenditure_amount) = expenditure_amount {
                expenditure.expenditure_amount = mod_expenditure_amount;
            }
            if let Some(mod_naics_code) = naics_code {
                expenditure.naics_code = Some(mod_naics_code.clone());
            }
            if let Some(mod_jobs_multiplier) = jobs_multiplier {
                expenditure.jobs_multiplier = Some(mod_jobs_multiplier);
            }

            Ok(())
        })?;

        Self::deposit_event(Event::ExpenditureUpdated(expenditure_id));
        Ok(())
    }

    fn do_delete_expenditure(
        expenditure_id: ExpenditureId,
    ) -> DispatchResult {
        // Ensure expenditure_id exists & get expenditure data
        let expenditure_data = <ExpendituresInfo<T>>::get(expenditure_id).ok_or(Error::<T>::ExpenditureNotFound)?;

        // Delete expenditure data from ExpendituresInfo
        <ExpendituresInfo<T>>::remove(expenditure_id);

        // Delete expenditure_id from ExpendituresByProject
        <ExpendituresByProject<T>>::try_mutate::<_,_,DispatchError,_>(expenditure_data.project_id, |expenditures| {
            expenditures.retain(|expenditure| expenditure != &expenditure_id);
            Ok(())
        })?;

        Self::deposit_event(Event::ExpenditureDeleted(expenditure_id));
        Ok(())
    }

    // D R A W D O W N S
    // --------------------------------------------------------------------------------------------
    // For now drawdowns functions are private, but in the future they may be public
    fn do_create_drawdown(
        project_id: ProjectId,
        drawdown_type: DrawdownType,
        drawdown_number: DrawdownNumber,
    ) -> DispatchResult {
        // TOOD: Ensure builder permissions
        //Self::is_superuser(admin.clone(), &Self::get_global_scope(), ProxyRole::Administrator.id())?;

        // Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        // Get timestamp
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        // Create drawdown id
        let drawdown_id = (project_id, drawdown_type, drawdown_number, timestamp).using_encoded(blake2_256);

        // Create drawdown data
        let drawdown_data = DrawdownData::<T> {
            project_id,
            drawdown_number,
            drawdown_type,
            total_amount: 0,
            status: DrawdownStatus::default(),
            documents: None,
            description: None,
            feedback: None,
            created_date: timestamp,
            close_date: 0,
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

		Self::do_edit_drawdown_status_in_project_info(project_id, drawdown_id, DrawdownStatus::default())?;
        //TOREVIEW: Check if an event is needed
        Ok(())
    }

    fn do_initialize_drawdowns(
        admin: T::AccountId,
        project_id: ProjectId,
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_authorized(admin.clone(), &Self::get_global_scope(), ProxyPermission::Expenditures)?;

        // Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        //Create a EB5 drawdown
        Self::do_create_drawdown(project_id, DrawdownType::EB5, 1)?;

        //Create a Construction Loan drawdown
        Self::do_create_drawdown(project_id, DrawdownType::ConstructionLoan, 1)?;

        //Create a Developer Equity drawdown
        Self::do_create_drawdown(project_id, DrawdownType::DeveloperEquity, 1)?;

        Ok(())
    }

    pub fn do_submit_drawdown(
        project_id: ProjectId,
        drawdown_id: DrawdownId,
    ) -> DispatchResult {
        // Ensure project exists & is not completed
        Self::is_project_completed(project_id)?;

        // Check if drawdown exists & is editable
        Self::is_drawdown_editable(drawdown_id)?;

        // Ensure drawdown has transactions
        ensure!(<TransactionsByDrawdown<T>>::contains_key(project_id, drawdown_id), Error::<T>::DrawdownHasNoTransactions);

        // Get drawdown transactions
        let drawdown_transactions = TransactionsByDrawdown::<T>::try_get(project_id, drawdown_id).map_err(|_| Error::<T>::DrawdownNotFound)?;

        // Update each transaction status to submitted
        for transaction_id in drawdown_transactions {
            // Get transaction data
            let transaction_data = TransactionsInfo::<T>::get(transaction_id).ok_or(Error::<T>::TransactionNotFound)?;

            // Ensure transaction is in draft or rejected status
            ensure!(transaction_data.status == TransactionStatus::Draft || transaction_data.status == TransactionStatus::Rejected, Error::<T>::CannotSubmitTransaction);

            // Update transaction status to submitted
            <TransactionsInfo<T>>::try_mutate::<_,_,DispatchError,_>(transaction_id, |transaction_data| {
                let transaction_data = transaction_data.as_mut().ok_or(Error::<T>::TransactionNotFound)?;
                transaction_data.status = TransactionStatus::Submitted;
                transaction_data.feedback = None;
                Ok(())
            })?;
        }

        // Update drawdown status
        <DrawdownsInfo<T>>::try_mutate::<_,_,DispatchError,_>(drawdown_id, |drawdown_data| {
            let drawdown_data = drawdown_data.as_mut().ok_or(Error::<T>::DrawdownNotFound)?;
            drawdown_data.status = DrawdownStatus::Submitted;
            Ok(())
        })?;

		Self::do_edit_drawdown_status_in_project_info(project_id, drawdown_id, DrawdownStatus::Submitted)?;

        //Event
        Self::deposit_event(Event::DrawdownSubmitted(drawdown_id));

        Ok(())
    }

    pub fn do_approve_drawdown(
        admin: T::AccountId,
        project_id: ProjectId,
        drawdown_id: DrawdownId,
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_authorized(admin.clone(), &Self::get_global_scope(), ProxyPermission::Expenditures)?;

        //  Get drawdown data & ensure drawdown exists
        let drawdown_data = DrawdownsInfo::<T>::get(drawdown_id).ok_or(Error::<T>::DrawdownNotFound)?;

        // Ensure drawdown is in submitted status
        ensure!(drawdown_data.status == DrawdownStatus::Submitted, Error::<T>::DrawdownIsNotInSubmittedStatus);

        // Ensure drawdown has transactions
        ensure!(<TransactionsByDrawdown<T>>::contains_key(project_id, drawdown_id), Error::<T>::DrawdownHasNoTransactions);

        // Get drawdown transactions
        let drawdown_transactions = TransactionsByDrawdown::<T>::try_get(project_id, drawdown_id).map_err(|_| Error::<T>::DrawdownNotFound)?;

        // Get timestamp
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;
        // Update each transaction status to approved
        for transaction_id in drawdown_transactions {
            // Get transaction data
            let transaction_data = TransactionsInfo::<T>::get(transaction_id).ok_or(Error::<T>::TransactionNotFound)?;

            // Ensure transaction is in submitted status
            ensure!(transaction_data.status == TransactionStatus::Submitted, Error::<T>::TransactionIsNotInSubmittedStatus);

            // Update transaction status to approved
            <TransactionsInfo<T>>::try_mutate::<_,_,DispatchError,_>(transaction_id, |transaction_data| {
                let transaction_data = transaction_data.as_mut().ok_or(Error::<T>::TransactionNotFound)?;
                transaction_data.status = TransactionStatus::Approved;
                transaction_data.closed_date = timestamp;
                Ok(())
            })?;
        }

        // Update drawdown status
        <DrawdownsInfo<T>>::try_mutate::<_,_,DispatchError,_>(drawdown_id, |drawdown_data| {
            let drawdown_data = drawdown_data.as_mut().ok_or(Error::<T>::DrawdownNotFound)?;
            drawdown_data.status = DrawdownStatus::Approved;
            drawdown_data.close_date = timestamp;
            Ok(())
        })?;

		Self::do_edit_drawdown_status_in_project_info(project_id, drawdown_id, DrawdownStatus::Approved)?;

        // Generate the next drawdown
        Self::do_create_drawdown(project_id, drawdown_data.drawdown_type, drawdown_data.drawdown_number + 1)?;

        // Event
        Self::deposit_event(Event::DrawdownApproved(drawdown_id));
        Ok(())
    }


    pub fn do_reject_drawdown(
        admin: T::AccountId,
        project_id: ProjectId,
        drawdown_id: DrawdownId,
        transactions_feedback: Option<BoundedVec<(TransactionId, FieldDescription), T::MaxRegistrationsAtTime>>,
        drawdown_feedback: Option<FieldDescription>,
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_authorized(admin.clone(), &Self::get_global_scope(), ProxyPermission::Expenditures)?;

        //  Get drawdown data & ensure drawdown exists
        let drawdown_data = DrawdownsInfo::<T>::get(drawdown_id).ok_or(Error::<T>::DrawdownNotFound)?;

        // Ensure drawdown is in submitted status
        ensure!(drawdown_data.status == DrawdownStatus::Submitted, Error::<T>::DrawdownIsNotInSubmittedStatus);

        // Match drawdown type in order to update transactions status
        match drawdown_data.drawdown_type {
            DrawdownType::EB5 => {
                // Ensure drawdown has transactions
                ensure!(<TransactionsByDrawdown<T>>::contains_key(project_id, drawdown_id), Error::<T>::DrawdownHasNoTransactions);

                // Get drawdown transactions
                let drawdown_transactions = TransactionsByDrawdown::<T>::try_get(project_id, drawdown_id).map_err(|_| Error::<T>::DrawdownNotFound)?;

                // Update each transaction status to rejected
                for transaction_id in drawdown_transactions {
                    // Get transaction data
                    let transaction_data = TransactionsInfo::<T>::get(transaction_id).ok_or(Error::<T>::TransactionNotFound)?;

                    // Ensure transaction is in submitted status
                    ensure!(transaction_data.status == TransactionStatus::Submitted, Error::<T>::TransactionIsNotInSubmittedStatus);

                    // Update transaction status to rejected
                    <TransactionsInfo<T>>::try_mutate::<_,_,DispatchError,_>(transaction_id, |transaction_data| {
                        let transaction_data = transaction_data.as_mut().ok_or(Error::<T>::TransactionNotFound)?;
                        transaction_data.status = TransactionStatus::Rejected;
                        Ok(())
                    })?;
                }

                // Ensure transactions feedback is provided
                let mod_transactions_feedback = transactions_feedback.ok_or(Error::<T>::EB5MissingFeedback)?;

                for (transaction_id, feedback) in mod_transactions_feedback {
                    // Update transaction feedback
                    <TransactionsInfo<T>>::try_mutate::<_,_,DispatchError,_>(transaction_id, |transaction_data| {
                        let transaction_data = transaction_data.as_mut().ok_or(Error::<T>::TransactionNotFound)?;
                        transaction_data.feedback = Some(feedback);
                        Ok(())
                    })?;
                }

            },
            _ => {
                // Construction Loan & Developer Equity drawdowns
                // If drawdown has transactions, update each transaction status to rejected
                if <TransactionsByDrawdown<T>>::contains_key(project_id, drawdown_id) {
                    // Get drawdown transactions
                    let drawdown_transactions = TransactionsByDrawdown::<T>::try_get(project_id, drawdown_id).map_err(|_| Error::<T>::DrawdownNotFound)?;

                    // Update each transaction status to rejected
                    for transaction_id in drawdown_transactions {
                        // Get transaction data
                        let transaction_data = TransactionsInfo::<T>::get(transaction_id).ok_or(Error::<T>::TransactionNotFound)?;

                        // Ensure transaction is in submitted status
                        ensure!(transaction_data.status == TransactionStatus::Submitted, Error::<T>::TransactionIsNotInSubmittedStatus);

                        // Update transaction status to rejected
                        <TransactionsInfo<T>>::try_mutate::<_,_,DispatchError,_>(transaction_id, |transaction_data| {
                            let transaction_data = transaction_data.as_mut().ok_or(Error::<T>::TransactionNotFound)?;
                            transaction_data.status = TransactionStatus::Rejected;
                            Ok(())
                        })?;
                    }
                }

                // Ensure drawdown feedback is provided
                let mod_drawdown_feedback = drawdown_feedback.ok_or(Error::<T>::NoFeedbackProvidedForBulkUpload)?;

                // Update drawdown feedback
                <DrawdownsInfo<T>>::try_mutate::<_,_,DispatchError,_>(drawdown_id, |drawdown_data| {
                    let drawdown_data = drawdown_data.as_mut().ok_or(Error::<T>::DrawdownNotFound)?;
                    drawdown_data.feedback = Some(mod_drawdown_feedback.clone());
                    Ok(())
                })?;
            },
        }

        // Update drawdown status
        <DrawdownsInfo<T>>::try_mutate::<_,_,DispatchError,_>(drawdown_id, |drawdown_data| {
            let drawdown_data = drawdown_data.as_mut().ok_or(Error::<T>::DrawdownNotFound)?;
            drawdown_data.status = DrawdownStatus::Rejected;
            Ok(())
        })?;

		Self::do_edit_drawdown_status_in_project_info(project_id, drawdown_id, DrawdownStatus::Rejected)?;

        //Event
        Self::deposit_event(Event::DrawdownRejected(drawdown_id));

        Ok(())
    }


    // T R A N S A C T I O N S
    // --------------------------------------------------------------------------------------------
    pub fn do_execute_transactions(
        project_id: ProjectId,
        drawdown_id: DrawdownId,
        transactions: BoundedVec<(
            Option<ExpenditureId>, // expenditure_id
            Option<ExpenditureAmount>, // amount
            Option<Documents<T>>, //Documents
            CUDAction, // Action
            Option<TransactionId>, // transaction_id
        ), T::MaxRegistrationsAtTime>,
    ) -> DispatchResult {
        // Ensure project exists & is not completed so helper private functions doesn't need to check it again 
        Self::is_project_completed(project_id)?;

        //Ensure drawdown exists so helper private functions doesn't need to check it again
        ensure!(DrawdownsInfo::<T>::contains_key(drawdown_id), Error::<T>::DrawdownNotFound);
        // Ensure transactions are not empty
        ensure!(!transactions.is_empty(), Error::<T>::EmptyTransactions);

        // Ensure if the selected drawdown is editable
        Self::is_drawdown_editable(drawdown_id)?;

        for transaction in transactions {
            match transaction.3 {
                CUDAction::Create => {
                    // Create transaction only needs (expenditure_id, amount, documents)
                    Self::do_create_transaction(
                        project_id,
                        drawdown_id,
                        transaction.0.ok_or(Error::<T>::ExpenditureIdRequired)?,
                        transaction.1.ok_or(Error::<T>::AmountRequired)?,
                        transaction.2,
                    )?;
                },
                CUDAction::Update => {
                    // Update transaction needs (amount, documents, transaction_id)
                    Self::do_update_transaction(
                        transaction.1,
                        transaction.2,
                        transaction.4.ok_or(Error::<T>::TransactionIdNotFound)?,
                    )?;
                },
                CUDAction::Delete => {
                    // Delete transaction needs (transaction_id)
                    Self::do_delete_transaction(
                        transaction.4.ok_or(Error::<T>::TransactionIdNotFound)?,
                    )?;
                },
            }

        }

        // Update total amount for the given drawdown
        Self::do_calculate_drawdown_total_amount(project_id, drawdown_id)?;

        Self::deposit_event(Event::TransactionsCompleted);
        Ok(())
    }

    fn do_create_transaction(
        project_id: ProjectId,
        drawdown_id: DrawdownId,
        expenditure_id: ExpenditureId,
        amount: Amount,
        documents: Option<Documents<T>>,
    ) -> DispatchResult {
        // Ensure amount is valid
        Self::is_amount_valid(amount)?;

        //TOREVIEW: If documents are mandatory, we need to check if they are provided

        // Get timestamp
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        // Create transaction id
        let transaction_id = (drawdown_id, amount, expenditure_id, timestamp, project_id).using_encoded(blake2_256);

        // Ensure expenditure id does not exist
        ensure!(ExpendituresInfo::<T>::contains_key(expenditure_id), Error::<T>::ExpenditureNotFound);

        // Create transaction data
        let transaction_data = TransactionData::<T> {
            project_id,
            drawdown_id,
            expenditure_id,
            created_date: timestamp,
            updated_date: timestamp,
            closed_date: 0,
            feedback: None,
            amount,
            status: TransactionStatus::default(),
            documents,
        };

        // Insert transaction data
        // Ensure transaction id is unique
        ensure!(!TransactionsInfo::<T>::contains_key(transaction_id), Error::<T>::TransactionAlreadyExists);
        <TransactionsInfo<T>>::insert(transaction_id, transaction_data);

        // Insert transaction id into TransactionsByDrawdown
        <TransactionsByDrawdown<T>>::try_mutate::<_,_,_,DispatchError,_>(project_id, drawdown_id, |transactions| {
            transactions.try_push(transaction_id).map_err(|_| Error::<T>::MaxTransactionsPerDrawdownReached)?;
            Ok(())
        })?;

        Self::deposit_event(Event::TransactionCreated(transaction_id));
        Ok(())

    }

    fn do_update_transaction(
        amount: Option<ExpenditureAmount>,
        documents: Option<Documents<T>>,
        transaction_id: TransactionId,
    ) -> DispatchResult {
        // Ensure transaction exists
        ensure!(TransactionsInfo::<T>::contains_key(transaction_id), Error::<T>::TransactionNotFound);

        // Ensure amount is valid.
        if let Some(amount) = amount {
            Self::is_amount_valid(amount)?;
        }

        // Ensure documents is not empty
        if let Some(mod_documents) = documents.clone() {
            ensure!(mod_documents.len() > 0, Error::<T>::DocumentsIsEmpty);
        }

        // Get timestamp
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        // Try mutate transaction data
        <TransactionsInfo<T>>::try_mutate::<_,_,DispatchError,_>(transaction_id, |transaction_data| {
            let mod_transaction_data = transaction_data.as_mut().ok_or(Error::<T>::TransactionNotFound)?;

            // Ensure expenditure exists
            ensure!(ExpendituresInfo::<T>::contains_key(mod_transaction_data.expenditure_id), Error::<T>::ExpenditureNotFound);

            if let Some(mod_amount) = amount {
                mod_transaction_data.amount = mod_amount;
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
        transaction_id: TransactionId
    ) -> DispatchResult {
        // Ensure transaction exists and get transaction data
        let transaction_data = TransactionsInfo::<T>::get(transaction_id).ok_or(Error::<T>::TransactionNotFound)?;

        // Ensure drawdown is not completed
        ensure!(Self::is_drawdown_editable(transaction_data.drawdown_id).is_ok(), Error::<T>::DrawdownIsAlreadyCompleted);

        // Ensure transaction is not completed
        ensure!(Self::is_transaction_editable(transaction_id).is_ok(), Error::<T>::TransactionIsAlreadyCompleted);

        // Remove transaction from TransactionsByDrawdown
        <TransactionsByDrawdown<T>>::try_mutate::<_,_,_,DispatchError,_>(transaction_data.project_id, transaction_data.drawdown_id, |transactions| {
            transactions.retain(|transaction| transaction != &transaction_id);
            Ok(())
        })?;

        // Remove transaction from TransactionsInfo
        <TransactionsInfo<T>>::remove(transaction_id);

        //TOREVIEW: Check if this event is needed
        Self::deposit_event(Event::TransactionDeleted(transaction_id));

        Ok(())
    }

    // B U L K   U P L O A D   T R A N S A C T I O N S

    pub fn do_up_bulk_upload(
        user: T::AccountId,
        project_id: ProjectId,
        drawdown_id: DrawdownId,
        description: FieldDescription,
        total_amount: TotalAmount,
        documents: Documents<T>,
    ) -> DispatchResult {
        // Ensure builder permissions
        Self::is_authorized(user, &project_id, ProxyPermission::UpBulkupload)?;

        // Ensure project is not completed
        Self::is_project_completed(project_id)?;

        // Ensure drawdown is not completed
        Self::is_drawdown_editable(drawdown_id)?;

        // Ensure amount is valid
        Self::is_amount_valid(total_amount)?;

        //Ensure only Construction loan & developer equity drawdowns can call bulk uploaded
        let drawdown_data = DrawdownsInfo::<T>::get(drawdown_id).ok_or(Error::<T>::DrawdownNotFound)?;
        ensure!(drawdown_data.drawdown_type == DrawdownType::ConstructionLoan || drawdown_data.drawdown_type == DrawdownType::DeveloperEquity, Error::<T>::DrawdownTypeNotSupportedForBulkUpload);

        //Ensure drawdown status is draft or rejected
        ensure!(drawdown_data.status == DrawdownStatus::Draft || drawdown_data.status == DrawdownStatus::Rejected, Error::<T>::DrawdownStatusNotSupportedForBulkUpload);

        // Ensure documents is not empty
        ensure!(!documents.is_empty(), Error::<T>::DocumentsIsEmpty);

        // Ensure description is not empty
        ensure!(!description.is_empty(), Error::<T>::BulkUploadDescriptionRequired);

        // Mutate drawdown data
        <DrawdownsInfo<T>>::try_mutate::<_,_,DispatchError,_>(drawdown_id, |drawdown_data| {
            let mod_drawdown_data = drawdown_data.as_mut().ok_or(Error::<T>::DrawdownNotFound)?;
            mod_drawdown_data.total_amount = total_amount;
            mod_drawdown_data.description = Some(description);
            mod_drawdown_data.documents = Some(documents);
            mod_drawdown_data.status = DrawdownStatus::Submitted;
            mod_drawdown_data.feedback = None;
            Ok(())
        })?;

		Self::do_edit_drawdown_status_in_project_info(project_id, drawdown_id, DrawdownStatus::Submitted)?;

        Ok(())
    }

    // I N F L A T I O N   A D J U S T M E N T
    // --------------------------------------------------------------------------------------------
    pub fn do_execute_inflation_adjustment(
        admin: T::AccountId,
        projects: BoundedVec<(ProjectId, Option<InflationRate>, CUDAction), T::MaxRegistrationsAtTime>,
    ) -> DispatchResult {
        // Ensure admin permissions
        Self::is_authorized(admin.clone(), &Self::get_global_scope(), ProxyPermission::Expenditures)?;

        // Ensure projects is not empty
        ensure!(!projects.is_empty(), Error::<T>::InflationRateMissingProjectIds);

        // Match each CUD action
        for project in projects {
            // Ensure project exists
            ensure!(ProjectsInfo::<T>::contains_key(project.0), Error::<T>::ProjectNotFound);
            match project.2 {
                // Delete needs: project_id
                CUDAction::Delete => {
                    // Mutate project data
                    <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project.0, |project_info| {
                        let mod_project_data = project_info.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
                        mod_project_data.inflation_rate = None;
                        Ok(())
                    })?;
                },
                // Creation & Update needs: project_id, inflation_rate
                _ => {
                    // Mutate project data

                    // Ensure inflation rate is provided
                    let inflation_rate = project.1.ok_or(Error::<T>::InflationRateRequired)?;

                    <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project.0, |project_info| {
                        let mod_project_data = project_info.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
                        mod_project_data.inflation_rate = Some(inflation_rate);
                        Ok(())
                    })?;
                },
            }
        }

        Ok(())
    }

    // J O B    E L I G I B L E S
    // --------------------------------------------------------------------------------------------
    pub fn do_execute_job_eligibles(
        admin: T::AccountId,
        project_id: ProjectId,
        job_eligibles: BoundedVec<(
            Option<FieldName>, // name
            Option<JobEligibleAmount>, // amount
            Option<NAICSCode>, // naics code
            Option<JobsMultiplier>, // jobs multiplier
            CUDAction, // action
            Option<JobEligibleId>, // job_eligible_id
        ), T::MaxRegistrationsAtTime>,
    ) -> DispatchResult {
        // Ensure admin permissions 
        Self::is_authorized(admin.clone(), &Self::get_global_scope(), ProxyPermission::JobEligible)?;

        // Ensure project exists
        ensure!(ProjectsInfo::<T>::contains_key(project_id), Error::<T>::ProjectNotFound);

        // Ensure job eligibles is not empty
        ensure!(!job_eligibles.is_empty(), Error::<T>::JobEligiblesIsEmpty);

        for job_eligible in job_eligibles {
            match job_eligible.4 {
                CUDAction::Create => {
                    // Create job eligible only needs: name, amount, naics code, jobs multiplier
                    Self::do_create_job_eligible(
                        project_id,
                        job_eligible.0.ok_or(Error::<T>::JobEligibleNameRequired)?,
                        job_eligible.1.ok_or(Error::<T>::JobEligibleAmountRequired)?,
                        job_eligible.2,
                        job_eligible.3,
                    )?;
                },
                CUDAction::Update => {
                    // Update job eligible needs: job_eligible_id, name, amount, naics code, jobs multiplier
                    Self::do_update_job_eligible(
                        project_id,
                        job_eligible.5.ok_or(Error::<T>::JobEligibleIdRequired)?,
                        job_eligible.0,
                        job_eligible.1,
                        job_eligible.2,
                        job_eligible.3,
                    )?;
                },
                CUDAction::Delete => {
                    // Delete job eligible needs: job_eligible_id
                    Self::do_delete_job_eligible(
                        job_eligible.5.ok_or(Error::<T>::JobEligibleIdRequired)?,
                    )?;
                },
            }
        }

        Ok(())
    }

    fn do_create_job_eligible(
        project_id: [u8;32],
        name: FieldName,
        job_eligible_amount: JobEligibleAmount,
        naics_code: Option<NAICSCode>,
        jobs_multiplier: Option<JobsMultiplier>,
    ) -> DispatchResult {
        // Ensure project exists & is not completed
        Self::is_project_completed(project_id)?;

        // Get timestamp
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        // Ensure job eligible name is not empty
        ensure!(!name.is_empty(), Error::<T>::JobEligiblesNameIsRequired);

        // Create job eligible id
        let job_eligible_id: JobEligibleId = (project_id, name.clone(), timestamp).using_encoded(blake2_256);

        // Create job eligible data
        let job_eligible_data = JobEligibleData {
            project_id,
            name,
            job_eligible_amount,
            naics_code,
            jobs_multiplier,
        };

        // Insert job eligible data into JobEligiblesInfo
        // Ensure job eligible id does not exist
        ensure!(!JobEligiblesInfo::<T>::contains_key(job_eligible_id), Error::<T>::JobEligibleIdAlreadyExists);
        <JobEligiblesInfo<T>>::insert(job_eligible_id, job_eligible_data);

        // Insert job eligible id into JobEligiblesByProject
        <JobEligiblesByProject<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |job_eligibles| {
            job_eligibles.try_push(job_eligible_id).map_err(|_| Error::<T>::MaxJobEligiblesPerProjectReached)?;
            Ok(())
        })?;

        Self::deposit_event(Event::JobEligibleCreated(project_id, job_eligible_id));
        Ok(())
    }

    fn do_update_job_eligible(
        project_id: ProjectId,
        job_eligible_id: JobEligibleId,
        name: Option<FieldName>,
        job_eligible_amount: Option<JobEligibleAmount>,
        naics_code: Option<NAICSCode>,
        jobs_multiplier: Option<JobsMultiplier>,
    ) -> DispatchResult {
        // Ensure project exists & is not completed
        Self::is_project_completed(project_id)?;

        // Ensure job eligible exists
        ensure!(JobEligiblesInfo::<T>::contains_key(job_eligible_id), Error::<T>::JobEligibleNotFound);

        // Mutate job eligible data
        <JobEligiblesInfo<T>>::try_mutate::<_,_,DispatchError,_>(job_eligible_id, |job_eligible_data| {
            let job_eligible = job_eligible_data.as_mut().ok_or(Error::<T>::JobEligibleNotFound)?;

            // Ensure job eligible belongs to the project
            ensure!(job_eligible.project_id == project_id, Error::<T>::JobEligibleDoesNotBelongToProject);

            if let Some(mod_name) = name {
                job_eligible.name = mod_name;
            }
            if let Some(mod_job_eligible_amount) = job_eligible_amount {
                job_eligible.job_eligible_amount = mod_job_eligible_amount;
            }
            if let Some(mod_naics_code) = naics_code {
                job_eligible.naics_code = Some(mod_naics_code);
            }
            if let Some(mod_jobs_multiplier) = jobs_multiplier {
                job_eligible.jobs_multiplier = Some(mod_jobs_multiplier);
            }
            Ok(())
        })?;

        Self::deposit_event(Event::JobEligibleUpdated(project_id, job_eligible_id));
        Ok(())
    }

    fn do_delete_job_eligible(
        job_eligible_id: JobEligibleId,     
    ) -> DispatchResult {
        // Ensure job eligible exists & get job eligible data
        let job_eligible_data = JobEligiblesInfo::<T>::get(job_eligible_id).ok_or(Error::<T>::JobEligibleNotFound)?;

        // Delete job eligible data from JobEligiblesInfo
        <JobEligiblesInfo<T>>::remove(job_eligible_id);

        // Delete job eligible id from JobEligiblesByProject
        <JobEligiblesByProject<T>>::try_mutate::<_,_,DispatchError,_>(job_eligible_data.project_id, |job_eligibles| {
            job_eligibles.retain(|job_eligible| job_eligible != &job_eligible_id);
            Ok(())
        })?;

        Self::deposit_event(Event::JobEligibleDeleted(job_eligible_data.project_id, job_eligible_id));

        Ok(())
    }

    // R E V E N U E S
    // --------------------------------------------------------------------------------------------
    pub fn do_execute_revenue_transactions(
        project_id: ProjectId,
        revenue_id: RevenueId,
        revenue_transactions: BoundedVec<(
            Option<JobEligibleId>, // job_eligible_id
            Option<RevenueAmount>, // revenue_amount
            Option<Documents<T>>, // documents
            CUDAction, // action
            Option<RevenueTransactionId>, // revenue_transaction_id
        ), T::MaxTransactionsPerRevenue>,
    ) -> DispatchResult {
        // Ensure project exists & is not completed so helper private functions doesn't need to check it again 
        Self::is_project_completed(project_id)?;

        // Ensure revenue exists so helper private functions doesn't need to check it again
        ensure!(RevenuesInfo::<T>::contains_key(revenue_id), Error::<T>::RevenueNotFound);

        //Ensure revenue transactions are not empty
        ensure!(!revenue_transactions.is_empty(), Error::<T>::RevenueTransactionsAreEmpty);

        // Ensure if the selected revenue is editable
        Self::is_revenue_editable(revenue_id)?;

        // for transaction in revenue_transactions {
        //     match transaction.3 {
        //         CUDAction::Create => {
        //             // Create transaction only needs (expenditure_id, amount, documents)
        //             Self::do_create_revenue_transaction(
        //                 project_id,
        //                 revenue_id,
        //                 transaction.0,
        //                 transaction.1,
        //                 transaction.2,
        //             )?;
        //         },
        //         CUDAction::Update => {
        //             Self::do_update_revenue_transaction(
        //                 project_id,
        //                 revenue_id,
        //                 transaction.0,
        //                 transaction.1,
        //                 transaction.2,
        //                 transaction.4.ok_or(Error::<T>::RevenueTransactionIdNotFound)?,
        //             )?;
        //         },
        //         CUDAction::Delete => {
        //             Self::do_delete_revenue_transaction(
        //                 project_id,
        //                 revenue_id,
        //                 transaction.4.ok_or(Error::<T>::RevenueTransactionIdNotFound)?,
        //             )?;
        //         },
        //     }
        // }

        Ok(())
    }

    fn do_create_revenue_transaction(
        project_id: ProjectId,
        revenue_id: RevenueId,
        job_eligible_id: JobEligibleId,
        revenue_amount: RevenueAmount,
        documents: Option<Documents<T>>,
    ) -> DispatchResult {
        // TOREVIEW: If documents are mandatory, then we need to check if they are empty

        // Get timestamp
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        // Create revenue transaction id
        let revenue_transaction_id = (revenue_id, job_eligible_id, project_id, timestamp).using_encoded(blake2_256);
        
        //Ensure revenue transaction id doesn't exist
        ensure!(!RevenueTransactionsInfo::<T>::contains_key(revenue_transaction_id), Error::<T>::RevenueTransactionIdAlreadyExists);

        // Create revenue transaction data
        let revenue_transaction_data = RevenueTransactionData {
            project_id,
            revenue_id,
            job_eligible_id,
            created_date: timestamp,
            updated_date: timestamp,
            closed_date: 0,
            feedback: None,
            amount: revenue_amount,
            status: RevenueTransactionStatus::default(),
            documents,
        };

        // Insert revenue transaction data into RevenueTransactionsInfo
        // Ensure revenue transaction id doesn't exist
        ensure!(!RevenueTransactionsInfo::<T>::contains_key(revenue_transaction_id), Error::<T>::RevenueTransactionIdAlreadyExists);
        <RevenueTransactionsInfo<T>>::insert(revenue_transaction_id, revenue_transaction_data);

        // Insert revenue transaction id into TransactionsByRevenue
        <TransactionsByRevenue<T>>::try_mutate::<_,_,_,DispatchError,_>(project_id, revenue_id, |revenue_transactions| {
            revenue_transactions.try_push(revenue_transaction_id).map_err(|_| Error::<T>::MaxTransactionsPerRevenueReached)?;
            Ok(())
        })?;

        Self::deposit_event(Event::RevenueTransactionCreated(evenue_transaction_id));

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

    /// Get global scope
    pub fn get_global_scope() -> [u8;32] {
        let global_scope = <GlobalScope<T>>::try_get().map_err(|_| Error::<T>::NoGlobalScopeValueWasFound).unwrap();
        global_scope
    }

    fn _change_project_status(
        admin: T::AccountId,
        project_id: ProjectId,
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

    fn is_project_completion_date_later(
        project_id: ProjectId,
    ) -> DispatchResult {
        // Get project data & ensure project exists
        let project_data = ProjectsInfo::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;

        // Ensure completion date is later than start date
        ensure!(project_data.completion_date > project_data.creation_date, Error::<T>::CompletionDateMustBeLater);
        Ok(())
    }

    fn add_project_role(
        project_id: ProjectId,
        user: T::AccountId,
        role: ProxyRole,
    ) -> DispatchResult {

        match role {
            ProxyRole::Administrator => {
                return Err(Error::<T>::CannotRegisterAdminRole.into());
            },
            ProxyRole::Builder => {
                //Mutate project data
                <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
                    let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
                    match project.builder.as_mut() {
                        Some(builder) => {
                            builder.try_push(user.clone()).map_err(|_| Error::<T>::MaxBuildersPerProjectReached)?;
                        },
                        None => {
                            let devs = project.builder.get_or_insert(BoundedVec::<T::AccountId, T::MaxBuildersPerProject>::default());
                            devs.try_push(user.clone()).map_err(|_| Error::<T>::MaxBuildersPerProjectReached)?;
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
        project_id: ProjectId,
        user: T::AccountId,
        role: ProxyRole,
    ) -> DispatchResult {

        match role {
            ProxyRole::Administrator => {
                return Err(Error::<T>::CannotRemoveAdminRole.into());
            },
            ProxyRole::Builder => {
                //TODO: Fix internal validations
                //TODO: move logic to a helper function to avoid boilerplate
                //Mutate project data
                <ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project| {
                    let project = project.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
                    match project.builder.as_mut() {
                        Some(builder) => {
                            //builder.clone().iter().find(|&u| *u == user).ok_or(Error::<T>::UserNotAssignedToProject)?;
                            builder.retain(|u| *u != user);
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

        // Ensure how many projects the user is assigned to
        let projects_count = <ProjectsByUser<T>>::get(user.clone()).len();

        match user_data.role {
            ProxyRole::Builder => {
                ensure!(projects_count < T::MaxProjectsPerBuilder::get() as usize, Error::<T>::MaxProjectsPerBuilderReached);
            },
            ProxyRole::Investor => {
                ensure!(projects_count < T::MaxProjectsPerInvestor::get() as usize, Error::<T>::MaxProjectsPerInvestorReached);
            },
            ProxyRole::Issuer => {
                ensure!(projects_count < T::MaxProjectsPerIssuer::get() as usize, Error::<T>::MaxProjectsPerIssuerReached);
            },
            ProxyRole::RegionalCenter => {
                ensure!(projects_count < T::MaxProjectsPerRegionalCenter::get() as usize, Error::<T>::MaxProjectsPerRegionalCenterReached);
            },
            ProxyRole::Administrator => {
                // This should never happen
                return Err(Error::<T>::CannotAddAdminRole.into());
            },
        }
        Ok(())
    }

    fn is_project_completed(
        project_id: ProjectId,
    ) -> DispatchResult {
        // Get project data & ensure project exists
        let project_data = ProjectsInfo::<T>::get(project_id).ok_or(Error::<T>::ProjectNotFound)?;

        // Ensure project is not completed
        ensure!(project_data.status != ProjectStatus::Completed, Error::<T>::ProjectIsAlreadyCompleted);

        Ok(())
    }

    fn is_drawdown_editable(
        drawdown_id: DrawdownId,
    ) -> DispatchResult {
        // Get drawdown data & ensure drawdown exists
        let drawdown_data = DrawdownsInfo::<T>::get(drawdown_id).ok_or(Error::<T>::DrawdownNotFound)?;

        // Match drawdown type
        match drawdown_data.drawdown_type {
            DrawdownType::EB5 => {
                // Match drawdown status
                // Ensure drawdown is in draft or rejected status
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
            },
            _ => {
                // Match drawdown status
                match drawdown_data.status {
                    DrawdownStatus::Approved => {
                        return Err(Error::<T>::CannotEditDrawdown.into());
                    },
                    _ => {
                        return Ok(())
                    },
                }
            }
        }
    }

    fn is_transaction_editable(
        transaction_id: TransactionId,
    ) -> DispatchResult {
        // Get transaction data
        let transaction_data = TransactionsInfo::<T>::get(transaction_id).ok_or(Error::<T>::TransactionNotFound)?;

        // Ensure transaction is in draft or rejected status
        // Match transaction status
        match transaction_data.status {
            TransactionStatus::Approved => {
                return Err(Error::<T>::CannotEditTransaction.into());
            },
            _ => {
                return Ok(())
            }
        }
    }


    pub fn is_authorized( authority: T::AccountId, project_id: &[u8;32], permission: ProxyPermission ) -> DispatchResult{
        T::Rbac::is_authorized(
            authority,
            Self::pallet_id(),
            project_id,
            &permission.id(),
        )
    }

    #[allow(dead_code)]
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

        // Add administrator to rbac pallet
        T::Rbac::assign_role_to_user(
            admin.clone(),
            Self::pallet_id(),
            &Self::get_global_scope(),
            ProxyRole::Administrator.id()
        )?;

        Ok(())
    }

    fn sudo_delete_admin( admin: T::AccountId ) -> DispatchResult{
        // check if user is already registered
        ensure!(<UsersInfo<T>>::contains_key(admin.clone()), Error::<T>::UserNotRegistered);

        //Remove user from UsersInfo storage map
        <UsersInfo<T>>::remove(admin.clone());

        // Remove administrator from rbac pallet
        T::Rbac::remove_role_from_user(
            admin.clone(),
            Self::pallet_id(),
            &Self::get_global_scope(),
            ProxyRole::Administrator.id()
        )?;

        Ok(())
    }

    //TODO: Remove this function
    #[allow(dead_code)]
    fn is_amount_valid(amount: u64,) -> DispatchResult {
        let minimun_amount: u64 = 0;
        ensure!(amount >= minimun_amount, Error::<T>::InvalidAmount);
        Ok(())
    }

    fn do_calculate_drawdown_total_amount(
        project_id: [u8;32],
        drawdown_id: [u8;32],
    ) -> DispatchResult {
        // Ensure drawdown exists
        ensure!(<DrawdownsInfo<T>>::contains_key(drawdown_id), Error::<T>::DrawdownNotFound);

        // Get drawdown transactions
        let drawdown_transactions = TransactionsByDrawdown::<T>::try_get(project_id, drawdown_id).map_err(|_| Error::<T>::DrawdownNotFound)?;
        // Calculate drawdown total amount
        let mut drawdown_total_amount: u64 = 0;

        for transaction_id in drawdown_transactions {
            // Get transaction data
            let transaction_data = TransactionsInfo::<T>::get(transaction_id).ok_or(Error::<T>::TransactionNotFound)?;

            // Add transaction amount to drawdown total amount
            drawdown_total_amount = drawdown_total_amount + transaction_data.amount;
        }

        // Update drawdown total amount
        <DrawdownsInfo<T>>::try_mutate::<_,_,DispatchError,_>(drawdown_id, |drawdown_data| {
            let drawdown_data = drawdown_data.as_mut().ok_or(Error::<T>::DrawdownNotFound)?;
            drawdown_data.total_amount = drawdown_total_amount;
            Ok(())
        })?;

       Ok(())
    }

	fn do_edit_drawdown_status_in_project_info(
		project_id: ProjectId,
		drawdown_id: DrawdownId,
		drawdown_status: DrawdownStatus
	) -> DispatchResult {
		let drawdown_data = DrawdownsInfo::<T>::get(drawdown_id).ok_or(Error::<T>::DrawdownNotFound)?;

        // Match drawdown type
        match drawdown_data.drawdown_type {
            DrawdownType::EB5 => {
				// Update EB5 drawdown status in project info
				<ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project_data| {
					let project_data = project_data.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
					project_data.eb5_drawdown_status = Some(drawdown_status);
					Ok(())
				})?;
                Ok(())
			},
			DrawdownType::ConstructionLoan => {
				// Update Construction Loan drawdown status in project info
				<ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project_data| {
					let project_data = project_data.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
					project_data.construction_loan_drawdown_status = Some(drawdown_status);
					Ok(())
				})?;
                Ok(())
			},
			DrawdownType::DeveloperEquity => {
				// Update Developer Equity drawdown status in project info
				<ProjectsInfo<T>>::try_mutate::<_,_,DispatchError,_>(project_id, |project_data| {
					let project_data = project_data.as_mut().ok_or(Error::<T>::ProjectNotFound)?;
					project_data.developer_equity_drawdown_status = Some(drawdown_status);
					Ok(())
				})?;
                Ok(())
			},
        }
	}

    fn is_revenue_editable(
        revenue_id: RevenueId,
    ) -> DispatchResult {
        // Get revenue data & ensure revenue exists
        let revenue_data = RevenuesInfo::<T>::get(revenue_id).ok_or(Error::<T>::RevenueNotFound)?;

        // Match revenue status
        match revenue_data.status {
            RevenueStatus::Draft => {
                return Ok(())
            },
            RevenueStatus::Rejected => {
                return Ok(())
            },
            _ => {
                return Err(Error::<T>::CannotEditRevenue.into());
            }
        }
    }






// Do not code beyond this line
}
