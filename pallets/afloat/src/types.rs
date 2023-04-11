#[derive(Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct User<T: Config> {
	pub first_name: BoundedVec<u8, ConstU32<32>>,
	pub last_name: BoundedVec<u8, ConstU32<32>>,
	pub email: BoundedVec<u8, ConstU32<32>>,
	pub image_url: BoundedVec<u8, ConstU32<32>>,
	pub activated: bool,
	pub lang_key: BoundedVec<u8, ConstU32<32>>,
	pub activation_key: BoundedVec<u8, ConstU32<32>>,
	pub reset_key: BoundedVec<u8, ConstU32<32>>,
	pub created_by: Option<T::AccountId>,
	pub created_date: Option<T::Moment>,
	pub reset_date: Option<T::Moment>,
	pub last_modified_by: Option<T::AccountId>,
	pub last_modified_date: Option<T::Moment>,
	pub phone: BoundedVec<u8, ConstU32<32>>,
	pub credits_needed: u32,
	pub cpa_id: BoundedVec<u8, ConstU32<32>>,
	pub tax_authority_id: BoundedVec<u8, ConstU32<32>>,
	pub lock_expiration_date: Option<T::Moment>,
}
