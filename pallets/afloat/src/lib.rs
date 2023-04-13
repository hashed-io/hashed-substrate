#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// mod functions;
mod types;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::ValueQuery;
	use frame_support::pallet_prelude::*;
	use frame_support::traits::Currency;
	use frame_support::traits::UnixTime;
	use frame_system::pallet_prelude::*;
	use sp_runtime::Permill;
	use frame_support::sp_io::hashing::blake2_256;
	use pallet_gated_marketplace::functions;
	use pallet_gated_marketplace::types::*;


	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	use crate::types::*;
	// use pallet_gated_marketplace::types::Marketplace;
	use pallet_rbac::types::RoleBasedAccessControl;



	pub type BalanceOf<T> = <<T as pallet_uniques::Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_gated_marketplace::Config {
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
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Marketplece ID is not set
		MarketplaceIdNotSet,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// #[pallet::storage]
	// #[pallet::getter(fn marketplace_id)]
	// /// Keeps track of the number of collections in existence.
	// pub(super) type MarketplaceIds<T: Config> = StorageValue<
	// 	_,
	// 	MarketplaceIds, // Marketplace identifier
	// >;

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
	pub(super) type AfloatMarketPlaceId<T: Config> = StorageValue<_, [u8; 32]>;



	#[pallet::call]
	impl<T: Config> Pallet<T> {



		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
		pub fn initial_setup(origin: OriginFor<T>) -> DispatchResult {
			pallet_gated_marketplace::Pallet::<T>::do_initial_setup()?;	
			let buy_fee = 2;
			let sell_fee = 4;
			let label = b"Afloat".to_vec();
			let label_bounded: BoundedVec<u8, T::LabelMaxLen> = BoundedVec::try_from(label).expect("Label too long");
			let who = ensure_signed(origin)?; // origin will be market owner
			let m: Marketplace<T> = Marketplace {
				label: label_bounded,
				buy_fee: Permill::from_percent(buy_fee),
				sell_fee: Permill::from_percent(sell_fee),
				creator: who.clone(),
			};
			let marketplace_id = m.clone().using_encoded(blake2_256);
			AfloatMarketPlaceId::<T>::put(marketplace_id);
			pallet_gated_marketplace::Pallet::do_create_marketplace(who.clone(), who.clone(), m)?;
			Ok(())
		}

/* 		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
		pub fn kill_storage(origin: OriginFor<T>) -> DispatchResult {
			<Marketplace<T>>::kill();
			let _ = <UserInfo<T>>::clear(1000, None);
			Ok(())
		} */

		#[pallet::call_index(2)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
		pub fn set_marketplace_id(origin: OriginFor<T>, id: MarketplaceId) -> DispatchResult {
			// let marketplace: pallet_gated_marketplace::Pallet<T>::Marketplace<T> = Marketplace {
			// 	label: ShortString::try_from(b"afloat".to_vec()).unwrap(),
			// 	creator: origin.clone(),
			// };

			<Marketplace<T>>::put(id);
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().reads_writes(1,1))]
		pub fn sign_up(origin: OriginFor<T>, args: SignUpArgs) -> DispatchResult {
			let who = ensure_signed(origin)?;

			if let Some(marketplace_id) = Self::marketplace_id() {
				let _ =
					pallet_gated_marketplace::Pallet::<T>::self_enroll(who.clone(), marketplace_id);
			} else {
				ensure!(false, Error::<T>::MarketplaceIdNotSet);
			}

			match args {
				SignUpArgs::BuyerOrSeller { first_name, last_name, email, state } => {
					let user: User<T> = User {
						first_name,
						last_name,
						email,
						lang_key: ShortString::try_from(b"en".to_vec()).unwrap(),
						created_by: Some(who.clone()),
						created_date: Some(T::TimeProvider::now().as_secs()),
						last_modified_by: Some(who.clone()),
						last_modified_date: Some(T::TimeProvider::now().as_secs()),
						phone: None,
						credits_needed: 0,
						cpa_id: ShortString::try_from(b"0".to_vec()).unwrap(),
						tax_authority_id: state,
						lock_expiration_date: None,
					};
					<UserInfo<T>>::insert(who.clone(), user);
					Self::deposit_event(Event::NewUser(who.clone()));
					Self::deposit_event(Event::NewUser(who.clone()));
				},
				SignUpArgs::CPA { first_name, last_name, email, license_number, state } => {
					let user: User<T> = User {
						first_name,
						last_name,
						email,
						lang_key: ShortString::try_from(b"en".to_vec()).unwrap(),
						created_by: Some(who.clone()),
						created_date: Some(T::TimeProvider::now().as_secs()),
						last_modified_by: Some(who.clone()),
						last_modified_date: Some(T::TimeProvider::now().as_secs()),
						phone: None,
						credits_needed: 0,
						cpa_id: license_number,
						tax_authority_id: state,
						lock_expiration_date: None,
					};
					<UserInfo<T>>::insert(who.clone(), user);
					Self::deposit_event(Event::NewUser(who.clone()));
					Self::deposit_event(Event::NewUser(who.clone()));
				},
			}
			let marketplace_id = AfloatMarketPlaceId::<T>::get().unwrap();
			pallet_gated_marketplace::Pallet::<T>::self_enroll(who.clone(), marketplace_id)?;
			Ok(())
		}
	}
}
