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
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	// use frame_support::PalletId;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	use pallet_rbac::types::RoleBasedAccessControl;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_uniques::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type RemoveOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// Maximum number of children a Frunique can have
		#[pallet::constant]
		type ChildMaxLen: Get<u32>;

		/// The fruniques pallet id, used for deriving its sovereign account ID.
		// #[pallet::constant]
		// type PalletId: Get<PalletId>;
		type Rbac: RoleBasedAccessControl<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
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
		// A frunique has been verified.
		FruniqueVerified(T::AccountId, CollectionId, ItemId),
		// A user has been invited to collaborate on a collection.
		InvitedToCollaborate(T::AccountId, T::AccountId, T::CollectionId),
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
		/// Frunique is bigger than the maximum allowed size
		ExceedMaxPercentage,
		// The parent doesn't exist
		ParentNotFound,
		// The frunique doesn't exist
		FruniqueNotFound,
		// Collection already exists
		CollectionAlreadyExists,
		// Frunique already exists
		FruniqueAlreadyExists,
		// Frunique already verified
		FruniqueAlreadyVerified,
	}

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
		T::CollectionId,
		ItemId, // The next frunique id for a collection.
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn frunique_info)]
	pub(super) type FruniqueInfo<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::CollectionId,
		Blake2_128Concat,
		ItemId,
		FruniqueData<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn frunique_verified)]
	/// Keeps track of verified fruniques.
	pub(super) type FruniqueVerified<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::CollectionId,
		Blake2_128Concat,
		T::ItemId,
		bool,
		ValueQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T: pallet_uniques::Config<CollectionId = CollectionId, ItemId = ItemId>,
	{
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(10))]
		pub fn initial_setup(origin: OriginFor<T>) -> DispatchResult {
			//Transfer the balance
        	// T::Currency::transfer(&buyer, &Self::pallet_account(), , KeepAlive)?;
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			Self::do_initial_setup()?;
			Ok(())
		}

		/// # Creation of a collection
		/// This function creates a collection and an asset class.
		/// The collection is a unique identifier for a set of fruniques.
		///
		/// ## Parameters
		/// - `origin`: The origin of the transaction.
		/// - `metadata`: The title of the collection.
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn create_collection(
			origin: OriginFor<T>,
			metadata: CollectionDescription<T>,
		) -> DispatchResult {
			let admin: T::AccountId = ensure_signed(origin.clone())?;

			Self::do_create_collection(origin, metadata, admin.clone())?;

			let next_collection_id: u32 = Self::next_collection();
			Self::deposit_event(Event::FruniqueCollectionCreated(admin, next_collection_id));

			Ok(())
		}

		/// ## Set multiple attributes to a frunique.
		/// - `origin` must be signed by the owner of the frunique.
		/// - `class_id` must be a valid class of the asset class.
		/// - `instance_id` must be a valid instance of the asset class.
		/// - `attributes` must be a list of pairs of `key` and `value`.
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn set_attributes(
			origin: OriginFor<T>,
			class_id: T::CollectionId,
			instance_id: T::ItemId,
			attributes: Attributes<T>,
		) -> DispatchResult {
			ensure!(Self::item_exists(&class_id, &instance_id), Error::<T>::FruniqueNotFound);

			// ! Ensure the admin is the one who can add attributes to the frunique.
			let admin = Self::admin_of(&class_id, &instance_id);
			let signer = core::prelude::v1::Some(ensure_signed(origin.clone())?);

			ensure!(signer == admin, Error::<T>::NotAdmin);

			ensure!(!attributes.is_empty(), Error::<T>::AttributesEmpty);
			for attribute in &attributes {
				Self::set_attribute(
					origin.clone(),
					&class_id.clone(),
					Self::u32_to_instance_id(instance_id),
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
		/// - `metadata` Title of the nft.
		/// - `attributes` An array of attributes (key, value) to be added to the NFT.
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(4))]
		pub fn spawn(
			origin: OriginFor<T>,
			class_id: CollectionId,
			metadata: CollectionDescription<T>,
			attributes: Option<Attributes<T>>,
		) -> DispatchResult {
			ensure!(Self::collection_exists(&class_id), Error::<T>::CollectionNotFound);
			let user: T::AccountId = ensure_signed(origin.clone())?;
			Self::is_authorized(user, class_id, Permission::Mint)?;

			let owner: T::AccountId = ensure_signed(origin.clone())?;

			Self::do_spawn(class_id, owner, metadata, attributes)?;

			Ok(())
		}

		/// ## Verification of the NFT
		/// ### Parameters:
		/// - `origin` must be signed by the owner of the frunique.
		/// - `class_id` must be a valid class of the asset class.
		/// - `instance_id` must be a valid instance of the asset class.
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn verify(
			origin: OriginFor<T>,
			class_id: CollectionId,
			instance_id: ItemId,
		) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			ensure!(Self::item_exists(&class_id, &instance_id), Error::<T>::FruniqueNotFound);

			let owner: T::AccountId = ensure_signed(origin.clone())?;

			<FruniqueVerified<T>>::insert(class_id, instance_id, true);

			Self::deposit_event(Event::FruniqueVerified(owner, class_id, instance_id));

			Ok(())
		}

		/// ## Invite a user to become a collaborator in a collection.
		/// ### Parameters:
		/// - `origin` must be signed by the owner of the frunique.
		/// - `class_id` must be a valid class of the asset class.
		/// - `invitee` must be a valid user.
		/// ### Considerations:
		/// This functions enables the owner of a collection to invite a user to become a collaborator.
		/// The user will be able to create NFTs in the collection.
		/// The user will be able to add attributes to the NFTs in the collection.
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn invite(
			origin: OriginFor<T>,
			class_id: CollectionId,
			invitee: T::AccountId,
		) -> DispatchResult {
			let owner: T::AccountId = ensure_signed(origin.clone())?;
			Self::insert_auth_in_frunique_collection(
				invitee.clone(),
				class_id,
				FruniqueRole::Collaborator,
			)?;

			Self::deposit_event(Event::InvitedToCollaborate(owner, invitee, class_id));
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
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn force_set_counter(
			origin: OriginFor<T>,
			class_id: T::CollectionId,
			instance_id: Option<T::ItemId>,
		) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin)?;

			if let Some(instance_id) = instance_id {
				ensure!(
					!Self::item_exists(&class_id, &instance_id),
					Error::<T>::FruniqueAlreadyExists
				);
				<NextFrunique<T>>::insert(class_id, instance_id);
			} else {
				ensure!(!Self::collection_exists(&class_id), Error::<T>::CollectionAlreadyExists);
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
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn force_destroy_collection(
			origin: OriginFor<T>,
			class_id: T::CollectionId,
			witness: pallet_uniques::DestroyWitness,
			maybe_check_owner: Option<T::AccountId>,
		) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin)?;

			ensure!(Self::collection_exists(&class_id), Error::<T>::CollectionNotFound);
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
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn kill_storage(origin: OriginFor<T>) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			<NextCollection<T>>::put(0);
			let _ = <NextFrunique<T>>::clear(1000, None);
			let _ = <FruniqueVerified<T>>::clear(1000, None);
			let _ = <FruniqueInfo<T>>::clear(1000, None);

			T::Rbac::remove_pallet_storage(Self::pallet_id())?;
			Ok(())
		}
	}
}
