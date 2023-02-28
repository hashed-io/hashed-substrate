//! Defines the types required by the fruniques pallet
use super::*;
use frame_support::pallet_prelude::*;

use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec;
use sp_runtime::Permill;

pub type AttributeKey<T> = BoundedVec<u8, <T as pallet_uniques::Config>::KeyLimit>;
pub type AttributeValue<T> = BoundedVec<u8, <T as pallet_uniques::Config>::ValueLimit>;
pub type Attributes<T> = Vec<(AttributeKey<T>, AttributeValue<T>)>;
pub type Children<T> = BoundedVec<ChildInfo<T>, <T as Config>::ChildMaxLen>;

pub type StringLimit<T> = BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>;

pub type CollectionId = u32;
pub type ItemId = u32;

pub type CollectionDescription<T> = StringLimit<T>;

pub type ParentId = ItemId;
pub type Hierarchical = bool;
pub type Percentage = u16;


#[derive(Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct ChildInfo<T: Config> {
	pub collection_id: T::CollectionId,
	pub child_id: T::ItemId,
	pub weight_inherited: Permill,
	pub is_hierarchical: bool,
}

#[derive(Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen, Copy)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct ParentInfo<T: Config> {
	pub collection_id: T::CollectionId,
	pub parent_id: T::ItemId,
	pub parent_weight: Permill,
	pub is_hierarchical: bool,
}

impl<T: Config> PartialEq for ParentInfo<T> {
	fn eq(&self, other: &Self) -> bool {
		self.collection_id == other.collection_id
			&& self.parent_id == other.parent_id
			&& self.parent_weight == other.parent_weight
			&& self.is_hierarchical == other.is_hierarchical
	}
}

impl<T: Config> Clone for ParentInfo<T> {
	fn clone(&self) -> Self {
		Self {
			collection_id: self.collection_id.clone(),
			parent_id: self.parent_id.clone(),
			parent_weight: self.parent_weight.clone(),
			is_hierarchical: self.is_hierarchical.clone(),
		}
	}
}

#[derive(Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen, Copy)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct ParentInfoCall<T: Config> {
	pub collection_id: T::CollectionId,
	pub parent_id: T::ItemId,
	pub parent_percentage: u32,
	pub is_hierarchical: bool,
}

impl<T: Config> ParentInfoCall<T> {
	pub fn new(
		collection_id: T::CollectionId,
		parent_id: T::ItemId,
		parent_percentage: u32,
		is_hierarchical: bool,
	) -> Self {
		Self { collection_id, parent_id, parent_percentage, is_hierarchical }
	}
}

impl<T: Config> PartialEq for ParentInfoCall<T> {
	fn eq(&self, other: &Self) -> bool {
		self.collection_id == other.collection_id
			&& self.parent_id == other.parent_id
			&& self.parent_percentage == other.parent_percentage
			&& self.is_hierarchical == other.is_hierarchical
	}
}

impl<T: Config> Clone for ParentInfoCall<T> {
	fn clone(&self) -> Self {
		Self {
			collection_id: self.collection_id.clone(),
			parent_id: self.parent_id.clone(),
			parent_percentage: self.parent_percentage.clone(),
			is_hierarchical: self.is_hierarchical.clone(),
		}
	}
}

#[derive(Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct FruniqueData<T: Config> {
	pub metadata: CollectionDescription<T>,
	pub weight: Permill,
	pub parent: Option<ParentInfo<T>>,
	pub children: Option<Children<T>>,
	pub frozen: bool,
	pub verified: bool,
	pub redeemed: bool,
	pub spawned_by: Option<T::AccountId>,
	pub verified_by: Option<T::AccountId>,
}

impl<T: Config> FruniqueData<T> {
	pub fn new(metadata: CollectionDescription<T>) -> Self {
		Self { metadata, weight: Permill::from_percent(100), parent: None, children: None, frozen: false, verified: false, redeemed: false, spawned_by: None , verified_by: None }
	}
}

#[derive(
	Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy,
)]
pub enum FruniqueRole {
	Owner,
	Admin,
	Collaborator,
	Collector,
	Holder,
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
			Self::Holder => "Holder".as_bytes().to_vec(),
		}
	}

	pub fn id(&self) -> [u8; 32] {
		self.to_vec().using_encoded(blake2_256)
	}

	pub fn get_owner_roles() -> Vec<Vec<u8>> {
		[Self::Owner.to_vec()].to_vec()
	}

	pub fn get_admin_roles() -> Vec<Vec<u8>> {
		[Self::Admin.to_vec()].to_vec()
	}

	pub fn get_collaborator_roles() -> Vec<Vec<u8>> {
		[Self::Collaborator.to_vec()].to_vec()
	}

	pub fn get_collector_roles() -> Vec<Vec<u8>> {
		[Self::Collector.to_vec()].to_vec()
	}

	pub fn get_holder_roles() -> Vec<Vec<u8>> {
		[Self::Holder.to_vec()].to_vec()
	}

	pub fn enum_to_vec() -> Vec<Vec<u8>> {
		use crate::types::FruniqueRole::*;
		[Owner.to_vec(), Admin.to_vec(), Collaborator.to_vec(), Collector.to_vec(), Holder.to_vec()]
			.to_vec()
	}
}

/// Extrinsics which require previous authorization to call them
#[derive(
	Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy,
)]
pub enum Permission {
	/// Not a permission
	None,
	/// Authorization required and must be approved by the owner
	Mint,
	/// Authorization required and must be approved by the owner or admin
	Burn,
	/// Authorization required and must be approved by a holder / collector
	Transfer,
	/// Allow a user to collaborate on a collection
	InviteCollaborator,
	/// Verify spawned NFTs
	Verify,
}

impl Permission {
	pub fn to_vec(self) -> Vec<u8> {
		match self {
			Self::None => "None".as_bytes().to_vec(),
			Self::Mint => "Mint".as_bytes().to_vec(),
			Self::Burn => "Burn".as_bytes().to_vec(),
			Self::Transfer => "Transfer".as_bytes().to_vec(),
			Self::InviteCollaborator => "InviteCollaborator".as_bytes().to_vec(),
			Self::Verify => "Verify".as_bytes().to_vec(),
		}
	}

	pub fn id(&self) -> [u8; 32] {
		self.to_vec().using_encoded(blake2_256)
	}

	pub fn get_permissions() -> Vec<Vec<u8>> {
		use crate::types::Permission::*;
		[None.to_vec(), Mint.to_vec(), Transfer.to_vec(), InviteCollaborator.to_vec()].to_vec()
	}

	pub fn owner_permissions() -> Vec<Vec<u8>> {
		use crate::types::Permission::*;
		[Mint.to_vec(), Burn.to_vec(), Transfer.to_vec(), InviteCollaborator.to_vec(), Verify.to_vec()].to_vec()
	}

	pub fn admin_permissions() -> Vec<Vec<u8>> {
		use crate::types::Permission::*;
		let mut admin_permissions =
			[Mint.to_vec(), Burn.to_vec(), InviteCollaborator.to_vec(), Verify.to_vec()].to_vec();
		admin_permissions.append(&mut Permission::holder_permissions());
		admin_permissions
	}

	pub fn collaborator_permissions() -> Vec<Vec<u8>> {
		use crate::types::Permission::*;
		[Mint.to_vec()].to_vec()
	}

	pub fn collector_permissions() -> Vec<Vec<u8>> {
		use crate::types::Permission::*;
		[None.to_vec()].to_vec()
	}

	pub fn holder_permissions() -> Vec<Vec<u8>> {
		use crate::types::Permission::*;
		[Transfer.to_vec()].to_vec()
	}
}
