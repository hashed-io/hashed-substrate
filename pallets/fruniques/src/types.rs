//! Defines the types required by the fruniques pallet
use super::*;
use frame_support::pallet_prelude::*;

use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::Permill;
use sp_runtime::sp_std::vec::Vec;

pub type AttributeKey<T> = BoundedVec<u8, <T as pallet_uniques::Config>::KeyLimit>;
pub type AttributeValue<T> = BoundedVec<u8, <T as pallet_uniques::Config>::ValueLimit>;
pub type Attributes<T> = Vec<(AttributeKey<T>, AttributeValue<T>)>;

// pub type CollectionDescription = [u8; 32];
pub type StringLimit<T> = BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>;

pub type CollectionId = u32;
pub type ItemId = u32;

pub type CollectionDescription<T> = StringLimit<T>;
// (ParentId, Hierarchical, Percentage)
pub type HierarchicalInfo = (ItemId, bool, u8);

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
pub struct ChildInfo {
	pub collection_id: CollectionId,
	pub child_id: ItemId,
	pub is_hierarchical: bool,
	pub weight: Permill,
}

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct FruniqueInheritance<T: Config> {
	pub parent: (CollectionId, ItemId),
	pub children: BoundedVec<ChildInfo, T::ChildMaxLen>,
}

#[derive(
	Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy,
)]
pub enum FruniqueRole {
	Owner,
	Admin,
	Collaborator,
	Collector,
}

impl Default for FruniqueRole {
	fn default() -> Self {
		FruniqueRole::Collector
	}
}

impl FruniqueRole {
	pub fn to_vec(self) -> Vec<u8> {
		match self {
			Self::Owner => "Owner".as_bytes().to_vec(),
			Self::Admin => "Admin".as_bytes().to_vec(),
			Self::Collaborator => "Collaborator".as_bytes().to_vec(),
			Self::Collector => "Collector".as_bytes().to_vec(),
		}
	}

	pub fn id(&self) -> [u8; 32] {
		self.to_vec().using_encoded(blake2_256)
	}

	pub fn enum_to_vec() -> Vec<Vec<u8>> {
		use crate::types::FruniqueRole::*;
		[Owner.to_vec(), Admin.to_vec(), Collaborator.to_vec(), Collector.to_vec()].to_vec()
	}
}

/// Extrinsics which require previous authorization to call them
#[derive(
	Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy,
)]
pub enum Permission {
	/// No authorization required
	None,
	/// Authorization required
	Required,
	/// Authorization required and must be approved by the owner
	Mint,
	/// Authorization required and must be approved by a holder / collector
	Transfer,
	/// Allow a user to collaborate on a collection
	Collaborate,
}

impl Permission {
	pub fn to_vec(self) -> Vec<u8> {
		match self {
			Self::None => "None".as_bytes().to_vec(),
			Self::Required => "Required".as_bytes().to_vec(),
			Self::Mint => "Mint".as_bytes().to_vec(),
			Self::Transfer => "Transfer".as_bytes().to_vec(),
			Self::Collaborate => "Collaborate".as_bytes().to_vec(),
		}
	}

	pub fn id(&self) -> [u8; 32] {
		self.to_vec().using_encoded(blake2_256)
	}

	pub fn owner_permissions() -> Vec<Vec<u8>> {
		use crate::types::Permission::*;
		[None.to_vec(), Required.to_vec(), Mint.to_vec(), Transfer.to_vec()].to_vec()
	}

	pub fn admin_permissions() -> Vec<Vec<u8>> {
		use crate::types::Permission::*;
		let mut admin_permissions = [
			None.to_vec(),
			Required.to_vec(),
			Mint.to_vec(),
			Transfer.to_vec(),
			Collaborate.to_vec(),
		]
		.to_vec();
		admin_permissions.append(&mut Permission::holder_permissions());
		admin_permissions
	}

	pub fn collaborator_permissions() -> Vec<Vec<u8>> {
		use crate::types::Permission::*;
		[Mint.to_vec()].to_vec()
	}

	pub fn holder_permissions() -> Vec<Vec<u8>> {
		use crate::types::Permission::*;
		[None.to_vec(), Required.to_vec(), Transfer.to_vec()].to_vec()
	}
}
