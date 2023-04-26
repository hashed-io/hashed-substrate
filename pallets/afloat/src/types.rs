use super::*;
use frame_support::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec;

pub type ShortString = BoundedVec<u8, ConstU32<35>>;
pub type LongString = BoundedVec<u8, ConstU32<255>>;
pub type Date = u64;
pub type CollectionId = u32;

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen, PartialEq)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct User<T: Config> {
	pub first_name: ShortString,
	pub last_name: ShortString,
	pub email: LongString,
	pub lang_key: ShortString,
	pub created_by: Option<T::AccountId>,
	pub created_date: Option<Date>,
	pub last_modified_by: Option<T::AccountId>,
	pub last_modified_date: Option<Date>,
	pub phone: Option<ShortString>,
	pub credits_needed: u32,
	pub cpa_id: ShortString,
	pub tax_authority_id: u32,
	pub lock_expiration_date: Option<Date>,
}

impl<T: Config> User<T> {
	pub fn new(
		first_name: ShortString,
		last_name: ShortString,
		email: LongString,
		lang_key: ShortString,
		created_by: Option<T::AccountId>,
		created_date: Option<Date>,
		last_modified_by: Option<T::AccountId>,
		last_modified_date: Option<Date>,
		phone: Option<ShortString>,
		credits_needed: u32,
		cpa_id: ShortString,
		tax_authority_id: u32,
		lock_expiration_date: Option<Date>,
	) -> Self {
		Self {
			first_name,
			last_name,
			email,
			lang_key,
			created_by,
			created_date,
			last_modified_by,
			last_modified_date,
			phone,
			credits_needed,
			cpa_id,
			tax_authority_id,
			lock_expiration_date,
		}
	}
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub enum UpdateUserArgs {
	Edit {
		first_name: Option<ShortString>,
		last_name: Option<ShortString>,
		email: Option<LongString>,
		lang_key: Option<ShortString>,
		phone: Option<Option<ShortString>>,
		credits_needed: Option<u32>,
		cpa_id: Option<ShortString>,
		state: Option<u32>,
	},
	Delete,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub enum SignUpArgs {
	BuyerOrSeller {
		first_name: ShortString,
		last_name: ShortString,
		email: LongString,
		state: u32,
	},
	CPA {
		first_name: ShortString,
		last_name: ShortString,
		email: LongString,
		license_number: ShortString,
		state: u32,
	},
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy,)]
pub enum AfloatRole {
	Owner,
	Admin,
	BuyerOrSeller,
	CPA,
}

impl Default for AfloatRole {
	fn default() -> Self {
		AfloatRole::BuyerOrSeller
	}
}

impl AfloatRole {
	pub fn to_vec(self) -> Vec<u8> {
		match self {
			Self::Owner => "Owner".as_bytes().to_vec(),
			Self::Admin => "Admin".as_bytes().to_vec(),
			Self::BuyerOrSeller => "BuyerOrSeller".as_bytes().to_vec(),
			Self::CPA => "CPA".as_bytes().to_vec(),
		}
	}

	pub fn id(&self) -> [u8; 32] {
		self.to_vec().using_encoded(blake2_256)
	}

	pub fn enum_to_vec() -> Vec<Vec<u8>> {
		use crate::types::AfloatRole::*;
		[
			Owner.to_vec(),
			Admin.to_vec(),
			BuyerOrSeller.to_vec(),
			CPA.to_vec(),
		]
		.to_vec()
	}
}

#[derive(
	Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy,
)]
pub enum Permission {
	CreateUser,
	EditUser,
	DeleteUser,
}

impl Permission {
	pub fn to_vec(self) -> Vec<u8> {
		match self {
			Self::CreateUser => "CreateUser".as_bytes().to_vec(),
			Self::EditUser => "EditUser".as_bytes().to_vec(),
			Self::DeleteUser => "DeleteUser".as_bytes().to_vec(),
		}
	}

	pub fn id(&self) -> [u8; 32] {
		self.to_vec().using_encoded(blake2_256)
	}

	pub fn admin_permissions() -> Vec<Vec<u8>> {
		use crate::types::Permission::*;
		let admin_permissions = [
			CreateUser.to_vec(),
			EditUser.to_vec(),
			DeleteUser.to_vec(),
		]
		.to_vec();
		admin_permissions
	}
}
