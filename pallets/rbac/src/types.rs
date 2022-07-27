use super::*;
use frame_support::pallet_prelude::*;

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo,)]
pub enum IdOrRole{
    Id([u8;32]),
    Role(BoundedVec<u8, ConstU32<100> >)
}


pub struct RoleConfiguration<T:Config>{
    pub role_name: BoundedVec<u8, ConstU32<100>>,
    pub permissions: BoundedVec<
        BoundedVec<u8 ,T::PermissionMaxLen>,
        T::MaxPermissionsPerRole
    >
}