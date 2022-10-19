//! Defines the types required by the fruniques pallet
use sp_runtime::{sp_std::vec::Vec};
use frame_support::pallet_prelude::*;

pub type AttributeKey<T> = BoundedVec<u8, <T as pallet_uniques::Config>::KeyLimit>;
pub type AttributeValue<T> = BoundedVec<u8, <T as pallet_uniques::Config>::ValueLimit>;
pub type Attributes<T> = Vec<(AttributeKey<T>, AttributeValue<T>)>;

pub type StringLimit<T> = BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>;

pub type CollectionId = u32;
pub type ItemId = u32;

pub type HierarchicalInfo = (ItemId, bool);

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
pub struct FruniqueChild {
	pub child_id: ItemId,
	pub collection_id: CollectionId,
	pub is_hierarchical: bool,
	pub weight: u32,
}

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct FruniqueInheritance<T: pallet_uniques::Config> {
	pub parent: Option<(CollectionId, ItemId)>,
	pub children: Vec<(CollectionId, ItemId)>,
}
