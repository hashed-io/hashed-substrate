#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use sp_core::{crypto::KeyTypeId};
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

pub mod crypto {
    use super::KEY_TYPE;
    use sp_core::sr25519::Signature as Sr25519Signature;
    use sp_runtime::{
        app_crypto::{app_crypto, sr25519},
        traits::Verify, MultiSignature, MultiSigner
    };
    app_crypto!(sr25519, KEY_TYPE);

    pub struct TestAuthId;

    // implemented for runtime
    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
    type RuntimeAppPublic = Public;
    type GenericSignature = sp_core::sr25519::Signature;
    type GenericPublic = sp_core::sr25519::Public;
    }
}

#[frame_support::pallet]
pub mod pallet {
use frame_support::pallet_prelude::*;
	use frame_system::{pallet_prelude::*, offchain::CreateSignedTransaction};

	use frame_support::{
		BoundedVec,
		transactional,
		sp_io::hashing::blake2_256,
	};
	use scale_info::prelude::boxed::Box;

	//use scale_info::TypeInfo;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_identity::Config + CreateSignedTransaction<Call<Self>>{
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
		XPubRemoved(T::AccountId),
		/// The PBST was succesuflly inserted and linked to the account
		PSBTStored(T::AccountId),
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
		/// Found Invalid name on an additional field 
		InvalidAdditionalField,
	}

	/// Stores hash-xpub pairs
	#[pallet::storage]
	#[pallet::getter(fn xpubs)]
	pub(super) type Xpubs<T: Config> = StorageMap<
		_,
		Identity,
		[u8 ; 32], //that's the blake 2 hash result
		BoundedVec<u8, T::XPubLen>,
		OptionQuery,
	>;

	/// Keeps track of what accounts own what xpub.
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
	pub(super) type PSBTs<T: Config> = StorageMap<
		_,
		Blake2_256,
		T::AccountId,
		BoundedVec<u8, T::PSBTMaxLen>,
		OptionQuery,
	>;

	pub enum XpubStatus {
		Owned,
		Free,
		Taken,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Offchain Worker entry point.
		///
		/// By implementing `fn offchain_worker` you declare a new offchain worker.
		/// This function will be called when the node is fully synced and a new best block is
		/// successfully imported.
		/// Note that it's not guaranteed for offchain workers to run on EVERY block, there might
		/// be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
		/// so the code should be able to handle that.
		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!("Hello from pallet-ocw. Block number {:?}",block_number);
			// The entry point of your code called by off-chain worker
		}
		// ...
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// ## Identity with XPub insertion
		/// 
		/// This extrinsic inserts a user-defined xpub as an additional field in the identity pallet,
		/// as well as in the pallet storage.
		///
		/// ### Parameters:
		/// - `info`: Contains all the default `pallet-identity` fields (display, legal, twitter, etc.).
		/// Additional fields may also be inserted with some minor limitations (see Considerations)
		/// - `xpub`: The unique identifier of the instance to be fractioned/divided 
		/// 
		/// ### Considerations 
		/// - The origin must be Signed and the sender must have sufficient funds free for the identity insertion.
		/// - This extrinsic is marked as transactional, so if an error is fired, all the changes will be reverted (but the
		///  fees will be applied nonetheless).
		/// - This extrinsic will insert an additional field named `xpub`. In order to avoid conflicts and malfunctioning,
		/// it is highly advised to refrain naming an additional field like that.
		/// - This extrinsic can handle a xpub update, but if other fields are needed to be updated, then using pallet-identity
		/// is ideal (while adding explicitly the xpub additional field).
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
		pub fn set_complete_identity(origin: OriginFor<T>, 
			mut info: Box< pallet_identity::IdentityInfo<T::MaxAdditionalFields> >,
			xpub: BoundedVec<u8, T::XPubLen>,
		) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin.clone())?;
			ensure!(xpub.len()>0,<Error<T>>::NoneValue);
			ensure!(Self::xpub_field_available(&info.additional),<Error<T>>::InvalidAdditionalField);
			let manual_hash = xpub.clone().using_encoded(blake2_256);
			// Assert if the input xpub is free to take (or if the user owns it)
			match Self::get_xpub_status(who.clone(),manual_hash.clone()) {
				XpubStatus::Owned => log::info!("Xpub owned, nothing to insert"),
				XpubStatus::Taken => Err(<Error<T>>::XPubAlreadyTaken )?, //xpub taken: abort tx
				XpubStatus::Free => { // xpub free: erase unused xpub and insert on maps
					if <XpubsByOwner<T>>::contains_key(who.clone()) {
						Self::remove_xpub_from_pallet_storage(who.clone())?;
					}
					<Xpubs<T>>::insert(manual_hash, xpub.clone() );
					// Confirm the xpub was inserted
					let mut inserted_hash  = <Xpubs<T>>::hashed_key_for(manual_hash);
					// the 2nd half of the inserted_hash is the real key hash.
					let partial_hash = inserted_hash.split_off(32);
					// The manually calculated hash should always be equal to the StorageMap hash
					ensure!(partial_hash.as_slice() == manual_hash,  <Error<T>>::HashingError);
					// If everything is ok, insert the xpub in the owner->hash map
					<XpubsByOwner<T>>::insert( who.clone() , manual_hash);
				},
			}

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

		/// ## Xpub removal
		/// 
		/// Removes the linked xpub from the account which signs the transaction.
		/// The xpub will be removed from both the pallet storage and identity registration.
		///
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
			// Search for the xpub field
			let xpub_index = identity.info.additional.iter().position(
				|field| field.0.eq(&pallet_identity::Data::Raw(key.clone()))
			).ok_or(<Error<T>>::XPubNotFound)?;
			// Removing the xpub field on the account's identity
			let mut xpub_id_field = (pallet_identity::Data::None, pallet_identity::Data::None);
			let updated_fields = identity.info.additional.clone().try_mutate(
				|additional_fields|{
					xpub_id_field = additional_fields.remove(xpub_index);
					()
				}
			).ok_or(Error::<T>::XPubNotFound)?;
			// Using the obtained xpub hash to remove it from the pallet's storage
			let old_xpub_hash: [u8 ; 32] = xpub_id_field.1.encode()[1..].try_into().expect("Error converting retrieved xpub");
			ensure!( <Xpubs<T>>::contains_key(old_xpub_hash), Error::<T>::HashingError);
			Self::remove_xpub_from_pallet_storage(who.clone())?;
			identity.info.additional.clone_from(&updated_fields);
		 	let identity_result = pallet_identity::Pallet::<T>::set_identity(origin, Box::new( identity.info )  )?;

		 	Self::deposit_event(Event::XPubRemoved( who));
			 Ok(identity_result)
		}

		/// ## PSBT insertion
		/// 
		/// At the current moment, only one PSTB is allowed to be inserted per account
		///
		/// ### Parameters:
		/// - `psbt`: Ideally encoded to base 64 and then to binary. Its size is restricted by 
		/// the constant `T::PSBTMaxLen`
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_psbt(
			origin: OriginFor<T>,
			psbt: BoundedVec<u8, T::PSBTMaxLen>,
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;
			<PSBTs<T>>::insert(who.clone(),psbt);

			Self::deposit_event(Event::PSBTStored( who ) );

			Ok(())
		}

	}

	impl<T: Config> Pallet<T> {
		/// Use with caution
		pub fn remove_xpub_from_pallet_storage(who : T::AccountId) -> Result<(), Error<T> > {
			// No error can be propagated from the remove functions
			if <XpubsByOwner<T>>::contains_key(who.clone()){
				let old_hash = <XpubsByOwner<T>>::take(who ).expect("Old hash not found");
				<Xpubs<T>>::remove(old_hash);
				return Ok(());
			}
			return Err(<Error<T>>::XPubNotFound );

		}

		// Ensure at that certain point, no xpub field exists on the identity
		pub fn xpub_field_available(fields: &BoundedVec<(pallet_identity::Data, pallet_identity::Data), T::MaxAdditionalFields>) -> bool{
			let key = BoundedVec::<u8,ConstU32<32> >::try_from(b"xpub".encode())
				.expect("Error on encoding the xpub key to BoundedVec");
			let xpub_count = fields.iter().find(|(k, _)| k == &pallet_identity::Data::Raw(key.clone()));
			xpub_count.is_none()
		}

		// check if the xpub is free to take/update or if its owned by the account
		pub fn get_xpub_status(who : T::AccountId, xpub_hash : [u8; 32]) -> XpubStatus{
			if <Xpubs<T>>::contains_key(xpub_hash) {
				if let Some(owned_hash) = <XpubsByOwner<T>>::get(who.clone()){
					match xpub_hash == owned_hash{
						true => return XpubStatus::Owned,
						false => return XpubStatus::Taken,
					}
				}else{
					// xpub registered and the account doesnt own it: unavailable
					return XpubStatus::Taken;
				}
				// Does the user owns the registered xpub? if yes, available
			}
			// new xpub registry: available 
			XpubStatus::Free
		}
    }


}