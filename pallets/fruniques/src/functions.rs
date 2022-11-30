use super::*;

use crate::types::*;
use frame_support::sp_io::hashing::blake2_256;
use frame_support::traits::tokens::nonfungibles::Inspect;
use frame_system::pallet_prelude::*;
use scale_info::prelude::string::String;

use pallet_rbac::types::*;

use frame_support::pallet_prelude::*;
use frame_support::traits::EnsureOriginWithArg;
use frame_system::RawOrigin;

use sp_runtime::traits::AccountIdConversion;
use sp_runtime::{sp_std::vec::Vec, Permill};

impl<T: Config> Pallet<T> {
	pub fn u32_to_instance_id(input: u32) -> T::ItemId
	where
		<T as pallet_uniques::Config>::ItemId: From<u32>,
	{
		T::ItemId::from(input)
	}

	pub fn u32_to_class_id(input: u32) -> T::CollectionId
	where
		<T as pallet_uniques::Config>::CollectionId: From<u32>,
	{
		T::CollectionId::from(input)
	}

	pub fn bytes_to_u32(input: Vec<u8>) -> u32 {
		u32::from_ne_bytes(input.try_into().unwrap())
	}

	pub fn percent_to_permill(input: u8) -> Permill {
		Permill::from_percent(input as u32)
	}

	pub fn bytes_to_string(input: Vec<u8>) -> String {
		let mut s = String::default();
		for x in input {
			//let c: char = x.into();
			s.push(x as char);
		}
		s
	}

	pub fn account_id_to_lookup_source(
		account_id: &T::AccountId,
	) -> <T::Lookup as sp_runtime::traits::StaticLookup>::Source {
		<T::Lookup as sp_runtime::traits::StaticLookup>::unlookup(account_id.clone())
	}

	/// Helper function for printing purposes
	pub fn get_nft_attribute(
		class_id: &T::CollectionId,
		instance_id: &T::ItemId,
		key: &[u8],
	) -> AttributeValue<T> {
		if let Some(a) = pallet_uniques::Pallet::<T>::attribute(class_id, instance_id, key) {
			return BoundedVec::<u8, T::ValueLimit>::try_from(a)
				.expect("Error on converting the attribute to BoundedVec");
		}
		BoundedVec::<u8, T::ValueLimit>::default()
	}

	pub fn admin_of(class_id: &T::CollectionId, instance_id: &T::ItemId) -> Option<T::AccountId> {
		pallet_uniques::Pallet::<T>::owner(*class_id, *instance_id)
	}

	pub fn collection_exists(class_id: &T::CollectionId) -> bool {
		if let Some(_owner) = pallet_uniques::Pallet::<T>::collection_owner(*class_id) {
			return true;
		}
		false
	}

	pub fn item_exists(class_id: &T::CollectionId, instance_id: &T::ItemId) -> bool {
		if let Some(_owner) = pallet_uniques::Pallet::<T>::owner(*class_id, *instance_id) {
			return true;
		}
		false
	}

	pub fn do_initial_setup() -> DispatchResult {
		let pallet: IdOrVec = Self::pallet_id();

		let owner_role_ids =
			T::Rbac::create_and_set_roles(pallet.clone(), FruniqueRole::get_owner_roles())?;

		for owner_role in owner_role_ids {
			T::Rbac::create_and_set_permissions(
				pallet.clone(),
				owner_role,
				Permission::owner_permissions(),
			)?;
		}

		let admin_role_ids =
			T::Rbac::create_and_set_roles(pallet.clone(), FruniqueRole::get_admin_roles())?;

		for admin_role in admin_role_ids {
			T::Rbac::create_and_set_permissions(
				pallet.clone(),
				admin_role,
				Permission::admin_permissions(),
			)?;
		}

		let collaborator_role_ids =
			T::Rbac::create_and_set_roles(pallet.clone(), FruniqueRole::get_collaborator_roles())?;

		for collaborator_role in collaborator_role_ids {
			T::Rbac::create_and_set_permissions(
				pallet.clone(),
				collaborator_role,
				Permission::collaborator_permissions(),
			)?;
		}

		let collector_role_ids =
			T::Rbac::create_and_set_roles(pallet.clone(), FruniqueRole::get_collector_roles())?;

		for collector_role in collector_role_ids {
			T::Rbac::create_and_set_permissions(
				pallet.clone(),
				collector_role,
				Permission::collector_permissions(),
			)?;
		}

		let holder_role_ids =
			T::Rbac::create_and_set_roles(pallet.clone(), FruniqueRole::get_holder_roles())?;

		for holder_role in holder_role_ids {
			T::Rbac::create_and_set_permissions(
				pallet.clone(),
				holder_role,
				Permission::holder_permissions(),
			)?;
		}

		Ok(())
	}

	pub fn set_attribute(
		origin: OriginFor<T>,
		class_id: &T::CollectionId,
		instance_id: T::ItemId,
		key: AttributeKey<T>,
		value: AttributeValue<T>,
	) -> DispatchResult {
		pallet_uniques::Pallet::<T>::set_attribute(
			origin,
			*class_id,
			Some(instance_id),
			key,
			value,
		)?;
		Ok(())
	}

	pub fn mint(
		origin: OriginFor<T>,
		class_id: &T::CollectionId,
		instance_id: T::ItemId,
		owner: <T::Lookup as sp_runtime::traits::StaticLookup>::Source,
	) -> DispatchResult {
		pallet_uniques::Pallet::<T>::mint(origin, *class_id, instance_id, owner)?;
		Ok(())
	}

	pub fn freeze(
		origin: OriginFor<T>,
		class_id: &T::CollectionId,
		instance_id: T::ItemId,
	) -> DispatchResult {
		pallet_uniques::Pallet::<T>::freeze(origin, *class_id, instance_id)?;
		Ok(())
	}

	pub fn burn(
		origin: OriginFor<T>,
		class_id: &T::CollectionId,
		instance_id: T::ItemId,
	) -> DispatchResult {
		let admin = Self::admin_of(class_id, &instance_id);
		ensure!(admin.is_some(), "Instance is not owned by anyone");

		pallet_uniques::Pallet::<T>::burn(
			origin,
			*class_id,
			instance_id,
			Some(Self::account_id_to_lookup_source(&admin.unwrap())),
		)?;
		Ok(())
	}

	/// Helper function to create a new collection
	/// Creates a collection and updates its metadata if needed.
	pub fn do_create_collection(
		origin: OriginFor<T>,
		class_id: T::CollectionId,
		metadata: CollectionDescription<T>,
		admin: T::AccountId,
	) -> DispatchResult {
		let owner = T::CreateOrigin::ensure_origin(origin.clone(), &class_id)?;

		let scope_id = class_id.using_encoded(blake2_256);
		T::Rbac::create_scope(Self::pallet_id(), scope_id)?;

		Self::insert_auth_in_frunique_collection(owner.clone(), class_id, FruniqueRole::Owner)?;

		pallet_uniques::Pallet::<T>::do_create_collection(
			class_id,
			Self::account_id(),
			Self::account_id(),
			T::CollectionDeposit::get(),
			false,
			pallet_uniques::Event::Created { collection: class_id, creator: admin, owner },
		)?;

		pallet_uniques::Pallet::<T>::set_collection_metadata(origin.clone(), class_id, metadata, false)?;

		pallet_uniques::Pallet::<T>::set_team(
			origin,
			class_id,
			Self::account_id_to_lookup_source(&Self::account_id()),
			Self::account_id_to_lookup_source(&Self::account_id()),
			Self::account_id_to_lookup_source(&Self::account_id()),
		)?;

		Ok(())
	}

	// TODO: add a function to transfer an instance
	pub fn do_create(
		origin: OriginFor<T>,
		class_id: T::CollectionId,
		instance_id: T::ItemId,
		numeric_value: Option<Permill>,
		admin: <T::Lookup as sp_runtime::traits::StaticLookup>::Source,
	) -> DispatchResult {

		pallet_uniques::Pallet::<T>::create(RawOrigin::Signed(Self::account_id()).into(), class_id, admin.clone())?;

		Self::mint(origin.clone(), &class_id, instance_id, admin)?;

		if let Some(n) = numeric_value {
			let num_value_key = BoundedVec::<u8, T::KeyLimit>::try_from(r#"num_value"#.encode())
				.expect("Error on encoding the numeric value key to BoundedVec");
			let num_value = BoundedVec::<u8, T::ValueLimit>::try_from(n.encode())
				.expect("Error on encoding the numeric value to BoundedVec");
			pallet_uniques::Pallet::<T>::set_attribute(
				origin,
				class_id,
				Some(instance_id),
				num_value_key,
				num_value,
			)?;
		}

		Ok(())
	}

	pub fn do_spawn(
		origin: OriginFor<T>,
		collection: T::CollectionId,
		item: T::ItemId,
		owner: T::AccountId,
		metadata: CollectionDescription<T>,
		attributes: Option<Attributes<T>>,
	) -> DispatchResult {
		ensure!(Self::collection_exists(&collection), <Error<T>>::CollectionNotFound);
		let user: T::AccountId = ensure_signed(origin.clone())?;
		Self::is_authorized(user, collection, Permission::Mint)?;

		// pallet_uniques::Pallet::<T>::do_mint(collection, item, owner, |_| Ok(()))?;
		pallet_uniques::Pallet::<T>::do_mint(collection, item, Self::account_id(), |_| Ok(()))?;

		pallet_uniques::Pallet::<T>::do_transfer(
			collection,
			item,
			owner,
			|_collection_details, _details| Ok(()),
		)?;

		pallet_uniques::Pallet::<T>::set_metadata(
			RawOrigin::Signed(Self::account_id()).into(),
			collection,
			item,
			metadata,
			false,
		)?;

		if let Some(attributes) = attributes {
			for (key, value) in attributes {
				pallet_uniques::Pallet::<T>::set_attribute(
					RawOrigin::Signed(Self::account_id()).into(),
					collection,
					Some(item),
					key,
					value,
				)?;
			}
		}

		Ok(())
	}

	/// Helper functions to sign as the pallet

	/// The account ID of the treasury pot.
	///
	/// This actually does computation. If you need to keep using it, then make sure you cache the
	/// value and only call this once.
	pub fn account_id() -> T::AccountId {
		T::PalletId.into_account_truncating()
	}

	/// Convert into an account ID. This is infallible.
	// fn into_account(&self) -> T::AccountId {
	// 	self.into_sub_account(&())
	// }

	// fn into_sub_account<S: Encode>(&self, sub: S) -> T {
	// 	(Id::TYPE_ID, self, sub)
	// 		.using_encoded(|b| T::decode(&mut TrailingZeroInput(b)))
	// 		.expect("`AccountId` type is never greater than 32 bytes; qed")
	// }

	/// Helper functions to interact with the RBAC module
	pub fn pallet_id() -> IdOrVec {
		IdOrVec::Vec(Self::module_name().as_bytes().to_vec())
	}

	pub fn insert_auth_in_frunique_collection(
		user: T::AccountId,
		class_id: T::CollectionId,
		role: FruniqueRole,
	) -> DispatchResult {
		T::Rbac::assign_role_to_user(
			user,
			Self::pallet_id(),
			&class_id.using_encoded(blake2_256),
			role.id(),
		)?;

		Ok(())
	}

	fn is_authorized(
		user: T::AccountId,
		collection_id: T::CollectionId,
		permission: Permission,
	) -> DispatchResult {
		let scope_id = collection_id.using_encoded(blake2_256);
		<T as pallet::Config>::Rbac::is_authorized(
			user,
			Self::pallet_id(),
			&scope_id,
			&permission.id(),
		)
	}
}
