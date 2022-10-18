use super::*;

use frame_system::pallet_prelude::*;
use crate::types::*;

use frame_support::traits::tokens::nonfungibles::Inspect;
use scale_info::prelude::string::String;

use frame_support::pallet_prelude::*;
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
		key: &Vec<u8>,
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
		metadata: Option<StringLimit<T>>,
		admin: <T::Lookup as sp_runtime::traits::StaticLookup>::Source,
	) -> DispatchResult {
		pallet_uniques::Pallet::<T>::create(
			origin.clone(),
			class_id,
			admin,
		)?;

		if let Some(metadata) = metadata {
			pallet_uniques::Pallet::<T>::set_collection_metadata(
			origin,
			class_id,
			metadata,
			false
			)?;
		}

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
		pallet_uniques::Pallet::<T>::create(origin.clone(), class_id.clone(), admin.clone())?;

		Self::mint(origin.clone(), &class_id, instance_id.clone(), admin.clone())?;

		if let Some(n) = numeric_value {
			let num_value_key = BoundedVec::<u8, T::KeyLimit>::try_from(r#"num_value"#.encode())
				.expect("Error on encoding the numeric value key to BoundedVec");
			let num_value = BoundedVec::<u8, T::ValueLimit>::try_from(n.encode())
				.expect("Error on encoding the numeric value to BoundedVec");
			pallet_uniques::Pallet::<T>::set_attribute(
				origin.clone(),
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
		owner: <T::Lookup as sp_runtime::traits::StaticLookup>::Source,
		_numeric_value: Option<Permill>,
		attributes: Option<Vec<(BoundedVec<u8, T::KeyLimit>, BoundedVec<u8, T::ValueLimit>)>>,
	) -> DispatchResult {

		ensure!(Self::collection_exists(&collection),  <Error<T>>::CollectionNotFound);
		pallet_uniques::Pallet::<T>::mint(
			origin.clone(),
			collection,
			item,
			owner
		)?;

		if let Some(attributes) = attributes {
			for (key, value) in attributes {
				pallet_uniques::Pallet::<T>::set_attribute(
					origin.clone(),
					collection,
					Some(item),
					key,
					value,
				)?;
			}
		}

		Ok(())
	}
}
