#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod functions;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/*--- Structs Section ---*/

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		#[pallet::constant]
		type LabelMaxLen: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn marketplaces_by_admin)]
	pub(super) type MarketplacesByAdmin<T: Config> =
		StorageMap<_, Blake2_256, T::AccountId, [u8; 32], OptionQuery>;

	#[pallet::storage]
	pub(super) type DobleMapa<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, 
		u32, 
		Blake2_128Concat, 
		T::AccountId, 
		u32, 
		ValueQuery
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Work In Progress
		NotYetImplemented,
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_marketplace(origin: OriginFor<T>, _label: BoundedVec<u8,T::LabelMaxLen> ) -> DispatchResult {
			let who = ensure_signed(origin)?;
			//ensure!(false,Error::<T>::NotYetImplemented);
			<DobleMapa<T>>::insert(1, who, 45);
			Ok(())
		}
		
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn apply(origin: OriginFor<T>, _marketplace_id: [u8;32], _note: BoundedVec<u8,T::LabelMaxLen> ) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			ensure!(false,Error::<T>::NotYetImplemented);
			Ok(())
		}

	}
}