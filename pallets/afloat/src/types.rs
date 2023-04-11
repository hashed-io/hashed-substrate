use super::*;
use frame_support::pallet_prelude::*;

use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec;

pub type ShortString = BoundedVec<u8, ConstU32<32>>;
pub type LongString = BoundedVec<u8, ConstU32<32>>;

#[derive(Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct User<T: Config> {
	pub first_name: BoundedVec<u8, ConstU32<32>>,
	pub last_name: BoundedVec<u8, ConstU32<32>>,
	pub email: BoundedVec<u8, ConstU32<32>>,
	pub lang_key: BoundedVec<u8, ConstU32<32>>,
	pub created_by: Option<T::AccountId>,
	pub created_date: Option<T::Moment>,
	pub last_modified_by: Option<T::AccountId>,
	pub last_modified_date: Option<T::Moment>,
	pub phone: BoundedVec<u8, ConstU32<32>>,
	pub credits_needed: u32,
	pub cpa_id: BoundedVec<u8, ConstU32<32>>,
	pub tax_authority_id: u32, //! this is a number that represents the state of the user
	pub lock_expiration_date: Option<T::Moment>,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub enum SignUpArgs {
	buyer_or_seller {
		first_name: LongString,
		last_name: LongString,
		email: LongString,
		state: u32,
	},
	cpa {
		first_name: LongString,
		last_name: LongString,
		email: LongString,
		license_number: u32,
		state: u32,
	},
}
