use super::*;
use frame_support::pallet_prelude::*;

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo,)]
pub enum IdOrString<MaxLen: Get<u32> >{
    Id([u8;32]),
    String(BoundedVec<u8, MaxLen >)
}


pub struct RoleConfiguration<T:Config>{
    pub role_name: BoundedVec<u8, ConstU32<100>>,
    pub permissions: BoundedVec<
        BoundedVec<u8 ,T::PermissionMaxLen>,
        T::MaxPermissionsPerRole
    >
}

pub trait RoleBasedAccessControl<T:Config>{
    fn create_scope(pallet_id: u32, scope_id: [u8;32]) -> DispatchResult;
    fn create_and_set_roles(pallet_id: u32, roles: BoundedVec<BoundedVec<u8,ConstU32<100> >, T::MaxRolesPerPallet>) -> 
        Result<BoundedVec<[u8;32], T::MaxRolesPerPallet>, DispatchError>;
    fn create_role(role: BoundedVec<u8, ConstU32<100>>)-> [u8;32];
    fn set_role_to_pallet(pallet_id: u32, role_id: [u8;32] )-> DispatchResult;
    fn set_multiple_pallet_roles(pallet_id: u32, roles: BoundedVec<[u8;32], T::MaxRolesPerPallet>)->DispatchResult;
    fn create_permission(pallet_id: u32, permission: BoundedVec<u8, T::PermissionMaxLen>) -> [u8;32];
    fn set_permission_to_role( pallet_id: u32, role: [u8;32], permission: [u8;32] ) -> DispatchResult;
    fn assign_role_to_user(user: T::AccountId, pallet_id: u32, scope_id: [u8;32], role_id: [u8;32]) -> DispatchResult;

}