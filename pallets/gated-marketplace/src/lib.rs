#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod functions;
pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::traits::{Currency, Time};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::Scale;
	use sp_runtime::Permill;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	use crate::types::*;
	use pallet_rbac::types::RoleBasedAccessControl;

	pub type BalanceOf<T> = <<T as pallet_uniques::Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_fruniques::Config + pallet_mapped_assets::Config  {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Moment: Parameter
			+ Default
			+ Scale<Self::BlockNumber, Output = Self::Moment>
			+ Copy
			+ MaxEncodedLen
			+ scale_info::StaticTypeInfo
			+ Into<u64>;

		type Timestamp: Time<Moment = Self::Moment>;

		// type RemoveOrigin: EnsureOrigin<Self::RuntimeOrigin>;
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
		#[pallet::constant]
		type MaxBlockedUsersPerMarket: Get<u32>;

		type Rbac: RoleBasedAccessControl<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/*--- Onchain storage section ---*/

	#[pallet::storage]
	#[pallet::getter(fn marketplaces)]
	pub(super) type Marketplaces<T: Config> = StorageMap<
		_,
		Identity,
		MarketplaceId,
		Marketplace<T>, // Value
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn applications)]
	pub(super) type Applications<T: Config> =
		StorageMap<_, Identity, ApplicationId, Application<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn applications_by_account)]
	pub(super) type ApplicationsByAccount<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId, // K1: account_id
		Identity,
		MarketplaceId,
		ApplicationId,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn applicants_by_marketplace)]
	pub(super) type ApplicantsByMarketplace<T: Config> = StorageDoubleMap<
		_,
		Identity,
		MarketplaceId,
		Blake2_128Concat,
		ApplicationStatus, //K2: application_status
		BoundedVec<T::AccountId, T::MaxApplicants>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn custodians)]
	pub(super) type Custodians<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId, //custodians
		Identity,
		MarketplaceId,
		BoundedVec<T::AccountId, T::MaxApplicationsPerCustodian>, //applicants
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn offers_by_item)]
	pub(super) type OffersByItem<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::CollectionId, //collection_id
		Blake2_128Concat,
		T::ItemId,                                  //item_id
		BoundedVec<OfferId, T::MaxOffersPerMarket>, // offer_id's
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn offers_by_account)]
	pub(super) type OffersByAccount<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,                               // account_id
		BoundedVec<OfferId, T::MaxOffersPerMarket>, // offer_id's
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn offers_by_marketplace)]
	pub(super) type OffersByMarketplace<T: Config> = StorageMap<
		_,
		Identity,
		MarketplaceId,
		BoundedVec<OfferId, T::MaxOffersPerMarket>, // offer_id's
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn offers_info)]
	pub(super) type OffersInfo<T: Config> = StorageMap<
		_,
		Identity,
		OfferId,
		//StorageDoubleMap -> marketplace_id(?)
		OfferData<T>, // offer data
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn asking_for_redemption)]
	pub(super) type AskingForRedemption<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		MarketplaceId,
		Blake2_128Concat,
		RedemptionId,
		RedemptionData<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_blocked_accounts)]
	pub(super) type BlockedUsersByMarketplace<T: Config> = StorageMap<
		_,
		Identity,
		MarketplaceId,
		BoundedVec<T::AccountId, T::MaxBlockedUsersPerMarket>, // Blocked accounts
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Marketplaces stored. [owner, admin, market_id]
		MarketplaceStored(T::AccountId, T::AccountId, MarketplaceId),
		/// Application stored on the specified marketplace. [application_id, market_id]
		ApplicationStored(ApplicationId, MarketplaceId),
		/// An applicant was accepted or rejected on the marketplace. [AccountOrApplication, market_id, status]
		ApplicationProcessed(AccountOrApplication<T>, MarketplaceId, ApplicationStatus),
		/// Add a new authority to the selected marketplace [account, authority]
		AuthorityAdded(T::AccountId, MarketplaceRole),
		/// Remove the selected authority from the selected marketplace [account, authority]
		AuthorityRemoved(T::AccountId, MarketplaceRole),
		/// The label of the selected marketplace has been updated. [market_id]
		MarketplaceLabelUpdated(MarketplaceId),
		/// The selected marketplace has been removed. [market_id]
		MarketplaceRemoved(MarketplaceId),
		/// Offer stored. [collection_id, item_id, [offer_id]]
		OfferStored(T::CollectionId, T::ItemId, OfferId),
		/// Offer was accepted [offer_id, account]
		OfferWasAccepted(OfferId, T::AccountId),
		/// Offer was duplicated. [new_offer_id, new_marketplace_id]
		OfferDuplicated(OfferId, MarketplaceId),
		/// Offer was removed. [offer_id], [marketplace_id]
		OfferRemoved(OfferId, MarketplaceId),
		/// Initial pallet setup
		MarketplaceSetupCompleted,
		/// A new redemption was requested. [marketplace_id, redemption_id], owner
		RedemptionRequested(MarketplaceId, RedemptionId, T::AccountId),
		/// A redemption was accepted. [marketplace_id, redemption_id], redemption_specialist
		RedemptionAccepted(MarketplaceId, RedemptionId, T::AccountId),
		/// User was blocked. [marketplace_id, account]
		UserBlocked(MarketplaceId, T::AccountId),
		/// User was unblocked. [marketplace_id, account]
		UserUnblocked(MarketplaceId, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
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
		/// This offer has bigger percentage than the allowed
		ExceedMaxPercentage,
		/// This offer has smaller percentage than the allowed
		ExceedMinPercentage,
		/// Application does not exist
		ApplicationNotFound,
		/// The user has not applied to that market before
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
		NotOwnerOrAdmin,
		/// There was no change regarding the application status
		AlreadyEnrolled,
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
		/// Admins cannot be deleted between them, only the owner can
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
		/// Owner can not buy its own offer
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
		/// Could not access to item metadata
		ItemMetadataNotFound,
		/// Redemption request not found
		RedemptionRequestNotFound,
		/// Redemption request already in place
		RedemptionRequestAlreadyExists,
		/// The redemption in question is already redeemed
		RedemptionRequestAlreadyRedeemed,
		/// User is blocked
		UserIsBlocked,
		/// The number of blocked users has reached the limit
		ExceedMaxBlockedUsers,
		/// User is already a participant in the marketplace
		UserAlreadyParticipant,
		/// User is not blocked
		UserIsNotBlocked,
		/// User is already blocked
		UserAlreadyBlocked,
		/// The owner of the NFT is not in the marketplace
		OwnerNotInMarketplace,
		/// MappedAssetId not found
		AssetNotFound,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T: pallet_uniques::Config<CollectionId = u32, ItemId = u32>,
	{
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(10))]
		pub fn initial_setup(origin: OriginFor<T>) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			Self::do_initial_setup()?;
			Ok(())
		}

		/// Create a new marketplace.
		///
		/// Creates a new marketplace with the given label
		/// .
		/// ### Parameters:
		/// - `origin`: The owner of the marketplace.
		/// - `admin`: The admin of the marketplace.
		/// - `label`: The name of the marketplace.
		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn create_marketplace(
			origin: OriginFor<T>,
			admin: T::AccountId,
			label: BoundedVec<u8, T::LabelMaxLen>,
			buy_fee: u32,
			sell_fee: u32,
			asset_id: T::AssetId,
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?; // origin will be market owner
			let m = Marketplace {
				label,
				buy_fee: Permill::from_percent(buy_fee),
				sell_fee: Permill::from_percent(sell_fee),
				asset_id,
				creator: who.clone(),
			};
			Self::do_create_marketplace(origin, admin, m)
		}

		/// Block or Unblock a user from apllying to a marketplace.
		///
		/// Blocks or Unblocks a user from applying to a marketplace.
		///
		/// ### Parameters:
		/// - `origin`: The admin of the marketplace.
		/// - `marketplace_id`: The id of the marketplace to block/unblock the user.
		/// - `user`: The id of the user to block/unblock.`
		///
		/// ### Considerations:
		/// - Once a user is blocked, the user won't be able to join the marketplace until unblocked.
		#[pallet::call_index(2)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn block_user(
			origin: OriginFor<T>,
			marketplace_id: MarketplaceId,
			block_args: BlockUserArgs<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			match block_args {
				BlockUserArgs::BlockUser(user) => Self::do_block_user(who, marketplace_id, user),
				BlockUserArgs::UnblockUser(user) => {
					Self::do_unblock_user(who, marketplace_id, user)
				},
			}
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
		#[pallet::call_index(3)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn apply(
			origin: OriginFor<T>,
			marketplace_id: [u8; 32],
			// Getting encoding errors from polkadotjs if an object vector have optional fields
			fields: Fields<T>,
			custodian_fields: Option<CustodianFields<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let (custodian, fields) = Self::set_up_application(fields, custodian_fields);

			let application = Application::<T> {
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
		#[pallet::call_index(4)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn reapply(
			origin: OriginFor<T>,
			marketplace_id: [u8; 32],
			// Getting encoding errors from polkadotjs if an object vector have optional fields
			fields: Fields<T>,
			custodian_fields: Option<CustodianFields<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let (custodian, fields) = Self::set_up_application(fields, custodian_fields);

			let application = Application::<T> {
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
		#[pallet::call_index(5)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn enroll(
			origin: OriginFor<T>,
			marketplace_id: [u8; 32],
			account_or_application: AccountOrApplication<T>,
			approved: bool,
			feedback: BoundedVec<u8, T::MaxFeedbackLen>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_enroll(who, marketplace_id, account_or_application, approved, feedback)
		}

		/// Invite a user to a marketplace.
		///
		/// The admin of the marketplace can invite a user to the marketplace.
		/// ### Parameters:
		/// - `origin`: The admin of the marketplace.
		/// - `marketplace_id`: The id of the marketplace where we want to invite a user.
		/// - `account`: The account to be invited.
		///
		/// ### Considerations:
		/// - You can only invite users to a marketplace where you are the admin.
		#[pallet::call_index(6)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn invite(
			origin: OriginFor<T>,
			marketplace_id: [u8; 32],
			account: T::AccountId,
			fields: Fields<T>,
			custodian_fields: Option<CustodianFields<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_invite(who, marketplace_id, account, fields, custodian_fields)
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
		#[pallet::call_index(7)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn add_authority(
			origin: OriginFor<T>,
			account: T::AccountId,
			authority_type: MarketplaceRole,
			marketplace_id: [u8; 32],
		) -> DispatchResult {
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
		#[pallet::call_index(8)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn remove_authority(
			origin: OriginFor<T>,
			account: T::AccountId,
			authority_type: MarketplaceRole,
			marketplace_id: [u8; 32],
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// TODO: review If we're allowing more than one role per user per marketplace, we should
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
		#[pallet::call_index(9)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn update_label_marketplace(
			origin: OriginFor<T>,
			marketplace_id: [u8; 32],
			new_label: BoundedVec<u8, T::LabelMaxLen>,
		) -> DispatchResult {
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
		#[pallet::call_index(10)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn remove_marketplace(
			origin: OriginFor<T>,
			marketplace_id: [u8; 32],
		) -> DispatchResult {
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
		#[pallet::call_index(11)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn enlist_sell_offer(
			origin: OriginFor<T>,
			marketplace_id: [u8; 32],
			collection_id: T::CollectionId,
			item_id: T::ItemId,
			price: T::Balance,
			percentage: u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_enlist_sell_offer(
				who,
				marketplace_id,
				collection_id,
				item_id,
				price,
				percentage,
			)?;

			Ok(())
		}

		/// Accepts a sell order.
		///
		/// This extrinsic is called by the user who wants to buy the item.
		/// Accepts a sell order in the selected marketplace.
		///
		/// ### Parameters:
		/// - `origin`: The user who performs the action.
		/// - 'offer_id`: The id of the sell order to be accepted.
		/// - `marketplace_id`: The id of the marketplace where we want to accept the sell order.
		///
		/// ### Considerations:
		/// - You don't need to be the owner of the item to accept the sell order.
		/// - Once the sell order is accepted, the ownership of the item is transferred to the buyer.
		/// - If you don't have the enough balance to accept the sell order, it will throw an error.
		#[pallet::call_index(12)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn take_sell_offer(origin: OriginFor<T>, offer_id: [u8; 32]) -> DispatchResult {
			ensure_signed(origin.clone())?;

			Self::do_take_sell_offer(origin, offer_id)
		}

		/// Delete an offer.
		///
		/// This extrinsic deletes an offer in the selected marketplace.
		///
		/// ### Parameters:
		/// - `origin`: The user who performs the action.
		/// - `offer_id`: The id of the offer to be deleted.
		///
		/// ### Considerations:
		/// - You can delete sell orders or buy orders.
		/// - You can only delete an offer if you are the creator of the offer.
		/// - Only open offers can be deleted.
		/// - If you need to delete multiple offers for the same item, you need to
		///  delete them one by one.
		#[pallet::call_index(13)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn remove_offer(origin: OriginFor<T>, offer_id: [u8; 32]) -> DispatchResult {
			//Currently, we can only remove one offer at a time.
			//TODO: Add support for removing multiple offers at a time.
			let who = ensure_signed(origin.clone())?;

			Self::do_remove_offer(who, offer_id)
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
		#[pallet::call_index(14)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn enlist_buy_offer(
			origin: OriginFor<T>,
			marketplace_id: [u8; 32],
			collection_id: T::CollectionId,
			item_id: T::ItemId,
			price: T::Balance,
			percentage: u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_enlist_buy_offer(
				who,
				marketplace_id,
				collection_id,
				item_id,
				price,
				percentage,
			)?;

			Ok(())
		}

		/// Accepts a buy order.
		///
		/// This extrinsic is called by the owner of the item who accepts the buy offer created by a market participant.
		/// Accepts a buy order in the selected marketplace.
		///
		/// ### Parameters:
		/// - `origin`: The user who performs the action.
		/// - `offer_id`: The id of the buy order to be accepted.
		/// - `marketplace_id`: The id of the marketplace where we accept the buy order.
		///
		/// ### Considerations:
		/// - You need to be the owner of the item to accept a buy order.
		/// - Owner of the item can accept only one buy order at a time.
		/// - When an offer is accepted, all the other offers for this item are closed.
		/// - The buyer needs to have the enough balance to accept the buy order.
		/// - Once the buy order is accepted, the ownership of the item is transferred to the buyer.
		#[pallet::call_index(15)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn take_buy_offer(origin: OriginFor<T>, offer_id: [u8; 32]) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;

			Self::do_take_buy_offer(who, offer_id)
		}

		/// Redeem an item.
		/// This extrinsic is called by the owner of the item who wants to redeem the item.
		/// The owner of the item can ask for redemption or accept redemption.
		/// ### Parameters:
		/// - `origin`: The user who performs the action.
		/// - `marketplace_id`: The id of the marketplace where we want to redeem the item.
		/// - `redeem`: The type of redemption.

		#[pallet::call_index(16)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn redeem(
			origin: OriginFor<T>,
			marketplace: MarketplaceId,
			redeem: RedeemArgs<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			match redeem {
				RedeemArgs::AskForRedemption { collection_id, item_id } => {
					return Self::do_ask_for_redeem(who, marketplace, collection_id, item_id);
				},
				RedeemArgs::AcceptRedemption(redemption_id) => {
					return Self::do_accept_redeem(who, marketplace, redemption_id);
				},
			}
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
		#[pallet::call_index(17)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn kill_storage(origin: OriginFor<T>) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			let _ = <Marketplaces<T>>::clear(1000, None);
			let _ = <Applications<T>>::clear(1000, None);
			let _ = <ApplicationsByAccount<T>>::clear(1000, None);
			let _ = <ApplicantsByMarketplace<T>>::clear(1000, None);
			let _ = <Custodians<T>>::clear(1000, None);
			let _ = <OffersByItem<T>>::clear(1000, None);
			let _ = <OffersByAccount<T>>::clear(1000, None);
			let _ = <OffersByMarketplace<T>>::clear(1000, None);
			let _ = <OffersInfo<T>>::clear(1000, None);
			let _ = <AskingForRedemption<T>>::clear(1000, None);
			<T as Config>::Rbac::remove_pallet_storage(Self::pallet_id())?;
			Ok(())
		}
	}
}
