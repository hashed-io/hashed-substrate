use super::*;

use crate::types::*;
use frame_support::sp_io::hashing::blake2_256;
use frame_support::traits::tokens::nonfungibles::Inspect;
use frame_support::traits::UnixTime;
use frame_system::pallet_prelude::*;
use scale_info::prelude::string::String;

use pallet_gated_marketplace::types::Marketplace;
use pallet_rbac::types::*;

use frame_support::pallet_prelude::*;
use frame_support::traits::EnsureOriginWithArg;
use frame_support::PalletId;
// use frame_support::traits::OriginTrait;
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::{sp_std::vec::Vec, Permill};

impl<T: Config> Pallet<T> {
	pub fn do_initial_setup(creator: T::AccountId) -> DispatchResult {
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
				Self::deposit_event(Event::NewUser(user_address.clone()));
			},
		}

		let marketplace_id = AfloatMarketPlaceId::<T>::get().unwrap();
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
		ensure!(<UserInfo<T>>::contains_key(user_address.clone()), Error::<T>::UserNotFound);

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
		ensure!(<UserInfo<T>>::contains_key(user_address.clone()), Error::<T>::UserNotFound);
		<UserInfo<T>>::remove(user_address.clone());
		Self::deposit_event(Event::UserDeleted(user_address.clone()));
		Ok(())
	}
}
