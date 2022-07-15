use super::*;

use frame_support::traits::tokens::nonfungibles::Inspect;
use frame_system::pallet_prelude::*;
use scale_info::prelude::string::String;
// use crate::types::*;
use frame_support::pallet_prelude::*;
use sp_runtime::sp_std::vec::Vec;

impl<T: Config> Pallet<T> {
	pub fn u32_to_instance_id(input: u32) -> T::ItemId
	where
		<T as pallet_uniques::Config>::ItemId: From<u32>,
	{
		T::ItemId::from(input)
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
	/// Helper function for printing purposes
	pub fn get_nft_attribute(
		class_id: &T::CollectionId,
		instance_id: &T::ItemId,
		key: &Vec<u8>,
	) -> BoundedVec<u8, T::ValueLimit> {
		if let Some(a) = pallet_uniques::Pallet::<T>::attribute(class_id, instance_id, key) {
			return BoundedVec::<u8, T::ValueLimit>::try_from(a)
				.expect("Error on converting the attribute to BoundedVec");
		}
		BoundedVec::<u8, T::ValueLimit>::default()
	}

	pub fn admin_of(class_id: &T::CollectionId, instance_id: &T::ItemId) -> Option<T::AccountId> {
		pallet_uniques::Pallet::<T>::owner(*class_id, *instance_id)
	}

	pub fn set_attribute(
		origin: OriginFor<T>,
		class_id: &T::CollectionId,
		instance_id: T::ItemId,
		key: BoundedVec<u8, T::KeyLimit>,
		value: BoundedVec<u8, T::ValueLimit>,
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
}
