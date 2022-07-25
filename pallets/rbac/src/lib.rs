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
mod types;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::{*, ValueQuery};
	use frame_support::traits::{PalletInfoAccess};
use frame_support::{PalletId, transactional};
	use frame_system::pallet_prelude::*;
	use crate::types::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type MaxScopesPerPallet: Get<u32>;

		type MaxRolesPerPallet: Get<u32>;

		type PermissionMaxLen: Get<u32>;

		type MaxPermissionsPerRole: Get<u32>;
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
		u32, // pallet_id
		BoundedVec<[u8;32], T::MaxScopesPerPallet>,  // scopes_id
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn roles)]
	pub(super) type Roles<T: Config> = StorageMap<
		_,
		Identity, 
		[u8;32], // role_id
		BoundedVec<u8, ConstU32<100> >,  // role
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn pallet_roles)]
	pub(super) type PalletRoles<T: Config> = StorageMap<
		_,
		Blake2_128Concat, 
		u32, // pallet_id
		BoundedVec<[u8;32], T::MaxRolesPerPallet >, // role_id
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn permissions)]
	pub(super) type Permissions<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, u32>,		// pallet_id
			NMapKey<Blake2_128Concat, [u8;32]>,	// scope_id
			NMapKey<Twox64Concat, [u8;32]>,		// role_id
		),
		BoundedVec<BoundedVec<u8, T::PermissionMaxLen >, T::MaxPermissionsPerRole >,	// permissions
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
		/// The pallet doesnt have scopes associated
		PalletNotFound,
		/// The specified scope doesnt exists
		ScopeNotFound,
		/// The specified role doesnt exists
		RoleNotFound,
		/// The role is already linked in the pallet
		DuplicateRole,
		DuplicatePermission,
		/// The pallet has too many scopes
		ExceedMaxScopesPerPallet,
		ExceedMaxRolesPerPallet,
		ExceedMaxPermissionsPerRole,
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