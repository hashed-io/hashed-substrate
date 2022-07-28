use super::*;
use frame_support::pallet_prelude::*;
use sp_runtime::sp_std::vec::Vec;

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

pub trait RoleBasedAccessControl<AccountId>{
    type MaxRolesPerPallet:  Get<u32>;
    type PermissionMaxLen: Get<u32>;
    // scopes
    fn create_scope(pallet_id: u32, scope_id: [u8;32]) -> DispatchResult;
    // roles
    fn create_and_set_roles(pallet_id: u32, roles: BoundedVec<BoundedVec<u8,ConstU32<100> >, Self::MaxRolesPerPallet>) -> 
        Result<BoundedVec<[u8;32], Self::MaxRolesPerPallet>, DispatchError>;
    fn create_role(role: BoundedVec<u8, ConstU32<100>>)-> [u8;32];
    fn set_role_to_pallet(pallet_id: u32, role_id: [u8;32] )-> DispatchResult;
    fn set_multiple_pallet_roles(pallet_id: u32, roles: BoundedVec<[u8;32], Self::MaxRolesPerPallet>)->DispatchResult;
    fn assign_role_to_user(user: AccountId, pallet_id: u32, scope_id: [u8;32], role_id: [u8;32]) -> DispatchResult;
    // permissions
    fn create_permission(pallet_id: u32, permission: BoundedVec<u8, Self::PermissionMaxLen>) -> [u8;32];
    fn set_permission_to_role( pallet_id: u32, role: [u8;32], permission: [u8;32] ) -> DispatchResult;
    // helpers
    fn is_user_authorized(user: AccountId, pallet_id: u32, scope_id: [u8;32], role: IdOrString<ConstU32<100>> ) -> DispatchResult;
    fn scope_exists(pallet_id: &u32, scope_id:&[u8;32]) -> DispatchResult;
    fn is_role_linked_to_pallet(pallet_id: &u32, role_id: &[u8;32])-> DispatchResult;
    fn get_role_id(id_or_role: IdOrString<ConstU32<100>>)->Result<[u8;32], DispatchError>;
    fn get_permission(pallet_id: u32 ,id_or_permission: IdOrString< Self::PermissionMaxLen>)->Result<[u8;32], DispatchError>;
    fn has_unique_elements(vec: Vec<u8>) -> bool;


}