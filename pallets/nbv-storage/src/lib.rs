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
		type XPubLen: Get<u32>;
		#[pallet::constant]
		type PSBTMaxLen: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		/// Xpub and hash stored. Linked to an account identity
		XPubStored([u8 ; 32], T::AccountId),
		/// Removed Xpub previously linked to the account
		XpubRemoved(T::AccountId),
		/// The PBST was succesuflly inserted and linked to the account
		PSBTStored( [u8 ; 32] , T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Work in progress!
		NotYetImplemented,
		/// Xpub shouldn't be empty. Use the identity pallet if needed.
		NoneValue,
		// The xpub has already been uploaded and taken by an account
		XPubAlreadyTaken,
		/// The identity doesn't have an xpub
		XPubNotFound,
		/// The generated Hashes aren't the same
		HashingError,
	}

	#[pallet::storage]
	#[pallet::getter(fn xpubs)]
	pub(super) type Xpubs<T: Config> = StorageMap<
		_,
		Identity,
		[u8 ; 32],
		BoundedVec<u8, T::XPubLen>,
		OptionQuery,
	>;

	/// Keeps track of what accounts own what xpub.
	/// The xpub needs to be the key in order to search for it
	/// TODO: Have an additional data structure for individual xpubs?
	/// have XpubsByOwner to save the xpub hash instead?
	#[pallet::storage]
	#[pallet::getter(fn xpubs_by_owner)]
	pub(super) type XpubsByOwner<T: Config> = StorageMap<
		_,
		Blake2_256,
		T::AccountId,
		[u8 ; 32],
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn psbts)]
	pub(super) type PSBT<T: Config> = StorageMap<
		_,
		Blake2_256,
		T::AccountId,
		BoundedVec<u8, T::PSBTMaxLen>,
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
			let manual_hash = xpub.clone().using_encoded(blake2_256);
			// Ensure that the xpub hasn't been taken before
			ensure!(!<Xpubs<T>>::contains_key(manual_hash) , <Error<T>>::XPubAlreadyTaken);
			// If there's a previous identity set for that account, this process
			// will overwrite it, so removing the xpub from the storage is necessary
			log::info!("User has xpub? {:?}", <XpubsByOwner<T>>::contains_key(who.clone()));
			log::info!("Xpub hash exists? {:?}", <Xpubs<T>>::contains_key(manual_hash));
			if <XpubsByOwner<T>>::contains_key(who.clone()) {
				Self::remove_xpub_from_pallet_storage(who.clone() );
				log::info!("Previous xpub entries removed");
			}
			log::info!("After: User has xpub? {:?}", <XpubsByOwner<T>>::contains_key(who.clone()));
			log::info!("After: Xpub hash exists? {:?}", <Xpubs<T>>::contains_key(manual_hash));
			// Insert the xpubs on both of the maps (hash-xpub and account-hash)
			<Xpubs<T>>::insert(manual_hash, xpub.clone() );
			<XpubsByOwner<T>>::insert( who.clone() , manual_hash);
			// Confirm the xpub was inserted
			let mut inserted_hash  = <Xpubs<T>>::hashed_key_for(manual_hash);
			// the 2nd half of the inserted_hash is the real key hash
			let partial_hash = inserted_hash.split_off(32);
			ensure!(partial_hash.as_slice() == manual_hash,  <Error<T>>::HashingError);
			// Setting up the xpub key/value pair
			let key = BoundedVec::<u8,ConstU32<32> >::try_from(b"xpub".encode())
				.expect("Error on encoding the xpub key to BoundedVec");
			// Try to push the key
			info.additional.try_push(
				(pallet_identity::Data::Raw(key),pallet_identity::Data::BlakeTwo256(manual_hash) ) 
			).map_err(|_| pallet_identity::Error::<T>::TooManyFields)?;
			// Insert identity
			let identity_result = pallet_identity::Pallet::<T>::set_identity(origin, info)?;
			// Emit a success event.
			Self::deposit_event(Event::XPubStored(manual_hash, who));

			// Return a successful DispatchResultWithPostInfo
			Ok(identity_result)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
		pub fn remove_xpub_from_identity(
			origin: OriginFor<T>, 
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin.clone())?;
			// Obtain account identity
			let mut identity = pallet_identity::Pallet::<T>::identity(who.clone())
				.ok_or(pallet_identity::Error::<T>::NoIdentity)?;
			let key = BoundedVec::<u8,ConstU32<32> >::try_from(b"xpub".encode())
				.expect("Error on encoding the xpub key to BoundedVec");
			let xpub_index = identity.info.additional.iter().position(
				|field| field.0.eq(&pallet_identity::Data::Raw(key.clone()))
			).ok_or(<Error<T>>::XPubNotFound)?;

			let mut xpub_tuple = (pallet_identity::Data::None, pallet_identity::Data::None);
			let updated_fields = identity.info.additional.clone().try_mutate(
				|addittional_fields|{
					xpub_tuple = addittional_fields.remove(xpub_index);
					//log::info!("xpub retrieved: {:?}", xpub_tuple.1.encode().drain(..0));
					// TODO: fix this (try_into?)
					//let old_xpub_hash: [u8 ; 32] = xpub_tuple.1.encode()[1..].try_into().expect("Error converting retireved xpub");
					Self::remove_xpub_from_pallet_storage(who.clone());
					()
				}
			).ok_or(Error::<T>::XPubNotFound)?;

			identity.info.additional.clone_from(&updated_fields);
		 	let identity_result = pallet_identity::Pallet::<T>::set_identity(origin, Box::new( identity.info )  )?;

		 	Self::deposit_event(Event::XpubRemoved( who));
			 Ok(identity_result)
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_psbt(
			origin: OriginFor<T>,
			psbt: BoundedVec<u8, T::PSBTMaxLen>,
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;
			<PSBT<T>>::insert(who.clone(),psbt);
			//ensure!(false,<Error<T>>::NotYetImplemented);

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
		/// Use with caution
		pub fn remove_xpub_from_pallet_storage(who : T::AccountId){
			// No error can be propagated from the remove functions
			let old_hash = <XpubsByOwner<T>>::take(who ).expect("Old hash not found");
			<Xpubs<T>>::remove(old_hash);

		}
    }


}