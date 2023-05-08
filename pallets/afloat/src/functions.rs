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

impl<T: Config> Pallet<T> {
	pub fn do_initial_setup(creator: T::AccountId, admin: T::AccountId) -> DispatchResult {

		Self::initialize_rbac()?;

		let creator_user: User<T> = User {
					first_name: ShortString::try_from(b"Afloat".to_vec()).unwrap(),
					last_name: ShortString::try_from(b"Creator".to_vec()).unwrap(),
					email: LongString::try_from(b"".to_vec()).unwrap(),
					lang_key: ShortString::try_from(b"en".to_vec()).unwrap(),
					created_by: Some(creator.clone()),
					created_date: Some(T::TimeProvider::now().as_secs()),
					last_modified_by: Some(creator.clone()),
					last_modified_date: Some(T::TimeProvider::now().as_secs()),
					phone: None,
					credits_needed: 0,
					cpa_id: ShortString::try_from(b"0".to_vec()).unwrap(),
					tax_authority_id: 1,
					lock_expiration_date: None,
				};
		<UserInfo<T>>::insert(creator.clone(), creator_user);
		Self::give_role_to_user(creator.clone(), AfloatRole::Owner)?;

		if admin != creator {
			let admin_user: User<T> = User {
				first_name: ShortString::try_from(b"Afloat".to_vec()).unwrap(),
				last_name: ShortString::try_from(b"Admin".to_vec()).unwrap(),
				email: LongString::try_from(b"".to_vec()).unwrap(),
				lang_key: ShortString::try_from(b"en".to_vec()).unwrap(),
				created_by: Some(admin.clone()),
				created_date: Some(T::TimeProvider::now().as_secs()),
				last_modified_by: Some(admin.clone()),
				last_modified_date: Some(T::TimeProvider::now().as_secs()),
				phone: None,
				credits_needed: 0,
				cpa_id: ShortString::try_from(b"0".to_vec()).unwrap(),
				tax_authority_id: 1,
				lock_expiration_date: None,
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
			SignUpArgs::BuyerOrSeller { first_name, last_name, email, state } => {
				let user: User<T> = User {
					first_name,
					last_name,
					email,
					lang_key: ShortString::try_from(b"en".to_vec()).unwrap(),
					created_by: Some(actor.clone()),
					created_date: Some(T::TimeProvider::now().as_secs()),
					last_modified_by: Some(actor.clone()),
					last_modified_date: Some(T::TimeProvider::now().as_secs()),
					phone: None,
					credits_needed: 0,
					cpa_id: ShortString::try_from(b"0".to_vec()).unwrap(),
					tax_authority_id: state,
					lock_expiration_date: None,
				};
				<UserInfo<T>>::insert(user_address.clone(), user);
				Self::give_role_to_user(user_address.clone(), AfloatRole::BuyerOrSeller)?;
				Self::deposit_event(Event::NewUser(user_address.clone()));
			},
			SignUpArgs::CPA { first_name, last_name, email, license_number, state } => {
				let user: User<T> = User {
					first_name,
					last_name,
					email,
					lang_key: ShortString::try_from(b"en".to_vec()).unwrap(),
					created_by: Some(user_address.clone()),
					created_date: Some(T::TimeProvider::now().as_secs()),
					last_modified_by: Some(user_address.clone()),
					last_modified_date: Some(T::TimeProvider::now().as_secs()),
					phone: None,
					credits_needed: 0,
					cpa_id: license_number,
					tax_authority_id: state,
					lock_expiration_date: None,
				};
				<UserInfo<T>>::insert(user_address.clone(), user);
				Self::give_role_to_user(user_address.clone(), AfloatRole::CPA)?;
				Self::deposit_event(Event::NewUser(user_address.clone()));
			},
		}

		let marketplace_id = AfloatMarketPlaceId::<T>::get().unwrap();

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
		first_name: Option<ShortString>,
		last_name: Option<ShortString>,
		email: Option<LongString>,
		lang_key: Option<ShortString>,
		phone: Option<Option<ShortString>>,
		credits_needed: Option<u32>,
		cpa_id: Option<ShortString>,
		state: Option<u32>,
	) -> DispatchResult {

		<UserInfo<T>>::try_mutate::<_, _, DispatchError, _>(user_address.clone(), |user| {
			let user = user.as_mut().ok_or(Error::<T>::FailedToEditUserAccount)?;

			user.last_modified_date = Some(T::TimeProvider::now().as_secs());
			user.last_modified_by = Some(actor.clone());

			if let Some(first_name) = first_name {
				user.first_name = first_name;
			}
			if let Some(last_name) = last_name {
				user.last_name = last_name;
			}
			if let Some(email) = email {
				user.email = email;
			}
			if let Some(lang_key) = lang_key {
				user.lang_key = lang_key;
			}
			if let Some(phone) = phone {
				user.phone = phone;
			}
			if let Some(credits_needed) = credits_needed {
				user.credits_needed = credits_needed;
			}
			if let Some(cpa_id) = cpa_id {
				user.cpa_id = cpa_id;
			}
			if let Some(state) = state {
				user.tax_authority_id = state;
			}

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

		let user_roles = Self::get_all_roles_for_user(user_address.clone());

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
	) -> DispatchResult
	{

		ensure!(!Self::get_all_roles_for_user(authority.clone()).is_empty(), Error::<T>::Unauthorized);

		let marketplace_id = AfloatMarketPlaceId::<T>::get().unwrap();
		let collection_id = AfloatCollectionId::<T>::get().unwrap();

		let offer_id = pallet_gated_marketplace::Pallet::<T>::do_enlist_sell_offer(
			authority.clone(),
			marketplace_id,
			collection_id,
			item_id,
			price,
			percentage,
		)?;

		Self::deposit_event(Event::SellOrderCreated(authority));

		Ok(())
	}

	pub fn do_create_buy_order(
		authority: T::AccountId,
		item_id: <T as pallet_uniques::Config>::ItemId,
		price: T::Balance,
		tax_credit_amount: u32,
	) -> DispatchResult
	{
		ensure!(!Self::get_all_roles_for_user(authority.clone()).is_empty(), Error::<T>::Unauthorized);

		let marketplace_id = AfloatMarketPlaceId::<T>::get().unwrap();
		let collection_id = AfloatCollectionId::<T>::get().unwrap();

		let offer_id = pallet_gated_marketplace::Pallet::<T>::do_enlist_buy_offer(
			authority.clone(),
			marketplace_id,
			collection_id,
			item_id,
			price,
			percentage,
		)?;

		Self::deposit_event(Event::BuyOrderCreated(authority));

		Ok(())
	}

	pub fn do_take_sell_order(
		authority: OriginFor<T>,
		order_id: [u8; 32],
	) -> DispatchResult
	where
	<T as pallet_uniques::Config>::ItemId: From<u32>
	{
		let who = ensure_signed(authority.clone())?;

		ensure!(!Self::get_all_roles_for_user(who.clone()).is_empty(), Error::<T>::Unauthorized);

		pallet_gated_marketplace::Pallet::<T>::do_take_sell_offer(
			authority.clone(),
			order_id,
		)?;

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
		ensure!(!Self::get_all_roles_for_user(authority.clone()).is_empty(), Error::<T>::Unauthorized);

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
		ensure!(!Self::get_all_roles_for_user(owner.clone()).is_empty(), Error::<T>::Unauthorized);

		let collection = AfloatCollectionId::<T>::get().unwrap();

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
		let collection_id = AfloatCollectionId::<T>::get().unwrap();
		pallet_fruniques::Pallet::<T>::insert_auth_in_frunique_collection(invitee,
		collection_id,
		role
		)
	}

	pub fn remove_from_afloat_collection(invitee: T::AccountId, role: FruniqueRole) -> DispatchResult {
		let collection_id = AfloatCollectionId::<T>::get().unwrap();
		pallet_fruniques::Pallet::<T>::remove_auth_from_frunique_collection(invitee,
		collection_id,
		role
		)
	}

	pub fn remove_from_afloat_marketplace(invitee: T::AccountId) -> DispatchResult {
		let marketplace_id = AfloatMarketPlaceId::<T>::get().unwrap();
		pallet_gated_marketplace::Pallet::<T>::remove_from_market_lists(invitee, MarketplaceRole::Participant, marketplace_id)
	}

	pub fn pallet_id() -> IdOrVec {
		IdOrVec::Vec(Self::module_name().as_bytes().to_vec())
	}

	pub fn is_admin_or_owner(account: T::AccountId) -> bool {
		let marketplace_id = AfloatMarketPlaceId::<T>::get().unwrap();
		<T as pallet::Config>::Rbac::has_role(
			account.clone(),
			Self::pallet_id(),
			&marketplace_id,
			[AfloatRole::Admin.id(), AfloatRole::Owner.id()].to_vec(),
		)
		.is_ok()
	}

	pub fn is_owner(account: T::AccountId) -> bool {
		let marketplace_id = AfloatMarketPlaceId::<T>::get().unwrap();
		<T as pallet::Config>::Rbac::has_role(
			account.clone(),
			Self::pallet_id(),
			&marketplace_id,
			[AfloatRole::Owner.id()].to_vec(),
		)
		.is_ok()
	}

	pub fn is_cpa(account: T::AccountId) -> bool {
		let marketplace_id = AfloatMarketPlaceId::<T>::get().unwrap();
		<T as pallet::Config>::Rbac::has_role(
			account.clone(),
			Self::pallet_id(),
			&marketplace_id,
			[AfloatRole::CPA.id()].to_vec(),
		)
		.is_ok()
	}

	pub fn give_role_to_user(
		authority: T::AccountId,
		role: AfloatRole,
	) -> DispatchResult {
		let marketplace_id = AfloatMarketPlaceId::<T>::get().unwrap();
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
		let marketplace_id = AfloatMarketPlaceId::<T>::get().unwrap();
		<T as pallet::Config>::Rbac::remove_role_from_user(
			authority,
			Self::pallet_id(),
			&marketplace_id,
			role.id(),
		)?;

		Ok(())
	}

	pub fn initialize_rbac() -> DispatchResult {
		let marketplace_id = AfloatMarketPlaceId::<T>::get().unwrap();
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

	fn get_all_roles_for_user(account_id: T::AccountId) -> Vec<AfloatRole> {
		let pallet_id = Self::pallet_id();
		let scope_id = AfloatMarketPlaceId::<T>::get().unwrap();

		let roles_storage = <T as pallet::Config>::Rbac::get_roles_by_user(account_id.clone(), pallet_id, &scope_id);

		roles_storage.into_iter().filter_map(Self::role_id_to_afloat_role).collect()
	}

	pub fn do_delete_all_users() -> DispatchResult {
		UserInfo::<T>::iter_keys().try_for_each(|account_id| {
			if !Self::is_admin_or_owner(account_id.clone()) {
				let user_roles = Self::get_all_roles_for_user(account_id.clone());

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
