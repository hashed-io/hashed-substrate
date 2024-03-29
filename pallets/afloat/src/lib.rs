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
  use frame_support::{
    pallet_prelude::*,
    sp_io::hashing::blake2_256,
    traits::{Currency, UnixTime},
  };
  use frame_system::{pallet_prelude::*, RawOrigin};
  use pallet_fruniques::types::{Attributes, CollectionDescription, FruniqueRole, ParentInfo};
  use pallet_gated_marketplace::types::*;
  use sp_runtime::Permill;
  const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

  use crate::types::*;
  use pallet_rbac::types::RoleBasedAccessControl;

  pub type BalanceOf<T> = <<T as pallet_uniques::Config>::Currency as Currency<
    <T as frame_system::Config>::AccountId,
  >>::Balance;

  /// Configure the pallet by specifying the parameters and types on which it depends.
  #[pallet::config]
  pub trait Config:
    frame_system::Config
    + pallet_gated_marketplace::Config
    + pallet_mapped_assets::Config
    + pallet_uniques::Config
  {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    type TimeProvider: UnixTime;
    type Rbac: RoleBasedAccessControl<Self::AccountId>;
    type RemoveOrigin: EnsureOrigin<Self::RuntimeOrigin>;
    type Currency: Currency<Self::AccountId>;
    type ItemId: Parameter + Member + Default;
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
    SellOrderCreated(T::AccountId),
    BuyOrderCreated(T::AccountId),
    SellOrderTaken(T::AccountId),
    BuyOrderTaken(T::AccountId),
    AfloatBalanceSet(T::AccountId, T::AccountId, T::Balance),
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
    // Failed to remove Fruniques role
    FailedToRemoveFruniquesRole,
    // User is not authorized to perform this action
    Unauthorized,
    // Pallet has not ben initialized yet
    NotInitialized,
    // Failed to remove afloat role
    FailedToRemoveAfloatRole,
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

  #[pallet::storage]
  #[pallet::getter(fn asset_id)]
  pub(super) type AfloatAssetId<T: Config> = StorageValue<
    _,
    <T as pallet_mapped_assets::Config>::AssetId, // Afloat's mapped collection asset_id
  >;

  #[pallet::storage]
  #[pallet::getter(fn afloat_offers)]
  pub(super) type AfloatOffers<T: Config> =
    StorageMap<_, Blake2_128Concat, StorageId, Offer<T>, OptionQuery>;

  #[pallet::storage]
  #[pallet::getter(fn afloat_transactions)]
  pub(super) type AfloatTransactions<T: Config> =
    StorageMap<_, Blake2_128Concat, StorageId, Transaction<T>, OptionQuery>;

  #[pallet::call]
  impl<T: Config> Pallet<T>
  where
    T: pallet_uniques::Config<CollectionId = CollectionId>,
    <T as pallet_uniques::Config>::ItemId: From<u32>,
  {
    #[pallet::call_index(0)]
    #[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
    pub fn initial_setup(
      origin: OriginFor<T>,
      creator: T::AccountId,
      admin: T::AccountId,
    ) -> DispatchResult {
      ensure_signed(origin.clone())?;
      let asset_id: T::AssetId = Default::default();
      AfloatAssetId::<T>::put(asset_id.clone());

      let metadata: CollectionDescription<T> =
        BoundedVec::try_from(b"Afloat".to_vec()).expect("Label too long");

      pallet_fruniques::Pallet::<T>::do_initial_setup()?;

      Self::create_afloat_collection(
        RawOrigin::Signed(creator.clone()).into(),
        metadata,
        admin.clone(),
      )?;

      pallet_gated_marketplace::Pallet::<T>::do_initial_setup()?;

      let label: BoundedVec<u8, T::LabelMaxLen> =
        BoundedVec::try_from(b"Afloat".to_vec()).expect("Label too long");
      let marketplace: Marketplace<T> = Marketplace {
        label,
        buy_fee: Permill::from_percent(2),
        sell_fee: Permill::from_percent(4),
        asset_id,
        creator: creator.clone(),
      };
      let marketplace_id = marketplace.clone().using_encoded(blake2_256);

      AfloatMarketPlaceId::<T>::put(marketplace_id);
      Self::add_to_afloat_collection(admin.clone(), FruniqueRole::Admin)?;
      pallet_gated_marketplace::Pallet::do_create_marketplace(
        RawOrigin::Signed(creator.clone()).into(),
        admin.clone(),
        marketplace,
      )?;

      Self::do_initial_setup(creator, admin)?;

      Ok(())
    }

    #[pallet::call_index(1)]
    #[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
    pub fn kill_storage(origin: OriginFor<T>) -> DispatchResult {
      let who = ensure_signed(origin.clone())?;
      // Self::do_delete_all_users()?;

      let _ = <UserInfo<T>>::clear(1000, None);
      let _ = <AfloatMarketPlaceId<T>>::kill();
      let _ = <AfloatCollectionId<T>>::kill();
      let _ = <AfloatOffers<T>>::clear(1000, None);
      let _ = <AfloatTransactions<T>>::clear(1000, None);

      <T as Config>::Rbac::remove_pallet_storage(Self::pallet_id())?;

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
      let who = ensure_signed(origin)?;

      ensure!(<UserInfo<T>>::contains_key(address.clone()), Error::<T>::UserNotFound);
      ensure!(
        who.clone() == address || Self::is_admin_or_owner(who.clone()),
        Error::<T>::Unauthorized
      );

      match args {
        UpdateUserArgs::Edit { cid, cid_creator } => {
          Self::do_edit_user(who, address, cid, cid_creator)?;
        },
        UpdateUserArgs::AdminEdit { cid, cid_creator, group } => {
          ensure!(Self::is_admin_or_owner(who.clone()), Error::<T>::Unauthorized);
          Self::do_admin_edit_user(who, address, cid, cid_creator, group)?;
        },
        UpdateUserArgs::Delete => {
          ensure!(Self::is_admin_or_owner(who.clone()), Error::<T>::Unauthorized);
          Self::do_delete_user(who, address)?;
        },
      }

      Ok(())
    }

    #[pallet::call_index(4)]
    #[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
    pub fn create_offer(origin: OriginFor<T>, args: CreateOfferArgs<T>) -> DispatchResult {
      let who = ensure_signed(origin)?;
      match args {
        CreateOfferArgs::Sell {
          tax_credit_amount,
          tax_credit_id,
          price_per_credit,
          expiration_date,
        } => {
          Self::do_create_sell_order(
            who,
            tax_credit_id,
            price_per_credit,
            tax_credit_amount,
            expiration_date,
          )?;
        },
        CreateOfferArgs::Buy {
          tax_credit_amount,
          tax_credit_id,
          price_per_credit,
          expiration_date,
        } => {
          Self::do_create_buy_order(
            who,
            tax_credit_id,
            price_per_credit,
            tax_credit_amount,
            expiration_date,
          )?;
        },
      }
      Ok(())
    }

    #[pallet::call_index(5)]
    #[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
    pub fn accept_offer(
      origin: OriginFor<T>,
      offer_id: [u8; 32],
      amount: Option<u64>,
    ) -> DispatchResult {
      ensure_signed(origin.clone())?;
      Ok(())
    }

    #[pallet::call_index(6)]
    #[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
    pub fn take_sell_order(origin: OriginFor<T>, offer_id: [u8; 32]) -> DispatchResult {
      ensure_signed(origin.clone())?;
      Self::do_take_sell_order(origin, offer_id)
    }

    #[pallet::call_index(7)]
    #[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
    pub fn take_buy_order(origin: OriginFor<T>, offer_id: [u8; 32]) -> DispatchResult {
      let who = ensure_signed(origin)?;
      Self::do_take_buy_order(who, offer_id)
    }

    #[pallet::call_index(8)]
    #[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(2,1))]
    pub fn create_tax_credit(
      origin: OriginFor<T>,
      metadata: CollectionDescription<T>,
      attributes: Option<Attributes<T>>,
      parent_info: Option<ParentInfo<T>>,
    ) -> DispatchResult {
      let who = ensure_signed(origin)?;
      Self::do_create_tax_credit(who, metadata, attributes, parent_info)
    }

    #[pallet::call_index(9)]
    #[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(2,1))]
    pub fn set_afloat_balance(
      origin: OriginFor<T>,
      beneficiary: T::AccountId,
      amount: T::Balance,
    ) -> DispatchResult {
      ensure_signed(origin.clone())?;

      // Only the owner can set afloat balance
      ensure!(Self::is_owner(ensure_signed(origin.clone())?), Error::<T>::Unauthorized);

      Self::do_set_afloat_balance(origin, beneficiary, amount)
    }
  }
}
