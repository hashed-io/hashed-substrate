use super::*;
use frame_support::pallet_prelude::*;
use sp_runtime::sp_std::vec::Vec;


pub type RoleId = [u8;32];
pub type ScopeId = [u8;32];
pub type PermissionId = [u8;32];

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
    fn create_scope(pallet_id: u64, scope_id: ScopeId) -> DispatchResult;
    // scope removal
    fn remove_scope(pallet_id: u64, scope_id: ScopeId) -> DispatchResult;
    // removes all from one pallet/application
    fn remove_pallet_storage(pallet_id: u64) -> DispatchResult;
    // roles creation and setting
    fn create_and_set_roles(pallet_id: u64, roles: Vec<Vec<u8>>) -> 
        Result<BoundedVec<RoleId, Self::MaxRolesPerPallet>, DispatchError>;
    fn create_role(role: Vec<u8>)-> Result<RoleId, DispatchError>;
    fn set_role_to_pallet(pallet_id: u64, role_id: RoleId )-> DispatchResult;
    fn set_multiple_pallet_roles(pallet_id: u64, roles: Vec<RoleId>)->DispatchResult;
    fn assign_role_to_user(user: AccountId, pallet_id: u64, scope_id: &ScopeId, role_id: RoleId) -> DispatchResult;
    // role removal
    fn remove_role_from_user(user: AccountId, pallet_id: u64, scope_id: &ScopeId, role_id: RoleId) -> DispatchResult;
    // permissions
    fn create_and_set_permissions(pallet_id: u64, role: RoleId, permissions: Vec<Vec<u8>>)->
        Result<BoundedVec<PermissionId, Self::MaxPermissionsPerRole>, DispatchError>;
    fn create_permission(pallet_id: u64, permissions: Vec<u8>) -> Result<PermissionId, DispatchError>;
    fn set_permission_to_role( pallet_id: u64, role: RoleId, permission: PermissionId ) -> DispatchResult;
    fn set_multiple_permissions_to_role(  pallet_id: u64, role: RoleId, permission: Vec<PermissionId> )-> DispatchResult;
    // helpers
    fn is_authorized(user: AccountId, pallet_id: u64, scope_id: &ScopeId, permission_id: &PermissionId ) -> DispatchResult;
    fn has_role(user: AccountId, pallet_id: u64, scope_id: &ScopeId, role_ids: Vec<RoleId>)->DispatchResult;
    fn scope_exists(pallet_id: u64, scope_id:&ScopeId) -> DispatchResult;
    fn permission_exists(pallet_id: u64, permission_id: &PermissionId)->DispatchResult;
    fn is_role_linked_to_pallet(pallet_id: u64, role_id: &RoleId)-> DispatchResult;
    fn is_permission_linked_to_role(pallet_id: u64, role_id: &RoleId, permission_id: &PermissionId)-> DispatchResult;
    fn get_role_users_len(pallet_id: u64, scope_id:&ScopeId, role_id: &RoleId) -> usize;
    fn has_unique_elements(vec: Vec<u8>) -> bool;

}