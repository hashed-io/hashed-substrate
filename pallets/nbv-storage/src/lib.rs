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
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use frame_support::{
		traits::{PreimageProvider, PreimageRecipient, Get},
		sp_runtime::traits::Hash,
		BoundedVec,
		transactional
	};
	use scale_info::prelude::boxed::Box;
	use scale_info::prelude::string::String;
	use scale_info::prelude::vec::Vec;

	//use scale_info::TypeInfo;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_identity::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		#[pallet::constant]
		/// Maximum amount added per invocation.
		type XPubLen: Get<u32>;
		/// The handler of pre-images.
		type PreimageProvider: PreimageProvider<Self::Hash> + PreimageRecipient<Self::Hash>;

		//type IdentityPallet: pallet_identity::Config;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		/// Xpub stored and linked to an account identity
		XPubStored(T::Hash, T::AccountId),
		
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Work in progress!
		NotYetImplemented,
		/// Xpub shouldn't be empty. Use the identity pallet if needed.
		NoneValue,
		// The xpub has already been uploaded and taken by another account
		XPubAlreadyTaken,
		/// Something went wrong when inserting the xpub preimage
		XPubNotFound,
	}

	// Tentative, but maybe we don't need it, code review required
	// #[pallet::storage]
	// #[pallet::getter(fn xpubs_by_owner)]
	// /// Keeps track of what accounts own what Kitty.
	// pub(super) type XPubs_by_owner<T: Config> = StorageMap<
	// 	_,
	// 	Twox64Concat,
	// 	T::AccountId,
	// 	BoundedVec<u8, T::XPubLen>,
	// 	ValueQuery,
	// >;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//TODO: 
		// set account-> xpub relationship ?
		// add valition on request_preimage <=1
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
		pub fn set_complete_identity(origin: OriginFor<T>, 
			mut info: Box< pallet_identity::IdentityInfo<T::MaxAdditionalFields> >,
			xpub: BoundedVec<u8, T::XPubLen>,
		) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin.clone())?;
			ensure!(xpub.len()>0,<Error<T>>::NoneValue);
			// Storing the preimage doesn't give you the hash, but this gives you the same hash
			let hash = T::Hashing::hash(&xpub);
			// Ensure that the xpub hasn't been taken before
			let uploaded_before = T::PreimageProvider::have_preimage(&hash);
			ensure!(!uploaded_before, <Error<T>>::XPubAlreadyTaken);
			log::info!("Manual hash result: {:?}",hash);
			// Requesting the hash first allows to insert data without a deposit
			T::PreimageProvider::request_preimage(&hash);
			// Inserting the xpub on pallet_preimage 
			T::PreimageProvider::note_preimage(xpub.to_vec().try_into().expect("Error trying to convert xpub to vec"));
			// Confirm the xpub was inserted
			ensure!(T::PreimageProvider::have_preimage(&hash),<Error<T>>::XPubNotFound);
			// Setting up the xpub key/value pair
			let key = BoundedVec::<u8,ConstU32<32> >::try_from(b"xpub".encode())
				.expect("Error on encoding the xpub key to BoundedVec");
			let hash_vec: BoundedVec<u8,ConstU32<32> > = hash.encode().try_into().expect("Error trying to convert the hash to vec");
			// Try to push the key
			info.additional.try_push(
				(pallet_identity::Data::Raw(key),pallet_identity::Data::Raw(hash_vec) ) 
			).map_err(|_| pallet_identity::Error::<T>::TooManyFields)?;
			//info.additional = copy_info;
			let identity_result = pallet_identity::Pallet::<T>::set_identity(origin, info)?;
			// Emit an event.
			Self::deposit_event(Event::XPubStored(hash, who));

			// Return a successful DispatchResultWithPostInfo
			Ok(identity_result)
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_psbt(
			origin: OriginFor<T>,
		) -> DispatchResult {
			let _who = ensure_signed(origin.clone())?;
			ensure!(false,<Error<T>>::NotYetImplemented);

			Ok(())
		}

	}

	impl<T: Config> Pallet<T> {
		pub fn bytes_to_string(input: Vec<u8>)->String{
			let mut s = String::default();
			for x in input{
				//let c: char = x.into();
				s.push(x as char);
			}
			s
		}
    }

}