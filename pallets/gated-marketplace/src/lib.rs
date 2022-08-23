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
	use frame_support::traits::Currency;
	//use sp_runtime::sp_std::vec::Vec;
	use crate::types::*;
	//use frame_support::traits::tokens::Balance;
	//use std::fmt::Debug;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_fruniques::Config + pallet_uniques::Config + pallet_timestamp::Config{
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type LocalCurrency: Currency<Self::AccountId>;
		//type Balance: Balance + MaybeSerializeDeserialize + Debug + MaxEncodedLen;

		type RemoveOrigin: EnsureOrigin<Self::Origin>;		
		#[pallet::constant]
		type MaxAuthsPerMarket: Get<u32>;
		#[pallet::constant]
		type MaxRolesPerAuth: Get<u32>;
		#[pallet::constant]
		type MaxApplicants: Get<u32>;
		#[pallet::constant]
		type LabelMaxLen: Get<u32>;
		#[pallet::constant]
		type MaxFeedbackLen: Get<u32>;
		#[pallet::constant]
		type NotesMaxLen: Get<u32>;
		#[pallet::constant]
		type NameMaxLen: Get<u32>;
		#[pallet::constant]
		type MaxFiles: Get<u32>;
		#[pallet::constant]
		type MaxApplicationsPerCustodian: Get<u32>;
		#[pallet::constant]	
		type MaxMarketsPerItem: Get<u32>;
		#[pallet::constant]	
		type MaxOffersPerMarket: Get<u32>;
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
		[u8; 32], // Key
		Marketplace<T>,  // Value
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn marketplaces_by_authority)]
	pub(super) type MarketplacesByAuthority<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, 
		T::AccountId, // K1: Authority 
		Blake2_128Concat, 
		[u8;32], // K2: marketplace_id 
		BoundedVec<MarketplaceAuthority, T::MaxRolesPerAuth >, // scales with MarketplaceAuthority cardinality
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn authorities_by_marketplace)]
	pub(super) type AuthoritiesByMarketplace<T: Config> = StorageDoubleMap<
		_, 
		Identity, 
		[u8;32], //K1: marketplace_id 
		Blake2_128Concat, 
		MarketplaceAuthority, //k2: authority
		BoundedVec<T::AccountId,T::MaxAuthsPerMarket>, 
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn applications)]
	pub(super) type Applications<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8;32], //K1: application_id
		Application<T>, 
		OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn applications_by_account)]
	pub(super) type ApplicationsByAccount<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, 
		T::AccountId, // K1: account_id
		Blake2_128Concat, 
		[u8;32], // k2: marketplace_id 
		[u8;32], //application_id
		OptionQuery
	>;


	#[pallet::storage]
	#[pallet::getter(fn applicants_by_marketplace)]
	pub(super) type ApplicantsByMarketplace<T: Config> = StorageDoubleMap<
		_, 
		Identity, 
		[u8;32], //K1: marketplace_id
		Blake2_128Concat, 
		ApplicationStatus, //K2: application_status
		BoundedVec<T::AccountId,T::MaxApplicants>, 
		ValueQuery
	>; 

	#[pallet::storage]
	#[pallet::getter(fn custodians)]
	pub(super) type Custodians<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, 
		T::AccountId, //custodians
		Blake2_128Concat, 
		[u8;32], //marketplace_id 
		BoundedVec<T::AccountId,T::MaxApplicationsPerCustodian>, //applicants 
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn offers_by_item)]
	pub(super) type OffersByItem<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, 
		T::CollectionId, //collection_id
		Blake2_128Concat, 
		T::ItemId, //item_id 
		BoundedVec<[u8;32], T::MaxOffersPerMarket>, // offer_id's
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn offers_by_account)]
	pub(super) type OffersByAccount<T: Config> = StorageMap<
		_, 
		Identity, 
		T::AccountId, // account_id
		BoundedVec<[u8;32], T::MaxOffersPerMarket>, // offer_id's
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn offers_by_marketplace)]
	pub(super) type OffersByMarketplace<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8; 32], // Marketplace_id
		BoundedVec<[u8;32], T::MaxOffersPerMarket>,  // offer_id's
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn offers_info)]
	pub(super) type OffersInfo<T: Config> = StorageMap<
		_, 
		Identity, 
		[u8; 32], // offer_id
		//StorageDoubleMap -> marketplace_id(?)
		OfferData<T>,  // offer data
		OptionQuery,
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
		/// Add a new authority to the selected marketplace [account, authority]
		AuthorityAdded(T::AccountId, MarketplaceAuthority),
		/// Remove the selected authority from the selected marketplace [account, authority]
		AuthorityRemoved(T::AccountId, MarketplaceAuthority),
		/// The label of the selected marketplace has been updated. [market_id]
		MarketplaceLabelUpdated([u8;32]),
		/// The selected marketplace has been removed. [market_id]
		MarketplaceRemoved([u8;32]),
		/// Offer stored. [collection_id, item_id]
		OfferStored(T::CollectionId, T::ItemId),
		/// Offer was accepted [offer_id, account]
		OfferWasAccepted([u8;32], T::AccountId),
		/// Offer was duplicated. [new_offer_id, new_marketplace_id]
		OfferDuplicated([u8;32], [u8;32]),
		/// Offer was removed. [offer_id], [marketplace_id]
		OfferRemoved([u8;32], [u8;32]), 
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Work In Progress
		NotYetImplemented,
		/// Error names should be descriptive.
		NoneValue,
		///Limit bounded vector exceeded
		LimitExceeded,
		/// The account supervises too many marketplaces
		ExceedMaxMarketsPerAuth,
		/// The account has too many roles in that marketplace 
		ExceedMaxRolesPerAuth,
		/// Too many applicants for this market! try again later
		ExceedMaxApplicants,
		/// This custodian has too many applications for this market, try with another one
		ExceedMaxApplicationsPerCustodian,
		/// Applicaion doesnt exist
		ApplicationNotFound,
		/// The user has not applicated to that market before
		ApplicantNotFound,
		/// The user cannot be custodian of its own application
		ApplicantCannotBeCustodian,
		/// A marketplace with the same data exists already
		MarketplaceAlreadyExists,
		/// The user has already applied to the marketplace (or an identical application exist)
		AlreadyApplied,
		/// The specified marketplace does not exist
		MarketplaceNotFound,
		/// You need to be an owner or an admin of the marketplace
		CannotEnroll,
		/// There cannot be more than one owner per marketplace
		OnlyOneOwnerIsAllowed,
		/// Cannot remove the owner of the marketplace
		CantRemoveOwner,
		/// Admin can not remove itself from the marketplace
		AdminCannotRemoveItself,
		/// User not found
		UserNotFound,
		/// Owner not found
		OwnerNotFound,
		// Rol not found for the selected user
		AuthorityNotFoundForUser,
		/// Admis cannot be deleted between them, only the owner can
		CannotDeleteAdmin,
		/// Application ID not found
		ApplicationIdNotFound,
		/// Application status is still pending, user cannot apply/reapply
		ApplicationStatusStillPending,
		/// The application has already been approved, application status is approved
		ApplicationHasAlreadyBeenApproved,
		/// Collection not found
		CollectionNotFound,
		/// User who calls the function is not the owner of the collection
		NotOwner,
		/// Offer already exists
		OfferAlreadyExists,
		/// Offer not found
		OfferNotFound,
		/// Offer is not available at the moment
		OfferIsNotAvailable,
		/// Owner cannnot buy its own offer
		CannotTakeOffer,
		/// User cannot remove the offer from the marketplace
		CannotRemoveOffer,
		/// Error related to the timestamp
		TimestampError,
		/// User does not have enough balance to buy the offer
		NotEnoughBalance,
		/// User cannot delete the offer because is closed
		CannotDeleteOffer,
		/// There was a problem storing the offer
		OfferStorageError,
		/// Price must be greater than zero
		PriceMustBeGreaterThanZero,
		/// User cannot create buy offers for their own items
		CannotCreateOffer,
		/// This items is not available for sale
		ItemNotForSale,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> 
	where 
		T: pallet_uniques::Config<CollectionId = u32, ItemId = u32>,
	{

		/// Create a new marketplace.
		/// 
		/// Creates a new marketplace with the given label
		/// .
		/// ### Parameters:
		/// - `origin`: The owner of the marketplace.
		/// - `admin`: The admin of the marketplace.
		/// - `label`: The name of the marketplace.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_marketplace(origin: OriginFor<T>, admin: T::AccountId,label: BoundedVec<u8,T::LabelMaxLen>) -> DispatchResult {
			let who = ensure_signed(origin)?; // origin will be market owner
			let m = Marketplace{
				label,
			};
			Self::do_create_marketplace(who, admin, m)
		}

		/// Apply to a marketplace.
		/// 
		/// Applies to the selected marketplace. 
		/// 
		/// ### Parameters:
		/// - `origin`: The applicant.
		/// - `marketplace_id`: The id of the marketplace where we want to apply.
		/// - `fields`: Confidential user documents, any files necessary for the application
		/// - `custodian_fields`: The custodian account and their documents.
		/// 
		/// ### Considerations:
		/// - You can add many documents, up to the maximum allowed (10).
		/// - The custodian account is optional. You can apply to a marketplace without a 
		/// custodian account.
		/// - All custodian fields are optional. 
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn apply(
			origin: OriginFor<T>, 
			marketplace_id: [u8;32],
			// Getting encoding errors from polkadotjs if an object vector have optional fields
			fields : BoundedVec<(BoundedVec<u8,ConstU32<100> >,BoundedVec<u8,ConstU32<100>> ), T::MaxFiles>,
			custodian_fields: Option<(T::AccountId, BoundedVec<BoundedVec<u8,ConstU32<100>>, T::MaxFiles> )> 
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let (custodian, fields) = 
				Self::set_up_application(fields,custodian_fields);
			

			let application = Application::<T>{
				status: ApplicationStatus::default(),
				fields,
				feedback: BoundedVec::<u8, T::MaxFeedbackLen>::default(),
			};

			Self::do_apply(who, custodian, marketplace_id, application)
		}

		/// Accept or reject a reapplyment.
		/// 
		/// Allows the applicant for a second chance to apply to the selected marketplace.
		/// 
		/// ### Parameters:
		/// - `origin`: The reapplicant.
		/// - `marketplace_id`: The id of the marketplace where we want to reapply.
		/// - `fields`: Confidential user documents, any files necessary for the reapplication
		/// - `custodian_fields`: The custodian account and their documents.
		/// 	
		/// ### Considerations:
		/// - Since this is a second chance, you can replace your previous documents, up to the maximum allowed (10).
		/// - The custodian account is optional. You can replace the previous custodian.
		/// - Since we know the application exists, we can check the current status of the application.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn reapply(
			origin: OriginFor<T>, 
			marketplace_id: [u8;32],
			// Getting encoding errors from polkadotjs if an object vector have optional fields
			fields : BoundedVec<(BoundedVec<u8,ConstU32<100> >,BoundedVec<u8,ConstU32<100>> ), T::MaxFiles>,
			custodian_fields: Option<(T::AccountId, BoundedVec<BoundedVec<u8,ConstU32<100>>, T::MaxFiles> )> 
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let (custodian, fields) = 
				Self::set_up_application(fields,custodian_fields);
			
			let application = Application::<T>{
				status: ApplicationStatus::default(),
				fields,
				feedback: BoundedVec::<u8, T::MaxFeedbackLen>::default(),
			};

			Self::is_application_in_rejected_status(who.clone(), marketplace_id)?;

			Self::do_apply(who, custodian, marketplace_id, application)
		}
		

		/// Accept or reject an application.
		/// 
		/// If the application is accepted, 
		/// the user will be added to the list of applicants. 
		/// If the application is rejected,
		/// the user will be moved to the list of rejected applicants.
		/// 
		/// ### Parameters:
		/// - `origin`:  The owner/admin of the marketplace.
		/// - `marketplace_id`: The id of the marketplace where we want to enroll users.
		/// - `account_or_application`: The account or application id to accept or reject.
		/// - `approved`:  Whether to accept or reject the account/application.
		/// 
		/// ### Considerations:
		/// - You can only accept or reject applications where you are the owner/admin of the marketplace.
		/// - Ensure that your extrinsic has selected the right option account/application
		/// because some fields changes. 
		/// - If you select `Account` you need to enter the account to be accepted. 
		/// - If you select `Application` you need to enter the `application_id` to be accepted. 
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn enroll(origin: OriginFor<T>, marketplace_id: [u8;32], account_or_application: AccountOrApplication<T>, approved: bool, feedback: BoundedVec<u8, T::MaxFeedbackLen>, ) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_enroll(who, marketplace_id, account_or_application, approved, feedback)
		}

		/// Add an Authority type 
		/// 
		/// This extrinsic adds an authority type for the selected account 
		/// from the selected marketplace.
		/// 
		/// ### Parameters:	
		/// - `origin`: The user who performs the action.
		/// - `account`: The account to be removed.
		/// - `authority_type`: The type of authority to be added.
		/// - `marketplace_id`: The id of the marketplace where we want to add the account.
		/// 
		/// ### Considerations:
		/// If the user has already applied to the marketplace for that particular 
		/// authority type, it will throw an error.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_authority(origin: OriginFor<T>, account: T::AccountId, authority_type: MarketplaceAuthority, marketplace_id: [u8;32]) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_authority(who, account, authority_type, marketplace_id)
		}

		/// Remove an Authority type
		/// 
		/// This extrinsic removes an authority type for the selected account from the selected marketplace.
		/// 
		/// ### Parameters:
		/// - `origin`: The user who performs the action.
		/// - `account`: The account to be removed.
		/// - `authority_type`: The type of authority to be removed.
		/// - `marketplace_id`: The id of the marketplace where we want to remove the account.
		/// 
		/// ### Considerations:
		/// - This extrinsic doesn't remove the account from the marketplace,
		/// it only removes the selected authority type for that account.
		/// If the user doesn't have the selected authority type, it will throw an error.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_authority(origin: OriginFor<T>, account: T::AccountId, authority_type: MarketplaceAuthority, marketplace_id: [u8;32]) -> DispatchResult {
			let who = ensure_signed(origin)?;
			//TOREVIEW: If we're allowing more than one role per user per marketplace, we should 
			// check what role we want to remove instead of removing the user completely from
			// selected marketplace. 
			Self::do_remove_authority(who, account, authority_type, marketplace_id)
		}

		/// Update marketplace's label.
		/// 
		/// This extrinsic updates the label of the selected marketplace.
		/// 
		/// ### Parameters:
		/// - `origin`: The user who performs the action.
		/// - `marketplace_id`: The id of the marketplace where we want to update the label.
		/// - `label`: The new label for the selected marketplace.
		/// 
		/// ### Considerations:
		/// - You can only update the label of the marketplace where you are the owner/admin of the marketplace.
		/// - The label must be less than or equal to `T::LabelMaxLen
		/// - If the selected marketplace doesn't exist, it will throw an error.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn update_label_marketplace(origin: OriginFor<T>, marketplace_id: [u8;32], new_label: BoundedVec<u8,T::LabelMaxLen>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_update_label_marketplace(who, marketplace_id, new_label)
		}

		/// Remove a particular marketplace.
		/// 
		/// This extrinsic removes the selected marketplace.
		/// It removes all the applications related with the marketplace.
		/// It removes all the authorities from the lists of the marketplace.
		/// 
		/// ### Parameters:
		/// - `origin`: The user who performs the action.
		/// - `marketplace_id`: The id of the marketplace to be removed.
		/// 
		/// ### Considerations:
		/// - You can only remove the marketplace where you are the owner/admin of the marketplace.
		/// - If the selected marketplace doesn't exist, it will throw an error.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_marketplace(origin: OriginFor<T>, marketplace_id: [u8;32]) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_remove_marketplace(who, marketplace_id)
		}
		
		/// Enlist a sell order.
		/// 
		/// This extrinsic creates a sell order in the selected marketplace.
		/// 
		/// ### Parameters:
		/// - `origin`: The user who performs the action.
		/// - `marketplace_id`: The id of the marketplace where we want to create the sell order.
		/// - `collection_id`: The id of the collection.
		/// - `item_id`: The id of the item inside the collection.
		/// - `price`: The price of the item.
		/// 
		/// ### Considerations:
		/// - You can only create a sell order in the marketplace if you are the owner of the item.
		/// - You can create only one sell order for each item per marketplace.
		/// - If the selected marketplace doesn't exist, it will throw an error.
		/// - If the selected collection doesn't exist, it will throw an error.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn enlist_sell_offer(origin: OriginFor<T>, marketplace_id: [u8;32], collection_id: T::CollectionId, item_id: T::ItemId, price: BalanceOf<T>,) -> DispatchResult {
			let who = ensure_signed(origin)?; 

			Self::do_enlist_sell_offer(who, marketplace_id, collection_id, item_id, price)
		}

		/// Accepts a sell order.
		/// 
		/// This extrinsic accepts a sell order in the selected marketplace.
		/// 
		/// ### Parameters:
		/// - `origin`: The user who performs the action.
		/// - `marketplace_id`: The id of the marketplace where we want to accept the sell order.
		/// - `collection_id`: The id of the collection.
		/// - `item_id`: The id of the item inside the collection.
		/// 
		/// ### Considerations:
		/// - You don't need to be the owner of the item to accept the sell order.
		/// - Once the sell order is accepted, the ownership of the item is transferred to the buyer.
		/// - If you don't have the enough balance to accept the sell order, it will throw an error.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn take_sell_offer(origin: OriginFor<T>, offer_id: [u8;32], marketplace_id: [u8;32], collection_id: T::CollectionId, item_id: T::ItemId,) -> DispatchResult {
			let who = ensure_signed(origin.clone())?; 

			Self::do_take_sell_offer(who, offer_id, marketplace_id, collection_id, item_id)
		}

		/// Allows a user to duplicate a sell order.
		/// 
		/// This extrinsic allows a user to duplicate a sell order in any marketplace.
		/// 
		/// ### Parameters:
		/// - `origin`: The user who performs the action.
		/// - `marketplace_id`: The id of the marketplace where we want to duplicate the sell order.
		/// - `collection_id`: The id of the collection.
		/// - `item_id`: The id of the item inside the collection.
		/// 
		/// ### Considerations:
		/// - You can only duplicate a sell order if you are the owner of the item.
		/// - The expiration date of the sell order is the same as the original sell order.
		/// - You can update the price of the sell order.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]	
		pub fn duplicate_offer(origin: OriginFor<T>, offer_id: [u8;32], marketplace_id: [u8;32], collection_id: T::CollectionId, item_id: T::ItemId, modified_price: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin.clone())?; 

			Self::do_duplicate_offer(who, offer_id, marketplace_id, collection_id, item_id, modified_price)
		}	


		/// Delete an offer.
		/// 
		/// This extrinsic deletes an offer in the selected marketplace.
		/// 
		/// ### Parameters:
		/// - `origin`: The user who performs the action.
		/// - `marketplace_id`: The id of the marketplace where we want to delete the offer.
		/// - `collection_id`: The id of the collection.
		/// - `item_id`: The id of the item inside the collection.
		/// 
		/// ### Considerations:
		/// - You can delete sell orders or buy orders.
		/// - You can only delete an offer if you are the creator of the offer.
		/// - Only open offers can be deleted.
		/// - If you need to delete multiple offers for the same item, you need to 
		///  delete them one by one.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_offer(origin: OriginFor<T>, offer_id: [u8;32], marketplace_id: [u8;32], collection_id: T::CollectionId, item_id: T::ItemId,) -> DispatchResult {
			//Currently, we can only remove one offer at a time.
			//TODO: Add support for removing multiple offers at a time.
			let who = ensure_signed(origin.clone())?; 

			Self::do_remove_offer(who, offer_id, marketplace_id, collection_id, item_id)
		}

		/// Enlist a buy order.
		/// 
		/// This extrinsic creates a buy order in the selected marketplace.
		/// 
		/// ### Parameters:
		/// - `origin`: The user who performs the action.
		/// - `marketplace_id`: The id of the marketplace where we want to create the buy order.
		/// - `collection_id`: The id of the collection.
		/// - `item_id`: The id of the item inside the collection.
		/// - `price`: The price of the item.
		/// 
		/// ### Considerations:
		/// - Any user can create a buy order in the marketplace.
		/// - An item can receive multiple buy orders at a time.
		/// - You need to have the enough balance to create the buy order.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]	
		pub fn enlist_buy_offer(origin: OriginFor<T>, marketplace_id: [u8;32], collection_id: T::CollectionId, item_id: T::ItemId, price: BalanceOf<T>,) -> DispatchResult {
			let who = ensure_signed(origin)?; 

			Self::do_enlist_buy_offer(who, marketplace_id, collection_id, item_id, price)
		}


		/// Accepts a buy order.
		/// 	
		/// This extrinsic accepts a buy order in the selected marketplace.
		/// 
		/// ### Parameters:
		/// - `origin`: The user who performs the action.
		/// - `marketplace_id`: The id of the marketplace where we accept the buy order.
		/// - `collection_id`: The id of the collection.
		/// - `item_id`: The id of the item inside the collection.
		/// 
		/// ### Considerations:
		/// - You need to be the owner of the item to accept a buy order.
		/// - Owner of the item can accept only one buy order at a time.
		/// - When an offer is accepted, all the other offers for this item are closed.
		/// - The buyer needs to have the enough balance to accept the buy order.
		/// - Once the buy order is accepted, the ownership of the item is transferred to the buyer.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn take_buy_offer(origin: OriginFor<T>, offer_id: [u8;32], marketplace_id: [u8;32], collection_id: T::CollectionId, item_id: T::ItemId,) -> DispatchResult {
			let who = ensure_signed(origin.clone())?; 

			Self::do_take_buy_offer(who, offer_id, marketplace_id, collection_id, item_id)
		}


		//TODO: Add CRUD operations for the offers

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
			<Marketplaces<T>>::remove_all(None);
			<MarketplacesByAuthority<T>>::remove_all(None);
			<AuthoritiesByMarketplace<T>>::remove_all(None);
			<Applications<T>>::remove_all(None);
			<ApplicationsByAccount<T>>::remove_all(None);
			<ApplicantsByMarketplace<T>>::remove_all(None);
			<Custodians<T>>::remove_all(None);
			Ok(())
		}


	}
}