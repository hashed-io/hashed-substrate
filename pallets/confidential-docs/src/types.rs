
use super::*;
use frame_support::pallet_prelude::*;

pub type CID = BoundedVec<u8,ConstU32<100>>;
pub type PublicKey = [u8;32];
pub type UserId = [u8;32];
pub type DocName<T> = BoundedVec<u8,<T as Config>::DocNameMaxLen>;
pub type DocDesc<T> = BoundedVec<u8,<T as Config>::DocDescMaxLen>;

#[derive(CloneNoBound,Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen, PartialEq)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Vault<T: Config>{
    pub cid: CID,
    pub owner: T::AccountId,
}

#[derive(CloneNoBound,Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen, PartialEq)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Document<T: Config>{
    pub name: DocName<T>,
    pub description: DocDesc<T>,
}

#[derive(CloneNoBound,Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen, PartialEq)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct SharedDocument<T: Config>{
    pub name: DocName<T>,
    pub description: DocDesc<T>,
    pub from: T::AccountId,
    pub to: T::AccountId,
    pub original_cid: CID
}