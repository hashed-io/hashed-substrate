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
		sp_runtime::traits::Hash,
		BoundedVec,
		transactional,
		sp_io::hashing::blake2_256,
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
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		/// Xpub stored and linked to an account identity
		XPubStored(T::Hash, T::AccountId),
		/// Removed Xpub previously linked to the account
		XpubRemoved(T::AccountId),
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
		/// The identity doesn't have an xpub
		XPubNotFound,
	}

	// Tentative, but maybe we don't need it, code review required
	#[pallet::storage]
	#[pallet::getter(fn xpubs_by_owner)]
	/// Keeps track of what accounts own what xpub.
	/// The xpub needs to be the key in order to search for it
	/// TODO: Have an additional data structure for individual xpubs?
	/// have XpubsByOwner to save the xpub hash instead?
	pub(super) type XpubsByOwner<T: Config> = StorageMap<
		_,
		Blake2_256,
		BoundedVec<u8, T::XPubLen>,
		T::AccountId,
		OptionQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
		pub fn set_complete_identity(origin: OriginFor<T>, 
			mut info: Box< pallet_identity::IdentityInfo<T::MaxAdditionalFields> >,
			xpub: BoundedVec<u8, T::XPubLen>,
		) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin.clone())?;
			ensure!(xpub.len()>0,<Error<T>>::NoneValue);
			let hash = T::Hashing::hash(&xpub);
			//let hash_vec: BoundedVec<u8,ConstU32<32> > = hash.encode().try_into().expect("Error trying to convert the hash to vec");
			//let xpub_vec = BoundedVec::<u8,T::XPubLen>::try_from(xpub.encode())
			//	.expect("Error on encoding the xpub key to BoundedVec");
			// Storing the preimage doesn't give you the hash, but this gives you the same hash
			// Ensure that the xpub hasn't been taken before
			//let uploaded_before = T::PreimageProvider::have_preimage(&hash);
			//ensure!(!uploaded_before, <Error<T>>::XPubAlreadyTaken);
			//log::info!("Manual hash result: {:?}",hash);
			// Requesting the hash first allows to insert data without a deposit
			//T::PreimageProvider::request_preimage(&hash);
			// Inserting the xpub on pallet_preimage 
			//T::PreimageProvider::note_preimage(xpub.to_vec().try_into().expect("Error trying to convert xpub to vec"));
			// Confirm the xpub was inserted
			//ensure!(T::PreimageProvider::have_preimage(&hash),<Error<T>>::XPubNotFound);
			ensure!(!<XpubsByOwner<T>>::contains_key(xpub.clone()), <Error<T>>::XPubAlreadyTaken);
			<XpubsByOwner<T>>::insert(xpub.clone() , who.clone());
			// the 2nd half of the inserted_hash is the real key hhash
			let mut inserted_hash  = <XpubsByOwner<T>>::hashed_key_for(xpub.clone());
			let manual_hash = xpub.clone().using_encoded(blake2_256);
			let manual_who = who.clone().using_encoded(blake2_256);
			log::info!("Hash len ({}): {:?}",inserted_hash.len(),inserted_hash);
			log::info!("Manual hash ({}): {:?}",manual_hash.len(),manual_hash);
			log::info!("Manual who ({}): {:?}",manual_who.len(),manual_who);
			//log::info!("{:?}",<XpubsByOwner<T>>::storage_info());
			let partial_hash = inserted_hash.split_off(32);
			//let inserted_hash_vec : BoundedVec<u8, ConstU32<32>> = inserted_hash.try_into().expect("Error trying to convert the hash to vec");
			let inserted_hash_array : [u8;32] = partial_hash.try_into().expect("Error trying to convert the hash to vec");
			//log::info!("Hash local ({}): {:?}",inserted_hash.len(), inserted_hash );

			// Setting up the xpub key/value pair
			let key = BoundedVec::<u8,ConstU32<32> >::try_from(b"xpub".encode())
				.expect("Error on encoding the xpub key to BoundedVec");
			// Try to push the key
			info.additional.try_push(
				(pallet_identity::Data::Raw(key),pallet_identity::Data::BlakeTwo256(inserted_hash_array) ) 
			).map_err(|_| pallet_identity::Error::<T>::TooManyFields)?;
			//info.additional = copy_info;
			let identity_result = pallet_identity::Pallet::<T>::set_identity(origin, info)?;
			// Emit an event.
			Self::deposit_event(Event::XPubStored(hash, who));

			// Return a successful DispatchResultWithPostInfo
			Ok(identity_result)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
		pub fn remove_xpub_from_identity(
			origin: OriginFor<T>, 
			//xpub: BoundedVec<u8, T::XPubLen>,
			// tentative: add option to keep the xpub stored on the preimage pallet
		) -> DispatchResultWithPostInfo
		{
			let who = ensure_signed(origin.clone())?;
			let mut identity = pallet_identity::Pallet::<T>::identity(who.clone())
				.ok_or(pallet_identity::Error::<T>::NoIdentity)?;
			let key = BoundedVec::<u8,ConstU32<32> >::try_from(b"xpub".encode())
			 		.expect("Error on encoding the xpub key to BoundedVec");
			let xpub_index = identity.info.additional.iter().position(
				|field| field.0.eq(&pallet_identity::Data::Raw(key.clone()))
				).ok_or(<Error<T>>::XPubNotFound)?;
			let mut xpub_tuple = (pallet_identity::Data::None, pallet_identity::Data::None);
			let updated_fields=  identity.info.additional.clone().try_mutate(|addittional_fields|{
				xpub_tuple = addittional_fields.remove(xpub_index);
				()
			}).ok_or(pallet_identity::Error::<T>::TooManyFields)?;
			// TODO: remove preimage too
			//let s = pallet_identity::Data::encode( &xpub_tuple.1);
			log::info!("Retrieved tuples: {:?}",xpub_tuple);
			//T::PreimageProvider::unrequest_preimage(xpub_tuple.1.encode());
			identity.info.additional.clone_from(&updated_fields);
			let identity_result = pallet_identity::Pallet::<T>::set_identity(origin, Box::new( identity.info )  )?;

			Self::deposit_event(Event::XpubRemoved( who));

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