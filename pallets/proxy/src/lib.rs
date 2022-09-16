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

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, BoundedVec};
	use frame_system::pallet_prelude::*;
	use frame_support::transactional;
	use sp_runtime::traits::Scale;
	use frame_support::traits::{Time};

	use crate::types::*;
	use pallet_rbac::types::RoleBasedAccessControl;


	#[pallet::config]
	pub trait Config: frame_system::Config {
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
		type MaxChildrens: Get<u32>;

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



		

	
		
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/*--- Onchain storage section ---*/

	#[pallet::storage]
	#[pallet::getter(fn users_info)]
	pub(super) type UsersInfo<T: Config> = StorageMap<
		_, 
		Identity, 
		T::AccountId, // Key
		UserData<T>,  // Value
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn projects)]
	pub(super) type ProjectsInfo<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], // Key project_id
		ProjectData<T>,  // Value
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn users_by_project)]
	pub(super) type UsersByProject<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], // Key project_id
		BoundedVec<T::AccountId, T::MaxUserPerProject>,  // Value
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn global_scope)]
	pub(super) type GlobalScope<T> = StorageValue<_, [u8;32], ValueQuery>;


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
		/// Project was deletedº
		ProjectDeleted([u8;32]),
		// administator added
		AdministratorAssigned(T::AccountId),
		// administator removed
		AdministratorRemoved(T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		/// TODO: map each constant type used by bounded vecs to a descriptive error
		NoneValue,
		/// Project ID is already in use
		ProjectIdAlreadyInUse,
		/// Timestamp error
		TimestampError,
		/// Completition date must be later than creation date
		CompletitionDateMustBeLater,
		/// User is already registered
		UserAlreadyRegistered,
		/// Project is not found
		ProjectNotFound,
		///Date can not be in the past
		DateCanNotBeInThePast,
		/// Can not modify project
		CannotEditCompletedProject,
		/// Creation date must be in the past
		CreationDateMustBeInThePast,
		/// Can not delete a completed project
		CannotDeleteCompletedProject,
		/// Global scope is not set
		GlobalScopeNotSet,

	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// I N I T I A L 
		// --------------------------------------------------------------------------------------------
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(10))]
		pub fn initial_setup(origin: OriginFor<T>) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			Self::do_initial_setup()?;
			Ok(())
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(10))]
		pub fn sudo_add_administrator(origin: OriginFor<T>, admin: T::AccountId) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			Self::do_sudo_add_administrator(admin)?;
			Ok(())
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(10))]
		pub fn sudo_remove_administrator(origin: OriginFor<T>, admin: T::AccountId) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			Self::do_sudo_remove_administrator(admin)?;
			Ok(())
		}




		// A C C O U N T S
		// --------------------------------------------------------------------------------------------
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn accounts_register_user(
			origin: OriginFor<T>, 
			user: T::AccountId, 
			role: ProxyRole, 
			related_projects: Option<BoundedVec<[u8;32], T::MaxProjectsPerUser>>, 
			documents: Option<BoundedVec<u8, T::MaxDocuments>> 
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_register_user(who, user, role, related_projects, documents)
		}
		
		// P R O J E C T S
		// --------------------------------------------------------------------------------------------
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_create_project(
			origin: OriginFor<T>, 
			tittle: BoundedVec<u8, T::ProjectNameMaxLen>, 
			description: BoundedVec<u8, T::ProjectDescMaxLen>, 
			image:  BoundedVec<u8, T::CIDMaxLen>, 
			completition_date: u64, 
			developer: Option<BoundedVec<T::AccountId, T::MaxDevelopersPerProject>>, 
			investor: Option<BoundedVec<T::AccountId, T::MaxInvestorsPerProject>>, 
			issuer: Option<BoundedVec<T::AccountId, T::MaxIssuersPerProject>>, 
			regional_center: Option<BoundedVec<T::AccountId, T::MaxRegionalCenterPerProject>>, 
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_create_project(who, tittle, description, image, completition_date, developer, investor, issuer, regional_center)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_edit_project(
			origin: OriginFor<T>, 
			project_id: [u8;32], 
			tittle: Option<BoundedVec<u8, T::ProjectNameMaxLen>>,
			description: Option<BoundedVec<u8, T::ProjectDescMaxLen>>, 
			image:  Option<BoundedVec<u8, T::CIDMaxLen>>, 
			creation_date: Option<u64>, 
			completition_date: Option<u64>,  
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin need to be an admin

			Self::do_edit_project(who, project_id, tittle, description, image, creation_date, completition_date)
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


		// B U D G E T  E X P E N D I T U R E 
		// --------------------------------------------------------------------------------------------


	}
}