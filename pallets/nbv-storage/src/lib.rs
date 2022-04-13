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
		BoundedVec
	};
	use scale_info::prelude::boxed::Box;
	use scale_info::prelude::string::String;
	use scale_info::prelude::vec::Vec;
	use frame_support::sp_io::hashing::blake2_256;

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
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_complete_identity(origin: OriginFor<T>, xpub: BoundedVec<u8, T::XPubLen>,
			mut info: Box< pallet_identity::IdentityInfo<T::MaxAdditionalFields> >
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin.clone())?;
			// Storing the preimage doesn't give you the hash, but this gives you the same hash (?)
			let hash = T::Hashing::hash(&xpub);
			log::info!("Manual Note result: {:?}",hash);
			// Requesting the hash first allows to insert data without a deposit
			T::PreimageProvider::request_preimage(&hash);
			// Inserting the xpub on pallet_preimage 
			T::PreimageProvider::note_preimage(xpub.to_vec().try_into().expect("Error trying to convert xpb to vec"));
			// Confirm the xpub was inserted
			let retrieved = T::PreimageProvider::get_preimage(&hash).expect("Error getting the preimage");
			log::info!("Retrieved file: {:?}",retrieved.to_ascii_lowercase() );
			// Setting up a the 
			let key = BoundedVec::<u8,ConstU32<32> >::try_from("extended_pub_key".encode())
				.expect("Error on encoding the xpub key to BoundedVec");
			let hash_vec: BoundedVec<u8,ConstU32<32> > = hash.encode().try_into().expect("Error trying to convert the hash to vec");
			//let mut copy_info = info.additional.clone();
			info.additional.try_push(
				(pallet_identity::Data::Raw(key),pallet_identity::Data::Raw(hash_vec) ) 
			).expect("Error while trying to add the xpub to the identity data structure");
			//info.additional = copy_info;
			pallet_identity::Pallet::<T>::set_identity(origin, info).expect("Error on setting up the account identity");
			let id = pallet_identity::Pallet::<T>::identity(&who).unwrap();
			log::info!("Testing identity pallet: {:?}",id);
			// Emit an event.
			Self::deposit_event(Event::SomethingStored(45, who));

			// Return a successful DispatchResultWithPostInfo
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