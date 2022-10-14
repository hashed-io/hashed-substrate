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

		//TODO: Update pallet errors related to bounded vecs bounds
		//ie: BoundedVec<T::AccountId, T::MaxDevelopersPerProject>
		// -> MaxDevelopersPerProjectReached 

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
		type MaxDevelopersPerProject: Get<u32>;

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
		[u8;32], // Value gobal scope id
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
		/// User assigned to project
		UserAssignedToProject,
		/// User removed from project
		UserUnassignedFromProject(T::AccountId, [u8;32]),
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
		/// Completition date must be later than creation date
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
		/// Max number of developers per project reached
		MaxDevelopersPerProjectReached,
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
		FieldNameCannotBeEmpty,
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
		pub fn users_register_user(
			origin: OriginFor<T>, 
			users: BoundedVec<(T::AccountId, FieldName, ProxyRole), T::MaxRegistrationsAtTime>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_register_user(who, users)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn users_update_user(
			origin: OriginFor<T>, 
			user: T::AccountId, 
			name: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,
			image: Option<BoundedVec<CID, T::MaxBoundedVecs>>,
			email: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,
			documents: Option<Documents<T>> 
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_update_user(who, user, name, image, email, documents)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn users_delete_user(
			origin: OriginFor<T>, 
			user: T::AccountId, 
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_delete_user(who, user)
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
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_create_project(who, title, description, image, address, project_type, completion_date, expenditures, users)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_edit_project(
			origin: OriginFor<T>, 
			project_id: [u8;32], 
			tittle: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,	
			description: Option<BoundedVec<FieldDescription, T::MaxBoundedVecs>>,
			image: Option<BoundedVec<CID, T::MaxBoundedVecs>>,
			adress: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>,
			completition_date: Option<u64>,  
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin
			//TOREVIEW: Should we allow project_type modification? 
			// It implies to change their expenditure types and so on...
			Self::do_edit_project(who, project_id, tittle, description, image, adress, completition_date)
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

		// Users: (user, role)
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_assign_user(
			origin: OriginFor<T>, 
			project_id: [u8;32],  
			users: BoundedVec<(T::AccountId, ProxyRole), T::MaxRegistrationsAtTime>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_assign_user(who, project_id, users)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_unassign_user(
			origin: OriginFor<T>, 
			user: T::AccountId,
			project_id: [u8;32],  
			role: ProxyRole,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_unassign_user(who, user, project_id, role)
		}


		// B U D G E T  E X P E N D I T U R E 
		// --------------------------------------------------------------------------------------------
		
		// Expenditures: (name, type, amount, naics code, jobs multiplier)
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn expenditures_create_expenditure(
			origin: OriginFor<T>, 
			project_id: [u8;32], 
			expenditures: BoundedVec<(
				FieldName,
				ExpenditureType,
				u64,
				Option<u32>,
				Option<u32>,
			), T::MaxRegistrationsAtTime>,  
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_create_expenditure(who, project_id, expenditures)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn expenditures_edit_expenditure(
			origin: OriginFor<T>, 
			project_id: [u8;32], 
			expenditure_id: [u8;32],
			name: Option<BoundedVec<FieldName, T::MaxBoundedVecs>>, 
			expenditure_amount: Option<u64>,
			naics_code: Option<u32>,
			jobs_multiplier: Option<u32>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_edit_expenditure(who, project_id, expenditure_id, name, expenditure_amount, naics_code, jobs_multiplier)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn expenditures_delete_expenditure(
			origin: OriginFor<T>, 
			project_id: [u8;32], 
			expenditure_id: [u8;32],
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_delete_expenditure(who, project_id, expenditure_id)
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