#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod functions;
pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::sp_io::hashing::blake2_256;
	use frame_support::traits::Currency;
	use frame_support::traits::UnixTime;
	use frame_system::pallet_prelude::*;
	use frame_system::RawOrigin;
	use pallet_gated_marketplace::types::*;
	use sp_runtime::Permill;
	use sp_runtime::traits::StaticLookup;
	use pallet_fruniques::types::CollectionDescription;
	use pallet_fruniques::types::FruniqueRole;
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	use crate::types::*;
	use pallet_rbac::types::RoleBasedAccessControl;

	pub type BalanceOf<T> = <<T as pallet_uniques::Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_gated_marketplace::Config + pallet_mapped_assets::Config + pallet_uniques::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type TimeProvider: UnixTime;
		type Rbac: RoleBasedAccessControl<Self::AccountId>;
		type RemoveOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
		NewUser(T::AccountId),
		UserEdited(T::AccountId),
		UserDeleted(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Marketplace not initialized
		MarketplaceNotInitialized,
		/// User not found
		UserNotFound,
		/// User already exists
		UserAlreadyExists,
		/// Failed to edit user account
		FailedToEditUserAccount,
		// Failed to create fruniques collection
		FailedToCreateFruniquesCollection,
	}

	#[pallet::storage]
	#[pallet::getter(fn user_info)]
	/// Keeps track of the number of fruniques in existence for a collection.
	pub(super) type UserInfo<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		User<T>, // User<T> is a struct that contains all the user info
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn marketplace_id)]
	pub(super) type AfloatMarketPlaceId<T: Config> = StorageValue<
		_,
		MarketplaceId, // Afloat's marketplace id
	>;

	#[pallet::storage]
	#[pallet::getter(fn collection_id)]
	pub(super) type AfloatCollectionId<T: Config> = StorageValue<
		_,
		<T as pallet_uniques::Config>::CollectionId, // Afloat's frunique collection id
	>;

	// SBP-M2 review: Missing extrinsic documentation and code is not well formatted
	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
	T: pallet_uniques::Config<CollectionId = CollectionId>
	{
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
		pub fn initial_setup(
			origin: OriginFor<T>,
			creator: T::AccountId,
			admin: T::AccountId,
		) -> DispatchResult

	{
			ensure_signed(origin.clone())?;
			let asset_id: T::AssetId = Default::default();
			let min_balance: T::Balance = T::Balance::from(1u32);
			// SBP-M2 review: Returning error is a good approach instead of panic.
			// Suggestion: A generic error enum for conversions can be used throughout the code
			let metadata: CollectionDescription<T> = BoundedVec::try_from(b"Afloat".to_vec()).expect("Label too long");

			pallet_mapped_assets::Pallet::<T>::create(
				origin.clone(),
				asset_id,
				T::Lookup::unlookup(creator.clone()),
				min_balance,
			)?;

			pallet_fruniques::Pallet::<T>::do_initial_setup()?;

			Self::create_afloat_collection(RawOrigin::Signed(creator.clone()).into(), metadata, admin.clone())?;

			pallet_gated_marketplace::Pallet::<T>::do_initial_setup()?;

			Self::do_initial_setup(creator.clone(), admin.clone())?;

			let label: BoundedVec<u8, T::LabelMaxLen> =
				BoundedVec::try_from(b"Afloat".to_vec()).expect("Label too long");
			let marketplace: Marketplace<T> = Marketplace {
				label,
				buy_fee: Permill::from_percent(2),
				sell_fee: Permill::from_percent(4),
				creator: creator.clone(),
			};
			let marketplace_id = marketplace.clone().using_encoded(blake2_256);

			AfloatMarketPlaceId::<T>::put(marketplace_id);
			Self::add_to_afloat_collection(admin.clone(),FruniqueRole::Admin)?;
			pallet_gated_marketplace::Pallet::do_create_marketplace(creator, admin, marketplace)?;
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
		pub fn kill_storage(origin: OriginFor<T>) -> DispatchResult {
			ensure_signed(origin.clone())?;
			<AfloatMarketPlaceId<T>>::kill();
			// SBP-M2 review: Remove let _, use ? operator instead
			let _ = <UserInfo<T>>::clear(1000, None);
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
		pub fn sign_up(origin: OriginFor<T>, args: SignUpArgs) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_create_user(who.clone(), who, args)
		}

		#[pallet::call_index(3)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
		pub fn update_user_info(
			origin: OriginFor<T>,
			address: T::AccountId,
			args: UpdateUserArgs,
		) -> DispatchResult {
			// SBP-M2 review: Pleaser resolve this
			// TODO: Check if the user is editing himself or is an admin
			let who = ensure_signed(origin)?;

			match args {
				UpdateUserArgs::Edit { first_name, last_name, email, lang_key, phone, credits_needed, cpa_id, state } => {
					Self::do_edit_user(who, address, first_name, last_name, email, lang_key, phone, credits_needed, cpa_id, state)?;
				}
				UpdateUserArgs::Delete => {
					Self::do_delete_user(who, address)?;
				}
			}

			Ok(())
		}
	}
}
