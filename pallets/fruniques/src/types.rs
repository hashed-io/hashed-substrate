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
