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
	/// pallet, which is used to enroll the user in the Afloat marketplace.
	pub fn do_create_user(
		actor: T::AccountId,
		user_address: T::AccountId,
		args: SignUpArgs,
	) -> DispatchResult {
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

	pub fn do_edit_user(
		actor: T::AccountId,
		user_address: T::AccountId,
		args: UpdateUserArgs,
	) -> DispatchResult {
		Ok(())
	}

	pub fn do_delete_user(actor: T::AccountId, user_address: T::AccountId) -> DispatchResult {
		Ok(())
	}
}
