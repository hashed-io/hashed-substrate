#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use sp_core::crypto::KeyTypeId;
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
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
	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};
	use frame_support::{
		pallet_prelude::{BoundedVec, MaxEncodedLen},
		traits::Get,
	};
	use frame_support::{sp_io::hashing::blake2_256, transactional};
	use frame_system::{
		offchain::{
			AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
			SignedPayload, Signer, SigningTypes, SubmitTransaction,
		},
		pallet_prelude::*,
	};
	use scale_info::prelude::boxed::Box;
	use sp_runtime::sp_std::str;
	use sp_runtime::sp_std::vec::Vec;
	use sp_io::offchain_index;
	use sp_runtime::RuntimeString;
	use sp_runtime::{
		offchain::{
			http,
			storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
			Duration,
		},
		traits::Zero,
		transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
		RuntimeDebug,
	};

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum PSBTStatus {
		Pending,
		Broadcasted,
	}

	// Struct for holding Vaults information.
	//#[derive(
	//	CloneNoBound, Encode, Decode, Eq, MaxEncodedLen, PartialEqNoBound, RuntimeDebugNoBound, TypeInfo,
	//)]
	//#[codec(mel_bound())]
	//#[cfg_attr(test, derive(frame_support::DefaultNoBound))]
	#[derive(Encode, Decode, Eq, PartialEq, RuntimeDebug, Default, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Vault<T: Config> {
		pub owner: T::AccountId,
		pub threshold: u32,
		pub description: BoundedVec<u8, T::VaultDescriptionMaxLen>,
		pub cosigners: BoundedVec<T::AccountId, T::MaxCosignersPerVault>,
		pub descriptor: BoundedVec<u8, T::OutputDescriptorMaxLen>,
		pub change_descriptor: Option<BoundedVec<u8, T::OutputDescriptorMaxLen>>,
	}

	impl<T: Config> Clone for Vault<T> {
		fn clone(&self) -> Self {
			Vault {
				owner: self.owner.clone(),
				threshold: self.threshold.clone(),
				cosigners: self.cosigners.clone(),
				description: self.description.clone(),
				descriptor: self.descriptor.clone(),
				change_descriptor: self.change_descriptor.clone(),
			}
		}

		fn clone_from(&mut self, source: &Self) {
			Self {
				owner: source.owner.clone(),
				threshold: source.threshold.clone(),
				cosigners: source.cosigners.clone(),
				description: source.description.clone(),
				descriptor: source.descriptor.clone(),
				change_descriptor: source.change_descriptor.clone(),
			};
			()
		}
	}

	// Struct for holding Proposal information.
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Proposal<T: Config> {
		pub proposer: T::AccountId,
		pub vault_id: [u8; 32],
		pub status: PSBTStatus,
		pub to_address: BoundedVec<u8, T::PSBTMaxLen>,
		pub amount: u64,
		pub fee_sat_per_vb: u32,
		pub description: BoundedVec<u8, T::PSBTMaxLen>,
		pub psbt: BoundedVec<u8, T::PSBTMaxLen>,
		pub signed_psbts: (T::AccountId, BoundedVec<BoundedVec<u8, T::PSBTMaxLen>, T::PSBTMaxLen>), // TODO: Cambiar a struct
	}

	#[derive(Debug, Encode, Decode)]
	pub enum Request<T: Config>{
		CreateVault(Vault<T>),
		Propose(Proposal<T>),
		SignProposal([u8; 32]), // proposal id?
		FinalizeTx([u8; 32]),
	}

	#[derive(Debug, Encode, Decode,)]
	struct IndexingRequest<T: Config>{
		pub request_author: T::AccountId,
		pub request: Request<T>,
	}

	use scale_info::TypeInfo;
	use lite_json::Serialize as jsonSerialize;
	use lite_json::json::{JsonValue, NumberValue};
	use lite_json::parse_json;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_identity::Config + CreateSignedTransaction<Call<Self>>
	{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/*--- PSBT params ---*/
		#[pallet::constant]
		type XPubLen: Get<u32>;
		#[pallet::constant]
		type PSBTMaxLen: Get<u32>;
		/*--- Vault params ---*/
		/// It counts both owned vaults and vaults where the user is a cosigner
		#[pallet::constant]
		type MaxVaultsPerUser: Get<u32>;
		#[pallet::constant]
		type MaxCosignersPerVault: Get<u32>;
		#[pallet::constant]
		type VaultDescriptionMaxLen: Get<u32>;
		#[pallet::constant]
		type OutputDescriptorMaxLen: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	const ONCHAIN_TX_KEY: &[u8] = b"nbv_storage::requests";
	const BDK_SERVICES_URL: &[u8] = b"http://127.0.0.1:8000";

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Xpub and hash stored. Linked to an account identity
		XPubStored([u8; 32], T::AccountId),
		/// Removed Xpub previously linked to the account
		XPubRemoved(T::AccountId),
		/// The PBST was succesfully inserted and linked to the account
		PSBTStored(T::AccountId),
		/// The vault was succesfully inserted and linked to the account as owner
		VaultStored([u8; 32], T::AccountId),
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
		/// The Account doesn't have an xpub
		XPubNotFound,
		/// The generated Hashes aren't the same
		HashingError,
		/// Found Invalid name on an additional field
		InvalidAdditionalField,
		/// The vault threshold cannot be greater than the number of vault participants
		InvalidVaultThreshold,
		/// A defined cosigner reached its vault limit
		SignerVaultLimit,
	}

	/// Stores hash-xpub pairs
	#[pallet::storage]
	#[pallet::getter(fn xpubs)]
	pub(super) type Xpubs<T: Config> = StorageMap<
		_,
		Identity,
		[u8; 32], //that's the blake 2 hash result
		BoundedVec<u8, T::XPubLen>,
		OptionQuery,
	>;

	/// Keeps track of what accounts own what xpub.
	#[pallet::storage]
	#[pallet::getter(fn xpubs_by_owner)]
	pub(super) type XpubsByOwner<T: Config> =
		StorageMap<_, Blake2_256, T::AccountId, [u8; 32], OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn psbts)]
	pub(super) type PSBTs<T: Config> =
		StorageMap<_, Blake2_256, T::AccountId, BoundedVec<u8, T::PSBTMaxLen>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn vaults)]
	pub(super) type Vaults<T: Config> = StorageMap<
		_,
		Identity,
		[u8; 32], //hash
		Vault<T>,
		OptionQuery,
	>;
	/// Keeps track of what accounts own what xpub.
	#[pallet::storage]
	#[pallet::getter(fn vaults_by_signer)]
	pub(super) type VaultsBySigner<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,                                  // signer
		BoundedVec<[u8; 32], T::MaxCosignersPerVault>, // vault ids
		ValueQuery,
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
			log::info!("Hello from pallet-ocw. Block number {:?}", block_number);
			// The entry point of your code called by off-chain worker
			//log::info!("{:?}",Self::fetch_price() );
			//let vaul_id: [u8;32] = e73199c614079"..try_into().expect("Error converting vaulid");
			//log::info!("Off chain accessing to ON chain: {:?}", vaul.unwrap().owner );

			// check for pending vaults to insert
			let mut pending_vaults = Self::get_pending_vaults();
			if pending_vaults.len() > 0 {
				log::info!("Vaults pendientes {:?}",pending_vaults.len());
				
				// Contact bdk services
				let result = Self::bdk_gen_vault(pending_vaults.pop().expect("No vaults left?"));
				log::info!("Resultado desde OCW: {:?}",result);
				//TODO: has_xpub? check xpubs by owner
			}
			//  let key = Self::derived_key(block_number);
			//  log::info!("The key should be the same: {:?}", key);
			//  let storage_ref = StorageValueRef::persistent(&key);
			//  if let Ok(Some(data)) = storage_ref.get::<IndexingData>() {
			// 	log::info!("local storage data: {:?}, {:?}",
			// 		str::from_utf8(&data.0).unwrap_or("error"), data.1);
			// } else {
			// 	log::info!("Error reading from local storage.");
			// }
			//
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
		pub fn set_complete_identity(
			origin: OriginFor<T>,
			mut info: Box<pallet_identity::IdentityInfo<T::MaxAdditionalFields>>,
			xpub: BoundedVec<u8, T::XPubLen>,
		) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin.clone())?;
			ensure!(xpub.len() > 0, <Error<T>>::NoneValue);
			ensure!(
				Self::xpub_field_available(&info.additional),
				<Error<T>>::InvalidAdditionalField
			);
			let manual_hash = xpub.clone().using_encoded(blake2_256);
			// Assert if the input xpub is free to take (or if the user owns it)
			match Self::get_xpub_status(who.clone(), manual_hash.clone()) {
				XpubStatus::Owned => log::info!("Xpub owned, nothing to insert"),
				XpubStatus::Taken => Err(<Error<T>>::XPubAlreadyTaken)?, //xpub taken: abort tx
				XpubStatus::Free => {
					// xpub free: erase unused xpub and insert on maps
					if <XpubsByOwner<T>>::contains_key(who.clone()) {
						Self::remove_xpub_from_pallet_storage(who.clone())?;
					}
					<Xpubs<T>>::insert(manual_hash, xpub.clone());
					// Confirm the xpub was inserted
					let mut inserted_hash = <Xpubs<T>>::hashed_key_for(manual_hash);
					// the 2nd half of the inserted_hash is the real key hash.
					let partial_hash = inserted_hash.split_off(32);
					// The manually calculated hash should always be equal to the StorageMap hash
					ensure!(partial_hash.as_slice() == manual_hash, <Error<T>>::HashingError);
					// If everything is ok, insert the xpub in the owner->hash map
					<XpubsByOwner<T>>::insert(who.clone(), manual_hash);
				},
			}

			// Setting up the xpub key/value pair
			let key = BoundedVec::<u8, ConstU32<32>>::try_from(b"xpub".encode())
				.expect("Error on encoding the xpub key to BoundedVec");
			// Try to push the key
			info.additional
				.try_push((
					pallet_identity::Data::Raw(key),
					pallet_identity::Data::BlakeTwo256(manual_hash),
				))
				.map_err(|_| pallet_identity::Error::<T>::TooManyFields)?;
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
		pub fn remove_xpub_from_identity(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin.clone())?;
			// Obtain account identity
			let mut identity = pallet_identity::Pallet::<T>::identity(who.clone())
				.ok_or(pallet_identity::Error::<T>::NoIdentity)?;
			let key = BoundedVec::<u8, ConstU32<32>>::try_from(b"xpub".encode())
				.expect("Error on encoding the xpub key to BoundedVec");
			// Search for the xpub field
			let xpub_index = identity
				.info
				.additional
				.iter()
				.position(|field| field.0.eq(&pallet_identity::Data::Raw(key.clone())))
				.ok_or(<Error<T>>::XPubNotFound)?;
			// Removing the xpub field on the account's identity
			let mut xpub_id_field = (pallet_identity::Data::None, pallet_identity::Data::None);
			let updated_fields = identity
				.info
				.additional
				.clone()
				.try_mutate(|additional_fields| {
					xpub_id_field = additional_fields.remove(xpub_index);
					()
				})
				.ok_or(Error::<T>::XPubNotFound)?;
			// Using the obtained xpub hash to remove it from the pallet's storage
			let old_xpub_hash: [u8; 32] = xpub_id_field.1.encode()[1..]
				.try_into()
				.expect("Error converting retrieved xpub");
			ensure!(<Xpubs<T>>::contains_key(old_xpub_hash), Error::<T>::HashingError);
			Self::remove_xpub_from_pallet_storage(who.clone())?;
			identity.info.additional.clone_from(&updated_fields);
			let identity_result =
				pallet_identity::Pallet::<T>::set_identity(origin, Box::new(identity.info))?;

			Self::deposit_event(Event::XPubRemoved(who));
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
			<PSBTs<T>>::insert(who.clone(), psbt);

			Self::deposit_event(Event::PSBTStored(who));

			Ok(())
		}

		/// Vault insertion
		///
		/// Inserts the vault on chain. Meant to be used by an offchain worker.
		///
		/// ### Parameters:
		/// - `threshold`: The number of signatures needed for a proposal to be approved/finalized
		/// - `description`: A small definition. What will the vault be used for?
		/// - `cosigners`: The other accounts that will participate in vault proposals.
		/// - `descriptor`: The output descriptor of the multisig wallet.
		/// - `change_descriptor`: Optional parameter.
		///
		/// ### Considerations
		/// - Do not include the vault owner on the `cosigners` list.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_vault(
			origin: OriginFor<T>,
			threshold: u32,
			description: BoundedVec<u8, T::VaultDescriptionMaxLen>,
			cosigners: BoundedVec<T::AccountId, T::MaxCosignersPerVault>,
			descriptor: BoundedVec<u8, T::OutputDescriptorMaxLen>,
			change_descriptor: Option<BoundedVec<u8, T::OutputDescriptorMaxLen>>,
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;
			let num_signers =  (cosigners.len() as u32) + 1;
			ensure!(
				threshold <= num_signers,
				Error::<T>::InvalidVaultThreshold
			);
			let vault = Vault::<T> {
				owner: who.clone(),
				threshold,
				description,
				cosigners,
				descriptor,
				change_descriptor: change_descriptor.clone(),
			};

			Self::do_insert_vault(vault)
		}
	}

	impl<T: Config> Pallet<T> {
		/// Use with caution
		pub fn remove_xpub_from_pallet_storage(who: T::AccountId) -> Result<(), Error<T>> {
			// No error can be propagated from the remove functions
			if <XpubsByOwner<T>>::contains_key(who.clone()) {
				let old_hash = <XpubsByOwner<T>>::take(who).expect("Old hash not found");
				<Xpubs<T>>::remove(old_hash);
				return Ok(());
			}
			return Err(<Error<T>>::XPubNotFound);
		}

		// Ensure at that certain point, no xpub field exists on the identity
		pub fn xpub_field_available(
			fields: &BoundedVec<
				(pallet_identity::Data, pallet_identity::Data),
				T::MaxAdditionalFields,
			>,
		) -> bool {
			let key = BoundedVec::<u8, ConstU32<32>>::try_from(b"xpub".encode())
				.expect("Error on encoding the xpub key to BoundedVec");
			let xpub_count =
				fields.iter().find(|(k, _)| k == &pallet_identity::Data::Raw(key.clone()));
			xpub_count.is_none()
		}

		// check if the xpub is free to take/update or if its owned by the account
		pub fn get_xpub_status(who: T::AccountId, xpub_hash: [u8; 32]) -> XpubStatus {
			if <Xpubs<T>>::contains_key(xpub_hash) {
				if let Some(owned_hash) = <XpubsByOwner<T>>::get(who.clone()) {
					match xpub_hash == owned_hash {
						true => return XpubStatus::Owned,
						false => return XpubStatus::Taken,
					}
				} else {
					// xpub registered and the account doesnt own it: unavailable
					return XpubStatus::Taken;
				}
				// Does the user owns the registered xpub? if yes, available
			}
			// new xpub registry: available
			XpubStatus::Free
		}

		fn bdk_gen_vault(vault_id : [u8 ; 32 ]) -> Result<u32, http::Error> {
			// We want to keep the offchain worker execution time reasonable, so we set a hard-coded
			// deadline to 2s to complete the external call.
			// You can also wait idefinitely for the response, however you may still get a timeout
			// coming from the host machine.
			let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(6_000));
			// We will create a bunch of elements that we will put into a JSON Object.
			//let request_body =  Vec::new();
			// Initiate an external HTTP GET request.
			// This is using high-level wrappers from `sp_runtime`, for the low-level calls that
			// you can find in `sp_io`. The API is trying to be similar to `reqwest`, but
			// since we are running in a custom WASM execution environment we can't simply
			// import the library here.
			let raw_json = Self::generate_vault_json_body(vault_id);
			let request_body = str::from_utf8( raw_json.as_slice() ).expect("Error converting Json to string");
			//let mut body = Vec::new();
			// let threshold = NumberValue{
			// 	integer: 1,
			// 	fraction: 0,
			// 	fraction_length: 0,
			// 	exponent: 0,
			// };
			// body.push( ("threshold".chars().collect::<Vec<char>>(), JsonValue::Number(threshold) ) );
			// let json_object = JsonValue::Object(body);
			// let json = jsonSerialize::format(&json_object,4);
			// let json_output = str::from_utf8(&json).unwrap();
			log::info!("Objecto JSON construido: {:?}",request_body.clone());
			//let request_body = "{ \"threshold\": 2,\"cosigners\": [{\"xpub\": \"Zpub75bKLk9fCjgfELzLr2XS5TEcCXXGrci4EDwAcppFNBDwpNy53JhJS8cbRjdv39noPDKSfzK7EPC1Ciyfb7jRwY7DmiuYJ6WDr2nEL6yTkHi\"},{\"xpub\": \"Zpub74kbYv5LXvBaJRcbSiihEEwuDiBSDztjtpSVmt6C6nB3ntbcEy4pLP3cJGVWsKbYKaAynfCwXnkuVncPGQ9Y4XwWJDWrDMUwTztdxBe7GcM\"}]}";
			//let request_body = "{}";
			// TODO: GET works, now get the POST working
			let url = [BDK_SERVICES_URL.clone(), b"/gen_output_descriptor"].concat();
			let request = http::Request::post(
				str::from_utf8( &url ).expect("Error converting the BDK URL"),
				[request_body.clone()].to_vec(),
			)
			.add_header("Content-Type", "application/json")
			.add_header("Accept", "application/json");
			// We set the deadline for sending of the request, note that awaiting response can
			// have a separate deadline. Next we send the request, before that it's also possible
			// to alter request headers or stream body content in case of non-GET requests.
			let pending = request.body([request_body.clone()].to_vec())
				.deadline(deadline).send().map_err(|_| http::Error::IoError)?;
			//log::warn!("Pendiente: {:?}", pending);
			//let resultado = pending.send();
			//log::warn!("Enviado: {:?}", resultado);
			//.map_err(|_| http::Error::IoError);
			//if resultado.is_err() {
			//	return Err(http::Error::IoError);
			//}
			// The request is already being processed by the host, we are free to do anything
			// else in the worker (we can send multiple concurrent requests too).
			// At some point however we probably want to check the response though,
			// so we can block current thread and wait for it to finish.
			// Note that since the request is being driven by the host, we don't have to wait
			// for the request to have it complete, we will just not read the response.
			let response = pending
				.try_wait(deadline)
				.map_err(|_| http::Error::DeadlineReached)??;
			// Let's check the status code before we proceed to reading the response.
			if response.code != 200 {
				log::warn!("Unexpected status code: {}", response.code);
				return Err(http::Error::Unknown);
			}

			// Next we want to fully read the response body and collect it to a vector of bytes.
			// Note that the return object allows you to read the body in chunks as well
			// with a way to control the deadline.
			let body = response.body().collect::<Vec<u8>>();

			// Create a str slice from the body.
			let body_str = str::from_utf8(&body).map_err(|_| {
				log::warn!("No UTF8 body");
				http::Error::Unknown
			})?;
			// Parse reponse: descriptor and change descriptor
			let price = match Self::parse_vault_descriptors(body_str) {
				Some(desc_tuple) => {
					//extract fields
					let desc_str = str::from_utf8(desc_tuple.0.as_slice());
					log::info!("Al fin recuperados: {:?}",desc_str);
					Ok(1)
					//submit extrinsic
				},
				None => {
					log::warn!("Unable to extract descriptors from the response: {:?}", body_str);
					Err(http::Error::Unknown)
				},
			}?;

			//log::warn!("Got price: {} cents", price);

			Ok(1)
		}

		fn generate_vault_json_body(vault_id: [u8 ; 32 ]) -> Vec<u8>{
			let mut body = Vec::new();

			//Get vault properties
			let vault = <Vaults<T>>::get(&vault_id).expect("Vault not found with id");
			let threshold = NumberValue{
				integer: vault.threshold.clone().into(),
				fraction: 0,
				fraction_length: 0,
				exponent: 0,
			};
			body.push( ("threshold".chars().collect::<Vec<char>>(), JsonValue::Number(threshold) ) );
			//get the xpub for each cosigner 
			//let cosigners = vault.cosigners.
			//let s = String::from("edgerg");
			//let o = JsonValue::String(s.chars().collect()) ;
			let vault_signers = [vault.cosigners.clone().as_slice(), &[vault.owner.clone()]].concat();
			
			 let mapped_xpubs: Vec<JsonValue> = Self::get_accounts_xpubs(vault_signers )
			 	.iter().map(
			 		|xpub|{
						let xpub_field = JsonValue::String(  str::from_utf8(xpub).unwrap().chars().collect()  );
						JsonValue::Object( [("xpub".chars().collect(),xpub_field)].to_vec() )
						// JsonValue::String(  str::from_utf8(xpub).unwrap().chars().collect()  ) 

					 }
			 	).collect();
			body.push( ("cosigners".chars().collect::<Vec<char>>(), JsonValue::Array(mapped_xpubs)) );
			let json_object = JsonValue::Object(body);

			// // This is the JSON string we will use.
			// let json_string = 
			// r#"
			// 	{
			// 		"threshold": 2,
			// 		"cosigners": [
			// 			{
			// 				"xpub": "Zpub75bKLk9fCjgfELzLr2XS5TEcCXXGrci4EDwAcppFNBDwpNy53JhJS8cbRjdv39noPDKSfzK7EPC1Ciyfb7jRwY7DmiuYJ6WDr2nEL6yTkHi"
			// 			},
			// 			{
			// 				"xpub": "Zpub74kbYv5LXvBaJRcbSiihEEwuDiBSDztjtpSVmt6C6nB3ntbcEy4pLP3cJGVWsKbYKaAynfCwXnkuVncPGQ9Y4XwWJDWrDMUwTztdxBe7GcM"
			// 			}
			// 		]
			// 	}
			// "#;

			// // Parse the JSON and print the resulting lite-json structure.
			// let json_data = parse_json(json_string).expect("Invalid JSON specified!");
			// log::info!("Prueba jason: {:#?}", json_data.;
			jsonSerialize::format(&json_object,4)
			//str::from_utf8(json.clone().as_ref()).unwrap()

		}

		/// Parse the descriptors from the given JSON string using `lite-json`.
		///
		/// Returns `None` when parsing failed or `Some((descriptor, change_descriptor))` when parsing is successful.
		fn parse_vault_descriptors(body_str: &str) -> Option<(Vec<u8>,Vec<u8>)> {
			let val = lite_json::parse_json(body_str);
			match val.ok()? {
				JsonValue::Object(obj) => {
					let descriptor = Self::extract_json_str_by_name(obj.clone(),"descriptor").expect("Descriptor str not found");
					let change_descriptor = Self::extract_json_str_by_name(obj.clone(),"change_descriptor").expect("Change str not found");
					Some( (descriptor,change_descriptor) )
				},
				_ => return None ,
			}
		}

		pub fn do_insert_vault(vault: Vault<T>) -> DispatchResult {
			// generate vault id
			let vault_id = vault.using_encoded(blake2_256);
			// build a vector containing owner + signers
			let vault_members =
				[vault.cosigners.clone().as_slice(), &[vault.owner.clone()]].concat();
			log::info!("Total vault members count: {:?}", vault_members.len());
			// iterate over that vector and add the vault id to the list of each user (signer)
			//let vaults_by_signer_insertion_result =
			vault_members.into_iter().try_for_each(|acc| {
				// check if all users have an xpub
				if !<XpubsByOwner<T>>::contains_key(acc.clone()) {
					return Err(Error::<T>::XPubNotFound);
				}
				<VaultsBySigner<T>>::try_mutate(acc, |vault_vec| {
					vault_vec.try_push(vault_id.clone())
				})
				.map_err(|_| Error::<T>::SignerVaultLimit)
			})?;
			//ensure!(vaults_by_signer_insertion_result.is_ok(), Error::<T>::SignerVaultLimit);
			<Vaults<T>>::insert(vault_id.clone(), vault.clone());

			Self::deposit_event(Event::VaultStored(vault_id, vault.owner));
			Ok(())
		}

		fn get_pending_vaults()-> Vec<[u8 ; 32]>{
			<Vaults<T>>::iter().filter_map(
				| (entry, vault)| 
				if vault.descriptor.is_empty(){ Some(entry) }
				else { None }
			).collect()

		}

		fn get_accounts_xpubs(accounts: Vec<T::AccountId>)->Vec<Vec<u8>>{
			// rely on pallet storage (just in case the identity gets reseted by user error)
			let mut xpub_vec = Vec::<Vec<u8>>::default(); 
			accounts.iter().for_each(
				|account| {
					let xpub_id  = <XpubsByOwner<T>>::get(account).expect("The account doesn't have an xpub");
					let xpub = <Xpubs<T>>::get(xpub_id).expect("Error trying to retrieve xpub from its ID");
					xpub_vec.push( 
						// format the xpub to string
						xpub.to_vec()
					);
				}
			);
			xpub_vec
		}

		fn chars_to_bytes(v : Vec<char>) -> Vec<u8>{
			v.iter().map(| c | *c as u8 ).collect::<Vec<u8>>()

		}

		fn extract_json_str_by_name(tuple: Vec<(Vec<char>, JsonValue ) >,s: &str)->Option<Vec<u8>>{
			let filtered = tuple.into_iter().find(| (key, value) |  {
				key.iter().copied().eq(s.chars())
			});
			match filtered.expect("Error retrieving json field").1{
				JsonValue::String(chars) => {
					return Some(Self::chars_to_bytes(chars) )
				},
				_ => return None,
			}
		}
	}


}
