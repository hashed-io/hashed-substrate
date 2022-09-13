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
		type CIDMaxLen: Get<u32>;

		

	
		
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/*--- Onchain storage section ---*/

	#[pallet::storage]
	#[pallet::getter(fn users)]
	pub(super) type Users<T: Config> = StorageMap<
		_, 
		Identity, 
		T::AccountId, // Key
		BoundedVec<u8, ConstU32<100>>,  // Value
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn projects)]
	pub(super) type Projects<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], // Key project_id
		Project<T>,  // Value
		OptionQuery,
	>;




	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		/// TODO: map each constant type used by bounded vecs to a descriptive error
		NoneValue,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {


		// A C C O U N T S
		// --------------------------------------------------------------------------------------------
		// #[transactional]
		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		// pub fn accounts_add_user(origin: OriginFor<T>, admin: T::AccountId,label: BoundedVec<u8,T::LabelMaxLen>) -> DispatchResult {
		// 	// let who = ensure_signed(origin)?; // origin will be market owner
		// 	// let m = Marketplace{
		// 	// 	label,
		// 	// };
		// 	// Self::do_create_marketplace(who, admin, m)
		// }

		
		// P R O J E C T S
		// --------------------------------------------------------------------------------------------
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn projects_create_project(origin: OriginFor<T>, tittle: BoundedVec<u8, T::ProjectNameMaxLen>, description: BoundedVec<u8, T::ProjectNameMaxLen>, image:  BoundedVec<u8, T::CIDMaxLen>, developer: Option<T::AccountId>, builder: Option<T::AccountId>, issuer: Option<T::AccountId>, regional_center: Option<T::AccountId>, 
		 ) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin will be admin

			Self::do_create_project(who, tittle, description, image, developer, builder, issuer, regional_center)
		}

	}
}