use super::*;
use crate::types::*;
use frame_support::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use pallet_rbac::types::*;
use scale_info::prelude::vec; // vec![] macro
use sp_runtime::sp_std::vec::Vec; // vec primitive

use frame_support::traits::Currency;
use frame_support::traits::ExistenceRequirement::KeepAlive;
use frame_support::traits::Time;
use sp_runtime::Permill;

impl<T: Config> Pallet<T> {
	pub fn do_initial_setup() -> DispatchResult {
		let pallet_id = Self::pallet_id();
		let super_roles = vec![MarketplaceRole::Owner.to_vec(), MarketplaceRole::Admin.to_vec()];
		let super_role_ids =
			<T as pallet::Config>::Rbac::create_and_set_roles(pallet_id.clone(), super_roles)?;
		for super_role in super_role_ids {
			<T as pallet::Config>::Rbac::create_and_set_permissions(
				pallet_id.clone(),
				super_role,
				Permission::admin_permissions(),
			)?;
		}
		// participant role and permissions
		let participant_role_id = <T as pallet::Config>::Rbac::create_and_set_roles(
			pallet_id.clone(),
			[MarketplaceRole::Participant.to_vec()].to_vec(),
		)?;
		<T as pallet::Config>::Rbac::create_and_set_permissions(
			pallet_id.clone(),
			participant_role_id[0],
			Permission::participant_permissions(),
		)?;
		// appraiser role and permissions
		let _appraiser_role_id = <T as pallet::Config>::Rbac::create_and_set_roles(
			pallet_id.clone(),
			[MarketplaceRole::Appraiser.to_vec()].to_vec(),
		)?;
		// redemption specialist role and permissions
		let _redemption_role_id = <T as pallet::Config>::Rbac::create_and_set_roles(
			pallet_id,
			[MarketplaceRole::RedemptionSpecialist.to_vec()].to_vec(),
		)?;

		Self::deposit_event(Event::MarketplaceSetupCompleted);
		Ok(())
	}

	pub fn do_create_marketplace(
		owner: T::AccountId,
		admin: T::AccountId,
		marketplace: Marketplace<T>,
	) -> DispatchResult {
		// Gen market id
		let marketplace_id = marketplace.using_encoded(blake2_256);
		// ensure the generated id is unique
		ensure!(
			!<Marketplaces<T>>::contains_key(marketplace_id),
			Error::<T>::MarketplaceAlreadyExists
		);
		//Insert on marketplaces and marketplaces by auth
		<T as pallet::Config>::Rbac::create_scope(Self::pallet_id(), marketplace_id)?;
		Self::insert_in_auth_market_lists(owner.clone(), MarketplaceRole::Owner, marketplace_id)?;
		Self::insert_in_auth_market_lists(admin.clone(), MarketplaceRole::Admin, marketplace_id)?;
		<Marketplaces<T>>::insert(marketplace_id, marketplace);
		Self::deposit_event(Event::MarketplaceStored(owner, admin, marketplace_id));
		Ok(())
	}

	pub fn do_apply(
		applicant: T::AccountId,
		custodian: Option<T::AccountId>,
		marketplace_id: [u8; 32],
		application: Application<T>,
	) -> DispatchResult {
		// marketplace exists?
		ensure!(<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceNotFound);
		// The user only can apply once by marketplace
		ensure!(
			!<ApplicationsByAccount<T>>::contains_key(applicant.clone(), marketplace_id),
			Error::<T>::AlreadyApplied
		);
		ensure!(
			!Self::is_user_blocked(applicant.clone(), marketplace_id),
			Error::<T>::UserIsBlocked
		);
		// Generate application Id
		let app_id =
			(marketplace_id, applicant.clone(), application.clone()).using_encoded(blake2_256);
		// Ensure another identical application doesnt exists
		ensure!(!<Applications<T>>::contains_key(app_id), Error::<T>::AlreadyApplied);

		if let Some(c) = custodian {
			// Ensure applicant and custodian arent the same
			ensure!(applicant.ne(&c), Error::<T>::ApplicantCannotBeCustodian);
			Self::insert_custodian(c, marketplace_id, applicant.clone())?;
		}

		Self::insert_in_applicants_lists(
			applicant.clone(),
			ApplicationStatus::default(),
			marketplace_id,
		)?;
		<ApplicationsByAccount<T>>::insert(applicant, marketplace_id, app_id);
		<Applications<T>>::insert(app_id, application);

		Self::deposit_event(Event::ApplicationStored(app_id, marketplace_id));
		Ok(())
	}

	pub fn do_invite(
		authority: T::AccountId,
		marketplace_id: [u8; 32],
		new_user: T::AccountId,
		fields: Fields<T>,
		custodian_fields: Option<CustodianFields<T>>,
	) -> DispatchResult {
		ensure!(<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceNotFound);
		// The user only can apply once by marketplace
		ensure!(
			!<ApplicationsByAccount<T>>::contains_key(new_user.clone(), marketplace_id),
			Error::<T>::AlreadyApplied
		);
		ensure!(
			!Self::is_user_blocked(new_user.clone(), marketplace_id),
			Error::<T>::UserIsBlocked
		);
		// ensure the origin is owner or admin
		Self::is_authorized(authority.clone(), &marketplace_id, Permission::Enroll)?;

		let (custodian, fields) = Self::set_up_application(fields, custodian_fields);

		let application = Application::<T> {
			status: ApplicationStatus::default(),
			fields,
			feedback: BoundedVec::<u8, T::MaxFeedbackLen>::default(),
		};

		Self::do_apply(new_user.clone(), custodian, marketplace_id, application)?;

		Self::do_enroll(
			authority,
			marketplace_id,
			AccountOrApplication::Account(new_user),
			true,
			BoundedVec::<u8, T::MaxFeedbackLen>::try_from(
				b"User enrolled by the marketplace admin".to_vec(),
			)
			.unwrap(),
		)?;

		Ok(())
	}

	pub fn do_enroll(
		authority: T::AccountId,
		marketplace_id: [u8; 32],
		account_or_application: AccountOrApplication<T>,
		approved: bool,
		feedback: BoundedVec<u8, T::MaxFeedbackLen>,
	) -> DispatchResult {
		// ensure the origin is owner or admin
		Self::is_authorized(authority, &marketplace_id, Permission::Enroll)?;
		let next_status = match approved {
			true => ApplicationStatus::Approved,
			false => ApplicationStatus::Rejected,
		};
		let applicant = match account_or_application.clone() {
			AccountOrApplication::Account(acc) => acc,
			AccountOrApplication::Application(application_id) => <ApplicationsByAccount<T>>::iter()
				.find_map(|(acc, m_id, app_id)| {
					if m_id == marketplace_id && app_id == application_id {
						return Some(acc);
					}
					None
				})
				.ok_or(Error::<T>::ApplicationNotFound)?,
		};
		// ensure the account is not blocked
		ensure!(!Self::is_user_blocked(applicant.clone(), marketplace_id), Error::<T>::UserIsBlocked);
		Self::change_applicant_status(applicant, marketplace_id, next_status, feedback)?;

		Self::deposit_event(Event::ApplicationProcessed(
			account_or_application,
			marketplace_id,
			next_status,
		));
		Ok(())
	}

	pub fn do_authority(
		authority: T::AccountId,
		account: T::AccountId,
		authority_type: MarketplaceRole,
		marketplace_id: [u8; 32],
	) -> DispatchResult {
		//ensure the origin is owner or admin
		//TODO: implement copy trait for MarketplaceAuthority & T::AccountId
		//Self::can_enroll(authority, marketplace_id)?;
		Self::is_authorized(authority, &marketplace_id, Permission::AddAuth)?;
		//ensure the account is not already an authority
		// handled by <T as pallet::Config>::Rbac::assign_role_to_user
		//ensure!(!Self::does_exist_authority(account.clone(), marketplace_id, authority_type), Error::<T>::AlreadyApplied);
		
		// ensure the account is not blocked
		ensure!(
			!Self::is_user_blocked(account.clone(), marketplace_id),
			Error::<T>::UserIsBlocked
		);
		match authority_type {
			MarketplaceRole::Owner => {
				ensure!(!Self::owner_exist(marketplace_id), Error::<T>::OnlyOneOwnerIsAllowed);
				Self::insert_in_auth_market_lists(account.clone(), authority_type, marketplace_id)?;
			},
			_ => {
				Self::insert_in_auth_market_lists(account.clone(), authority_type, marketplace_id)?;
			},
		}

		Self::deposit_event(Event::AuthorityAdded(account, authority_type));
		Ok(())
	}

	pub fn do_remove_authority(
		authority: T::AccountId,
		account: T::AccountId,
		authority_type: MarketplaceRole,
		marketplace_id: [u8; 32],
	) -> DispatchResult {
		//ensure the origin is owner or admin
		//Self::can_enroll(authority.clone(), marketplace_id)?;
		Self::is_authorized(authority.clone(), &marketplace_id, Permission::RemoveAuth)?;
		//ensure the account has the selected authority before to try to remove
		// <T as pallet::Config>::Rbac handles the if role doesnt hasnt been asigned to the user
		//ensure!(Self::does_exist_authority(account.clone(), marketplace_id, authority_type), Error::<T>::AuthorityNotFoundForUser);

		match authority_type {
			MarketplaceRole::Owner => {
				ensure!(Self::owner_exist(marketplace_id), Error::<T>::OwnerNotFound);
				return Err(Error::<T>::CantRemoveOwner.into());
			},
			MarketplaceRole::Admin => {
				// Admins can not delete themselves
				ensure!(authority != account, Error::<T>::AdminCannotRemoveItself);

				// Admis cannot be deleted between them, only the owner can
				ensure!(!Self::is_admin(authority, marketplace_id), Error::<T>::CannotDeleteAdmin);

				Self::remove_from_market_lists(account.clone(), authority_type, marketplace_id)?;
			},
			_ => {
				Self::remove_from_market_lists(account.clone(), authority_type, marketplace_id)?;
			},
		}

		Self::deposit_event(Event::AuthorityRemoved(account, authority_type));
		Ok(())
	}

	pub fn do_update_label_marketplace(
		authority: T::AccountId,
		marketplace_id: [u8; 32],
		new_label: BoundedVec<u8, T::LabelMaxLen>,
	) -> DispatchResult {
		//ensure the marketplace exists
		ensure!(<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceNotFound);
		//ensure the origin is owner or admin
		//Self::can_enroll(authority, marketplace_id)?;
		Self::is_authorized(authority, &marketplace_id, Permission::UpdateLabel)?;
		//update marketplace
		Self::update_label(marketplace_id, new_label)?;
		Self::deposit_event(Event::MarketplaceLabelUpdated(marketplace_id));
		Ok(())
	}

	pub fn do_remove_marketplace(
		authority: T::AccountId,
		marketplace_id: [u8; 32],
	) -> DispatchResult {
		//ensure the marketplace exists
		ensure!(<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceNotFound);
		//ensure the origin is owner or admin
		//Self::can_enroll(authority, marketplace_id)?;
		Self::is_authorized(authority, &marketplace_id, Permission::RemoveMarketplace)?;
		//remove marketplace
		Self::remove_selected_marketplace(marketplace_id)?;
		Self::deposit_event(Event::MarketplaceRemoved(marketplace_id));
		Ok(())
	}

	pub fn do_enlist_sell_offer(
		authority: T::AccountId,
		marketplace_id: [u8; 32],
		collection_id: T::CollectionId,
		item_id: T::ItemId,
		price: BalanceOf<T>,
		percentage: u32,
	) -> DispatchResult {
		//This function is only called by the owner of the marketplace
		//ensure the marketplace exists
		ensure!(<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceNotFound);
		Self::is_authorized(authority.clone(), &marketplace_id, Permission::EnlistSellOffer)?;
		//ensure the collection exists
		if let Some(a) = pallet_uniques::Pallet::<T>::owner(collection_id, item_id) {
			ensure!(a == authority, Error::<T>::NotOwner);
		} else {
			return Err(Error::<T>::CollectionNotFound.into());
		}

		//ensure the price is valid
		Self::is_the_offer_valid(price, Permill::from_percent(percentage))?;

		//Add timestamp to the offer
		let creation_date =
			Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

		//create an offer_id
		let offer_id = (marketplace_id, authority.clone(), collection_id, creation_date)
			.using_encoded(blake2_256);

		//create offer structure

		let marketplace =
			<Marketplaces<T>>::get(marketplace_id).ok_or(Error::<T>::MarketplaceNotFound)?;

		let offer_data = OfferData::<T> {
			marketplace_id,
			collection_id,
			item_id,
			creator: authority.clone(),
			price,
			fee: price * Permill::deconstruct(marketplace.fee).into() / 1_000_000u32.into(),
			percentage: Permill::from_percent(percentage),
			creation_date: creation_date,
			status: OfferStatus::Open,
			offer_type: OfferType::SellOrder,
			buyer: None,
		};

		//ensure there is no a previous sell offer for this item
		Self::can_this_item_receive_sell_orders(collection_id, item_id, marketplace_id)?;

		//insert in OffersByItem
		<OffersByItem<T>>::try_mutate(collection_id, item_id, |offers| offers.try_push(offer_id))
			.map_err(|_| Error::<T>::OfferStorageError)?;

		//insert in OffersByAccount
		<OffersByAccount<T>>::try_mutate(authority, |offers| offers.try_push(offer_id))
			.map_err(|_| Error::<T>::OfferStorageError)?;

		//insert in OffersInfo
		// ensure the offer_id doesn't exist
		ensure!(!<OffersInfo<T>>::contains_key(offer_id), Error::<T>::OfferAlreadyExists);
		<OffersInfo<T>>::insert(offer_id, offer_data);

		//Insert in OffersByMarketplace
		<OffersByMarketplace<T>>::try_mutate(marketplace_id, |offers| offers.try_push(offer_id))
			.map_err(|_| Error::<T>::OfferStorageError)?;

		pallet_fruniques::Pallet::<T>::do_freeze(&collection_id, item_id)?;

		Self::deposit_event(Event::OfferStored(collection_id, item_id, offer_id));
		Ok(())
	}

	pub fn do_enlist_buy_offer(
		authority: T::AccountId,
		marketplace_id: [u8; 32],
		collection_id: T::CollectionId,
		item_id: T::ItemId,
		price: BalanceOf<T>,
		percentage: u32,
	) -> DispatchResult {
		//ensure the item is for sale, if not, return error
		Self::can_this_item_receive_buy_orders(collection_id, item_id, marketplace_id)?;

		//ensure the marketplace exists
		ensure!(<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceNotFound);
		Self::is_authorized(authority.clone(), &marketplace_id, Permission::EnlistBuyOffer)?;

		//ensure the collection exists
		//For this case user doesn't need to be the owner of the collection
		//but the owner of the item cannot create a buy offer for their own collection
		if let Some(a) = pallet_uniques::Pallet::<T>::owner(collection_id, item_id) {
			ensure!(a != authority, Error::<T>::CannotCreateOffer);
		} else {
			return Err(Error::<T>::CollectionNotFound.into());
		}

		//ensure user has enough balance to create the offer
		let total_user_balance = T::Currency::total_balance(&authority);
		ensure!(total_user_balance >= price, Error::<T>::NotEnoughBalance);

		//ensure the price is valid
		Self::is_the_offer_valid(price, Permill::from_percent(percentage))?;

		//Add timestamp to the offer
		let creation_date =
			Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

		//create an offer_id
		let offer_id = (marketplace_id, authority.clone(), collection_id, creation_date)
			.using_encoded(blake2_256);

		//create offer structure
		let marketplace =
			<Marketplaces<T>>::get(marketplace_id).ok_or(Error::<T>::MarketplaceNotFound)?;
		let offer_data = OfferData::<T> {
			marketplace_id,
			collection_id,
			item_id,
			creator: authority.clone(),
			price,
			fee: price * Permill::deconstruct(marketplace.fee).into() / 1_000_000u32.into(),
			percentage: Permill::from_percent(percentage),
			creation_date: creation_date,
			status: OfferStatus::Open,
			offer_type: OfferType::BuyOrder,
			buyer: None,
		};

		//insert in OffersByItem
		//An item can receive multiple buy offers
		<OffersByItem<T>>::try_mutate(collection_id, item_id, |offers| offers.try_push(offer_id))
			.map_err(|_| Error::<T>::OfferStorageError)?;

		//insert in OffersByAccount
		<OffersByAccount<T>>::try_mutate(authority, |offers| offers.try_push(offer_id))
			.map_err(|_| Error::<T>::OfferStorageError)?;

		//insert in OffersInfo
		// ensure the offer_id doesn't exist
		ensure!(!<OffersInfo<T>>::contains_key(offer_id), Error::<T>::OfferAlreadyExists);
		<OffersInfo<T>>::insert(offer_id, offer_data);

		//Insert in OffersByMarketplace
		<OffersByMarketplace<T>>::try_mutate(marketplace_id, |offers| offers.try_push(offer_id))
			.map_err(|_| Error::<T>::OfferStorageError)?;

		Self::deposit_event(Event::OfferStored(collection_id, item_id, offer_id));
		Ok(())
	}

	pub fn do_take_sell_offer(buyer: T::AccountId, offer_id: [u8; 32]) -> DispatchResult
	where
		<T as pallet_uniques::Config>::ItemId: From<u32>,
	{
		//This extrinsic is called by the user who wants to buy the item
		//get offer data
		let offer_data = <OffersInfo<T>>::get(offer_id).ok_or(Error::<T>::OfferNotFound)?;

		Self::is_authorized(buyer.clone(), &offer_data.marketplace_id, Permission::TakeSellOffer)?;

		//ensure the collection & owner exists
		let owner_item =
			pallet_uniques::Pallet::<T>::owner(offer_data.collection_id, offer_data.item_id)
				.ok_or(Error::<T>::OwnerNotFound)?;

		//ensure owner is not the same as the buyer
		ensure!(owner_item != buyer, Error::<T>::CannotTakeOffer);

		//ensure the offer_id exists in OffersByItem
		Self::does_exist_offer_id_for_this_item(
			offer_data.collection_id,
			offer_data.item_id,
			offer_id,
		)?;

		//ensure the offer is open and available
		ensure!(offer_data.status == OfferStatus::Open, Error::<T>::OfferIsNotAvailable);
		//TODO: Use free_balance instead of total_balance
		//Get the buyer's balance
		let total_amount_buyer = T::Currency::total_balance(&buyer);
		//ensure the buyer has enough balance to buy the item
		ensure!(total_amount_buyer > offer_data.price, Error::<T>::NotEnoughBalance);

		let marketplace =
			<Marketplaces<T>>::get(offer_data.marketplace_id).ok_or(Error::<T>::OfferNotFound)?;
		let owners_cut: BalanceOf<T> = offer_data.price - offer_data.fee;
		//Transfer the balance
		T::Currency::transfer(&buyer, &owner_item, owners_cut, KeepAlive)?;
		T::Currency::transfer(&buyer, &marketplace.creator, offer_data.fee, KeepAlive)?;

		pallet_fruniques::Pallet::<T>::do_thaw(&offer_data.collection_id, offer_data.item_id)?;
		if offer_data.percentage == Permill::from_percent(100) {
			//Use uniques transfer function to transfer the item to the buyer
			pallet_uniques::Pallet::<T>::do_transfer(
				offer_data.collection_id,
				offer_data.item_id,
				buyer.clone(),
				|_, _| Ok(()),
			)?;
		} else {
			let parent_info = pallet_fruniques::types::ParentInfo {
				collection_id: offer_data.collection_id,
				parent_id: offer_data.item_id,
				parent_weight: offer_data.percentage,
				is_hierarchical: true,
			};
			let metadata = pallet_fruniques::Pallet::<T>::get_nft_metadata(
				offer_data.collection_id,
				offer_data.item_id,
			);

			pallet_fruniques::Pallet::<T>::do_spawn(
				offer_data.collection_id,
				buyer.clone(),
				metadata,
				None,
				Some(parent_info),
			)?;
		}

		//update offer status from all marketplaces
		Self::update_offers_status(
			buyer.clone(),
			offer_data.collection_id,
			offer_data.item_id,
			offer_data.marketplace_id,
		)?;

		//remove all the offers associated with the item
		Self::delete_all_offers_for_this_item(offer_data.collection_id, offer_data.item_id)?;

		Self::deposit_event(Event::OfferWasAccepted(offer_id, buyer));
		Ok(())
	}

	pub fn do_take_buy_offer(authority: T::AccountId, offer_id: [u8; 32]) -> DispatchResult
	where
		<T as pallet_uniques::Config>::ItemId: From<u32>,
	{
		//This extrinsic is called by the owner of the item who accepts the buy offer created by a marketparticipant
		//get offer data
		let offer_data = <OffersInfo<T>>::get(offer_id).ok_or(Error::<T>::OfferNotFound)?;

		Self::is_authorized(
			authority.clone(),
			&offer_data.marketplace_id,
			Permission::TakeBuyOffer,
		)?;

		//ensure the collection & owner exists
		let owner_item =
			pallet_uniques::Pallet::<T>::owner(offer_data.collection_id, offer_data.item_id)
				.ok_or(Error::<T>::OwnerNotFound)?;

		//ensure only owner of the item can call the extrinsic
		ensure!(owner_item == authority, Error::<T>::NotOwner);

		//ensure owner is not the same as the buy_offer_creator
		ensure!(owner_item != offer_data.creator, Error::<T>::CannotTakeOffer);

		//ensure the offer_id exists in OffersByItem
		Self::does_exist_offer_id_for_this_item(
			offer_data.collection_id,
			offer_data.item_id,
			offer_id,
		)?;

		//ensure the offer is open and available
		ensure!(offer_data.status == OfferStatus::Open, Error::<T>::OfferIsNotAvailable);

		//TODO: Use free_balance instead of total_balance
		//Get the buyer's balance
		let total_amount_buyer = T::Currency::total_balance(&offer_data.creator);
		//ensure the buy_offer_creator has enough balance to buy the item
		ensure!(total_amount_buyer > offer_data.price, Error::<T>::NotEnoughBalance);

		let marketplace =
			<Marketplaces<T>>::get(offer_data.marketplace_id).ok_or(Error::<T>::OfferNotFound)?;
		let owners_cut: BalanceOf<T> = offer_data.price - offer_data.fee;
		//Transfer the balance to the owner of the item
		T::Currency::transfer(&offer_data.creator, &owner_item, owners_cut, KeepAlive)?;
		T::Currency::transfer(
			&offer_data.creator,
			&marketplace.creator,
			offer_data.fee,
			KeepAlive,
		)?;
		pallet_fruniques::Pallet::<T>::do_thaw(&offer_data.collection_id, offer_data.item_id)?;

		if offer_data.percentage == Permill::from_percent(100) {
			//Use uniques transfer function to transfer the item to the buyer
			pallet_uniques::Pallet::<T>::do_transfer(
				offer_data.collection_id,
				offer_data.item_id,
				offer_data.creator.clone(),
				|_, _| Ok(()),
			)?;
		} else {
			let parent_info = pallet_fruniques::types::ParentInfo {
				collection_id: offer_data.collection_id,
				parent_id: offer_data.item_id,
				parent_weight: offer_data.percentage,
				is_hierarchical: true,
			};
			let metadata = pallet_fruniques::Pallet::<T>::get_nft_metadata(
				offer_data.collection_id,
				offer_data.item_id,
			);

			pallet_fruniques::Pallet::<T>::do_spawn(
				offer_data.collection_id,
				offer_data.creator.clone(),
				metadata,
				None,
				Some(parent_info),
			)?;
		}

		//update offer status from all marketplaces
		Self::update_offers_status(
			offer_data.creator.clone(),
			offer_data.collection_id,
			offer_data.item_id,
			offer_data.marketplace_id,
		)?;

		//remove all the offers associated with the item
		Self::delete_all_offers_for_this_item(offer_data.collection_id, offer_data.item_id)?;

		Self::deposit_event(Event::OfferWasAccepted(offer_id, offer_data.creator));
		Ok(())
	}

	pub fn do_remove_offer(authority: T::AccountId, offer_id: [u8; 32]) -> DispatchResult {
		//ensure the offer_id exists
		ensure!(<OffersInfo<T>>::contains_key(offer_id), Error::<T>::OfferNotFound);

		//get offer data
		let offer_data = <OffersInfo<T>>::get(offer_id).ok_or(Error::<T>::OfferNotFound)?;
		Self::is_authorized(
			authority.clone(),
			&offer_data.marketplace_id,
			Permission::RemoveOffer,
		)?;

		//ensure the offer status is Open
		ensure!(offer_data.status == OfferStatus::Open, Error::<T>::CannotDeleteOffer);

		// ensure the authority is the creator of the offer
		ensure!(offer_data.creator == authority, Error::<T>::CannotRemoveOffer);

		//ensure the offer_id exists in OffersByItem
		Self::does_exist_offer_id_for_this_item(
			offer_data.collection_id,
			offer_data.item_id,
			offer_id,
		)?;

		if offer_data.offer_type == OfferType::SellOrder {
			pallet_fruniques::Pallet::<T>::do_thaw(&offer_data.collection_id, offer_data.item_id)?;
		}

		//remove the offer from OfferInfo
		<OffersInfo<T>>::remove(offer_id);

		//remove the offer from OffersByMarketplace
		<OffersByMarketplace<T>>::try_mutate(offer_data.marketplace_id, |offers| {
			let offer_index =
				offers.iter().position(|x| *x == offer_id).ok_or(Error::<T>::OfferNotFound)?;
			offers.remove(offer_index);
			Ok(())
		})
		.map_err(|_: Error<T>| Error::<T>::OfferNotFound)?;

		//remove the offer from OffersByAccount
		<OffersByAccount<T>>::try_mutate(authority, |offers| {
			let offer_index =
				offers.iter().position(|x| *x == offer_id).ok_or(Error::<T>::OfferNotFound)?;
			offers.remove(offer_index);
			Ok(())
		})
		.map_err(|_: Error<T>| Error::<T>::OfferNotFound)?;

		//remove the offer from OffersByItem
		<OffersByItem<T>>::try_mutate(offer_data.collection_id, offer_data.item_id, |offers| {
			let offer_index =
				offers.iter().position(|x| *x == offer_id).ok_or(Error::<T>::OfferNotFound)?;
			offers.remove(offer_index);
			Ok(())
		})
		.map_err(|_: Error<T>| Error::<T>::OfferNotFound)?;

		Self::deposit_event(Event::OfferRemoved(offer_id, offer_data.marketplace_id));

		Ok(())
	}

	/*---- Helper functions ----*/

	pub fn set_up_application(
		fields: Fields<T>,
		custodian_fields: Option<CustodianFields<T>>,
	) -> (Option<T::AccountId>, BoundedVec<ApplicationField, T::MaxFiles>) {
		let mut f: Vec<ApplicationField> = fields
			.iter()
			.map(|tuple| ApplicationField {
				display_name: tuple.0.clone(),
				cid: tuple.1.clone(),
				custodian_cid: None,
			})
			.collect();
		let custodian = match custodian_fields {
			Some(c_fields) => {
				for (i, field) in f.iter_mut().enumerate() {
					field.custodian_cid = Some(c_fields.1[i].clone());
				}

				Some(c_fields.0)
			},
			_ => None,
		};
		(custodian, BoundedVec::<ApplicationField, T::MaxFiles>::try_from(f).unwrap_or_default())
	}

	fn insert_in_auth_market_lists(
		authority: T::AccountId,
		role: MarketplaceRole,
		marketplace_id: [u8; 32],
	) -> DispatchResult {
		<T as pallet::Config>::Rbac::assign_role_to_user(
			authority,
			Self::pallet_id(),
			&marketplace_id,
			role.id(),
		)?;

		Ok(())
	}

	fn insert_in_applicants_lists(
		applicant: T::AccountId,
		status: ApplicationStatus,
		marketplace_id: [u8; 32],
	) -> DispatchResult {
		<ApplicantsByMarketplace<T>>::try_mutate(marketplace_id, status, |applicants| {
			applicants.try_push(applicant)
		})
		.map_err(|_| Error::<T>::ExceedMaxApplicants)?;

		Ok(())
	}

	fn insert_custodian(
		custodian: T::AccountId,
		marketplace_id: [u8; 32],
		applicant: T::AccountId,
	) -> DispatchResult {
		<Custodians<T>>::try_mutate(custodian, marketplace_id, |applications| {
			applications.try_push(applicant)
		})
		.map_err(|_| Error::<T>::ExceedMaxApplicationsPerCustodian)?;

		Ok(())
	}

	fn remove_from_applicants_lists(
		applicant: T::AccountId,
		status: ApplicationStatus,
		marketplace_id: [u8; 32],
	) -> DispatchResult {
		<ApplicantsByMarketplace<T>>::try_mutate::<_, _, _, DispatchError, _>(
			marketplace_id,
			status,
			|applicants| {
				let applicant_index = applicants
					.iter()
					.position(|a| *a == applicant.clone())
					.ok_or(Error::<T>::ApplicantNotFound)?;
				applicants.remove(applicant_index);

				Ok(())
			},
		)
	}

	fn remove_from_market_lists(
		account: T::AccountId,
		author_type: MarketplaceRole,
		marketplace_id: [u8; 32],
	) -> DispatchResult {
		<T as pallet::Config>::Rbac::remove_role_from_user(
			account,
			Self::pallet_id(),
			&marketplace_id,
			author_type.id(),
		)?;
		Ok(())
	}

	fn change_applicant_status(
		applicant: T::AccountId,
		marketplace_id: [u8; 32],
		next_status: ApplicationStatus,
		feedback: BoundedVec<u8, T::MaxFeedbackLen>,
	) -> DispatchResult {
		let mut prev_status = ApplicationStatus::default();
		let app_id = <ApplicationsByAccount<T>>::get(applicant.clone(), marketplace_id)
			.ok_or(Error::<T>::ApplicationNotFound)?;
		<Applications<T>>::try_mutate::<_, _, DispatchError, _>(app_id, |application| {
			application.as_ref().ok_or(Error::<T>::ApplicationNotFound)?;
			if let Some(a) = application {
				prev_status.clone_from(&a.status);
				a.feedback = feedback;
				a.status.clone_from(&next_status)
			}
			Ok(())
		})?;
		ensure!(prev_status != next_status, Error::<T>::AlreadyEnrolled);
		//remove from previous state list
		Self::remove_from_applicants_lists(applicant.clone(), prev_status, marketplace_id)?;

		//insert in current state list
		Self::insert_in_applicants_lists(applicant.clone(), next_status, marketplace_id)?;

		if prev_status == ApplicationStatus::Approved {
			<T as pallet::Config>::Rbac::remove_role_from_user(
				applicant.clone(),
				Self::pallet_id(),
				&marketplace_id,
				MarketplaceRole::Participant.id(),
			)?;
		}
		if next_status == ApplicationStatus::Approved {
			<T as pallet::Config>::Rbac::assign_role_to_user(
				applicant,
				Self::pallet_id(),
				&marketplace_id,
				MarketplaceRole::Participant.id(),
			)?
		}

		Ok(())
	}

	pub fn do_block_user(
		authority: T::AccountId,
		marketplace_id: [u8; 32],
		user: T::AccountId,
	) -> DispatchResult {
		// ensure the marketplace exists
		ensure!(<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceNotFound);
		// ensure the origin is owner or admin
		Self::is_authorized(authority.clone(), &marketplace_id, Permission::EnlistBlockedUser)?;
		//ensure the user is not already a participant of the marketplace
		ensure!(
			Self::is_authorized(
				user.clone(), 
				&marketplace_id,
				 Permission::EnlistBuyOffer
				).is_err(), 
				Error::<T>::UserAlreadyParticipant
			);
		// check if the user is already blocked
		if Self::try_unblock_user(user.clone(), marketplace_id).is_err() {
			// if the user is not blocked, block it
			<BlockedUsersByMarketplace<T>>::try_mutate(marketplace_id, |blocked_list| blocked_list.try_push(user.clone()))
				.map_err(|_| Error::<T>::ExceedMaxBlockedUsers)?;
			Self::deposit_event(Event::UserBlocked(marketplace_id, user.clone()));
			if let Ok(application_id) = <ApplicationsByAccount<T>>::try_get(user.clone(), marketplace_id){
					// remove application information
					if let Ok(application) = <Applications<T>>::try_get(application_id){
						Self::remove_from_applicants_lists(user.clone(), application.status, marketplace_id)?;
					}
					<Applications<T>>::remove(application_id);
					<ApplicationsByAccount<T>>::remove(user.clone(), marketplace_id);
				}
			return Ok(());
		}
		Self::deposit_event(Event::UserUnblocked(marketplace_id, user));
		Ok(())
	}

	fn try_unblock_user(
		user: T::AccountId,
		marketplace_id: [u8; 32],
	) -> DispatchResult {
		<BlockedUsersByMarketplace<T>>::try_mutate::<_, _, DispatchError, _>(
			marketplace_id,
			|blocked_users| {
				let user_index = blocked_users
					.iter()
					.position(|a| *a == user.clone())
					.ok_or(Error::<T>::UserNotFound)?;
				blocked_users.remove(user_index);
				Ok(())
			},
		)
	}

	fn is_user_blocked(
		user: T::AccountId,
		marketplace_id: [u8; 32],
	) -> bool {
		if <BlockedUsersByMarketplace<T>>::try_mutate::<_, _, DispatchError, _>(
			marketplace_id,
			|blocked_users| {
				let check = blocked_users
					.iter()
					.position(|a| *a == user.clone())
					.is_some();
				if check {
					Ok(())
				} else {
					Err(Error::<T>::UserNotFound.into())
				}
				
			},).is_ok() {
				return true;
			}
		false
	}

	fn is_authorized(
		authority: T::AccountId,
		marketplace_id: &[u8; 32],
		permission: Permission,
	) -> DispatchResult {
		<T as pallet::Config>::Rbac::is_authorized(
			authority,
			Self::pallet_id(),
			marketplace_id,
			&permission.id(),
		)
	}

	///Lets us know if the selected user is an admin.
	/// It returns true if the user is an admin, false otherwise.
	fn is_admin(account: T::AccountId, marketplace_id: [u8; 32]) -> bool {
		<T as pallet::Config>::Rbac::has_role(
			account,
			Self::pallet_id(),
			&marketplace_id,
			[MarketplaceRole::Admin.id()].to_vec(),
		)
		.is_ok()
	}

	/// Let us know if the selected account has the selected authority type.
	/// It returns true if the account has the authority type, false otherwise
	// fn  does_exist_authority(account: T::AccountId, marketplace_id: [u8;32], authority_type: MarketplaceRole) -> bool{
	//     let roles = match <MarketplacesByAuthority<T>>::try_get(account, marketplace_id){
	//         Ok(roles) => roles,
	//         Err(_) => return false,
	//     };

	//     roles.iter().any(|authority| authority == &authority_type)
	// }

	/// Let us know if there's an owner for the selected marketplace.
	/// It returns true if there's an owner, false otherwise
	fn owner_exist(marketplace_id: [u8; 32]) -> bool {
		// let owners =  match <AuthoritiesByMarketplace<T>>::try_get( marketplace_id, MarketplaceAuthority::Owner){
		//     Ok(owners) => owners,
		//     Err(_) => return false,
		// };

		//owners.len() == 1
		<T as pallet::Config>::Rbac::get_role_users_len(
			Self::pallet_id(),
			&marketplace_id,
			&MarketplaceRole::Owner.id(),
		) == 1
	}

	/// Let us update the marketplace's label.
	/// It returns ok if the update was successful, error otherwise.
	fn update_label(
		marketplace_id: [u8; 32],
		new_label: BoundedVec<u8, T::LabelMaxLen>,
	) -> DispatchResult {
		<Marketplaces<T>>::try_mutate(marketplace_id, |marketplace| {
			let market = marketplace.as_mut().ok_or(Error::<T>::MarketplaceNotFound)?;
			market.label = new_label;
			Ok(())
		})
	}

	/// Let us delete the selected marketplace
	/// and remove all of its associated authorities from all the storage sources.
	/// If returns ok if the deletion was successful, error otherwise.
	/// Errors only could happen if the storage sources are corrupted.
	fn remove_selected_marketplace(marketplace_id: [u8; 32]) -> DispatchResult {
		//TODO: evaluate use iter_key_prefix ->instead iter()
		//Before to remove the marketplace, we need to remove all its associated authorities
		// as well as the applicants/applications.

		//First we need to get the list of all the authorities for the marketplace.
		let mut applications = Vec::new();

		// remove from Applications lists
		for ele in <ApplicationsByAccount<T>>::iter() {
			if ele.1 == marketplace_id {
				applications.push(ele.2);
			}
		}

		for application in applications {
			<Applications<T>>::remove(application);
		}

		// remove from ApplicationsByAccount list
		<ApplicationsByAccount<T>>::iter().for_each(|(_k1, _k2, _k3)| {
			<ApplicationsByAccount<T>>::remove(_k1, marketplace_id);
		});

		// remove from ApplicantsByMarketplace list
		let _ = <ApplicantsByMarketplace<T>>::clear_prefix(marketplace_id, 1000, None);

		// remove from Custodians list
		<Custodians<T>>::iter().for_each(|(_k1, _k2, _k3)| {
			<Custodians<T>>::remove(_k1, marketplace_id);
		});

		// remove from Marketplaces list
		<Marketplaces<T>>::remove(marketplace_id);

		<T as pallet::Config>::Rbac::remove_scope(Self::pallet_id(), marketplace_id)?;

		Ok(())
	}

	/// Let us check the curent status of the selected application.
	/// If the status is rejected, we can safely remove its data from the storage sources
	/// so the user can apply again.
	/// It doesn't affect any other storage source/workflow.
	pub fn is_application_in_rejected_status(
		account: T::AccountId,
		marketplace_id: [u8; 32],
	) -> DispatchResult {
		//check if user is blocked
		ensure!(
			!Self::is_user_blocked(account.clone(), marketplace_id),
			Error::<T>::UserIsBlocked
		);
		let application_id = <ApplicationsByAccount<T>>::try_get(account.clone(), marketplace_id)
			.map_err(|_| Error::<T>::ApplicationIdNotFound)?;

		let application = <Applications<T>>::try_get(application_id)
			.map_err(|_| Error::<T>::ApplicationNotFound)?;

		match application.status {
			ApplicationStatus::Pending => {
				return Err(Error::<T>::ApplicationStatusStillPending.into())
			},
			ApplicationStatus::Approved => {
				return Err(Error::<T>::ApplicationHasAlreadyBeenApproved.into())
			},
			ApplicationStatus::Rejected => {
				//If status is Rejected, we need to delete the previous application from all the storage sources.
				<Applications<T>>::remove(application_id);
				<ApplicationsByAccount<T>>::remove(account, marketplace_id);
				<ApplicantsByMarketplace<T>>::remove(marketplace_id, ApplicationStatus::Rejected);
			},
		}
		Ok(())
	}

	fn get_timestamp_in_milliseconds() -> Option<u64> {
		let timestamp: u64 = T::Timestamp::now().into();

		Some(timestamp)
	}

	fn _is_offer_status(offer_id: [u8; 32], offer_status: OfferStatus) -> bool {
		//we already know that the offer exists, so we don't need to check it here.
		if let Some(offer) = <OffersInfo<T>>::get(offer_id) {
			offer.status == offer_status
		} else {
			false
		}
	}

	fn does_exist_offer_id_for_this_item(
		collection_id: T::CollectionId,
		item_id: T::ItemId,
		offer_id: [u8; 32],
	) -> DispatchResult {
		let offers = <OffersByItem<T>>::try_get(collection_id, item_id)
			.map_err(|_| Error::<T>::OfferNotFound)?;
		//find the offer_id in the vector of offers_ids
		offers.iter().find(|&x| *x == offer_id).ok_or(Error::<T>::OfferNotFound)?;
		Ok(())
	}

	//sell orders here...

	fn update_offers_status(
		buyer: T::AccountId,
		collection_id: T::CollectionId,
		item_id: T::ItemId,
		marketplace_id: [u8; 32],
	) -> DispatchResult {
		let offer_ids = <OffersByItem<T>>::try_get(collection_id, item_id)
			.map_err(|_| Error::<T>::OfferNotFound)?;

		for offer_id in offer_ids {
			<OffersInfo<T>>::try_mutate::<_, _, DispatchError, _>(offer_id, |offer| {
				let offer = offer.as_mut().ok_or(Error::<T>::OfferNotFound)?;
				offer.status = OfferStatus::Closed;
				offer.buyer = Some((buyer.clone(), marketplace_id));
				Ok(())
			})?;
		}
		Ok(())
	}

	fn is_the_offer_valid(price: BalanceOf<T>, percentage: Permill) -> DispatchResult {
		let minimun_amount: BalanceOf<T> = 1000u32.into();
		ensure!(price > minimun_amount, Error::<T>::PriceMustBeGreaterThanZero);
		ensure!(percentage <= Permill::from_percent(99), Error::<T>::ExceedMaxPercentage);
		ensure!(percentage >= Permill::from_percent(1), Error::<T>::ExceedMinPercentage);
		Ok(())
	}

	fn can_this_item_receive_sell_orders(
		collection_id: T::CollectionId,
		item_id: T::ItemId,
		marketplace_id: [u8; 32],
	) -> DispatchResult {
		let offers = <OffersByItem<T>>::get(collection_id, item_id);

		//if len is == 0, it means that there is no offers for this item, maybe it's the first entry
		if offers.len() > 0 {
			for offer in offers {
				let offer_info = <OffersInfo<T>>::get(offer).ok_or(Error::<T>::OfferNotFound)?;
				//ensure the offer_type is SellOrder, because this vector also contains buy offers.
				if offer_info.marketplace_id == marketplace_id
					&& offer_info.offer_type == OfferType::SellOrder
				{
					return Err(Error::<T>::OfferAlreadyExists.into());
				}
			}
		}

		Ok(())
	}

	fn can_this_item_receive_buy_orders(
		collection_id: T::CollectionId,
		item_id: T::ItemId,
		marketplace_id: [u8; 32],
	) -> DispatchResult {
		//First we check if the item has is for sale, if not, return error
		ensure!(
			<OffersByItem<T>>::contains_key(collection_id, item_id),
			Error::<T>::ItemNotForSale
		);

		//ensure the item can receive buy offers on the selected marketplace
		let offers = <OffersByItem<T>>::get(collection_id, item_id);

		for offer in offers {
			let offer_info = <OffersInfo<T>>::get(offer).ok_or(Error::<T>::OfferNotFound)?;
			//ensure the offer_type is SellOrder, because this vector also contains buy offers.
			if offer_info.marketplace_id == marketplace_id
				&& offer_info.offer_type == OfferType::SellOrder
			{
				return Ok(());
			}
		}

		Err(Error::<T>::ItemNotForSale.into())
	}

	fn _delete_all_sell_orders_for_this_item(
		collection_id: T::CollectionId,
		item_id: T::ItemId,
	) -> DispatchResult {
		//ensure the item has offers associated with it.
		ensure!(<OffersByItem<T>>::contains_key(collection_id, item_id), Error::<T>::OfferNotFound);

		let offers_ids = <OffersByItem<T>>::take(collection_id, item_id);
		//let mut remaining_offer_ids: Vec<[u8;32]> = Vec::new();
		let mut buy_offer_ids: BoundedVec<[u8; 32], T::MaxOffersPerMarket> = BoundedVec::default();

		for offer_id in offers_ids {
			let offer_info = <OffersInfo<T>>::get(offer_id).ok_or(Error::<T>::OfferNotFound)?;
			//ensure the offer_type is SellOrder, because this vector also contains offers of BuyOrder OfferType.
			if offer_info.offer_type != OfferType::SellOrder {
				buy_offer_ids.try_push(offer_id).map_err(|_| Error::<T>::LimitExceeded)?;
			}
		}
		//ensure we already took the entry from the storagemap, so we can insert it again.
		ensure!(
			!<OffersByItem<T>>::contains_key(collection_id, item_id),
			Error::<T>::OfferNotFound
		);
		<OffersByItem<T>>::insert(collection_id, item_id, buy_offer_ids);

		Ok(())
	}

	fn delete_all_offers_for_this_item(
		collection_id: T::CollectionId,
		item_id: T::ItemId,
	) -> DispatchResult {
		pallet_fruniques::Pallet::<T>::do_thaw(&collection_id, item_id)?;
		<OffersByItem<T>>::remove(collection_id, item_id);
		Ok(())
	}

pub fn do_ask_for_redeem(
		who: T::AccountId,
		marketplace: MarketplaceId,
		collection_id: T::CollectionId,
		item_id: T::ItemId,
	) -> DispatchResult {
		ensure!(<Marketplaces<T>>::contains_key(marketplace), Error::<T>::MarketplaceNotFound);
		Self::is_authorized(who.clone(), &marketplace, Permission::AskForRedemption)?;
		//ensure the collection exists
		if let Some(a) = pallet_uniques::Pallet::<T>::owner(collection_id, item_id) {
			ensure!(a == who, Error::<T>::NotOwner);
		} else {
			return Err(Error::<T>::CollectionNotFound.into());
		}

		let redemption_data: RedemptionData<T> = RedemptionData {
			creator: who.clone(),
			redeemed_by: None,
			collection_id,
			item_id,
			is_redeemed: false,
		};

		// Gen market id
		let redemption_id = redemption_data.using_encoded(blake2_256);
		// ensure the generated id is unique
		ensure!(
			!<AskingForRedemption<T>>::contains_key(marketplace, redemption_id),
			Error::<T>::RedemptionRequestAlreadyExists
		);

		<AskingForRedemption<T>>::insert(marketplace, redemption_id, redemption_data);
		Self::deposit_event(Event::RedemptionRequested(marketplace, redemption_id, who));

		Ok(())
	}

	pub fn do_accept_redeem(
		who: T::AccountId,
		marketplace: MarketplaceId,
		redemption_id: RedemptionId,
	) -> DispatchResult
	where
		<T as pallet_uniques::Config>::ItemId: From<u32>,
	{
		ensure!(<Marketplaces<T>>::contains_key(marketplace), Error::<T>::MarketplaceNotFound);
		Self::is_authorized(who.clone(), &marketplace, Permission::AcceptRedemption)?;

		ensure!(
			<AskingForRedemption<T>>::contains_key(marketplace, redemption_id),
			Error::<T>::RedemptionRequestNotFound
		);

		<AskingForRedemption<T>>::try_mutate::<_, _, _, DispatchError, _>(
			marketplace,
			redemption_id,
			|redemption_data| -> DispatchResult {
				let redemption_data =
					redemption_data.as_mut().ok_or(Error::<T>::RedemptionRequestNotFound)?;
				ensure!(
					redemption_data.is_redeemed == false,
					Error::<T>::RedemptionRequestAlreadyRedeemed
				);
				ensure!(
					redemption_data.is_redeemed == false,
					Error::<T>::RedemptionRequestAlreadyRedeemed
				);
				redemption_data.is_redeemed = true;
				redemption_data.redeemed_by = Some(who.clone());
				Self::deposit_event(Event::RedemptionAccepted(marketplace, redemption_id, who));
				pallet_fruniques::Pallet::<T>::do_redeem(
					redemption_data.collection_id,
					redemption_data.item_id,
				)?;

				Ok(())
			},
		)?;

		Ok(())
	}
	pub fn pallet_id() -> IdOrVec {
		IdOrVec::Vec(Self::module_name().as_bytes().to_vec())
	}
}
