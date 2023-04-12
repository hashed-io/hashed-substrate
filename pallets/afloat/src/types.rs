use super::*;
use frame_support::pallet_prelude::*;

use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec;

pub type ShortString = BoundedVec<u8, ConstU32<32>>;
pub type LongString = BoundedVec<u8, ConstU32<32>>;
pub type Date = u64;

#[derive(Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct User<T: Config> {
	pub first_name: BoundedVec<u8, ConstU32<32>>,
	pub last_name: BoundedVec<u8, ConstU32<32>>,
	pub email: BoundedVec<u8, ConstU32<32>>,
	pub lang_key: BoundedVec<u8, ConstU32<32>>,
	pub created_by: Option<T::AccountId>,
	pub created_date: Option<Date>,
	pub last_modified_by: Option<T::AccountId>,
	pub last_modified_date: Option<Date>,
	pub phone: Option<BoundedVec<u8, ConstU32<32>>>,
	pub credits_needed: u32,
	pub cpa_id: BoundedVec<u8, ConstU32<32>>,
	pub tax_authority_id: u32,
	pub lock_expiration_date: Option<Date>,
}

impl<T: Config> User<T> {
	pub fn new(
		first_name: BoundedVec<u8, ConstU32<32>>,
		last_name: BoundedVec<u8, ConstU32<32>>,
		email: BoundedVec<u8, ConstU32<32>>,
		lang_key: BoundedVec<u8, ConstU32<32>>,
		created_by: Option<T::AccountId>,
		created_date: Option<Date>,
		last_modified_by: Option<T::AccountId>,
		last_modified_date: Option<Date>,
		phone: Option<BoundedVec<u8, ConstU32<32>>>,
		credits_needed: u32,
		cpa_id: BoundedVec<u8, ConstU32<32>>,
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

impl Default for User<T> {
	fn default() -> Self {
		Self {
			first_name: BoundedVec::default(),
			last_name: BoundedVec::default(),
			email: BoundedVec::default(),
			lang_key: BoundedVec::default(),
			created_by: None,
			created_date: None,
			last_modified_by: None,
			last_modified_date: None,
			phone: None,
			credits_needed: 0,
			cpa_id: BoundedVec::default(),
			tax_authority_id: 0,
			lock_expiration_date: None,
		}
	}
}
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub enum SignUpArgs {
	BuyerOrSeller {
		first_name: LongString,
		last_name: LongString,
		email: LongString,
		state: u32,
	},
	CPA {
		first_name: LongString,
		last_name: LongString,
		email: LongString,
		license_number: LongString,
		state: u32,
	},
}
