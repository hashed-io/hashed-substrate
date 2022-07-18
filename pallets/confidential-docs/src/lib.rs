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
	#[pallet::getter(fn documents)]
	pub(super) type Documents<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		CID,
		Document<T>,
		OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn shared_documents)]
	pub(super) type SharedDocuments<T: Config> = StorageMap<
		_, 
		Blake2_256,
		CID,
		SharedDocument<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn shared_documents_by_to)]
	pub(super) type SharedDocumentsByTo<T: Config> = StorageMap<
		_,
		Blake2_256,
		T::AccountId,
		CID,
		OptionQuery
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		VaultStored(UserId, PublicKey, Vault<T>),
		DocStored(T::AccountId, CID, Document<T>),
		SharedDocStored(T::AccountId, CID, SharedDocument<T>),
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
		/// The document has already been share with user
		DocumentAlreadySharedWithUser,
		/// Shared with self
		DocumentSharedWithSelf,
		
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
		pub fn set_document(origin: OriginFor<T>, cid: CID, doc_name: DocName<T>, doc_desc: DocDesc<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_set_document(who, cid, doc_name, doc_desc)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn share_document(origin: OriginFor<T>, cid: CID, shared_doc: SharedDocument<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_share_document(who, cid, shared_doc)
		}
	}
}