use super::*;
use frame_support::pallet_prelude::*;

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo,)]
pub enum IdOrRole{
    Id([u8;32]),
    Role(BoundedVec<u8, ConstU32<100> >)
}
