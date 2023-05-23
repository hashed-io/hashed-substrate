use super::*;

use crate::types::*;
use frame_support::traits::UnixTime;
use frame_system::pallet_prelude::*;
use pallet_gated_marketplace::types::MarketplaceRole;
use pallet_fruniques::types::{CollectionDescription, FruniqueRole, Attributes, ParentInfo};
use frame_support::pallet_prelude::*;
// use frame_support::traits::OriginTrait;
use pallet_rbac::types::IdOrVec;
use pallet_rbac::types::RoleBasedAccessControl;
use pallet_rbac::types::RoleId;
use scale_info::prelude::vec;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::str;
use sp_runtime::sp_std::vec::Vec;
use sp_runtime::traits::StaticLookup;
use frame_support::traits::Time;
use core::convert::TryInto;
use sp_runtime::traits::Zero;

impl<T: Config> Pallet<T> {
	pub fn do_initial_setup(creator: T::AccountId, admin: T::AccountId) -> DispatchResult {

		Self::initialize_rbac()?;

		let creator_user: User<T> = User {
					cid: ShortString::try_from(b"5HeWymtD558YYKHaZvipqysBHR6PCWgvy96Hg2oah2x7CEH5".to_vec()).unwrap(),
					group: ShortString::try_from(b"HCD:QmZcSrTcqBdHck73xYw2WHgEQ9tchPrwNq6hM3a3rvXAAV".to_vec()).unwrap(),
					created_by: Some(creator.clone()),
					created_date: Some(T::TimeProvider::now().as_secs()),
					last_modified_by: Some(creator.clone()),
					last_modified_date: Some(T::TimeProvider::now().as_secs()),
				};
		<UserInfo<T>>::insert(creator.clone(), creator_user);
		Self::give_role_to_user(creator.clone(), AfloatRole::Owner)?;

		if admin != creator {
			let admin_user: User<T> = User {
				cid: ShortString::try_from(b"5E7RDXG1e98KFsY6qtjRsdnArSMPyRe8fYH9BWPLeLtFMA2w".to_vec()).unwrap(),
				group: ShortString::try_from(b"HCD:QmbhAm22mGMVrTmAfkjUzbtZrSXVSLV28Xah5ca5NtwQ3U".to_vec()).unwrap(),
				created_by: Some(admin.clone()),
				created_date: Some(T::TimeProvider::now().as_secs()),
				last_modified_by: Some(admin.clone()),
				last_modified_date: Some(T::TimeProvider::now().as_secs()),
			};
			<UserInfo<T>>::insert(admin.clone(), admin_user);
			Self::give_role_to_user(admin, AfloatRole::Admin)?;
		}

		Ok(())
	}

	// ! User management

	/// This function creates a new user with the given actor, user address, and sign up arguments.
	///
	/// # Inputs
	///
	/// * `actor` - An account ID of the user who initiated this action.
	/// * `user_address` - An account ID of the user to be created.
	/// * `args` - Sign up arguments. It could be either a `BuyerOrSeller` or a `CPA`, and contains
	///            the first name, last name, email, and state of the user.
	///
	/// # Errors
	///
	/// This function may return an error if there is an issue with the `pallet_gated_marketplace`
	/// pallet, which is used to enroll the user in the Afloat marketplace. It may also return an
	/// error if the user already exists.
	///
	/// # Returns
	///
	/// Returns `Ok(())` on success.
	///
	pub fn do_create_user(
		actor: T::AccountId,
		user_address: T::AccountId,
		args: SignUpArgs,
	) -> DispatchResult {
		ensure!(!<UserInfo<T>>::contains_key(user_address.clone()), Error::<T>::UserAlreadyExists);
		match args {
			SignUpArgs::BuyerOrSeller { cid, group } => {
				let user: User<T> = User {
					cid: cid,
					group: group,
					created_by: Some(actor.clone()),
					created_date: Some(T::TimeProvider::now().as_secs()),
					last_modified_by: Some(actor.clone()),
					last_modified_date: Some(T::TimeProvider::now().as_secs()),
				};
				<UserInfo<T>>::insert(user_address.clone(), user);
				Self::give_role_to_user(user_address.clone(), AfloatRole::BuyerOrSeller)?;
				Self::deposit_event(Event::NewUser(user_address.clone()));
			},
			SignUpArgs::CPA { cid, group } => {
				let user: User<T> = User {
					cid: cid,
					group: group,
					created_by: Some(actor.clone()),
					created_date: Some(T::TimeProvider::now().as_secs()),
					last_modified_by: Some(actor.clone()),
					last_modified_date: Some(T::TimeProvider::now().as_secs()),
				};
				<UserInfo<T>>::insert(user_address.clone(), user);
				Self::give_role_to_user(user_address.clone(), AfloatRole::CPA)?;
				Self::deposit_event(Event::NewUser(user_address.clone()));
			},
		}

		let marketplace_id = AfloatMarketPlaceId::<T>::get().ok_or(Error::<T>::MarketPlaceIdNotFound)?;

		Self::add_to_afloat_collection(user_address.clone(),FruniqueRole::Collaborator)?;
		pallet_gated_marketplace::Pallet::<T>::self_enroll(user_address, marketplace_id)?;

		Ok(())
	}
	/// Function for editing user information.
	///
	/// - `actor`: The `AccountId` of the actor performing the edit.
	/// - `user_address`: The `AccountId` of the user account to edit.
	/// - `first_name`: An optional `ShortString` containing the user's first name.
	/// - `last_name`: An optional `ShortString` containing the user's last name.
	/// - `email`: An optional `LongString` containing the user's email address.
	/// - `lang_key`: An optional `ShortString` containing the language code for the user.
	/// - `phone`: An optional `Option<ShortString>` containing the user's phone number, or None if no phone number is provided.
	/// - `credits_needed`: An optional `u32` containing the number of credits needed for the user's account.
	/// - `cpa_id`: An optional `ShortString` containing the user's CPA ID.
	/// - `state`: An optional `u32` containing the user's state tax authority ID.
	///
	/// # Errors
	///
	/// Returns an `Error` if the requested user account is not found or if the edit operation fails.
	///
	/// # Returns
	///
	/// Returns `Ok(())` on success.
	///
	pub fn do_edit_user(
		actor: T::AccountId,
		user_address: T::AccountId,
		cid: ShortString,
	) -> DispatchResult {

		<UserInfo<T>>::try_mutate::<_, _, DispatchError, _>(user_address.clone(), |user| {
			let user = user.as_mut().ok_or(Error::<T>::FailedToEditUserAccount)?;

			user.last_modified_date = Some(T::TimeProvider::now().as_secs());
			user.last_modified_by = Some(actor.clone());
			user.cid = cid;

			Ok(())
		})?;

		Ok(())
	}

	pub fn do_admin_edit_user(
		actor: T::AccountId,
		user_address: T::AccountId,
		cid: ShortString,
		group: ShortString
	) -> DispatchResult {

		<UserInfo<T>>::try_mutate::<_, _, DispatchError, _>(user_address.clone(), |user| {
			let user = user.as_mut().ok_or(Error::<T>::FailedToEditUserAccount)?;

			user.last_modified_date = Some(T::TimeProvider::now().as_secs());
			user.last_modified_by = Some(actor.clone());
			user.cid = cid;
			user.group = group;

			Ok(())
		})?;

		Ok(())

	}
	/// Function for deleting a user account.
	///
	/// - _actor: The AccountId of the actor performing the deletion. This parameter is currently unused.
	/// - user_address: The AccountId of the user account to delete.
	///
	/// # Errors
	///
	/// Returns an Error if the requested user account is not found.
	///
	/// # Returns
	///
	/// Returns Ok(()) on success.
	///
	pub fn do_delete_user(_actor: T::AccountId, user_address: T::AccountId) -> DispatchResult {

		Self::remove_from_afloat_collection(user_address.clone(), FruniqueRole::Collaborator)?;
		Self::remove_from_afloat_marketplace(user_address.clone())?;

		let user_roles = Self::get_all_roles_for_user(user_address.clone())?;

		if !user_roles.is_empty() {
			for role in user_roles {
				Self::remove_role_from_user(user_address.clone(), role)?;
			}
		}
		<UserInfo<T>>::remove(user_address.clone());
		Self::deposit_event(Event::UserDeleted(user_address.clone()));
		Ok(())
	}

	pub fn do_set_afloat_balance(
		origin: OriginFor<T>,
		user_address: T::AccountId,
		amount: T::Balance,
	) -> DispatchResult {

		let authority = ensure_signed(origin.clone())?;
		let asset_id = AfloatAssetId::<T>::get().expect("AfloatAssetId should be set");

		ensure!(UserInfo::<T>::contains_key(user_address.clone()), Error::<T>::UserNotFound);

		let current_balance = Self::do_get_afloat_balance(user_address.clone());

		if current_balance > amount {
			let diff = current_balance - amount;
			pallet_mapped_assets::Pallet::<T>::burn(
				origin.clone(),
				asset_id.into(),
				T::Lookup::unlookup(user_address.clone()),
				diff,
			)?;
		} else if current_balance < amount {
			let diff = amount - current_balance;
			pallet_mapped_assets::Pallet::<T>::mint(
				origin.clone(),
				asset_id.into(),
				T::Lookup::unlookup(user_address.clone()),
				diff,
			)?;
		}

		Self::deposit_event(Event::AfloatBalanceSet(authority, user_address, amount));
		Ok(())
	}

	pub fn do_get_afloat_balance(user_address: T::AccountId) -> T::Balance {
		let asset_id = AfloatAssetId::<T>::get().expect("AfloatAssetId should be set");
		pallet_mapped_assets::Pallet::<T>::balance(asset_id.into(), user_address)
	}


	pub fn do_create_sell_order(
		authority: T::AccountId,
		item_id: <T as pallet_uniques::Config>::ItemId,
		price: T::Balance,
		tax_credit_amount: u32,
		expiration_date: Date,
	) -> DispatchResult
	{

		let maybe_roles = Self::get_all_roles_for_user(authority.clone())?;
		ensure!(!maybe_roles.is_empty(), Error::<T>::Unauthorized);

		let marketplace_id = AfloatMarketPlaceId::<T>::get().ok_or(Error::<T>::MarketPlaceIdNotFound)?;
		let collection_id = AfloatCollectionId::<T>::get().ok_or(Error::<T>::CollectionIdNotFound)?;
		let transactions = TransactionBoundedVec::default();

		let offer_id = pallet_gated_marketplace::Pallet::<T>::do_enlist_sell_offer(
			authority.clone(),
			marketplace_id,
			collection_id,
			item_id,
			price * tax_credit_amount.into(),
			tax_credit_amount,
		)?;

		let offer: Offer<T> = Offer {
			tax_credit_amount,
			tax_credit_amount_remaining: tax_credit_amount.into(),
			price_per_credit: price,
			creation_date: T::TimeProvider::now().as_secs(),
			expiration_date: expiration_date,
			tax_credit_id: item_id,
			creator_id: authority.clone(),
			status: OfferStatus::default(),
			offer_type: OfferType::Sell,
			cancellation_date: None,
			transactions,
		};

		<AfloatOffers<T>>::insert(offer_id, offer);


		Self::deposit_event(Event::SellOrderCreated(authority));

		Ok(())
	}

	pub fn do_create_buy_order(
		authority: T::AccountId,
		item_id: <T as pallet_uniques::Config>::ItemId,
		price: T::Balance,
		tax_credit_amount: u32,
		expiration_date: Date,
	) -> DispatchResult
	{
		let maybe_roles = Self::get_all_roles_for_user(authority.clone())?;
		ensure!(!maybe_roles.is_empty(), Error::<T>::Unauthorized);

		let marketplace_id = AfloatMarketPlaceId::<T>::get().ok_or(Error::<T>::MarketPlaceIdNotFound)?;
		let collection_id = AfloatCollectionId::<T>::get().ok_or(Error::<T>::CollectionIdNotFound)?;
		let transactions = TransactionBoundedVec::default();

		let offer_id = pallet_gated_marketplace::Pallet::<T>::do_enlist_buy_offer(
			authority.clone(),
			marketplace_id,
			collection_id,
			item_id,
			price * tax_credit_amount.into(),
			tax_credit_amount,
		)?;

		let offer: Offer<T> = Offer {
			tax_credit_amount,
			tax_credit_amount_remaining: tax_credit_amount.into(),
			price_per_credit: price,
			creation_date: T::TimeProvider::now().as_secs(),
			expiration_date: expiration_date,
			tax_credit_id: item_id,
			creator_id: authority.clone(),
			status: OfferStatus::default(),
			offer_type: OfferType::Buy,
			cancellation_date: None,
			transactions,
		};

		<AfloatOffers<T>>::insert(offer_id, offer);

		Self::deposit_event(Event::BuyOrderCreated(authority));

		Ok(())
	}

	/// Starts the process of taking a sell order.
	///
	/// # Arguments
	///
	/// * `authority` - The origin of the call, from where the function is triggered.
	/// * `order_id` - The unique identifier of the order.
	/// * `tax_credit_amount` - The amount of tax credit to to take/buy from the original offer.
	///
	/// # Return
	///
	/// * Returns a `DispatchResult` to indicate the success or failure of the operation.
	///
	/// # Errors
	///
	/// This function will return an error if:
	/// * The caller does not have any roles.
	/// * The specified offer does not exist.
	/// * The specified offer is not a sell offer.
	/// * The specified offer has expired.
	/// * The specified offer has been cancelled.
	/// * The specified offer has already been taken.
	/// * The specified offer does not have enough tax credits available for sale.
	/// * The caller does not have enough afloat balance to take the offer.
	///
	/// # Side Effects
	///
	/// * If the function is successful, it will mutate the state of the order and create a transaction.
	///
	/// # Panics
	///
	/// * This function does not panic.
	///
	/// # Safety
	///
	/// * This function does not use any unsafe blocks.
	/// 
	pub fn do_start_take_sell_order(
		authority: OriginFor<T>,
		order_id: [u8; 32],
		tax_credit_amount: T::Balance,
	) -> DispatchResult
	where
	<T as pallet_uniques::Config>::ItemId: From<u32>
	{
		let who = ensure_signed(authority.clone())?;

		let maybe_roles = Self::get_all_roles_for_user(who.clone())?;
		ensure!(!maybe_roles.is_empty(), Error::<T>::Unauthorized);
		// ensure offer exists 
		ensure!(<AfloatOffers<T>>::contains_key(order_id), Error::<T>::OfferNotFound);
		//get offer details
		let offer = <AfloatOffers<T>>::get(order_id).unwrap();
		//ensure offer is a sell offer
		ensure!(offer.offer_type == OfferType::Sell, Error::<T>::WrongOfferType);
		//ensure offer is not expired
		ensure!(offer.expiration_date > T::TimeProvider::now().as_secs(), Error::<T>::OfferExpired);
		//ensure offer is not cancelled
		ensure!(offer.cancellation_date.is_none(), Error::<T>::OfferCancelled);
		//ensure offer is not taken
		ensure!(offer.status == OfferStatus::default(), Error::<T>::OfferTaken);
		//ensure offer has enough tax credits for sale
		ensure!(offer.tax_credit_amount_remaining >= tax_credit_amount, Error::<T>::NotEnoughTaxCreditsAvailable);
		//ensure user has enough afloat balance
		ensure!(Self::do_get_afloat_balance(who.clone()) >= offer.price_per_credit * tax_credit_amount.into(), Error::<T>::NotEnoughAfloatBalanceAvailable);
		let zero_balance: T::Balance = Zero::zero();
		//ensure tax credit amount is greater than zero
		ensure!(tax_credit_amount > zero_balance, Error::<T>::Underflow);

		let creation_date: u64 = T::Timestamp::now().into();
		let price_per_credit: T::Balance = offer.price_per_credit.into();
		let total_price: T::Balance = price_per_credit * tax_credit_amount;
		let fee: Option<T::Balance> = None;
		let tax_credit_id: <T as pallet_uniques::Config>::ItemId = offer.tax_credit_id;
		let seller_id: T::AccountId = offer.creator_id;
		let buyer_id: T::AccountId = who.clone();
		let offer_id: StorageId = order_id;
		let seller_confirmation_date: Option<Date> = None;
		let buyer_confirmation_date: Option<Date> = Some(creation_date);
		let confirmed: bool = false;
		let completed: bool = false;
		let child_offer_id: Option<StorageId> = None;

		let transaction = Transaction {
		 tax_credit_amount,
		 price_per_credit,
		 total_price,
		 fee,
		 creation_date,
		 cancellation_date: None,
		 tax_credit_id,
		 seller_id,
		 buyer_id,
		 offer_id,
		 child_offer_id,
		 seller_confirmation_date,
		 buyer_confirmation_date,
		 confirmed,
		 completed,
		};

		let transaction_id = transaction.clone().using_encoded(blake2_256);

		<AfloatOffers<T>>::try_mutate(order_id, |offer| -> DispatchResult {
			let offer = offer.as_mut().ok_or(Error::<T>::OfferNotFound)?;
			offer.transactions.try_push(transaction_id.clone())
				.map_err(|_| Error::<T>::MaxTransactionsReached)?;
			Ok(())
		})?;

		<AfloatTransactions<T>>::insert(transaction_id, transaction);

		Ok(())
	}

	/// Confirms a sell transaction.
	///
	/// # Arguments
	///
	/// * `authority` - The origin of the call, from where the function is triggered.
	/// * `transaction_id` - The unique identifier of the transaction.
	///
	/// # Return
	///
	/// * Returns a `DispatchResult` to indicate the success or failure of the operation.
	///
	/// # Errors
	///
	/// This function will return an error if:
	/// * The caller does not have any roles.
	/// * The specified transaction does not exist.
	/// * The caller is not the seller in the transaction.
	/// * The specified transaction has been cancelled.
	/// * The specified transaction has already been confirmed by the seller.
	/// * The specified transaction has not been confirmed by the buyer.
	/// * The `AfloatMarketPlaceId` or `AfloatCollectionId` does not exist.
	/// * The tax credit amount overflows when converting from ``T::Balance`` to `u32`.
	///
	/// # Side Effects
	///
	/// * If the function is successful, it will mutate the state of the transaction, setting the seller confirmation date,
	/// confirming the transaction, and linking the transaction to a new child offer.
	///
	/// # Panics
	///
	/// * This function does not panic.
	///
	/// # Safety
	///
	/// * This function does not use any unsafe blocks.
	///
	/// # Note
	///
	/// * Before calling this function, make sure that the transaction_id exists and the caller is the seller.
	/// 
	pub fn do_confirm_sell_transaction(
		authority: OriginFor<T>,
		transaction_id: [u8; 32],
	) -> DispatchResult
	where
	<T as pallet_uniques::Config>::ItemId: From<u32>
	{
		let who = ensure_signed(authority.clone())?;
	
		let maybe_roles = Self::get_all_roles_for_user(who.clone())?;
		ensure!(!maybe_roles.is_empty(), Error::<T>::Unauthorized);
	
		// Ensure the transaction exists before trying to get it
		ensure!(<AfloatTransactions<T>>::contains_key(transaction_id), Error::<T>::TransactionNotFound);
	
		// Get transaction details
		let transaction = <AfloatTransactions<T>>::get(transaction_id).ok_or(Error::<T>::TransactionNotFound)?;
	
		// Ensure user is the seller
		ensure!(transaction.seller_id == who.clone(), Error::<T>::Unauthorized);
	
		// Ensure transaction is not cancelled
		ensure!(transaction.cancellation_date.is_none(), Error::<T>::TransactionCancelled);
	
		// Ensure transaction is not already confirmed by the seller
		ensure!(transaction.seller_confirmation_date.is_none(), Error::<T>::TransactionAlreadyConfirmedBySeller);
	
		// Ensure transaction has buyer confirmation
		ensure!(transaction.buyer_confirmation_date.is_some(), Error::<T>::TransactionNotConfirmedByBuyer);
	
		let confirmation_date: u64 = T::Timestamp::now().into();
		let confirmed: bool = true;
	
		let marketplace_id = <AfloatMarketPlaceId<T>>::get().ok_or(Error::<T>::MarketPlaceIdNotFound)?;
		let collection_id = <AfloatCollectionId<T>>::get().ok_or(Error::<T>::CollectionIdNotFound)?;
	
		let tax_credit_amount_u32 = if let Ok(amount) = transaction.tax_credit_amount.try_into() {
			amount
		} else {
			return Err(Error::<T>::TaxCreditAmountOverflow.into())
		};
	
		let offer = <AfloatOffers<T>>::get(transaction.offer_id).ok_or(Error::<T>::OfferNotFound)?;
	
		let children_offer_id = if tax_credit_amount_u32 != offer.tax_credit_amount {
			pallet_gated_marketplace::Pallet::<T>::do_enlist_sell_offer(
				who,
				marketplace_id,
				collection_id,
				transaction.tax_credit_id,
				transaction.total_price,
				tax_credit_amount_u32,
			)?
		} else {
			transaction.offer_id
		};
	
		<AfloatTransactions<T>>::try_mutate(transaction_id, |transaction| -> DispatchResult {
			let mut transaction = transaction.as_mut().ok_or(Error::<T>::TransactionNotFound)?;
			transaction.seller_confirmation_date = Some(confirmation_date);
			transaction.confirmed = confirmed;
			transaction.child_offer_id = Some(children_offer_id);
			Ok(())
		})?;
	
		Ok(())
	}


	/// Finishes the process of taking a sell transaction.
	///
	/// # Arguments
	///
	/// * `authority` - The origin of the call, from where the function is triggered.
	/// * `transaction_id` - The unique identifier of the transaction.
	///
	/// # Return
	///
	/// * Returns a `DispatchResult` to indicate the success or failure of the operation.
	///
	/// # Errors
	///
	/// This function will return an error if:
	/// * The caller does not have any roles.
	/// * The specified transaction does not exist.
	/// * The specified transaction has been cancelled.
	/// * The specified transaction has not been confirmed.
	/// * The child offer id in the transaction does not exist.
	/// * The specified offer does not exist.
	/// * The tax credit amount in the offer is less than the tax credit amount in the transaction (underflow).
	///
	/// # Side Effects
	///
	/// * If the function is successful, it will trigger the transfer of tax credits and Balance between buyer and seller,
	///  mutate the state of the offer and transaction and emit a `SellOrderTaken` event.
	///
	/// # Panics
	///
	/// * This function does not panic.
	///
	/// # Safety
	///
	/// * This function does not use any unsafe blocks.
	///
	/// # Note
	///
	/// * Before calling this function, make sure that the transaction id exists, the transaction is confirmed, and the caller is authorized.
	/// 
	pub fn do_finish_take_sell_transaction(
		authority: OriginFor<T>,
		transaction_id: [u8; 32],
	) -> DispatchResult
	where
	<T as pallet_uniques::Config>::ItemId: From<u32>
	{
		let who = ensure_signed(authority.clone())?;
	
		let maybe_roles = Self::get_all_roles_for_user(who.clone())?;
		ensure!(!maybe_roles.is_empty(), Error::<T>::Unauthorized);
	
		// Ensure the transaction exists before trying to get it
		ensure!(<AfloatTransactions<T>>::contains_key(transaction_id), Error::<T>::TransactionNotFound);
	
		// Get transaction details
		let transaction = <AfloatTransactions<T>>::get(transaction_id).ok_or(Error::<T>::TransactionNotFound)?;
	
		// Ensure transaction is not cancelled
		ensure!(transaction.cancellation_date.is_none(), Error::<T>::TransactionCancelled);
	
		// Ensure transaction is confirmed
		ensure!(transaction.confirmed, Error::<T>::TransactionNotConfirmed);
		
		// Ensure the child offer id exists
		let child_offer_id = transaction.child_offer_id.ok_or(Error::<T>::ChildOfferIdNotFound)?;
		let offer_id = transaction.offer_id;
	
		pallet_gated_marketplace::Pallet::<T>::do_take_sell_offer(
			authority.clone(),
			child_offer_id,
		)?;
	
		<AfloatOffers<T>>::try_mutate(offer_id, |offer| -> DispatchResult {
			let offer = offer.as_mut().ok_or(Error::<T>::OfferNotFound)?;
			if transaction.tax_credit_amount > offer.tax_credit_amount_remaining {
				return Err(Error::<T>::Underflow.into());
			}
			offer.tax_credit_amount_remaining = offer.tax_credit_amount_remaining - transaction.tax_credit_amount;
			Ok(())
		})?;
		
		<AfloatTransactions<T>>::try_mutate(transaction_id, |transaction| -> DispatchResult {
			let mut transaction = transaction.as_mut().ok_or(Error::<T>::TransactionNotFound)?;
			transaction.completed = true;
			Ok(())
		})?;

		Self::deposit_event(Event::SellOrderTaken(who));
		Ok(())
		
	}

	pub fn do_take_buy_order(
		authority: T::AccountId,
		order_id: [u8; 32],
	) -> DispatchResult
	where
	<T as pallet_uniques::Config>::ItemId: From<u32>
	{
		let maybe_roles = Self::get_all_roles_for_user(authority.clone())?;
		ensure!(!maybe_roles.is_empty(), Error::<T>::Unauthorized);

		pallet_gated_marketplace::Pallet::<T>::do_take_buy_offer(
			authority.clone(),
			order_id,
		)?;

		Self::deposit_event(Event::BuyOrderTaken(authority));
		Ok(())
	}

	pub fn do_create_tax_credit(
        owner: T::AccountId,
        metadata: CollectionDescription<T>,
        attributes: Option<Attributes<T>>,
        parent_info: Option<ParentInfo<T>>,
    ) -> DispatchResult
	where
		<T as pallet_uniques::Config>::ItemId: From<u32>,
		<T as pallet_uniques::Config>::CollectionId: From<u32>,
	{
		let maybe_roles = Self::get_all_roles_for_user(owner.clone())?;
		ensure!(!maybe_roles.is_empty(), Error::<T>::Unauthorized);

		let collection = AfloatCollectionId::<T>::get().ok_or(Error::<T>::CollectionIdNotFound)?;

        pallet_fruniques::Pallet::<T>::do_spawn(
            collection,
            owner,
            metadata,
            attributes,
            parent_info,
        )
    }

	pub fn create_afloat_collection(origin: OriginFor<T>,
		metadata: CollectionDescription<T>,
		admin: T::AccountId, ) -> DispatchResult
		where
		<T as pallet_uniques::Config>::CollectionId: From<u32>,
		{

		let collection_id = pallet_fruniques::Pallet::<T>::do_create_collection(
			origin.clone(),
			metadata,
			admin.clone(),
		);
		if let Ok(collection_id) = collection_id {
			AfloatCollectionId::<T>::put(collection_id);
			Ok(())
		} else {
			return Err(Error::<T>::FailedToCreateFruniquesCollection.into());
		}
	}

	pub fn add_to_afloat_collection(invitee: T::AccountId, role: FruniqueRole) -> DispatchResult {
		let collection_id = AfloatCollectionId::<T>::get().ok_or(Error::<T>::CollectionIdNotFound)?;
		pallet_fruniques::Pallet::<T>::insert_auth_in_frunique_collection(invitee,
		collection_id,
		role
		)
	}

	pub fn remove_from_afloat_collection(invitee: T::AccountId, role: FruniqueRole) -> DispatchResult {
		let collection_id = AfloatCollectionId::<T>::get().ok_or(Error::<T>::CollectionIdNotFound)?;
		pallet_fruniques::Pallet::<T>::remove_auth_from_frunique_collection(invitee,
		collection_id,
		role
		)
	}

	pub fn remove_from_afloat_marketplace(invitee: T::AccountId) -> DispatchResult {
		let marketplace_id = AfloatMarketPlaceId::<T>::get().ok_or(Error::<T>::MarketPlaceIdNotFound)?;
		pallet_gated_marketplace::Pallet::<T>::remove_from_market_lists(invitee, MarketplaceRole::Participant, marketplace_id)
	}

	pub fn pallet_id() -> IdOrVec {
		IdOrVec::Vec(Self::module_name().as_bytes().to_vec())
	}

	pub fn is_admin_or_owner(account: T::AccountId) -> Result<bool, DispatchError> {
		let marketplace_id = AfloatMarketPlaceId::<T>::get().ok_or(Error::<T>::MarketPlaceIdNotFound)?;
		
		let maybe_super_role = <T as pallet::Config>::Rbac::has_role(
			account.clone(),
			Self::pallet_id(),
			&marketplace_id,
			[AfloatRole::Admin.id(), AfloatRole::Owner.id()].to_vec(),
		);
		
		Ok(maybe_super_role.is_ok())
	}

	pub fn is_owner(account: T::AccountId) -> Result<bool, DispatchError> {
		let marketplace_id = AfloatMarketPlaceId::<T>::get().ok_or(Error::<T>::MarketPlaceIdNotFound)?;
		
		let maybe_owner = <T as pallet::Config>::Rbac::has_role(
			account.clone(),
			Self::pallet_id(),
			&marketplace_id,
			[AfloatRole::Owner.id()].to_vec(),
		);
		
		Ok(maybe_owner.is_ok())
	}

	pub fn is_cpa(account: T::AccountId) -> Result<bool, DispatchError> {
		let marketplace_id = AfloatMarketPlaceId::<T>::get().ok_or(Error::<T>::MarketPlaceIdNotFound)?;

		let maybe_cpa = <T as pallet::Config>::Rbac::has_role(
			account.clone(),
			Self::pallet_id(),
			&marketplace_id,
			[AfloatRole::CPA.id()].to_vec(),
		);

		Ok(maybe_cpa.is_ok())
	}

	pub fn give_role_to_user(
		authority: T::AccountId,
		role: AfloatRole,
	) -> DispatchResult {
		let marketplace_id = AfloatMarketPlaceId::<T>::get().ok_or(Error::<T>::MarketPlaceIdNotFound)?;
		<T as pallet::Config>::Rbac::assign_role_to_user(
			authority,
			Self::pallet_id(),
			&marketplace_id,
			role.id(),
		)?;

		Ok(())
	}

	pub fn remove_role_from_user(
		authority: T::AccountId,
		role: AfloatRole,
	) -> DispatchResult {
		let marketplace_id = AfloatMarketPlaceId::<T>::get().ok_or(Error::<T>::MarketPlaceIdNotFound)?;
		<T as pallet::Config>::Rbac::remove_role_from_user(
			authority,
			Self::pallet_id(),
			&marketplace_id,
			role.id(),
		)?;

		Ok(())
	}

	pub fn initialize_rbac() -> DispatchResult {
		let marketplace_id = AfloatMarketPlaceId::<T>::get().ok_or(Error::<T>::MarketPlaceIdNotFound)?;
		<T as pallet::Config>::Rbac::create_scope(Self::pallet_id(), marketplace_id)?;
		let pallet_id = Self::pallet_id();
		let super_roles = vec![AfloatRole::Owner.to_vec(), AfloatRole::Admin.to_vec()];
		let super_role_ids =
		<T as pallet::Config>::Rbac::create_and_set_roles(pallet_id.clone(), super_roles)?;
		let super_permissions = Permission::admin_permissions();
		for super_role in super_role_ids {
			<T as pallet::Config>::Rbac::create_and_set_permissions(
				pallet_id.clone(),
				super_role,
				super_permissions.clone(),
			)?;
		}
		let participant_roles = vec![AfloatRole::BuyerOrSeller.to_vec(), AfloatRole::CPA.to_vec()];
		<T as pallet::Config>::Rbac::create_and_set_roles(
			pallet_id.clone(),
			participant_roles,
		)?;

		Ok(())
	}

	fn role_id_to_afloat_role(role_id: RoleId) -> Option<AfloatRole> {
		AfloatRole::enum_to_vec()
			.iter()
			.find(|role_bytes| role_bytes.using_encoded(blake2_256) == role_id)
			.map(|role_bytes| {
				let role_str = str::from_utf8(role_bytes).expect("Role bytes should be valid UTF-8");

				match role_str {
					"Owner" => AfloatRole::Owner,
					"Admin" => AfloatRole::Admin,
					"BuyerOrSeller" => AfloatRole::BuyerOrSeller,
					"CPA" => AfloatRole::CPA,
					_ => panic!("Unexpected role string"),
				}
			})
	}

	fn get_all_roles_for_user(account_id: T::AccountId) -> Result<Vec<AfloatRole>, DispatchError> {
		let pallet_id = Self::pallet_id();
		let scope_id = AfloatMarketPlaceId::<T>::get().ok_or(Error::<T>::MarketPlaceIdNotFound)?;

		let roles_storage = <T as pallet::Config>::Rbac::get_roles_by_user(account_id.clone(), pallet_id, &scope_id);

		Ok(roles_storage.into_iter().filter_map(Self::role_id_to_afloat_role).collect())
	}

	pub fn do_delete_all_users() -> DispatchResult {
		UserInfo::<T>::iter_keys().try_for_each(|account_id| {
			let is_admin_or_owner = Self::is_admin_or_owner(account_id.clone())?;

			if !is_admin_or_owner {
				let user_roles = Self::get_all_roles_for_user(account_id.clone())?;

				if !user_roles.is_empty() {
					for role in user_roles {
						Self::remove_role_from_user(account_id.clone(), role)?;
					}
				}

				Self::remove_from_afloat_collection(account_id.clone(), FruniqueRole::Collaborator)?;
				Self::remove_from_afloat_marketplace(account_id.clone())?;
				UserInfo::<T>::remove(account_id);
			}
			Ok::<(), DispatchError>(())
		})?;
		Ok(())
	}

}
