#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;


mod functions;
pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::{*, ValueQuery};
	use frame_support::traits::{PalletInfoAccess};
	use frame_support::{transactional};
	use frame_system::pallet_prelude::*;
	use crate::types::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type MaxScopesPerPallet: Get<u32>;

		type MaxRolesPerPallet: Get<u32>;

		type RoleMaxLen: Get<u32>;
		
		type PermissionMaxLen: Get<u32>;

		type MaxPermissionsPerRole: Get<u32>;

		type MaxRolesPerUser: Get<u32>;

		type MaxUsersPerRole: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/*--- Onchain storage section ---*/

	#[pallet::storage]
	#[pallet::getter(fn scopes)]
	pub(super) type Scopes<T: Config> = StorageMap<
		_, 
		Blake2_128Concat, 
		u64, // pallet_id
		BoundedVec<[u8;32], T::MaxScopesPerPallet>,  // scopes_id
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn roles)]
	pub(super) type Roles<T: Config> = StorageMap<
		_,
		Identity, 
		[u8;32], // role_id
		BoundedVec<u8, T::RoleMaxLen >,  // role
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn pallet_roles)]
	pub(super) type PalletRoles<T: Config> = StorageMap<
		_,
		Blake2_128Concat, 
		u64, // pallet_id
		BoundedVec<[u8;32], T::MaxRolesPerPallet >, // role_id
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn permissions)]
	pub(super) type Permissions<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat, 
		u64, 			// pallet_id
		Blake2_128Concat, 
		[u8;32],		// permission_id
		BoundedVec<u8, T::PermissionMaxLen >,	// permission str
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn permissions_by_role)]
	pub(super) type PermissionsByRole<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat, 
		u64, 			// pallet_id
		Blake2_128Concat, 
		[u8;32],		// role_id
		BoundedVec<[u8;32], T::MaxPermissionsPerRole >,	// permission_ids
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn roles_by_user)]
	pub(super) type RolesByUser<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::AccountId>,// user
			// getting "the trait bound `usize: scale_info::TypeInfo` is not satisfied" errors
			NMapKey<Blake2_128Concat, u64>,			// pallet_id
			NMapKey<Twox64Concat, [u8;32]>,		// scope_id
		),
		BoundedVec<[u8;32], T::MaxRolesPerUser>,	// roles (ids)
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn users_by_scope)]
	pub(super) type UsersByScope<T: Config> = StorageNMap<
		_,
		(
			// getting "the trait bound `usize: scale_info::TypeInfo` is not satisfied" errors
			//  on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.
			NMapKey<Blake2_128Concat, u64>,		// pallet_id
			NMapKey<Twox64Concat, [u8;32]>,		// scope_id
			NMapKey<Blake2_128Concat, [u8;32]>,	// role_id
		),
		BoundedVec<T::AccountId, T::MaxUsersPerRole>,	// users
		ValueQuery,
	>;




	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// The pallet doesn't have scopes associated
		PalletNotFound,
		/// The specified scope doesn't exists
		ScopeNotFound,
		/// The scope is already linked with the pallet
		ScopeAlreadyExists,
		/// The specified role doesn't exist
		RoleNotFound,
		/// The permission doesn't exist in the pallet
		PermissionNotFound,
		/// The specified user hasn't been asigned to this scope
		UserNotFound,
		/// The role is already linked in the pallet
		DuplicateRole,
		/// The permission is already linked to that role in that scope
		DuplicatePermission,
		/// The user has that role asigned in that scope
		UserAlreadyHasRole,
		/// The role exists but it hasn't been linked to the pallet
		RoleNotLinkedToPallet,
		/// The user doesn't have any roles in this pallet
		UserHasNoRoles,
		/// The role doesn't have any users assigned to it
		RoleHasNoUsers,
		/// The pallet has too many scopes
		ExceedMaxScopesPerPallet,
		/// The pallet cannot have more roles
		ExceedMaxRolesPerPallet,
		/// The specified role cannot have more permission in this scope
		ExceedMaxPermissionsPerRole,
		/// The user cannot have more roles in this scope
		ExceedMaxRolesPerUser,
		/// This role cannot have assigned to more users in this scope
		ExceedMaxUsersPerRole,
		/// The role string is too long
		ExceedRoleMaxLen,
		/// The permission string is too long
		ExceedPermissionMaxLen,
		/// The user does not have the specified role 
		NotAuthorized,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn get_pallet_id(
			origin: OriginFor<T>, 
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;
			let a = Self::index();
			log::info!("henlo {:?}", a);
			log::warn!("Name: {:?}  Module Name: {:?}",Self::name(), Self::module_name());
			Self::deposit_event(Event::SomethingStored(a.try_into().unwrap(), who));

			Ok(())
		}
	}
}