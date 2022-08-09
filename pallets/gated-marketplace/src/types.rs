
use super::*;
use frame_support::pallet_prelude::*;
//use frame_system::pallet_prelude::*;
use frame_support::traits::Currency;

pub type BalanceOf<T> = <<T as Config>::LocalCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(CloneNoBound,Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Marketplace<T: Config>{
    pub label: BoundedVec<u8,T::LabelMaxLen>,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub enum AccountOrApplication<T: Config>{
    Account(T::AccountId),
    Application([u8;32]),
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum MarketplaceAuthority{
    Owner,
    Admin,
    Appraiser,
    RedemptionSpecialist,
}

impl Default for MarketplaceAuthority{
    fn default() -> Self {
        MarketplaceAuthority::Appraiser
    }
}

#[derive(CloneNoBound,Encode, Decode, Eq, PartialEq, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct  Application< T: Config >{
    pub status : ApplicationStatus,
    pub fields: BoundedVec<ApplicationField, T::MaxFiles>,
    pub feedback: BoundedVec<u8, T::MaxFeedbackLen>,
}


#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum ApplicationStatus{
    Pending,
    Approved,
    Rejected,
}

impl Default for ApplicationStatus{
    fn default() -> Self {
        ApplicationStatus::Pending
    }
}

#[derive(CloneNoBound, Encode ,Decode, Eq, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
pub struct ApplicationField{
    pub display_name: BoundedVec<u8,ConstU32<100> >,
    pub cid: BoundedVec<u8, ConstU32<100> >,
    pub custodian_cid: Option<BoundedVec<u8, ConstU32<100> > >,
}
// Eq macro didnt work (binary operation `==` cannot be applied to type...)
impl PartialEq for ApplicationField{
    fn eq(&self, other: &Self) -> bool{
        self.cid == other.cid && self.display_name == other.display_name
    }
}


//offers
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum OfferStatus{
    Open,
    Freezed,
    Closed,
    NotFound,
}

impl Default for OfferStatus{
    fn default() -> Self {
        OfferStatus::Open
    }
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum OfferType{
    SellOrder,
    BuyOrder,
    NotFound,
}

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct OfferData<T: Config>{
    pub marketplace_id: [u8;32],
    pub collection_id: T::CollectionId,
    pub item_id: T::ItemId,
    pub creator: T::AccountId,
    pub price:  BalanceOf<T>,
    pub status: OfferStatus,
    pub creation_date: u64,
    pub expiration_date: u64,
    pub offer_type: OfferType,
    pub buyer: Option<(T::AccountId, [u8;32])>,
}
