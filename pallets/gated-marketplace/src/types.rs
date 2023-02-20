use super::*;
use frame_support::pallet_prelude::*;
//use frame_system::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec;

pub type Fields<T> = BoundedVec<(FieldName, Cid), <T as Config>::MaxFiles>;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type FieldName = BoundedVec<u8, ConstU32<100>>;
pub type Cid = BoundedVec<u8, ConstU32<100>>;
pub type Cids<Len> = BoundedVec<Cid, Len>;
pub type CustodianFields<T> = (AccountIdOf<T>, Cids<<T as Config>::MaxFiles>);

pub type MarketplaceId = [u8; 32];
pub type ApplicationId = [u8; 32];
pub type OfferId = [u8; 32];
pub type RedemptionId = [u8; 32];

use sp_runtime::Permill;

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Marketplace<T: Config> {
	pub label: BoundedVec<u8, T::LabelMaxLen>,
	pub fee: Permill,
	pub creator: T::AccountId,
}

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct RedemptionData<T: Config> {
	pub creator: T::AccountId,
	pub redeemed_by: Option<T::AccountId>,
	pub collection_id: T::CollectionId,
	pub item_id: T::ItemId,
	pub is_redeemed: bool,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]

pub enum RedeemArgs<T: Config> {
	AskForRedemption { collection_id: T::CollectionId, item_id: T::ItemId, },
	AcceptRedemption (RedemptionId),
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub enum BlockUserArgs<T: Config> {
	BlockUser (T::AccountId),
	UnblockUser (T::AccountId),
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub enum AccountOrApplication<T: Config> {
	Account(T::AccountId),
	Application([u8; 32]),
}

#[derive(
	Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy,
)]
pub enum MarketplaceRole {
	Owner,
	Admin,
	Appraiser,
	RedemptionSpecialist,
	Participant,
}

impl Default for MarketplaceRole {
	fn default() -> Self {
		MarketplaceRole::Participant
	}
}

impl MarketplaceRole {
	pub fn to_vec(self) -> Vec<u8> {
		match self {
			Self::Owner => "Owner".as_bytes().to_vec(),
			Self::Admin => "Admin".as_bytes().to_vec(),
			Self::Appraiser => "Appraiser".as_bytes().to_vec(),
			Self::RedemptionSpecialist => "Redemption_specialist".as_bytes().to_vec(),
			Self::Participant => "Participant".as_bytes().to_vec(),
		}
	}

	pub fn id(&self) -> [u8; 32] {
		self.to_vec().using_encoded(blake2_256)
	}

	pub fn enum_to_vec() -> Vec<Vec<u8>> {
		use crate::types::MarketplaceRole::*;
		[
			Owner.to_vec(),
			Admin.to_vec(),
			Appraiser.to_vec(),
			RedemptionSpecialist.to_vec(),
			Participant.to_vec(),
		]
		.to_vec()
	}
}

/// Extrinsics which require previous authorization to call them
#[derive(
	Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy,
)]
pub enum Permission {
	Enroll,
	AddAuth,
	RemoveAuth,
	UpdateLabel,
	RemoveMarketplace,
	EnlistSellOffer,
	TakeSellOffer,
	DuplicateOffer,
	RemoveOffer,
	EnlistBuyOffer,
	TakeBuyOffer,
	AskForRedemption,
	AcceptRedemption,
	EnlistBlockedUser,
}

impl Permission {
	pub fn to_vec(self) -> Vec<u8> {
		match self {
			Self::Enroll => "Enroll".as_bytes().to_vec(),
			Self::AddAuth => "AddAuth".as_bytes().to_vec(),
			Self::RemoveAuth => "RemoveAuth".as_bytes().to_vec(),
			Self::UpdateLabel => "UpdateLabel".as_bytes().to_vec(),
			Self::RemoveMarketplace => "RemoveMarketplace".as_bytes().to_vec(),
			Self::EnlistSellOffer => "EnlistSellOffer".as_bytes().to_vec(),
			Self::TakeSellOffer => "TakeSellOffer".as_bytes().to_vec(),
			Self::DuplicateOffer => "DuplicateOffer".as_bytes().to_vec(),
			Self::RemoveOffer => "RemoveOffer".as_bytes().to_vec(),
			Self::EnlistBuyOffer => "EnlistBuyOffer".as_bytes().to_vec(),
			Self::TakeBuyOffer => "TakeBuyOffer".as_bytes().to_vec(),
			Self::AskForRedemption => "AskForRedemption".as_bytes().to_vec(),
			Self::AcceptRedemption => "AcceptRedemption".as_bytes().to_vec(),
			Self::EnlistBlockedUser => "EnlistBlockedUser".as_bytes().to_vec(),
		}
	}

	pub fn id(&self) -> [u8; 32] {
		self.to_vec().using_encoded(blake2_256)
	}

	pub fn admin_permissions() -> Vec<Vec<u8>> {
		use crate::types::Permission::*;
		let mut admin_permissions = [
			Enroll.to_vec(),
			AddAuth.to_vec(),
			RemoveAuth.to_vec(),
			UpdateLabel.to_vec(),
			RemoveMarketplace.to_vec(),
			AcceptRedemption.to_vec(),
			EnlistBlockedUser.to_vec(),
		]
		.to_vec();
		admin_permissions.append(&mut Permission::participant_permissions());
		admin_permissions
	}

	pub fn participant_permissions() -> Vec<Vec<u8>> {
		use crate::types::Permission::*;
		[
			EnlistSellOffer.to_vec(),
			TakeSellOffer.to_vec(),
			DuplicateOffer.to_vec(),
			RemoveOffer.to_vec(),
			EnlistBuyOffer.to_vec(),
			TakeBuyOffer.to_vec(),
			AskForRedemption.to_vec(),
		]
		.to_vec()
	}
}

#[derive(
	CloneNoBound,
	Encode,
	Decode,
	Eq,
	PartialEq,
	RuntimeDebugNoBound,
	Default,
	TypeInfo,
	MaxEncodedLen,
)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Application<T: Config> {
	pub status: ApplicationStatus,
	pub fields: BoundedVec<ApplicationField, T::MaxFiles>,
	pub feedback: BoundedVec<u8, T::MaxFeedbackLen>,
}

#[derive(
	Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy,
)]
pub enum ApplicationStatus {
	Pending,
	Approved,
	Rejected,
}

impl Default for ApplicationStatus {
	fn default() -> Self {
		ApplicationStatus::Pending
	}
}

#[derive(
	CloneNoBound, Encode, Decode, Eq, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen,
)]
pub struct ApplicationField {
	pub display_name: BoundedVec<u8, ConstU32<100>>,
	pub cid: Cid,
	pub custodian_cid: Option<Cid>,
}
// Eq macro didnt work (binary operation `==` cannot be applied to type...)
impl PartialEq for ApplicationField {
	fn eq(&self, other: &Self) -> bool {
		self.cid == other.cid && self.display_name == other.display_name
	}
}

//offers
#[derive(
	Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy,
)]
pub enum OfferStatus {
	Open,
	Closed,
}

impl Default for OfferStatus {
	fn default() -> Self {
		OfferStatus::Open
	}
}

#[derive(
	Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy,
)]
pub enum OfferType {
	SellOrder,
	BuyOrder,
}
#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct OfferData<T: Config> {
	pub marketplace_id: [u8; 32],
	pub collection_id: T::CollectionId,
	pub item_id: T::ItemId,
	pub percentage: Permill,
	pub creator: T::AccountId,
	pub price: BalanceOf<T>,
	pub fee: BalanceOf<T>,
	pub status: OfferStatus,
	pub creation_date: u64,
	pub offer_type: OfferType,
	pub buyer: Option<(T::AccountId, [u8; 32])>,
}
