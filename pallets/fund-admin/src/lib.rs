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
//TODO: Remobe unused parameters, types, etc used for development
// - Remove unused constants
// - Change extrinsic names
// - Update extrinsic names to beign like CURD actions ( create, update, read, delete)
// - Remove unused pallet errors
// - Remove unused pallet events
// - Add internal documentation for each extrinsic
// - Add external documentation for each extrinsic
// - Update hasher for each storage map depending on the use case 
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
		type ProjectNameMaxLen: Get<u32>;

		#[pallet::constant]
		type ProjectDescMaxLen: Get<u32>;

		#[pallet::constant]
		type MaxDocuments: Get<u32>;

		#[pallet::constant]
		type MaxAccountsPerTransaction: Get<u32>;

		#[pallet::constant]
		type MaxProjectsPerUser: Get<u32>;

		#[pallet::constant]
		type MaxUserPerProject: Get<u32>;

		#[pallet::constant]
		type CIDMaxLen: Get<u32>;

		#[pallet::constant]
		type MaxBuildersPerProject: Get<u32>;

		#[pallet::constant]
		type MaxInvestorsPerProject: Get<u32>;

		#[pallet::constant]
		type MaxIssuersPerProject: Get<u32>;

		#[pallet::constant]
		type MaxRegionalCenterPerProject: Get<u32>;

		#[pallet::constant]
		type MaxBoundedVecs: Get<u32>;

		#[pallet::constant]
		type MaxDrawdownsPerProject: Get<u32>;

		#[pallet::constant]
		type MaxTransactionsPerDrawdown: Get<u32>;

		#[pallet::constant]
		type MaxRegistrationsAtTime: Get<u32>;

		#[pallet::constant]
		type MaxDrawdownsByStatus: Get<u32>;

		#[pallet::constant]
		type MaxExpendituresPerProject: Get<u32>;
		
		
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
		Identity, 
		T::AccountId, // Key account_id
		UserData<T>,  // Value UserData<T>
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn projects_info)]
	pub(super) type ProjectsInfo<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], // Key project_id
		ProjectData<T>,  // Value ProjectData<T>
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn users_by_project)]
	pub(super) type UsersByProject<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], // Key project_id
		BoundedVec<T::AccountId, T::MaxUserPerProject>,  // Value users
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn projects_by_user)]
	pub(super) type ProjectsByUser<T: Config> = StorageMap<
		_, 
		Identity, 
		T::AccountId, // Key account_id
		BoundedVec<[u8;32], T::MaxProjectsPerUser>,  // Value projects
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn expenditures_info)]
	pub(super) type ExpendituresInfo<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], // Key expenditure_id
		ExpenditureData,  // Value ExpenditureData<T>
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn expenditures_by_project)]
	pub(super) type ExpendituresByProject<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], // Key project_id
		BoundedVec<[u8;32], T::MaxExpendituresPerProject>,  // Value expenditures
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn drawdowns_info)]
	pub(super) type DrawdownsInfo<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], // Key drawdown id
		DrawdownData<T>,  // Value DrawdownData<T>
		OptionQuery,
	>;
	
	#[pallet::storage]
	#[pallet::getter(fn drawdowns_by_project)]
	pub(super) type DrawdownsByProject<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], // Key project_id
		BoundedVec<[u8;32], T::MaxDrawdownsPerProject>,  // Value Drawdowns
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn transactions_info)]
	pub(super) type TransactionsInfo<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], // Key transaction id
		TransactionData<T>,  // Value TransactionData<T>
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn transactions_by_drawdown)]
	pub(super) type TransactionsByDrawdown<T: Config> = StorageDoubleMap<
		_, 
		Identity, 
		[u8;32], //K1: project id
		Identity, 
		[u8;32], //K2: drawdown id
		BoundedVec<[u8;32], T::MaxTransactionsPerDrawdown>, // Value transactions
		ValueQuery
	>; 

	// E V E N T S
	// ------------------------------------------------------------------------------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Project was created
		ProjectCreated(T::AccountId, [u8;32]),
		/// Proxy setup completed
		ProxySetupCompleted,
		/// User registered successfully
		UserAdded(T::AccountId),
		/// Project was edited
		ProjectEdited([u8;32]),
		/// Project was deleted
		ProjectDeleted([u8;32]),
		/// Administator added
		AdministratorAssigned(T::AccountId),
		/// Administator removed
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
		/// A bugdet was created successfully
		BudgetCreated([u8;32]),
		/// Expenditure was edited successfully
		ExpenditureEdited([u8;32]),
		/// Expenditure was deleted successfully
		ExpenditureDeleted([u8;32]),
		/// Trasactions was completed successfully
		TransactionsCompleted,
		/// Transaction was created successfully
		TransactionCreated([u8;32]),
		/// Transaction was edited successfully
		TransactionEdited([u8;32]),
		/// Transaction was deleted successfully
		TransactionDeleted([u8;32]),
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

	}

	// E R R O R S
	// ------------------------------------------------------------------------------------------------------------
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		/// TODO: map each constant type used by bounded vecs to a pallet error
		/// when boundaries are exceeded
		/// TODO: Update and remove unused  pallet errors
		NoneValue,
		/// Project ID is already in use
		ProjectIdAlreadyInUse,
		/// Timestamp error
		TimestampError,
		/// Completion date must be later than creation date
		CompletionDateMustBeLater,
		/// User is already registered
		UserAlreadyRegistered,
		/// Project is not found
		ProjectNotFound,
		///Date can not be in the past
		DateCanNotBeInThePast,
		/// Project is not active anymore
		ProjectIsAlreadyCompleted,
		/// Can not delete a completed project
		CannotDeleteCompletedProject,
		/// Global scope is not set
		GlobalScopeNotSet,
		/// User is not registered
		UserNotRegistered,
		/// User has been already added to the project
		UserAlreadyAssignedToProject,
		/// Max number of users per project reached
		MaxUsersPerProjectReached,
		/// Max number of projects per user reached
		MaxProjectsPerUserReached,
		/// User already has the role
		UserAlreadyHasRole,
		/// User is not assigned to the project
		UserNotAssignedToProject,
		/// Can not register administator role 
		CannotRegisterAdminRole,
		/// Max number of builders per project reached
		MaxBuildersPerProjectReached,
		/// Max number of investors per project reached
		MaxInvestorsPerProjectReached,
		/// Max number of issuers per project reached
		MaxIssuersPerProjectReached,
		/// Max number of regional centers per project reached
		MaxRegionalCenterPerProjectReached,
		/// Can not remove administator role
		CannotRemoveAdminRole,
		/// Can not delete an user with active projects
		CannotDeleteUserWithAssignedProjects,
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
		/// Name for expenditure is too long
		NameTooLong,
		/// There is no expenditure with such project id
		NoExpendituresFound, 
		/// Field name can not be empty
		EmptyExpenditureName,
		/// Expenditure does not belong to the project
		ExpenditureDoesNotBelongToProject,
		/// There is no budgets for the project
		ThereIsNoBudgetsForTheProject,
		/// Budget id is not found
		BudgetNotFound,
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
		/// Can not delete a completed drawdown
		CannotDeleteCompletedDrawdown,
		/// Can not modify a transaction at this moment
		CannotEditTransaction,
		/// Can not delete a completed transaction
		CannotDeleteCompletedTransaction,
		/// Drawdown is already completed
		DrawdownIsAlreadyCompleted,
		/// Transaction is already completed
		TransactionIsAlreadyCompleted,
		/// Expenditure type does not match project type
		InvalidExpenditureType,
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
		/// Drawdown can not be submitted
		CannotSubmitDrawdown,
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
		/// Administator can not delete themselves
		AdministatorsCannotDeleteThemselves,
		/// No transactions were provided for bulk upload
		NoTransactionsProvidedForBulkUpload,
		/// No feedback was provided for bulk upload
		NoFeedbackProvidedForBulkUpload,
		/// Feedback provided for bulk upload should be one
		FeedbackProvidedForBulkUploadShouldBeOne,
		/// Bulkupdate param is missed to execute a bulkupload drawdown
		BulkUpdateIsRequired,
		/// NO feedback for EN5 drawdown was provided
		EB5FeebackMissing,
		/// Inflation rate extrinsic is missing an array of changes
		ProjectsIsEmpty,
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

	}

	// E X T R I N S I C S
	// ------------------------------------------------------------------------------------------------------------
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// I N I T I A L 
		// --------------------------------------------------------------------------------------------
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(10))]
		pub fn initial_setup(
			origin: OriginFor<T>,
		) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			Self::do_initial_setup()?;
			Ok(())
		}

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
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn users(
			origin: OriginFor<T>, 
			users: BoundedVec<(
				T::AccountId, // account id
				Option<BoundedVec<FieldName, T::MaxBoundedVecs>>, // name
				Option<ProxyRole>, // role
				CUDAction, // action
			), T::MaxRegistrationsAtTime>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_execute_users(who, users)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn users_edit_user(
			origin: OriginFor<T>, 
			user: T::AccountId, 
			name: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,
			image: Option<BoundedVec<CID, T::MaxBoundedVecs>>,
			email: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,
			documents: Option<Documents<T>> 
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			Self::do_edit_user(user, name, image, email, documents)
		}	

		// P R O J E C T S
		// --------------------------------------------------------------------------------------------
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_create_project(
			origin: OriginFor<T>, 
			title: FieldName, 
			description: FieldDescription, 
			image: CID, 
			address: FieldName,
			creation_date: u64,
			completion_date: u64, 
			expenditures: BoundedVec<(
				Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,
				Option<ExpenditureType>,
				Option<u64>,
				Option<u32>,
				Option<u32>,
				CUDAction,
				Option<[u8;32]>,
			), T::MaxRegistrationsAtTime>,
			users: Option<BoundedVec<(
				T::AccountId, 
				ProxyRole,
				AssignAction,
			), T::MaxRegistrationsAtTime>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_create_project(who, title, description, image, address, creation_date, completion_date, expenditures, users)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_edit_project(
			origin: OriginFor<T>, 
			project_id: [u8;32], 
			title: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,
			description: Option<BoundedVec<FieldDescription, T::MaxBoundedVecs>>,
			image: Option<BoundedVec<CID, T::MaxBoundedVecs>>,
			address: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,
			creation_date: Option<u64>,
			completion_date: Option<u64>,  
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_edit_project(who, project_id, title, description, image, address, creation_date, completion_date)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_delete_project(
			origin: OriginFor<T>, 
			project_id: [u8;32],  
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_delete_project(who, project_id)
		}

		// Users: (user, role, assign action)
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_assign_user(
			origin: OriginFor<T>, 
			project_id: [u8;32],  
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
		
		// Expenditures: (name, type, amount, naics code, jobs multiplier, CUDAction, expenditure_id)
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn expenditures(
			origin: OriginFor<T>, 
			project_id: [u8;32], 
			expenditures: BoundedVec<(
				Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,
				Option<ExpenditureType>,
				Option<u64>,
				Option<u32>,
				Option<u32>,
				CUDAction,
				Option<[u8;32]>,
			), T::MaxRegistrationsAtTime>,  
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_execute_expenditures(who, project_id, expenditures)
		}

		// T R A N S A C T I O N S   &  D R A W D O W N S
		// --------------------------------------------------------------------------------------------

		/// Create a transaction
		/// 
		/// - `project_id`: The project id
		/// - `drawdown_id`: The drawdown id
		/// - `transactions`: The array of transactions as follows:
		/// 	- `expenditure_id`: The expenditure id
		/// 	- `amount`: The amount
		/// 	- `documents`: The array of documents
		/// 	- `CUDAction`: The action to perform (create, update, delete)
		/// 	- `transaction_id`: The transaction id
		/// 	Note that all parameters are optional because 
		///     it depends on the action to perform
		/// - `submit`: Boolean to indicate if the drawdown is submitted or 
		/// saved as draft
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn submit_drawdown(
			origin: OriginFor<T>, 
			project_id: [u8;32],
			drawdown_id: [u8;32],
			transactions: Option<BoundedVec<(
				Option<[u8;32]>, // expenditure_id
				Option<u64>, // amount
				Option<Documents<T>>, //Documents
				CUDAction, // Action
				Option<[u8;32]>, // transaction_id
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
					// Check if there's transactions to execute
					if let Some(transactions) = transactions {
						// Do execute transactions
						Self::do_execute_transactions(
							project_id,
							drawdown_id,
							transactions,
						)?;
					}

					// Do submit drawdown
					Self::do_submit_drawdown(project_id, drawdown_id)
				},
			}

		}

		/// Approve a drawdown
		/// 
		/// - `origin`: The admin
		/// - `project_id`: The project id
		/// - `drawdown_id`: The drawdown id
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn approve_drawdown(
			origin: OriginFor<T>, 
			project_id: [u8;32],
			drawdown_id: [u8;32],
			bulkupload: Option<bool>,
			transactions: Option<BoundedVec<(
				Option<[u8;32]>, // expenditure_id
				Option<u64>, // amount
				Option<Documents<T>>, //Documents
				CUDAction, // Action
				Option<[u8;32]>, // transaction_id
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

		/// Reject a drawdown
		/// 
		/// - `origin`: The admin
		/// - `project_id`: The project id
		/// - `drawdown_id`: The drawdown id
		/// 
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn reject_drawdown(
			origin: OriginFor<T>, 
			project_id: [u8;32],
			drawdown_id: [u8;32],
			transactions_feedback: Option<BoundedVec<([u8;32], FieldDescription), T::MaxRegistrationsAtTime>>,
			drawdown_feedback: Option<BoundedVec<FieldDescription, T::MaxBoundedVecs>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_reject_drawdown(who, project_id, drawdown_id, transactions_feedback, drawdown_feedback)
		}

		/// Bulk upload drawdowns
		/// 
		/// This extrinsic is called by the builder 
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn up_bulkupload(
			origin: OriginFor<T>, 
			project_id: [u8;32],
			drawdown_id: [u8;32],
			description: FieldDescription,
			total_amount: u64,
			documents: Documents<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be a builder

			Self::do_up_bulk_upload(who, project_id, drawdown_id, description, total_amount, documents)
		}

		/// Modify inflation rate 
		/// 
		/// projects: project_id, inflation_rate, CUDAction
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn inflation_rate(
			origin: OriginFor<T>, 
			projects: BoundedVec<([u8;32], Option<u32>, CUDAction), T::MaxRegistrationsAtTime>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be a builder

			Self::do_execute_inflation_adjustment(who, projects)
		}


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

			T::Rbac::remove_pallet_storage(Self::pallet_id())?;
			Ok(())
		}

		// /// Testing extrinsic  
		// #[transactional]
		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn testing_extrinsic(
		// 	origin: OriginFor<T>, 
		// 	transactions: BoundedVec<(
		// 		[u8;32], // expenditure_id
		// 		u64, // amount
		// 		Option<Documents<T>> //Documents
		// 	), T::MaxRegistrationsAtTime>,
		// ) -> DispatchResult {
		// 	let who = ensure_signed(origin)?; // origin need to be an admin

		// 	Self::do_execute_transactions(who, transactions)
		// }


	}
}