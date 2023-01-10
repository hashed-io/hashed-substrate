use super::*;

use crate::types::*;
use frame_support::sp_io::hashing::blake2_256;
use frame_support::traits::tokens::nonfungibles::Inspect;
use frame_system::pallet_prelude::*;
use scale_info::prelude::string::String;

use pallet_rbac::types::*;

use frame_support::pallet_prelude::*;
use frame_support::traits::EnsureOriginWithArg;
use frame_support::PalletId;
// use frame_support::traits::OriginTrait;
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::{sp_std::vec::Vec, Permill};
// use sp_runtime::traits::StaticLookup;

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

	pub fn percent_to_permill(input: u32) -> Permill {
		Permill::from_percent(input)
	}

	pub fn permill_to_percent(input: Permill) -> u32 {
		input.deconstruct() as u32
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

	pub fn instance_exists(class_id: &T::CollectionId, instance_id: &T::ItemId) -> bool {
		if let Some(_owner) = pallet_uniques::Pallet::<T>::owner(*class_id, *instance_id) {
			return true;
		}
		false
	}

	// helper to initialize the roles for the RBAC module
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

	// Helper function to set an attribute to a given NFT
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

	// Helper function to mint a new NFT
	pub fn do_mint(
		collection: T::CollectionId,
		owner: T::AccountId,
		metadata: CollectionDescription<T>,
		attributes: Option<Attributes<T>>,
	) -> DispatchResult
	where
		<T as pallet_uniques::Config>::ItemId: From<u32>,
	{
		let nex_item: ItemId = <NextFrunique<T>>::try_get(collection).unwrap_or(0);
		<NextFrunique<T>>::insert(collection, nex_item + 1);

		let item = Self::u32_to_instance_id(nex_item);
		pallet_uniques::Pallet::<T>::do_mint(collection, item, owner, |_| Ok(()))?;

		pallet_uniques::Pallet::<T>::set_metadata(
			frame_system::RawOrigin::Root.into(),
			collection,
			item.clone(),
			metadata,
			false,
		)?;

		if let Some(attributes) = attributes {
			for (key, value) in attributes {
				pallet_uniques::Pallet::<T>::set_attribute(
					frame_system::RawOrigin::Root.into(),
					collection,
					Some(item),
					key,
					value,
				)?;
			}
		}

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
		metadata: CollectionDescription<T>,
		admin: T::AccountId,
	) -> DispatchResult
	where
		<T as pallet_uniques::Config>::CollectionId: From<u32>,
	{
		let next_collection: u32 = Self::next_collection();
		let class_id = Self::u32_to_class_id(next_collection);

		let owner = T::CreateOrigin::ensure_origin(origin.clone(), &class_id)?;

		let scope_id = class_id.using_encoded(blake2_256);
		T::Rbac::create_scope(Self::pallet_id(), scope_id)?;

		Self::insert_auth_in_frunique_collection(owner.clone(), class_id, FruniqueRole::Owner)?;

		pallet_uniques::Pallet::<T>::do_create_collection(
			class_id,
			owner.clone(),
			admin.clone(),
			T::CollectionDeposit::get(),
			false,
			pallet_uniques::Event::Created { collection: class_id, creator: admin.clone(), owner },
		)?;

		pallet_uniques::Pallet::<T>::set_collection_metadata(
			origin,
			class_id.clone(),
			metadata,
			false,
		)?;

		<NextCollection<T>>::put(Self::next_collection() + 1);

		Ok(())
	}

	// Create a new NFT for a given collection
	pub fn do_spawn(
		collection: T::CollectionId,
		owner: T::AccountId,
		metadata: CollectionDescription<T>,
		attributes: Option<Attributes<T>>,
		parent_info: Option<ParentInfoCall<T>>,
	) -> DispatchResult
	where
		<T as pallet_uniques::Config>::ItemId: From<u32>,
	{
		ensure!(Self::collection_exists(&collection), Error::<T>::CollectionNotFound);

		let nex_item: ItemId = <NextFrunique<T>>::try_get(collection).unwrap_or(0);
		let item = Self::u32_to_instance_id(nex_item);

		Self::do_mint(collection, owner, metadata, attributes)?;

		if let Some(ref parent_info) = parent_info {
			return Self::do_nft_division(collection, item, parent_info);
		}

		let frunique_data =
			FruniqueData { weight: Self::percent_to_permill(100), parent: None, children: None };
		<FruniqueInfo<T>>::insert(collection, item, frunique_data);

		Ok(())
	}

	// Takes cares of the division of the NFT
	pub fn do_nft_division(
		collection: T::CollectionId,
		item: T::ItemId,
		parent_info: &ParentInfoCall<T>,
	) -> DispatchResult
	where
		<T as pallet_uniques::Config>::ItemId: From<u32>,
	{
		ensure!(
			Self::collection_exists(&parent_info.collection_id),
			Error::<T>::CollectionNotFound
		);
		ensure!(
			Self::instance_exists(&parent_info.collection_id, &parent_info.parent_id),
			Error::<T>::FruniqueNotFound
		);

		let frunique_parent =
			<FruniqueInfo<T>>::try_get(&parent_info.collection_id, &parent_info.parent_id).unwrap();

		let child_percentage = Permill::from_percent(parent_info.parent_percentage) * frunique_parent.weight;


		let parent_data = ParentInfo {
			collection_id: parent_info.collection_id,
			parent_id: parent_info.parent_id,
			parent_weight: child_percentage,
			is_hierarchical: parent_info.is_hierarchical,
		};

		let frunique_data = FruniqueData {
			weight: Self::percent_to_permill(100),
			parent: Some(parent_data),
			children: None,
		};

		<FruniqueInfo<T>>::insert(collection, item, frunique_data);

		let frunique_child = ChildInfo {
			collection_id: collection,
			child_id: item,
			weight_inherited: child_percentage,
			is_hierarchical: parent_info.is_hierarchical,
		};

		<FruniqueInfo<T>>::try_mutate::<_, _, _, DispatchError, _>(
			parent_info.collection_id,
			parent_info.parent_id,
			|frunique_data| -> DispatchResult {
				let frunique = frunique_data.as_mut().ok_or(Error::<T>::FruniqueNotFound)?;
				match frunique.children.as_mut() {
					Some(children) => children
						.try_push(frunique_child)
						.map_err(|_| Error::<T>::MaxNumberOfChildrenReached)?,
					None => {
						let child = frunique.children.get_or_insert(Children::default());
						child
							.try_push(frunique_child)
							.map_err(|_| Error::<T>::MaxNumberOfChildrenReached)?;
					},
				}
				frunique.weight = frunique.weight - child_percentage;
				Ok(())
			},
		)?;

		Ok(())
	}

	/// Helper functions to interact with the RBAC module
	pub fn pallet_id() -> IdOrVec {
		IdOrVec::Vec(Self::module_name().as_bytes().to_vec())
	}

	// Helper function to get the pallet account as a AccountId
	pub fn pallet_account() -> T::AccountId {
		let pallet_name = Self::module_name().as_bytes().to_vec();
		let pallet_account_name: [u8; 8] =
			pallet_name.as_slice().try_into().unwrap_or(*b"frunique");
		let pallet_id = PalletId(pallet_account_name);
		pallet_id.try_into_account().unwrap()
	}

	// Helper add RBAC roles for collections
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

	// Helper function to check if a user has a specific role in a collection
	pub fn is_authorized(
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
