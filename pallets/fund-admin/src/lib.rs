#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod functions;
mod types;
//TODO: Remove unused parameters, types, etc
// - Change extrinsic names
// - Update extrinsic names to beign like CURD actions ( create, update, read, delete)
// - Add external documentation for each extrinsic
// - Fix typos

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::{*, ValueQuery}, BoundedVec};
	use frame_system::pallet_prelude::*;
	use frame_support::transactional;
	use sp_runtime::traits::Scale;
	use frame_support::traits::{Time};

	use crate::types::*;
	use pallet_rbac::types::RoleBasedAccessControl;


	#[pallet::config]
	pub trait Config: frame_system::Config {
		//TODO: change all accounts names for users
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Moment: Parameter
		+ Default
		+ Scale<Self::BlockNumber, Output = Self::Moment>
		+ Copy
		+ MaxEncodedLen
		+ scale_info::StaticTypeInfo
		+ Into<u64>;

		type Timestamp: Time<Moment = Self::Moment>;

		type Rbac : RoleBasedAccessControl<Self::AccountId>;

		type RemoveOrigin: EnsureOrigin<Self::Origin>;

		#[pallet::constant]
		type MaxDocuments: Get<u32>;

		#[pallet::constant]
		type MaxProjectsPerUser: Get<u32>;

		#[pallet::constant]
		type MaxUserPerProject: Get<u32>;

		#[pallet::constant]
		type MaxBuildersPerProject: Get<u32>;

		#[pallet::constant]
		type MaxInvestorsPerProject: Get<u32>;

		#[pallet::constant]
		type MaxIssuersPerProject: Get<u32>;

		#[pallet::constant]
		type MaxRegionalCenterPerProject: Get<u32>;

		//todo:remove MaxBoundedVecs
		#[pallet::constant]
		type MaxBoundedVecs: Get<u32>;

		#[pallet::constant]
		type MaxDrawdownsPerProject: Get<u32>;

		#[pallet::constant]
		type MaxTransactionsPerDrawdown: Get<u32>;

		#[pallet::constant]
		type MaxRegistrationsAtTime: Get<u32>;

		#[pallet::constant]
		type MaxExpendituresPerProject: Get<u32>;

		#[pallet::constant]
		type MaxProjectsPerBuilder: Get<u32>;

		#[pallet::constant]
		type MaxProjectsPerInvestor: Get<u32>;

		#[pallet::constant]
		type MaxProjectsPerIssuer: Get<u32>;

		#[pallet::constant]
		type MaxProjectsPerRegionalCenter: Get<u32>;

		#[pallet::constant]
		type MaxBanksPerProject: Get<u32>;

		#[pallet::constant]
		type MaxJobEligiblesByProject: Get<u32>;

		#[pallet::constant]
		type MaxRevenuesByProject: Get<u32>;

		#[pallet::constant]
		type MaxTransactionsPerRevenue: Get<u32>;

	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/*--- Onchain storage section ---*/

	#[pallet::storage]
	#[pallet::getter(fn global_scope)]
	pub(super) type GlobalScope<T> = StorageValue<
		_,
		[u8;32], // Value global scope id
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn users_info)]
	pub(super) type UsersInfo<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId, // Key account_id
		UserData<T>,  // Value UserData<T>
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn projects_info)]
	pub(super) type ProjectsInfo<T: Config> = StorageMap<
		_,
		Identity,
		ProjectId, // Key project_id
		ProjectData<T>,  // Value ProjectData<T>
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn users_by_project)]
	pub(super) type UsersByProject<T: Config> = StorageMap<
		_,
		Identity,
		ProjectId, // Key project_id
		BoundedVec<T::AccountId, T::MaxUserPerProject>,  // Value users
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn projects_by_user)]
	pub(super) type ProjectsByUser<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId, // Key account_id
		BoundedVec<[u8;32], T::MaxProjectsPerUser>,  // Value projects
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn expenditures_info)]
	pub(super) type ExpendituresInfo<T: Config> = StorageMap<
		_,
		Identity,
		ExpenditureId, // Key expenditure_id
		ExpenditureData,  // Value ExpenditureData<T>
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn expenditures_by_project)]
	pub(super) type ExpendituresByProject<T: Config> = StorageMap<
		_,
		Identity,
		ProjectId, // Key project_id
		BoundedVec<[u8;32], T::MaxExpendituresPerProject>,  // Value expenditures
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn drawdowns_info)]
	pub(super) type DrawdownsInfo<T: Config> = StorageMap<
		_,
		Identity,
		DrawdownId, // Key drawdown id
		DrawdownData<T>,  // Value DrawdownData<T>
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn drawdowns_by_project)]
	pub(super) type DrawdownsByProject<T: Config> = StorageMap<
		_,
		Identity,
		ProjectId, // Key project_id
		BoundedVec<DrawdownId, T::MaxDrawdownsPerProject>,  // Value Drawdowns
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn transactions_info)]
	pub(super) type TransactionsInfo<T: Config> = StorageMap<
		_,
		Identity,
		TransactionId, // Key transaction id
		TransactionData<T>,  // Value TransactionData<T>
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn transactions_by_drawdown)]
	pub(super) type TransactionsByDrawdown<T: Config> = StorageDoubleMap<
		_,
		Identity,
		ProjectId, //K1: project id
		Identity,
		DrawdownId, //K2: drawdown id
		BoundedVec<TransactionId, T::MaxTransactionsPerDrawdown>, // Value transactions
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn job_eligibles_info)]
	pub(super) type JobEligiblesInfo<T: Config> = StorageMap<
		_,
		Identity,
		JobEligibleId, // Key transaction id
		JobEligibleData,  // Value JobEligibleData
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn job_eligibles_by_project)]
	pub(super) type JobEligiblesByProject<T: Config> = StorageMap<
		_,
		Identity,
		ProjectId, // Key project_id
		BoundedVec<JobEligibleId, T::MaxJobEligiblesByProject>,  // Value job eligibles
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn revenues_info)]
	pub(super) type RevenuesInfo<T: Config> = StorageMap<
		_,
		Identity,
		RevenueId, // Key revenue id
		RevenueData<T>,  // Value RevenueData<T>
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn revenues_by_project)]
	pub(super) type RevenuesByProject<T: Config> = StorageMap<
		_,
		Identity,
		ProjectId, // Key project_id
		BoundedVec<RevenueId, T::MaxDrawdownsPerProject>,  // Value Drawdowns
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn revenue_transactions_info)]
	pub(super) type RevenueTransactionsInfo<T: Config> = StorageMap<
		_,
		Identity,
		RevenueTransactionId, // Key revenue transaction id
		RevenueTransactionData<T>,  // Value RevenueTransactionData<T>
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn transactions_by_revenue)]
	pub(super) type TransactionsByRevenue<T: Config> = StorageDoubleMap<
		_,
		Identity,
		ProjectId, //K1: project id
		Identity,
		RevenueId, //K2: revenue id
		BoundedVec<RevenueTransactionId, T::MaxTransactionsPerRevenue>, // Value revenue transactions
		ValueQuery
	>;


	// E V E N T S
	// ------------------------------------------------------------------------------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Project was created successfully
		ProjectCreated(T::AccountId, ProjectId),
		/// Proxy initial setup completed
		ProxySetupCompleted,
		/// User registered successfully
		UserAdded(T::AccountId),
		/// Project was edited
		ProjectEdited(ProjectId),
		/// Project was deleted
		ProjectDeleted(ProjectId),
		/// Administrator added
		AdministratorAssigned(T::AccountId),
		/// Administrator removed
		AdministratorRemoved(T::AccountId),
		/// Users has been assigned from the selected project
		UsersAssignationCompleted([u8;32]),
		/// Users has been removed from the selected project
		UsersUnassignationCompleted([u8;32]),
		/// User info updated
		UserUpdated(T::AccountId),
		/// User removed
		UserDeleted(T::AccountId),
		/// Expenditure was created successfully
		ExpenditureCreated,
		/// Expenditure was edited successfully
		ExpenditureUpdated(ExpenditureId),
		/// Expenditure was deleted successfully
		ExpenditureDeleted(ExpenditureId),
		/// Trasactions was completed successfully
		TransactionsCompleted,
		/// Transaction was created successfully
		TransactionCreated(TransactionId),
		/// Transaction was edited successfully
		TransactionEdited(TransactionId),
		/// Transaction was deleted successfully
		TransactionDeleted(TransactionId),
		/// Users extrinsic was completed successfully
		UsersExecuted,
		/// Assign users extrinsic was completed successfully
		UsersAssignationExecuted([u8;32]),
		/// Drawdown was submitted successfully
		DrawdownSubmitted([u8;32]),
		/// Drawdown was approved successfully
		DrawdownApproved([u8;32]),
		/// Drawdown was rejected successfully
		DrawdownRejected([u8;32]),
		/// Job eligible was created successfully
		JobEligibleCreated(ProjectId, JobEligibleId),
		/// Job eligible was updated successfully
		JobEligibleUpdated(ProjectId, JobEligibleId),
		/// Job eligible was deleted successfully
		JobEligibleDeleted(ProjectId, JobEligibleId),
		/// Revenue transaction was created successfully
		RevenueTransactionCreated(RevenueTransactionId),
		/// Revenue transaction was updated successfully
		RevenueTransactionUpdated(RevenueTransactionId),
		/// Revenue transaction was deleted successfully
		RevenueTransactionDeleted(RevenueTransactionId),
		/// Revenue transactions were executed successfully
		RevenueTransactionsExecuted(ProjectId, RevenueId),
	}

	// E R R O R S
	// ------------------------------------------------------------------------------------------------------------
	#[pallet::error]
	pub enum Error<T> {
		/// No value was found for the global scope
		NoGlobalScopeValueWasFound,
		/// Project ID is already in use
		ProjectIdAlreadyInUse,
		/// Timestamp was not genereated correctly
		TimestampError,
		/// Completion date must be later than creation date
		CompletionDateMustBeLater,
		/// User is already registered in the site
		UserAlreadyRegistered,
		/// Project was not found
		ProjectNotFound,
		/// Project is not active anymore
		ProjectIsAlreadyCompleted,
		/// Can not delete a completed project
		CannotDeleteCompletedProject,
		/// User is not registered
		UserNotRegistered,
		/// User has been already added to the project
		UserAlreadyAssignedToProject,
		/// Max number of users per project reached
		MaxUsersPerProjectReached,
		/// Max number of projects per user reached
		MaxProjectsPerUserReached,
		/// User is not assigned to the project
		UserNotAssignedToProject,
		/// Can not register administrator role
		CannotRegisterAdminRole,
		/// Max number of builders per project reached
		MaxBuildersPerProjectReached,
		/// Max number of investors per project reached
		MaxInvestorsPerProjectReached,
		/// Max number of issuers per project reached
		MaxIssuersPerProjectReached,
		/// Max number of regional centers per project reached
		MaxRegionalCenterPerProjectReached,
		/// Can not remove administrator role
		CannotRemoveAdminRole,
		/// Can not add admin role at user project assignment
		CannotAddAdminRole,
		/// User can not have more than one role at the same time
		UserCannotHaveMoreThanOneRole,
		/// Expenditure not found
		ExpenditureNotFound,
		/// Expenditure already exist
		ExpenditureAlreadyExists,
		/// Max number of expenditures per project reached
		MaxExpendituresPerProjectReached,
		/// Field name can not be empty
		EmptyExpenditureName,
		/// Expenditure does not belong to the project
		ExpenditureDoesNotBelongToProject,
		/// Drowdown id is not found
		DrawdownNotFound,
		/// Invalid amount
		InvalidAmount,
		/// Documents field is empty
		DocumentsIsEmpty,
		/// Transaction id is not found
		TransactionNotFound,
		/// Transaction already exist
		TransactionAlreadyExists,
		/// Max number of transactions per drawdown reached
		MaxTransactionsPerDrawdownReached,
		/// Drawdown already exist
		DrawdownAlreadyExists,
		/// Max number of drawdowns per project reached
		MaxDrawdownsPerProjectReached,
		/// Can not modify a completed drawdown
		CannotEditDrawdown,
		/// Can not perform any action on a submitted transaction
		CannotPerformActionOnSubmittedTransaction,
		/// Can not perform any action on a approved transaction
		CannotPerformActionOnApprovedTransaction,
		/// Can not perform any action on a submitted drawdown
		CannotPerformActionOnSubmittedDrawdown,
		/// Can not perform any action on a approved drawdown
		CannotPerformActionOnApprovedDrawdown,
		/// Transaction is already completed
		TransactionIsAlreadyCompleted,
		/// User does not have the specified role
		UserDoesNotHaveRole,
		/// Transactions vector is empty
		EmptyTransactions,
		/// Transaction ID was not found in do_execute_transaction
		TransactionIdNotFound,
		/// Drawdown can not be submitted if does not has any transactions
		DrawdownHasNoTransactions,
		/// Cannot submit transaction
		CannotSubmitTransaction,
		/// Drawdown can not be approved if is not in submitted status
		DrawdownIsNotInSubmittedStatus,
		/// Transactions is not in submitted status
		TransactionIsNotInSubmittedStatus,
		/// Array of expenditures is empty
		EmptyExpenditures,
		/// Expenditure name is required
		ExpenditureNameRequired,
		/// Expenditure type is required
		ExpenditureTypeRequired,
		/// Expenditure amount is required
		ExpenditureAmountRequired,
		/// Expenditure id is required
		ExpenditureIdRequired,
		/// User name is required
		UserNameRequired,
		/// User role is required
		UserRoleRequired,
		/// Amount is required
		AmountRequired,
		/// Can not delete a user if the user is assigned to a project
		UserHasAssignedProjects,
		/// Can not send a drawdown to submitted status if it has no transactions
		NoTransactionsToSubmit,
		/// Bulk upload description is required
		BulkUploadDescriptionRequired,
		/// Administrator can not delete themselves
		AdministratorsCannotDeleteThemselves,
		/// No feedback was provided for bulk upload
		NoFeedbackProvidedForBulkUpload,
		/// NO feedback for EN5 drawdown was provided
		EB5MissingFeedback,
		/// Inflation rate extrinsic is missing an array of project ids
		InflationRateMissingProjectIds,
		/// Inflation rate was not provided
		InflationRateRequired,
		/// Bulkupload drawdowns are only allowed for Construction Loan & Developer Equity
		DrawdownTypeNotSupportedForBulkUpload,
		/// Cannot edit user role if the user is assigned to a project
		UserHasAssignedProjectsCannotUpdateRole,
		/// Cannot delete user if the user is assigned to a project
		UserHasAssignedProjectsCannotDelete,
		/// Cannot send a bulkupload drawdown if the drawdown status isn't in draft or rejected
		DrawdownStatusNotSupportedForBulkUpload,
		/// Only investors can update/edit their documents
		UserIsNotAnInvestor,
		/// Max number of projects per builder has been reached
		MaxProjectsPerBuilderReached,
		/// Max number of projects per investor has been reached
		MaxProjectsPerInvestorReached,
		/// Max number of projects per issuer has been reached
		MaxProjectsPerIssuerReached,
		/// Max number of projects per regional center has been reached
		MaxProjectsPerRegionalCenterReached,
		/// Jobs eligibles array is empty
		JobEligiblesIsEmpty,
		/// JOb eligible name is required
		JobEligiblesNameIsRequired,
		/// Job eligible id already exists
		JobEligibleIdAlreadyExists,
		/// Max number of job eligibles per project reached
		MaxJobEligiblesPerProjectReached,
		/// Job eligible id not found
		JobEligibleNotFound,
		/// Jopb eligible does not belong to the project
		JobEligibleDoesNotBelongToProject,
		/// Job eligible name is required
		JobEligibleNameRequired,
		/// Job eligible amount is required
		JobEligibleAmountRequired,
		/// Job eligible id is required
		JobEligibleIdRequired,
		/// Revenue id was not found
		RevenueNotFound,
		/// Transactions revenue array is empty
		RevenueTransactionsAreEmpty,
		/// Revenue can not be edited 
		CannotEditRevenue,
		/// Revenue transaction id already exists
		RevenueTransactionIdAlreadyExists,
		/// Max number of transactions per revenue reached
		MaxTransactionsPerRevenueReached,
		/// Revenue transaction id not found
		RevenueTransactionNotFound,
		/// Revenue transaction can not be edited
		CannotEditRevenueTransaction,
		/// Can not perform any action on a submitted revenue
		CannotPerformActionOnSubmittedRevenue,
		/// Can not perform any action on a approved revenue
		CannotPerformActionOnApprovedRevenue,
		/// Can not perform any action on a submitted revenue transaction
		CannotPerformActionOnApprovedRevenueTransaction,
		/// Can not perform any action on a approved revenue transaction
		CannotPerformActionOnSubmittedRevenueTransaction,
		/// Revenue amoun is required
		RevenueAmountRequired,
		/// Revenue transaction id is required
		RevenueTransactionIdRequired,
	}

	// E X T R I N S I C S
	// ------------------------------------------------------------------------------------------------------------
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// I N I T I A L
		// --------------------------------------------------------------------------------------------
		/// Initialize the pallet by setting the permissions for each role
		/// & the global scope
		///
		/// # Considerations:
		/// - This function can only be called once
		/// - This function can only be called usinf the sudo pallet
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(10))]
		pub fn initial_setup(
			origin: OriginFor<T>,
		) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			Self::do_initial_setup()?;
			Ok(())
		}

		/// Adds an administrator account to the site
		///
		/// # Parameters:
		/// - origin: The sudo account
		/// - admin: The administrator account to be added
		/// - name: The name of the administrator account
		///
		/// # Considerations:
		/// - This function can only be called using the sudo pallet
		/// - This function is used to add the first administrator to the site
		/// - If the user is already registered, the function will return an error: UserAlreadyRegistered
		/// - This function grants administator permissions to the user from the rbac pallet
		/// - Administator role have global scope permissions
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(10))]
		pub fn sudo_add_administrator(
			origin: OriginFor<T>,
			admin: T::AccountId,
			name: FieldName,
		) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			Self::do_sudo_add_administrator(admin, name)?;
			Ok(())
		}

		/// Removes an administrator account from the site
		///
		/// # Parameters:
		/// - origin: The sudo account
		/// - admin: The administrator account to be removed
		///
		/// # Considerations:
		/// - This function can only be called using the sudo pallet
		/// - This function is used to remove any administrator from the site
		/// - If the user is not registered, the function will return an error: UserNotFound
		/// - This function removes administator permissions of the user from the rbac pallet
		///
		/// # Note:
		/// WARNING: Administrators can remove themselves from the site,
		/// but they can add themselves back
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(10))]
		pub fn sudo_remove_administrator(
			origin: OriginFor<T>,
			admin: T::AccountId
		) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			Self::do_sudo_remove_administrator(admin)?;
			Ok(())
		}


		// U S E R S
		// --------------------------------------------------------------------------------------------
		/// This extrinsic is used to create, update, or delete a user account
		///
		/// # Parameters:
		/// - origin: The administrator account
		/// - user: The target user account to be registered, updated, or deleted.
		/// It is an array of user accounts where each entry it should be a tuple of the following:
		/// - 0: The user account
		/// - 1: The user name
		/// - 2: The user role
		/// - 3: The CUD operation to be performed on the user account. CUD action is ALWAYS required.
		///
		/// # Considerations:
		/// - Users parameters are optional because depends on the CUD action as follows:
		/// * **Create**: The user account, user name, user role & CUD action are required
		/// * **Update**: The user account & CUD action are required. The user name & user role are optionals.
		/// * **Delete**: The user account & CUD action are required.
		/// - This function can only be called by an administrator account
		/// - Multiple users can be registered, updated, or deleted at the same time, but
		/// the user account must be unique. Multiple actions over the same user account
		/// in the same call, it could result in an unexpected behavior.
		/// - If the user is already registered, the function will return an error: UserAlreadyRegistered
		/// - If the user is not registered, the function will return an error: UserNotFound
		///
		/// # Note:
		/// WARNING: It is possible to register, update, or delete administators accounts using this extrinsic,
		/// but administrators can not delete themselves.
		/// WARNING: This function only registers, updates, or deletes users from the site.
		/// DOESN'T grant or remove permissions from the rbac pallet.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn users(
			origin: OriginFor<T>,
			users: BoundedVec<(
				T::AccountId, // account id
				Option<FieldName>, // name
				Option<ProxyRole>, // role
				CUDAction, // action
			), T::MaxRegistrationsAtTime>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_execute_users(who, users)
		}

		/// Edits an user account
		///
		/// # Parameters:
		/// - origin: The user account which is being edited
		/// - name: The name of the user account which is being edited
		/// - image: The image of the user account which is being edited
		/// - email: The email of the user account which is being edited
		/// - documents: The documents of the user account which is being edited.
		/// ONLY available for the investor role.
		///
		///
		/// # Considerations:
		/// - If the user is not registered, the function will return an error: UserNotFound
		/// - This function can only be called by a registered user account
		/// - This function will be called by the user account itself
		/// - ALL parameters are optional because depends on what is being edited
		/// - ONLY the investor role can edit or update the documents
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn users_edit_user(
			origin: OriginFor<T>,
			name: Option<FieldName>,
			image: Option<CID>,
			email: Option<FieldName>,
			documents: Option<Documents<T>>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_edit_user(who, name, image, email, documents)
		}

		// P R O J E C T S
		// --------------------------------------------------------------------------------------------
		/// Registers a new project.
		///
		/// # Parameters:
		/// - origin: The administrator account
		/// - title: The title of the project
		/// - description: The description of the project
		/// - image: The image of the project (CID)
		/// - address: The address of the project
		/// - creation_date: The creation date of the project
		/// - completion_date: The completion date of the project
		/// - expenditures: The expenditures of the project. It is an array of tuples where each entry
		/// is a tuple of the following:
		/// * 0: The expenditure name
		/// * 1: The expenditure type
		/// * 2: The expenditure amount
		/// * 3: The expenditure NAICS code
		/// * 4: The expenditure jobs multiplier
		/// * 5: The CUD action to be performed on the expenditure. CUD action is ALWAYS required.
		/// * 6: The expenditure id. It is optional because it is only required when updating or deleting
		/// - job_eligibles: The job eligibles to be created/updated/deleted. This is a vector of tuples
		/// where each entry is composed by:
		/// * 0: The job eligible name
		/// * 1: The amount of the job eligible
		/// * 2: The NAICS code of the job eligible
		/// * 3: The jobs multiplier of the job eligible
		/// * 4: The job eligible action to be performed. (Create, Update or Delete)
		/// * 5: The job eligible id. This is only used when updating or deleting a job eligible.
		/// - users: The users who will be assigned to the project. It is an array of tuples where each entry
		/// is a tuple of the following:
		/// * 0: The user account
		/// * 1: The user role
		/// * 2: The AssignAction to be performed on the user.
		///
		/// # Considerations:
		/// - This function can only be called by an administrator account
		/// - For users assignation, the user account must be registered. If the user is not registered,
		/// the function will return an error. ALL parameters are required.
		/// - For expenditures, apart from the expenditure id, naics code & jopbs multiplier, ALL parameters are required because for this
		/// flow, the expenditures are always created. The naics code & the jobs multiplier
		/// can be added later by the administrator.
		/// - Creating a project will automatically create a scope for the project.
		///
		/// # Note:
		/// WARNING: If users are provided, the function will assign the users to the project, granting them
		/// permissions in the rbac pallet.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_create_project(
			origin: OriginFor<T>,
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
				Option<ExpenditureId>
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
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_create_project(who, title, description, image, address, banks, creation_date, completion_date, expenditures, job_eligibles, users)
		}

		/// Edits a project.
		///
		/// # Parameters:
		/// - origin: The administrator account
		/// - project_id: The selected project id that will be edited
		/// - title: The title of the project to be edited
		/// - description: The description of the project to be edited
		/// - image: The image of the project to be edited
		/// - address: The address of the project to be edited
		/// - creation_date: The creation date of the project to be edited
		/// - completion_date: The completion date of the project to be edited
		///
		/// # Considerations:
		/// - This function can only be called by an administrator account
		/// - ALL parameters are optional because depends on what is being edited
		/// - The project id is required because it is the only way to identify the project
		/// - The project id must be registered. If the project is not registered,
		/// the function will return an error: ProjectNotFound
		/// - It is not possible to edit the expenditures or the users assigned to the project
		/// through this function. For that, the administrator must use the extrinsics:
		/// * expenditures
		/// * projects_assign_user
		/// - Project can only be edited in the Started status
		/// - Completion date must be greater than creation date
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_edit_project(
			origin: OriginFor<T>,
			project_id: ProjectId,
			title: Option<FieldName>,
			description: Option<FieldDescription>,
			image: Option<CID>,
			address: Option<FieldName>,
			banks: Option<BoundedVec<(BankName, BankAddress), T::MaxBanksPerProject>>,
			creation_date: Option<CreationDate>,
			completion_date: Option<CompletionDate>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_edit_project(who, project_id, title, description, image, address, banks, creation_date, completion_date)
		}

		/// Deletes a project.
		///
		/// # Parameters:
		/// - origin: The administrator account
		/// - project_id: The selected project id that will be deleted
		///
		/// # Considerations:
		/// - This function can only be called by an administrator account
		/// - The project id is required because it is the only way to identify the project
		/// - The project id must be registered. If the project is not registered,
		/// the function will return an error: ProjectNotFound
		///
		/// # Note:
		/// - WARNING: Deleting a project will delete ALL stored information associated with the project.
		/// BE CAREFUL.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_delete_project(
			origin: OriginFor<T>,
			project_id: ProjectId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_delete_project(who, project_id)
		}

		/// Assigns a user to a project.
		///
		/// # Parameters:
		/// - origin: The administrator account
		/// - project_id: The selected project id where user will be assigned
		/// - users: The users to be assigned to the project. This is a vector of tuples
		/// where each entry is composed by:
		/// * 0: The user account id
		/// * 1: The user role
		/// * 2: The AssignAction to be performed. (Assign or Unassign)
		///
		/// # Considerations:
		/// - This function can only be called by an administrator account
		/// - This extrinsic allows multiple users to be assigned/unassigned at the same time.
		/// - The project id is required because it is the only way to identify the project
		/// - This extrinsic is used for both assigning and unassigning users to a project
		/// depending on the AssignAction.
		/// - After a user is assigned to a project, the user will be able to perform actions
		/// in the project depending on the role assigned to the user.
		/// - After a user is unassigned from a project, the user will not be able to perform actions
		/// in the project anymore.
		/// - If the user is already assigned to the project, the function will return an erro.
		///
		/// # Note:
		/// - WARNING: ALL provided users needs to be registered in the site. If any of the users
		/// is not registered, the function will return an error.
		/// - Assigning or unassigning a user to a project will add or remove permissions to the user
		/// from the RBAC pallet.
		/// - Warning: Cannot assign a user to a project with a different role than the one they
		/// have in UsersInfo. If the user has a different role, the function will return an error.
		/// - Warning: Cannot unassign a user from a project with a different role than the one they
		/// have in UsersInfo. If the user has a different role, the function will return an error.
		/// - Warning: Do not perfom multiple actions over the same user in the same call, it could
		/// result in an unexpected behavior.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_assign_user(
			origin: OriginFor<T>,
			project_id: ProjectId,
			users: BoundedVec<(
				T::AccountId,
				ProxyRole,
				AssignAction,
			), T::MaxRegistrationsAtTime>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_execute_assign_users(who, project_id, users)
		}

		// B U D G E T  E X P E N D I T U R E
		// --------------------------------------------------------------------------------------------
		/// This extrinsic is used to create, update or delete expenditures.
		///
		/// # Parameters:
		/// - origin: The administrator account
		/// - project_id: The selected project id where the expenditures will be created/updated/deleted
		/// - expenditures: The expenditures to be created/updated/deleted. This is a vector of tuples
		/// where each entry is composed by:
		/// * 0: The name of the expenditure
		/// * 1: The expenditure type
		/// * 2: The amount of the expenditure
		/// * 3: The naics code of the expenditure
		/// * 4: The jobs multiplier of the expenditure
		/// * 5: The expenditure action to be performed. (Create, Update or Delete)
		/// * 6: The expenditure id. This is only used when updating or deleting an expenditure.
		/// - job_eligibles: The job eligibles to be created/updated/deleted. This is a vector of tuples
		/// where each entry is composed by:
		/// * 0: The job eligible name
		/// * 1: The amount of the job eligible
		/// * 2: The NAICS code of the job eligible
		/// * 3: The jobs multiplier of the job eligible
		/// * 4: The job eligible action to be performed. (Create, Update or Delete)
		/// * 5: The job eligible id. This is only used when updating or deleting a job eligible.
		///
		/// # Considerations:
		/// - Naics code and jobs multiplier are always optional.
		/// - This function can only be called by an administrator account
		/// - This extrinsic allows multiple expenditures to be created/updated/deleted at the same time.
		/// - The project id is required because it is the only way to identify the project
		/// - Expentiture parameters are optional because depends on the action to be performed:
		/// * **Create**: Name, Type & Amount are required. Nacis code & Jobs multiplier are optional.
		/// * **Update**: Except for the expenditure id & action, all parameters are optional.
		/// * **Delete**: Only the expenditure id & action is required.
		/// - Multiple actions can be performed at the same time. For example, you can create a new
		/// expenditure and update another one at the same time.
		/// - Do not perform multiple actions over the same expenditure in the same call, it could
		/// result in an unexpected behavior.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn expenditures_and_job_eligibles(
			origin: OriginFor<T>,
			project_id: ProjectId,
			expenditures: Option<BoundedVec<(
				Option<FieldName>, // name
				Option<ExpenditureType>, // type
				Option<ExpenditureAmount>, // amount
				Option<NAICSCode>, // naics code
				Option<JobsMultiplier>, // jobs multiplier
				CUDAction, // action
				Option<ExpenditureId>, // expenditure_id
			), T::MaxRegistrationsAtTime>>,
			job_eligibles: Option<BoundedVec<(
				Option<FieldName>, // name
				Option<JobEligibleAmount>, // amount
				Option<NAICSCode>, // naics code
				Option<JobsMultiplier>, // jobs multiplier
				CUDAction, // action
				Option<JobEligibleId>, // job_eligible_id
			), T::MaxRegistrationsAtTime>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			if let Some(mod_expenditures) = expenditures {
				Self::do_execute_expenditures(who.clone(), project_id, mod_expenditures)?;
			}

			if let Some(mod_job_eligibles) = job_eligibles {
				Self::do_execute_job_eligibles(who.clone(), project_id, mod_job_eligibles)?;
			}

			Ok(())
		}

		// T R A N S A C T I O N S   &  D R A W D O W N S
		// --------------------------------------------------------------------------------------------

		/// This extrinsic is used to create, update or delete transactions.
		/// Transactions status can be saved as draft or submitted.
		///
		/// # Parameters:
		/// - origin: The user account who is creating the transactions
		/// - project_id: The selected project id where the transactions will be created
		/// - drawdown_id: The selected drawdown id where the transactions will be created
		/// - transactions: The transactions to be created/updated/deleted. This is a vector of tuples
		/// where each entry is composed by:
		/// * 0: The expenditure id where the transaction will be created
		/// * 1: The transaction amount
		/// * 2: Documents associated to the transaction
		/// * 3: The transaction action to be performed. (Create, Update or Delete)
		/// * 4: The transaction id. This is only used when updating or deleting a transaction.
		/// - submit: If true, the transactions will be submitted.
		/// If false, the transactions array will be saved as draft.
		///
		/// # Considerations:
		/// - This function can only be called by a builder role account
		/// - This extrinsic allows multiple transactions to be created/updated/deleted at the same time.
		/// - The project id and drawdown id are required because are required for the reports.
		/// - Transaction parameters are optional because depends on the action to be performed:
		/// * **Create**: Expenditure id, Amount, Documents & Action are required.
		/// * **Update**: Except for the transaction id & action, all parameters are optional.
		/// * **Delete**: Only the transaction id & action is required.
		/// - Multiple actions can be performed at the same time. For example, you can create a new
		/// transaction and update another one at the same time.
		/// - Do not perform multiple actions over the same transaction in the same call, it could
		/// result in an unexpected behavior.
		/// - If a drawdown is submitted, all transactions must be submitted too. If the drawdown do not contain
		/// any transaction, it will be returned an error.
		/// - After a drawdown is submitted, it can not be updated or deleted.
		/// - After a drawdown is rejected, builders will use this extrinsic to update the transactions.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn submit_drawdown(
			origin: OriginFor<T>,
			project_id: ProjectId,
			drawdown_id: DrawdownId,
			transactions: Option<BoundedVec<(
				Option<ExpenditureId>, // expenditure_id
				Option<ExpenditureAmount>, // amount
				Option<Documents<T>>, //Documents
				CUDAction, // Action
				Option<TransactionId>, // transaction_id
			), T::MaxRegistrationsAtTime>>,
			submit: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin
			// Ensure builder permissions
			Self::is_authorized(who, &project_id, ProxyPermission::SubmitDrawdown)?;

			match submit{
				false => {
					// Do execute transactions
					Self::do_execute_transactions(
						project_id,
						drawdown_id,
						transactions.ok_or(Error::<T>::EmptyTransactions)?,
					)
				},
				true => {
					// Check if there are transactions to execute
					if let Some(transactions) = transactions {
						// Do execute transactions
						if transactions.len() > 0 {
							Self::do_execute_transactions(
								project_id,
								drawdown_id,
								transactions)?;
						}
					// 	Self::do_execute_transactions(
					// 		project_id,
					// 		drawdown_id,
					// 		transactions,
					// 	)?;
					}

					// Do submit drawdown
					Self::do_submit_drawdown(project_id, drawdown_id)
				},
			}

		}

		/// Approves a drawdown
		///
		/// # Parameters:
		/// ### For EB5 drawdowns:
		/// - origin: The administator account who is approving the drawdown
		/// - project_id: The selected project id where the drawdown will be approved
		/// - drawdown_id: The selected drawdown id to be approved
		///
		/// ### For Construction Loan & Developer Equity (bulk uploads) drawdowns:
		/// - origin: The administator account who is approving the drawdown
		/// - project_id: The selected project id where the drawdown will be approved
		/// - drawdown_id: The selected drawdown id to be approved.
		/// - bulkupload: Optional bulkupload parameter. If true, the drawdown will be saved in a pseudo
		/// draft status. If false, the drawdown will be approved directly.
		/// - transactions: The transactions to be created/updated/deleted. This is a vector of tuples
		/// where each entry is composed by:
		/// * 0: The expenditure id where the transaction will be created
		/// * 1: The transaction amount
		/// * 2: Documents associated to the transaction
		/// * 3: The transaction action to be performed. (Create, Update or Delete)
		/// * 4: The transaction id. This is only used when updating or deleting a transaction.
		///
		/// # Considerations:
		/// - This function can only be called by an administrator account
		/// - This extrinsic allows multiple transactions to be created/updated/deleted at the same time
		/// (only for Construction Loan & Developer Equity drawdowns).
		/// - Transaction parameters are optional because depends on the action to be performed:
		/// * **Create**: Expenditure id, Amount, Documents & Action are required.
		/// * **Update**: Except for the transaction id & action, all parameters are optional.
		/// * **Delete**: Only the transaction id & action is required.
		/// - Multiple actions can be performed at the same time. For example, you can create a new
		/// transaction and update another one at the same time.
		/// - Do not perform multiple actions over the same transaction in the same call, it could
		/// result in an unexpected behavior.
		/// - After a drawdown is approved, it can not be updated or deleted.
		/// - After a drawdown is approved, the next drawdown will be automatically created.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn approve_drawdown(
			origin: OriginFor<T>,
			project_id: ProjectId,
			drawdown_id: DrawdownId,
			bulkupload: Option<bool>,
			transactions: Option<BoundedVec<(
				Option<ExpenditureId>, // expenditure_id
				Option<u64>, // amount
				Option<Documents<T>>, //Documents
				CUDAction, // Action
				Option<TransactionId>, // transaction_id
			), T::MaxRegistrationsAtTime>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			// Match bulkupdate
			match bulkupload {
				Some(approval) => {
					// Execute bulkupload flow (construction loan & developer equity)
					match approval {
						false => {
							// 1. Do execute transactions
							Self::do_execute_transactions(
								project_id,
								drawdown_id,
								transactions.ok_or(Error::<T>::EmptyTransactions)?,
							)?;

							// 2. Do submit drawdown
							Self::do_submit_drawdown(project_id, drawdown_id)

						},
						true  => {
							// 1.Execute transactions if provided
							if let Some(transactions) = transactions {
								// Do execute transactions
								Self::do_execute_transactions(
									project_id,
									drawdown_id,
									transactions,
								)?;

								// 2. Submit drawdown
								Self::do_submit_drawdown(project_id, drawdown_id)?;
							}

							// 3. Approve drawdown
							Self::do_approve_drawdown(who, project_id, drawdown_id)
						},
					}

				},
				None => {
					// Execute normal flow (EB5)
					Self::do_approve_drawdown(who, project_id, drawdown_id)
				}
			}

		}

		/// Rejects a drawdown
		///
		/// # Parameters:
		/// - origin: The administator account who is rejecting the drawdown
		/// - project_id: The selected project id where the drawdown will be rejected
		/// - drawdown_id: The selected drawdown id to be rejected
		///
		/// Then the next two feedback parameters are optional because depends on the drawdown type:
		/// #### EB5 drawdowns:
		/// - transactions_feedback: Administrator will provide feedback for each transaction
		/// that is wrong. This is a vector of tuples where each entry is composed by:
		/// * 0: The transaction id
		/// * 1: The transaction feedback
		///
		/// #### Construction Loan & Developer Equity drawdowns:
		/// - drawdown_feedback: Administrator will provide feedback for the WHOLE drawdown.
		///
		/// # Considerations:
		/// - This function can only be called by an administrator account
		/// - This extrinsic allows multiple transactions to be rejected at the same time
		/// (only for EB5 drawdowns).
		/// - For EB5 drawdowns, the administrator can provide feedback for each transaction
		/// that is wrong.
		/// - For Construction Loan & Developer Equity drawdowns, the administrator can provide
		/// feedback for the WHOLE drawdown.
		/// - After a builder re-submits a drawdown, the administrator will have to review
		/// the drawdown again.
		/// - After a builder re-submits a drawdown, the feedback field will be cleared automatically.
		/// - If a single EB5 transaction is wrong, the administrator WILL reject the WHOLE drawdown.
		/// There is no way to reject a single transaction.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn reject_drawdown(
			origin: OriginFor<T>,
			project_id: ProjectId,
			drawdown_id: DrawdownId,
			transactions_feedback: Option<BoundedVec<(TransactionId, FieldDescription), T::MaxRegistrationsAtTime>>,
			drawdown_feedback: Option<FieldDescription>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_reject_drawdown(who, project_id, drawdown_id, transactions_feedback, drawdown_feedback)
		}

		/// Bulk upload drawdowns.
		///
		/// # Parameters:
		/// - origin: The administator account who is uploading the drawdowns
		/// - project_id: The selected project id where the drawdowns will be uploaded
		/// - drawdown_id: The drawdowns to be uploaded
		/// - description: The description of the drawdown provided by the builder
		/// - total_amount: The total amount of the drawdown
		/// - documents: The documents provided by the builder for the drawdown
		///
		/// # Considerations:
		/// - This function can only be called by a builder account
		/// - This extrinsic allows only one drawdown to be uploaded at the same time.
		/// - The drawdown will be automatically submitted.
		/// - Only available for Construction Loan & Developer Equity drawdowns.
		/// - After a builder uploads a drawdown, the administrator will have to review it.
		/// - After a builder re-submits a drawdown, the feedback field will be cleared automatically.
		/// - Bulkuploads does not allow individual transactions.
		/// - After a builder uploads a drawdown, the administrator will have to
		/// insert each transaction manually.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn up_bulkupload(
			origin: OriginFor<T>,
			project_id: ProjectId,
			drawdown_id: DrawdownId,
			description: FieldDescription,
			total_amount: TotalAmount,
			documents: Documents<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be a builder

			Self::do_up_bulk_upload(who, project_id, drawdown_id, description, total_amount, documents)
		}

		/// Modifies the inflation rate of a project.
		///
		/// # Parameters:
		/// - origin: The administator account who is modifying the inflation rate
		/// - projects: The projects where the inflation rate will be modified.
		/// This is a vector of tuples where each entry is composed by:
		/// * 0: The project id
		/// * 1: The inflation rate
		/// * 2: The action to be performed (Create, Update or Delete)
		///
		/// # Considerations:
		/// - This function can only be called by an administrator account
		/// - This extrinsic allows multiple projects to be modified at the same time.
		/// - The inflation rate can be created, updated or deleted.
		/// - The inflation rate is optional because depends on the CUDAction parameter:
		/// * **Create**: The inflation rate will be created. Project id, inflation rate and action are required.
		/// * **Update**: The inflation rate will be updated. Project id, inflation rate and action are required.
		/// * **Delete**: The inflation rate will be deleted. Project id and action are required.
		/// - The inflation rate can only be modified if the project is in the "started" status.
		///
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn inflation_rate(
			origin: OriginFor<T>,
			projects: BoundedVec<(ProjectId, Option<InflationRate>, CUDAction), T::MaxRegistrationsAtTime>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_execute_inflation_adjustment(who, projects)
		}

		// #[transactional]
		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn job_eligibles(
		// 	origin: OriginFor<T>,
		// 	project_id: ProjectId,
		// 	job_eligibles: BoundedVec<(
		// 		Option<FieldName>, // name
		// 		Option<JobEligibleAmount>, // amount
		// 		Option<FieldDescription>, // naics code
		// 		Option<JobsMultiplier>, // jobs multiplier
		// 		CUDAction, // action
		// 		Option<JobEligibleId>, // job_eligible_id
		// 	), T::MaxRegistrationsAtTime>,
		// ) -> DispatchResult {
		// 	let who = ensure_signed(origin)?; // origin need to be an admin

		// 	Self::do_execute_job_eligibles(who, project_id, job_eligibles)
		// }

		/// Kill all the stored data.
		///
		/// This function is used to kill ALL the stored data.
		/// Use it with caution!
		///
		/// ### Parameters:
		/// - `origin`: The user who performs the action.
		///
		/// ### Considerations:
		/// - This function is only available to the `admin` with sudo access.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn kill_storage(
			origin: OriginFor<T>,
		) -> DispatchResult{
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			let _ = <GlobalScope<T>>::kill();
			let _ = <UsersInfo<T>>::clear(1000, None);
			let _ = <ProjectsInfo<T>>::clear(1000, None);
			let _ = <UsersByProject<T>>::clear(1000, None);
			let _ = <ProjectsByUser<T>>::clear(1000, None);
			let _ = <ExpendituresInfo<T>>::clear(1000, None);
			let _ = <ExpendituresByProject<T>>::clear(1000, None);
			let _ = <DrawdownsInfo<T>>::clear(1000, None);
			let _ = <DrawdownsByProject<T>>::clear(1000, None);
			let _ = <TransactionsInfo<T>>::clear(1000, None);
			let _ = <TransactionsByDrawdown<T>>::clear(1000, None);

			//TODO: Remove Revenues storages

			T::Rbac::remove_pallet_storage(Self::pallet_id())?;
			Ok(())
		}

	}
}
