#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

pub mod types;
mod functions;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, transactional};
	use frame_system::pallet_prelude::*;
	use crate::types::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type RemoveOrigin: EnsureOrigin<Self::Origin>;

		#[pallet::constant]
		type MaxOwnedDocs: Get<u32>;
		#[pallet::constant]
		type MaxSharedToDocs: Get<u32>;
		#[pallet::constant]
		type DocNameMinLen: Get<u32>;
		#[pallet::constant]
		type DocNameMaxLen: Get<u32>;
		#[pallet::constant]
		type DocDescMinLen: Get<u32>;
		#[pallet::constant]
		type DocDescMaxLen: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn vaults)]
	pub(super) type Vaults<T: Config> = StorageMap<
		_,
		Blake2_256,
		UserId, //user identifier
		Vault<T>,
		OptionQuery
	>;

  #[pallet::storage]
	#[pallet::getter(fn public_keys)]
	pub(super) type PublicKeys<T: Config> = StorageMap<
		_,
		Blake2_256,
		T::AccountId,
		PublicKey,
		OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn owned_docs)]
	pub(super) type OwnedDocs<T: Config> = StorageMap<
		_, 
		Blake2_256,
		CID,
		OwnedDoc<T>,
		OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn owned_docs_by_owner)]
	pub(super) type OwnedDocsByOwner<T: Config> = StorageMap<
		_,
		Blake2_256,
		T::AccountId,
		BoundedVec<CID, T::MaxOwnedDocs>,
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn shared_docs)]
	pub(super) type SharedDocs<T: Config> = StorageMap<
		_, 
		Blake2_256,
		CID,
		SharedDoc<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn shared_docs_by_to)]
	pub(super) type SharedDocsByTo<T: Config> = StorageMap<
		_,
		Blake2_256,
		T::AccountId,
		BoundedVec<CID, T::MaxSharedToDocs>,
		ValueQuery
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		VaultStored(UserId, PublicKey, Vault<T>),
		OwnedDocStored(OwnedDoc<T>),
		SharedDocStored(SharedDoc<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Empty CID
		CIDNoneValue,
		/// Document Name is too short
		DocNameTooShort,
		/// Document Desc is too short
		DocDescTooShort,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// The user already has a vault
		UserAlreadyHasVault,
		/// The user already has a public key
		AccountAlreadyHasPublicKey,
		/// User is not document owner
		NotDocOwner,
		/// The document has already been shared
		DocAlreadyShared,
		/// Shared with self
		DocSharedWithSelf,
		/// Account has no public key
		AccountHasNoPublicKey,
		/// Max owned documents has been exceeded
		ExceedMaxOwnedDocs,
		/// Max documents shared with the "to" account has been exceeded
		ExceedMaxSharedToDocs,
		
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_vault(origin: OriginFor<T>, user_id: UserId, public_key: PublicKey, cid: CID) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_set_vault(who, user_id, public_key, cid)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_owned_document(origin: OriginFor<T>, owned_doc: OwnedDoc<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_set_owned_document(who, owned_doc)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn share_document(origin: OriginFor<T>, shared_doc: SharedDoc<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_share_document(who, shared_doc)
		}

		/// Kill all the stored data.
		/// 
		/// This function is used to kill ALL the stored data.
		/// Use with caution!
		/// 
		/// ### Parameters:
		/// - `origin`: The user who performs the action. 
		/// 
		/// ### Considerations:
		/// - This function is only available to the `admin` with sudo access.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn kill_storage(
			origin: OriginFor<T>,
		) -> DispatchResult{
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			<Vaults<T>>::remove_all(None);
			<PublicKeys<T>>::remove_all(None);
			<OwnedDocs<T>>::remove_all(None);
			<OwnedDocsByOwner<T>>::remove_all(None);
			<SharedDocs<T>>::remove_all(None);
			<SharedDocsByTo<T>>::remove_all(None);
			Ok(())
		}
	}
}