
use super::*;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;


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

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo)]
pub enum MarketplaceAuthority{
    Owner,
    Admin,
    Appraiser,
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
    pub notes : BoundedVec<u8, T::NotesMaxLen>,
    pub files: BoundedVec<ApplicationFile<T::NameMaxLen>, T::MaxFiles>
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo)]
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
#[scale_info(skip_type_params(M))]
#[codec(mel_bound())]
pub struct ApplicationFile< M : Get<u32> >{
    pub display_name: BoundedVec<u8,M >,
    pub cid: BoundedVec<u8, ConstU32<100> >
}
// Eq macro didnt work (binary operation `==` cannot be applied to type...)
impl<M : Get<u32>> PartialEq for ApplicationFile<M>{
    fn eq(&self, other: &Self) -> bool{
        self.cid == other.cid && self.display_name == other.display_name
    }
}
