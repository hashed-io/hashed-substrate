#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

mod functions;
pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use crate::types::*;
	use frame_support::{pallet_prelude::*, transactional, BoundedVec};
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_uniques::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type RemoveOrigin: EnsureOrigin<Self::Origin>;

		/// Maximum number of children a Frunique can have
		#[pallet::constant]
		type ChildMaxLen: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// A frunique and asset class were successfully created!
		FruniqueCollectionCreated(T::AccountId, T::CollectionId),
		// A frunique and asset class were successfully created!
		FruniqueCreated(T::AccountId, T::AccountId, T::CollectionId, T::ItemId),
		// A frunique/unique was successfully divided!
		FruniqueDivided(T::AccountId, T::AccountId, T::CollectionId, T::ItemId),
		// Counter should work?
		NextFrunique(u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		// The user does not have permission to perform this action
		NoPermission,
		// Only the owner of the Frunique can perform this action
		NotAdmin,
		// The storage is full
		StorageOverflow,
		// A feature not implemented yet
		NotYetImplemented,
		// Too many fruniques were minted
		FruniqueCntOverflow,
		// The asset_id is not linked to a frunique or it doesn't exists
		NotAFrunique,
		// The key of an attribute it's too long
		KeyTooLong,
		// The value of an attribute it's too long
		ValueTooLong,
		// Calling set on a non-existing attributes
		AttributesEmpty,
		// The collection doesn't exist
		CollectionNotFound,
		// The parent doesn't exist
		ParentNotFound,
		// The frunique doesn't exist
		FruniqueNotFound,
		// Collection already exists
		CollectionAlreadyExists,
		// Frunique already exists
		FruniqueAlreadyExists,
	}

	#[pallet::storage]
	#[pallet::getter(fn frunique_cnt)]
	/// Keeps track of the number of Kitties in existence.
	pub(super) type FruniqueCnt<T: Config> = StorageValue<_, ItemId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn next_collection)]
	/// Keeps track of the number of collections in existence.
	pub(super) type NextCollection<T: Config> = StorageValue<
		_,
		CollectionId, // Next collection id.
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn next_frunique)]
	/// Keeps track of the number of fruniques in existence for a collection.
	pub(super) type NextFrunique<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		CollectionId,
		ItemId, // The next frunique id for a collection.
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn frunique_parent)]
	/// Keeps track of hierarchical information for a frunique.
	pub(super) type FruniqueParent<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		CollectionId,
		Blake2_128Concat,
		ItemId,                   // FruniqueId
		Option<HierarchicalInfo>, // ParentId and flag if it inherit attributes
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn frunique_child)]
	/// Keeps track of hierarchical information for a frunique.
	pub(super) type FruniqueChild<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		CollectionId, // Parent collection id
		Blake2_128Concat,
		ItemId,            // Parent item id
		Option<ChildInfo>, // ParentId and flag if it inherit attributes
		ValueQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T: pallet_uniques::Config<CollectionId = CollectionId, ItemId = ItemId>,
	{
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(10))]
		pub fn initial_setup(origin: OriginFor<T>) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			// Self::do_initial_setup()?;
			Ok(())
		}

		/// # Creation of a collection
		/// This function creates a collection and an asset class.
		/// The collection is a unique identifier for a set of fruniques.
		///
		/// ## Parameters
		/// - `origin`: The origin of the transaction.
		/// - `Metadata`: The title of the collection.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_collection(
			origin: OriginFor<T>,
			metadata: Option<CollectionDescription<T>>,
		) -> DispatchResult {
			let admin: T::AccountId = ensure_signed(origin.clone())?;

			let new_collection_id: u32 = Self::next_collection().try_into().unwrap();

			Self::do_create_collection(
				origin,
				new_collection_id,
				metadata,
				Self::account_id_to_lookup_source(&admin),
			)?;

			Self::deposit_event(Event::FruniqueCollectionCreated(admin, new_collection_id));

			<NextCollection<T>>::put(Self::next_collection() + 1);

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(4))]
		pub fn instance_exists(
			_origin: OriginFor<T>,
			_class_id: T::CollectionId,
			_instance_id: T::ItemId,
		) -> DispatchResult {
			// Always returns an empty iterator?
			//let instances = pallet_uniques::Pallet::<T>::;
			//println!("Instances found in class {:?}",instances.count());
			//log::info!("Instances found in class {:?}", instances.count());
			//println!("\tIterator? {}",instances.count());
			//Self::deposit_event(Event::NextFrunique(instances.count().try_into().unwrap()  ));
			//instances.into_iter().for_each(|f| println!("\tInstance:{:?}",f));
			Ok(())
		}

		/// ## Set multiple attributes to a frunique.
		/// - `origin` must be signed by the owner of the frunique.
		/// - `class_id` must be a valid class of the asset class.
		/// - `instance_id` must be a valid instance of the asset class.
		/// - `attributes` must be a list of pairs of `key` and `value`.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_attributes(
			origin: OriginFor<T>,
			class_id: T::CollectionId,
			instance_id: T::ItemId,
			attributes: Vec<(BoundedVec<u8, T::KeyLimit>, BoundedVec<u8, T::ValueLimit>)>,
		) -> DispatchResult {
			ensure!(Self::item_exists(&class_id, &instance_id), <Error<T>>::FruniqueNotFound);

			// ! Ensure the admin is the one who can add attributes to the frunique.
			let admin = Self::admin_of(&class_id, &instance_id);
			let signer = core::prelude::v1::Some(ensure_signed(origin.clone())?);

			ensure!(signer == admin, <Error<T>>::NotAdmin);

			ensure!(!attributes.is_empty(), Error::<T>::AttributesEmpty);
			for attribute in &attributes {
				Self::set_attribute(
					origin.clone(),
					&class_id.clone(),
					Self::u32_to_instance_id(instance_id.clone()),
					attribute.0.clone(),
					attribute.1.clone(),
				)?;
			}
			Ok(())
		}

		/// ## NFT creation
		/// ### Parameters:
		/// - `origin` must be signed by the owner of the frunique.
		/// - `class_id` must be a valid class of the asset class.
		/// - `parent_info` Optional value needed for the NFT division.
		/// - `attributes` An array of attributes (key, value) to be added to the NFT.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(4))]
		pub fn spawn(
			origin: OriginFor<T>,
			class_id: CollectionId,
			parent_info: Option<HierarchicalInfo>,
			attributes: Option<Vec<(BoundedVec<u8, T::KeyLimit>, BoundedVec<u8, T::ValueLimit>)>>,
		) -> DispatchResult {
			ensure!(Self::collection_exists(&class_id), <Error<T>>::CollectionNotFound);

			if let Some(parent_info) = parent_info {
				ensure!(Self::item_exists(&class_id, &parent_info.0), <Error<T>>::ParentNotFound);
			}

			let owner: T::AccountId = ensure_signed(origin.clone())?;
			let account_id = Self::account_id_to_lookup_source(&owner);

			let instance_id: ItemId = <NextFrunique<T>>::try_get(class_id).unwrap_or(0);
			<NextFrunique<T>>::insert(class_id, instance_id + 1);

			if let Some(parent_info) = parent_info {
				ensure!(Self::item_exists(&class_id, &parent_info.0), <Error<T>>::ParentNotFound);
				<FruniqueParent<T>>::insert(class_id, instance_id, Some(parent_info));

				let child_info = ChildInfo {
					collection_id: class_id,
					child_id: instance_id,
					is_hierarchical: parent_info.1,
					weight: Self::percent_to_permill(parent_info.2),
				};

				<FruniqueChild<T>>::insert(class_id, instance_id, Some(child_info));
			}

			Self::do_spawn(origin.clone(), class_id, instance_id, account_id, attributes)?;

			Self::deposit_event(Event::FruniqueCreated(
				owner.clone(),
				owner,
				class_id,
				instance_id,
			));

			Ok(())
		}

		/// ## Force set counter
		/// ### Parameters:
		/// `origin` must be signed by the Root origin.
		/// - `class_id` must be a valid class of the asset class.
		/// - `instance_id` must be a valid instance of the asset class.
		///
		/// ### Considerations:
		/// This function is only used for testing purposes. Or in case someone calls uniques pallet directly.
		/// This function it's not expected to be used in production as it can lead to unexpected results.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn force_set_counter(
			origin: OriginFor<T>,
			class_id: T::CollectionId,
			instance_id: Option<T::ItemId>,
		) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;

			if let Some(instance_id) = instance_id {
				ensure!(!Self::item_exists(&class_id, &instance_id), <Error<T>>::FruniqueAlreadyExists);
				<NextFrunique<T>>::insert(class_id, instance_id);
			} else {
				ensure!(!Self::collection_exists(&class_id), <Error<T>>::CollectionAlreadyExists);
				<NextCollection<T>>::set(class_id);
			}

			Ok(())
		}

		/// ## Force destroy collection
		/// ### Parameters:
		/// - `origin` must be signed by the Root origin.
		/// - `class_id` must be a valid class of the asset class.
		/// - `witness` the witness data to destroy the collection. This is used to prevent accidental destruction of the collection. The witness data is retrieved from the `class` storage.
		/// - `maybe_check_owner` Optional value to check if the owner of the collection is the same as the signer.
		/// ### Considerations:
		/// This function is only used for testing purposes. Or in case someone calls uniques pallet directly.
		/// This function it's not expected to be used in production as it can lead to unexpected results.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn force_destroy_collection(
			origin: OriginFor<T>,
			class_id: T::CollectionId,
			witness: pallet_uniques::DestroyWitness,
			maybe_check_owner: Option<T::AccountId>,
		) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;

			ensure!(Self::collection_exists(&class_id), <Error<T>>::CollectionNotFound);
			pallet_uniques::Pallet::<T>::do_destroy_collection(
				class_id,
				witness,
				maybe_check_owner,
			)?;
			Ok(())
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
		pub fn kill_storage(origin: OriginFor<T>) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			let _ = <FruniqueCnt<T>>::put(0);
			let _ = <NextCollection<T>>::put(0);
			let _ = <NextFrunique<T>>::clear(1000, None);
			let _ = <FruniqueParent<T>>::clear(1000, None);
			let _ = <FruniqueChild<T>>::clear(1000, None);

			Ok(())
		}
	}
}
