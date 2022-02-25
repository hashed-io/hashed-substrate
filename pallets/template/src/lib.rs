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

#[frame_support::pallet]
pub mod pallet {
	use bdk::blockchain::{noop_progress, ElectrumBlockchain};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use bdk::electrum_client::Client;
	use bdk::{database::MemoryDatabase, Wallet};
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
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
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, _something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			let _private_descriptor = "wpkh(tprv8ZgxMBicQKsPd4aJQ2f3nmwJ6bWm7Z4Vcem2GAMEDziTKmDMpNetmchqo6KewkXKy4By2DHZBJ7ZiK1Ccy6fidauw1bnqrP8JTzpntNLW58/*)";
			let _public_descriptor = "wpkh(tpubD6NzVbkrYhZ4WXc6HgKeCBbQfd2hGtFQBxMoYgPXeGWrAFU8SmUUx7KhyEU8APbDY8pjTx9SbRs5ctniJxpZzha3m66HWrT7rJPA5gUv86W/*)#cy6nz2j5";
			let client = Client::new("localhost:50000").unwrap();
			let blockchain = ElectrumBlockchain::from(client);

			let wallet = Wallet::new(
				_public_descriptor,
				Some(_public_descriptor),
				bitcoin::Network::Regtest,
				MemoryDatabase::default(),
				blockchain,
			)
			.unwrap();
			wallet.sync(noop_progress(), None).unwrap();

			let balance = wallet.get_balance().unwrap();

			// Update storage.
			// <Something<T>>::put(balance.tryinto().clone());

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(balance.try_into().unwrap(), who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
