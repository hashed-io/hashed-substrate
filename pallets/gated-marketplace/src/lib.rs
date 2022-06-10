#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod functions;
mod types;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::{*, OptionQuery}, transactional};
	use frame_system::pallet_prelude::*;
	use crate::types::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type RemoveOrigin: EnsureOrigin<Self::Origin>;
		
		#[pallet::constant]
		type MaxMarketsPerAuth: Get<u32>;
		#[pallet::constant]
		type MaxApplicants: Get<u32>;
		#[pallet::constant]
		type LabelMaxLen: Get<u32>;
		#[pallet::constant]
		type NotesMaxLen: Get<u32>;
		#[pallet::constant]
		type NameMaxLen: Get<u32>;
		#[pallet::constant]
		type MaxFiles: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/*--- Onchain storage section ---*/

	#[pallet::storage]
	#[pallet::getter(fn marketplaces)]
	pub(super) type Marketplaces<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8; 32], 
		Marketplace<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn marketplaces_by_authority)]
	pub(super) type MarketplacesByAuthority<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, 
		T::AccountId, 
		Blake2_128Concat, 
		[u8;32], //marketplace_id 
		BoundedVec<MarketplaceAuthority, T::MaxMarketsPerAuth>, 
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn authorities_by_marketplace)]
	pub(super) type AuthoritiesByMarketplace<T: Config> = StorageDoubleMap<
		_, 
		Identity, 
		[u8;32], // marketplace_id 
		Blake2_128Concat, 
		MarketplaceAuthority, 
		BoundedVec<T::AccountId,T::MaxMarketsPerAuth>, 
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn applications)]
	pub(super) type Applications<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], 
		Application<T>, 
		OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn applications_by_account)]
	pub(super) type ApplicationsByAccount<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, 
		T::AccountId, 
		Blake2_128Concat, 
		[u8;32], //marketplace_id 
		[u8;32], //application_id
		OptionQuery
	>;


	#[pallet::storage]
	#[pallet::getter(fn applicants_by_marketplace)]
	pub(super) type ApplicantsByMarketplace<T: Config> = StorageDoubleMap<
		_, 
		Identity, 
		[u8;32], 
		Blake2_128Concat, 
		ApplicationStatus, 
		BoundedVec<T::AccountId,T::MaxApplicants>, 
		ValueQuery
	>;



	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Marketplaces stored. [owner, admin, market_id]
		MarketplaceStored(T::AccountId, T::AccountId, [u8;32]),
		/// Application stored on the specified marketplace. [application_id, market_id]
		ApplicationStored([u8;32], [u8;32]),
		/// An applicant was accepted or rejected on the marketplace. [AccountOrApplication, market_id, status]
		ApplicationProcessed(AccountOrApplication<T>,[u8;32], ApplicationStatus),
		
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Work In Progress
		NotYetImplemented,
		/// Error names should be descriptive.
		NoneValue,
		/// The account supervises too many marketplaces
		ExceedMaxMarketsPerAuth,
		/// Too many applicants for this market! try again later
		ExceedMaxApplicants,
		/// Applicaion doesnt exist
		ApplicationNotFound,
		/// The user has not applicated to that market before
		ApplicantNotFound,
		/// A marketplace with the same data exists already
		MarketplaceAlreadyExists,
		/// The user has already applied to the marketplace
		AlreadyApplied,
		/// The specified marketplace does not exist
		MarketplaceNotFound,
		/// You need to be an owner or an admin of the marketplace
		CannotEnroll,

	}
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_marketplace(origin: OriginFor<T>, admin: T::AccountId,label: BoundedVec<u8,T::LabelMaxLen>) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin will be market owner
			let m = Marketplace{
				label,
			};
			Self::do_create_marketplace(who, admin, m)
		}
		
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn apply(
			origin: OriginFor<T>, 
			marketplace_id: [u8;32], 
			notes: BoundedVec<u8,T::NotesMaxLen>, 
			files : BoundedVec<ApplicationFile<T::NameMaxLen>, T::MaxFiles>, 
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let application = Application::<T>{
				status: ApplicationStatus::default(),
				notes,
				files,
			};
			Self::do_apply(who, marketplace_id, application)
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn enroll(origin: OriginFor<T>, marketplace_id: [u8;32], account_or_application: AccountOrApplication<T>, approved: bool ) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_enroll(who, marketplace_id, account_or_application, approved)
		}

		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn kill_storage(
			origin: OriginFor<T>,
		) -> DispatchResult{
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			<Marketplaces<T>>::remove_all(None);
			<MarketplacesByAuthority<T>>::remove_all(None);
			<AuthoritiesByMarketplace<T>>::remove_all(None);
			<Applications<T>>::remove_all(None);
			<ApplicationsByAccount<T>>::remove_all(None);
			<ApplicantsByMarketplace<T>>::remove_all(None);
			Ok(())
		}


	}
}