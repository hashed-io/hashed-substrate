#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod types;
mod functions;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	//#[cfg(feature = "std")]
	//use frame_support::serde::{Deserialize, Serialize};
	use crate::types::{UNSIGNED_TXS_PRIORITY, LOCK_BLOCK_EXPIRATION, LOCK_TIMEOUT_EXPIRATION};
	use frame_support::{
		pallet_prelude::{BoundedVec, MaxEncodedLen},
		traits::Get,
	};
	use frame_support::{sp_io::hashing::blake2_256, transactional};
	use frame_system::{
		offchain::{
			AppCrypto, CreateSignedTransaction, SendUnsignedTransaction,
			SignedPayload, Signer, SigningTypes,
		},
		pallet_prelude::*,
	};
	use sp_runtime::sp_std::str;
	use sp_runtime::sp_std::vec::Vec;
	use sp_runtime::{
		transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
		offchain::{Duration, storage_lock::{StorageLock,BlockAndTime}},
		RuntimeDebug,
	};
	use scale_info::TypeInfo;

	/*--- Structs Section ---*/
	#[derive(
		Encode,
		Decode,
		Default,
		Eq,
		PartialEq,
		CloneNoBound,
		RuntimeDebugNoBound,
		TypeInfo,
		MaxEncodedLen,
	)]
	#[scale_info(skip_type_params(MaxLen))]
	#[codec(mel_bound())]
	pub struct Descriptors<MaxLen: Get<u32>> {
		pub output_descriptor: BoundedVec<u8, MaxLen>,
		pub change_descriptor: Option<BoundedVec<u8, MaxLen>>,
	}
	
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
	#[codec(mel_bound())]
	pub struct VaultsPayload<Public> {
		pub vaults_payload:Vec<SingleVaultPayload>,
		pub public: Public,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
	#[codec(mel_bound())]
	pub struct SingleVaultPayload{
		// Not successful, macros/generics issue
		// descriptors: Descriptors<u8>,
		pub vault_id: [u8;32],
		pub output_descriptor: Vec<u8>,
		pub change_descriptor: Vec<u8>,
	}
	
	impl<S: SigningTypes> SignedPayload<S> for VaultsPayload<S::Public> {
		fn public(&self) -> S::Public {
			self.public.clone()
		}
	}

	// Struct for holding Vaults information.
	#[derive(
		Encode, Decode, Eq, PartialEq, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen,
	)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Vault<T : Config> {
		pub owner: T::AccountId,
		pub threshold: u32,
		pub description: BoundedVec<u8, T::VaultDescriptionMaxLen>,
		pub cosigners: BoundedVec<T::AccountId, T::MaxCosignersPerVault>,
		pub descriptors: Descriptors<T::OutputDescriptorMaxLen>,
	}

	impl<T: Config> Clone for Vault<T> {
		fn clone(&self) -> Self {
			Vault {
				owner: self.owner.clone(),
				threshold: self.threshold.clone(),
				cosigners: self.cosigners.clone(),
				description: self.description.clone(),
				descriptors: self.descriptors.clone(),
			}
		}
	}

	#[derive(Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct ProposalSignatures<T: Config> {
		pub signer: T::AccountId,
		pub signature: BoundedVec<u8, T::PSBTMaxLen>,
	}

	impl<T: Config> Clone for ProposalSignatures<T>{
		fn clone(&self) -> Self {
			Self{
				signer: self.signer.clone(),
				signature: self.signature.clone(),
			}
		}
	}
	// Struct for holding Proposal information.
	#[derive(Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Proposal<T: Config> {
		pub proposer: T::AccountId,
		pub vault_id: [u8; 32],
		pub status: ProposalStatus,
		pub to_address: BoundedVec<u8, T::XPubLen>,
		pub amount: u64,
		pub fee_sat_per_vb: u32,
		pub description: BoundedVec<u8, T::VaultDescriptionMaxLen>,
		pub psbt: BoundedVec<u8, T::PSBTMaxLen>,
		pub signed_psbts: BoundedVec<ProposalSignatures<T>, T::MaxCosignersPerVault>,
	}

	impl<T: Config> Clone for Proposal<T>{
		fn clone(&self) -> Self {
			Self{
				proposer: self.proposer.clone(),
				vault_id: self.vault_id.clone(),
				status: self.status.clone(),
				to_address: self.to_address.clone(),
				amount: self.amount.clone(),
				fee_sat_per_vb: self.fee_sat_per_vb.clone(),
				description: self.description.clone(),
				psbt: self.psbt.clone(),
				signed_psbts: self.signed_psbts.clone(),
			}
		}
	}

	pub enum XpubStatus {
		Owned,
		Free,
		Taken,
	}
	
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum ProposalStatus {
		Pending,
		Broadcasted,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config + CreateSignedTransaction<Call<Self>>
	{
		/// The identifier type for an offchain worker.
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
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
		#[pallet::constant]
		type MaxProposalsPerVault: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Xpub and hash stored
		XPubStored([u8; 32], T::AccountId),
		/// Removed Xpub previously linked to the account
		XPubRemoved(T::AccountId),
		/// The PBST was succesfully inserted and linked to the account
		PSBTStored(T::AccountId),
		/// The vault was succesfully inserted and linked to the account as owner
		VaultStored([u8; 32], T::AccountId),
		/// The vault was succesfully removed by its owner
		VaultRemoved([u8; 32],T::AccountId),
		/// An offchain worker inserted a vault's descriptor 
		DescriptorsStored([u8;32]),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Work in progress!
		NotYetImplemented,
		/// Xpub shouldn't be empty
		NoneValue,
		// The xpub has already been uploaded and taken by an account
		XPubAlreadyTaken,
		/// The Account doesn't have an xpub
		XPubNotFound,
		/// The user already has an xpub, try to remove it first
		UserAlreadyHasXpub,
		/// The Xpub cant be removed/changed because a vault needs it
		XpubLinkedToVault,
		/// The generated Hashes aren't the same
		HashingError,
		/// Found Invalid name on an additional field
		InvalidAdditionalField,
		/// The vault threshold cannot be greater than the number of vault participants
		InvalidVaultThreshold,
		/// A defined cosigner reached its vault limit
		SignerVaultLimit,
		/// Vault not found
		VaultNotFound,
		/// A vault needs at least 1 cosigner
		NotEnoughCosigners,
		/// Only the owner of this vault can do this transaction
		VaultOwnerPermissionsNeeded,
		/// Vault members cannot be duplicate
		DuplicateVaultMembers,
		/// The account must participate in the vault to make a proposal
		SignerPermissionsNeeded,
		/// The vault has too many proposals 
		ExceedMaxProposalsPerVault
	}

	/*--- Onchain storage section ---*/
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
	#[pallet::getter(fn proposals)]
	pub(super) type Proposals<T: Config> = StorageMap<
		_,
		Identity,
		[u8; 32], //proposalid
		Proposal<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn proposals_by_vault)]
	pub(super) type ProposalsByVault<T: Config> = StorageMap<
		_,
		Identity,
		[u8; 32], //vaultId
		BoundedVec<[u8; 32],T::MaxProposalsPerVault>, //proposalId
		ValueQuery,
	>;

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
			// check if the node has an account available, the offchain worker can't submit
			// transactions without it
			let signer = Signer::<T, T::AuthorityId>::any_account();
			if !signer.can_sign(){
				return;	
			}

			// Check if this OCW can modify the vaults
			let mut lock = StorageLock::<BlockAndTime<Self>>::with_block_and_time_deadline(
				b"nbv::vault-storage-lock",
				LOCK_BLOCK_EXPIRATION,
				Duration::from_millis(LOCK_TIMEOUT_EXPIRATION)
			);
			if let Ok(_guard) = lock.try_lock() {
				// check for pending vaults to insert
				let pending_vaults = Self::get_pending_vaults();
				log::info!("Pending vaults {:?}", pending_vaults.len());
				// This validation needs to be done after the lock: 
				if pending_vaults.len()<1 { return;}
				let mut generated_vaults = Vec::<SingleVaultPayload>::new();
				pending_vaults.iter().for_each(|vault_to_complete| {
					
					log::warn!("Trying to gen vault at block {:?}", block_number);
					// Contact bdk services and get descriptors
					let vault_result = Self::bdk_gen_vault(vault_to_complete.clone())
						.expect("Error while generating the vault's output descriptors");
					// Build offchain vaults struct and push it to a Vec
					generated_vaults.push(SingleVaultPayload{
						vault_id: vault_to_complete.clone(),
						output_descriptor: vault_result.0.clone(),
						change_descriptor: vault_result.1.clone(),
					});
				});
				if let Some((_, res)) = signer.send_unsigned_transaction(
					// this line is to prepare and return payload
					|acct| VaultsPayload {
						vaults_payload: generated_vaults.clone(),
						public: acct.public.clone(),
					},
					|payload, signature| Call::ocw_insert_descriptors { payload, signature },
				) {
					match res {
						Ok(()) => log::info!("unsigned tx with signed payload successfully sent."),
						Err(()) => log::error!("sending unsigned tx with signed payload failed."),
					};
				} else {
					// The case of `None`: no account is available for sending
					log::error!("No local account available");
				}
				
			}else {
				log::error!("This OCW couln't get the locc");
			};

		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// ## XPub insertion
		///
		/// This extrinsic inserts a user-defined xpub
		/// as well as in the pallet storage.
		///
		/// ### Parameters:
		/// - `xpub`: Extended public key, it can be sent with or without fingerprint/derivation path
		///
		/// ### Considerations
		/// - The origin must be Signed and the sender must have sufficient funds free for the transaction fee.
		/// - This extrinsic is marked as transactional, so if an error is fired, all the changes will be reverted (but the
		///  fees will be applied nonetheless).
		/// - This extrinsic cannot handle a xpub update (yet). if it needs to be updated, remove it first and insert
		/// a new one.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
		pub fn set_xpub(
			origin: OriginFor<T>,
			xpub: BoundedVec<u8, T::XPubLen>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin.clone())?;
			ensure!(xpub.len() > 0, <Error<T>>::NoneValue);
			ensure!(!<XpubsByOwner<T>>::contains_key(who.clone()) , <Error<T>>::UserAlreadyHasXpub);
			let manual_hash = xpub.clone().using_encoded(blake2_256);
			// Assert if the input xpub is free to take (or if the user owns it)
			match Self::get_xpub_status(who.clone(), manual_hash.clone()) {
				XpubStatus::Owned => log::info!("Xpub owned, nothing to insert"),
				XpubStatus::Taken => Err(<Error<T>>::XPubAlreadyTaken)?, //xpub taken: abort tx
				XpubStatus::Free => {
					// xpub free: erase unused xpub and insert on maps
					// if <XpubsByOwner<T>>::contains_key(who.clone()) {
					// 	Self::remove_xpub_from_pallet_storage(who.clone())?;
					// }
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
			// Emit a success event.
			Self::deposit_event(Event::XPubStored(manual_hash, who));

			// Return a successful DispatchResult
			Ok(())
		}

		/// ## Xpub removal
		///
		/// Removes the linked xpub from the account which signs the transaction.
		/// The xpub will be removed from both the pallet storage and identity registration.
		///
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(2))]
		pub fn remove_xpub(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;
			// The xpub must exists
			ensure!(<XpubsByOwner<T>>::contains_key(who.clone()), Error::<T>::XPubNotFound);
			// The xpub must not be used on a vault
			let vaults: Vec<[u8;32]>= <VaultsBySigner<T>>::get(who.clone()).iter().filter(|id|{
				match <Vaults<T>>::get(id){
					Some(vault) =>{
						let vault_members = [
							vault.cosigners.as_slice(),
							&[vault.owner.clone()],
						].concat();
						vault_members.contains(&who.clone())
					},
					None => false,
				}
			}).cloned().collect::<Vec<_>>();
			ensure!(vaults.is_empty(),  Error::<T>::XpubLinkedToVault);
			
			Self::do_remove_xpub(who.clone())
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
			_psbt: BoundedVec<u8, T::PSBTMaxLen>,
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;
			//<PSBTs<T>>::insert(who.clone(), psbt);

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
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;
			//  Cosigners are already bounded, only is necessary to check if its not empty
			ensure!( !cosigners.is_empty() , Error::<T>::NotEnoughCosigners);
			// Threshold needs to be greater than 0 and less than the current owner+cosigners number
			ensure!( threshold>0 && threshold <= (1+cosigners.len() as u32), Error::<T>::InvalidVaultThreshold);
			let vault = Vault::<T> {
				owner: who.clone(),
				threshold,
				description,
				cosigners,
				descriptors: Descriptors::<T::OutputDescriptorMaxLen> {
					output_descriptor: BoundedVec::<u8, T::OutputDescriptorMaxLen>::try_from(
						b"".encode(),
					)
					.expect("Error on encoding the descriptor to BoundedVec"),
					change_descriptor: None,
				},
			};

			Self::do_insert_vault(vault)
		}

		// Vault removal
		// Tries to remove vault
		// TODO: Add PSBT validation when they get implemented
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_vault(
			origin: OriginFor<T>,
			vault_id: [u8; 32],
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;
			// Ensure vault exists and get it
			let vault = <Vaults<T>>::get(vault_id).ok_or(Error::<T>::VaultNotFound)?;
			ensure!(vault.owner.eq(&who), Error::<T>::VaultOwnerPermissionsNeeded);

			Self::do_remove_vault(vault_id)
		}

		#[transactional]
		#[pallet::weight(0)]
		pub fn ocw_insert_descriptors(
			origin: OriginFor<T>,
			payload: VaultsPayload<T::Public>,
			_signature: T::Signature,
		) -> DispatchResult {
			// This ensures that the function can only be called via unsigned transaction.
			ensure_none(origin.clone())?;
			payload.vaults_payload.iter().find_map(
				|vault_payload|{
					let output_descriptor = BoundedVec::<u8, T::OutputDescriptorMaxLen>::
						try_from(vault_payload.output_descriptor.clone()).expect("Error trying to convert desc to bounded vec");
					let change_descriptor = BoundedVec::<u8, T::OutputDescriptorMaxLen>::
						try_from(vault_payload.change_descriptor.clone()).expect("Error trying to convert change desc to bounded vec");
					let descriptors = Descriptors::<T::OutputDescriptorMaxLen>{
						output_descriptor : output_descriptor,
						change_descriptor : Some(change_descriptor),
					};
					//assert!(Self::do_insert_descriptors(vault_payload.vault_id,descriptors).is_ok());
					let tx_res = Self::do_insert_descriptors(vault_payload.vault_id,descriptors);
					if tx_res.is_err(){
						return Some(tx_res);
					}
					None

				}
			).unwrap_or(Ok(()))?;
			Ok(())
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn propose(
			origin: OriginFor<T>,
			vault_id: [u8; 32],
			recipient_address: BoundedVec<u8, T::XPubLen>,
			amount_in_sats: u64,
			description: BoundedVec<u8, T::VaultDescriptionMaxLen>,
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;
			//ensure!(false, Error::<T>::NotYetImplemented);
			ensure!(Self::get_vault_members(vault_id.clone()).contains(&who),Error::<T>::SignerPermissionsNeeded);
			// ensure user is in the vault
			let proposal = Proposal::<T>{
				proposer: who.clone(),
				vault_id,
				status: ProposalStatus::Pending,
				to_address: recipient_address,
				amount: amount_in_sats,
				fee_sat_per_vb: 1,
				description,
				psbt: BoundedVec::<u8, T::PSBTMaxLen>::try_from(
					b"".encode()
				).expect("Error on encoding the descriptor to BoundedVec"),
				signed_psbts: BoundedVec::<ProposalSignatures<T>, T::MaxCosignersPerVault>::default(),
			};
			Self::do_propose(proposal)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn generate_new_address(
			origin: OriginFor<T>,
			_vault_id: [u8; 32],
		) -> DispatchResult {
			let _who = ensure_signed(origin.clone())?;
			ensure!(false, Error::<T>::NotYetImplemented);

			Ok(())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {

		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			let valid_tx = |provide| {
				ValidTransaction::with_tag_prefix("bdks")
					.priority(UNSIGNED_TXS_PRIORITY)
					.and_provides([&provide])
					.longevity(3)
					.propagate(true)
					.build()
			};

			match call {
				Call::ocw_insert_descriptors { ref payload, ref signature } => {
					if !SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone()) {
						return InvalidTransaction::BadProof.into();
					}
					valid_tx(b"unsigned_extrinsic_with_signed_payload".to_vec())
				},
				_ => InvalidTransaction::Call.into(),
			}
		}
	}
}