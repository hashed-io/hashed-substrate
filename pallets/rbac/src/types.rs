use super::*;
use frame_support::pallet_prelude::*;
use sp_runtime::sp_std::vec::Vec;

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo,)]
pub enum IdOrString<MaxLen: Get<u32> >{
    Id([u8;32]),
    String(BoundedVec<u8, MaxLen >)
}

pub trait RoleBasedAccessControl<AccountId>{
    type MaxRolesPerPallet:  Get<u32>;
    type MaxPermissionsPerRole: Get<u32>;
    type RoleMaxLen: Get<u32>;
    type PermissionMaxLen: Get<u32>;
    // scopes
    fn create_scope(pallet_id: u64, scope_id: [u8;32]) -> DispatchResult;
    // scope removal
    fn remove_scope(pallet_id: u64, scope_id: [u8;32]) -> DispatchResult;
    // roles creation and setting
    fn create_and_set_roles(pallet_id: u64, roles: Vec<Vec<u8>>) -> 
        Result<BoundedVec<[u8;32], Self::MaxRolesPerPallet>, DispatchError>;
    fn create_role(role: Vec<u8>)-> Result<[u8;32], DispatchError>;
    fn set_role_to_pallet(pallet_id: u64, role_id: [u8;32] )-> DispatchResult;
    fn set_multiple_pallet_roles(pallet_id: u64, roles: Vec<[u8;32]>)->DispatchResult;
    fn assign_role_to_user(user: AccountId, pallet_id: u64, scope_id: &[u8;32], role_id: [u8;32]) -> DispatchResult;
    // role removal
    fn remove_role_from_user(user: AccountId, pallet_id: u64, scope_id: &[u8;32], role_id: [u8;32]) -> DispatchResult;
    // permissions
    fn create_and_set_permissions(pallet_id: u64, role: [u8;32], permissions: Vec<Vec<u8>>)->
        Result<BoundedVec<[u8;32], Self::MaxPermissionsPerRole>, DispatchError>;
    fn create_permission(pallet_id: u64, permission: Vec<u8>) -> Result<[u8;32], DispatchError>;
    fn set_permission_to_role( pallet_id: u64, role: [u8;32], permission: [u8;32] ) -> DispatchResult;
    fn set_multiple_permisions_to_role(  pallet_id: u64, role: [u8;32], permission: Vec<[u8;32]> )-> DispatchResult;
    // helpers
    fn is_user_authorized(user: AccountId, pallet_id: u64, scope_id: &[u8;32], role_id: &[u8;32] ) -> DispatchResult;
    fn has_role(user: AccountId, pallet_id: u64, scope_id: &[u8;32], role_ids: Vec<[u8;32]>)->DispatchResult;
    fn scope_exists(pallet_id: u64, scope_id:&[u8;32]) -> DispatchResult;
    fn is_role_linked_to_pallet(pallet_id: u64, role_id: &[u8;32])-> DispatchResult;
    fn get_role_users_len(pallet_id: u64, scope_id:&[u8;32], role_id: &[u8;32]) -> usize;
    fn get_role_id(id_or_role: IdOrString<Self::RoleMaxLen>)->Result<[u8;32], DispatchError>;
    fn get_permission(pallet_id: u64 ,id_or_permission: IdOrString< Self::PermissionMaxLen>)->Result<[u8;32], DispatchError>;
    fn has_unique_elements(vec: Vec<u8>) -> bool;

}