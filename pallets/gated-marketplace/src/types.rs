
use super::*;
use frame_support::pallet_prelude::*;
//use frame_system::pallet_prelude::*;
use sp_runtime::sp_std::vec::Vec;
use frame_support::sp_io::hashing::blake2_256;

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
    Applicant,
    Participant,
}

impl Default for MarketplaceAuthority{
    fn default() -> Self {
        MarketplaceAuthority::Applicant
    }
}

impl MarketplaceAuthority{
    pub fn to_vec(&self) -> Vec<u8>{
        match self{
            Self::Owner => "Owner".as_bytes().to_vec(),
            Self::Admin => "Admin".as_bytes().to_vec(),
            Self::Appraiser => "Appraiser".as_bytes().to_vec(),
            Self::RedemptionSpecialist => "Redemption_specialist".as_bytes().to_vec(),
            Self::Applicant => "Applicant".as_bytes().to_vec(),
            Self::Participant => "Participant".as_bytes().to_vec(),
        }
    }

    pub fn get_id(&self) -> [u8;32]{
        self.to_vec().using_encoded(blake2_256)
    }

    pub fn enum_to_vec() -> Vec<Vec<u8>>{
        use crate::types::MarketplaceAuthority::*;
        [Owner.to_vec(), Admin.to_vec(), Appraiser.to_vec(), RedemptionSpecialist.to_vec(),
        Applicant.to_vec(), Participant.to_vec()].to_vec()
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
