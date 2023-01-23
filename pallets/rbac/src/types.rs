//use super::*;
use frame_support::pallet_prelude::*;
use sp_runtime::sp_std::vec::Vec;
use frame_support::sp_io::hashing::blake2_256;


pub type PalletId = [u8;32];
pub type RoleId = [u8;32];
pub type ScopeId = [u8;32];
pub type PermissionId = [u8;32];

#[derive(Encode, Decode, Debug, Clone, Eq, PartialEq,TypeInfo,)]
pub enum IdOrVec{
    Id([u8;32]),
    Vec(Vec<u8>)
}

impl IdOrVec{
    pub fn to_id_enum(&self)->Self{
        match self{
            Self::Id(_) => self.clone(),
            Self::Vec(_) => Self::Id(Self::to_id(self))
        }
    }

    pub fn to_id(&self)->[u8;32]{
        match self{
            Self::Id(id) => *id,
            Self::Vec(v) => v.clone().using_encoded(blake2_256)
        }
    }
}

pub trait RoleBasedAccessControl<AccountId>{
    type MaxRolesPerPallet:  Get<u32>;
    type MaxPermissionsPerRole: Get<u32>;
    type RoleMaxLen: Get<u32>;
    type PermissionMaxLen: Get<u32>;
    // scopes
    fn create_scope(pallet: IdOrVec, scope_id: ScopeId) -> DispatchResult;
    // scope removal
    fn remove_scope(pallet: IdOrVec, scope_id: ScopeId) -> DispatchResult;
    // removes all from one pallet/application
    fn remove_pallet_storage(pallet: IdOrVec) -> DispatchResult;
    // roles creation and setting
    fn create_and_set_roles(pallet: IdOrVec, roles: Vec<Vec<u8>>) -> 
        Result<BoundedVec<RoleId, Self::MaxRolesPerPallet>, DispatchError>;
    fn create_role(role: Vec<u8>)-> Result<RoleId, DispatchError>;
    fn set_role_to_pallet(pallet: IdOrVec, role_id: RoleId )-> DispatchResult;
    fn set_multiple_pallet_roles(pallet: IdOrVec, roles: Vec<RoleId>)->DispatchResult;
    fn assign_role_to_user(user: AccountId, pallet: IdOrVec, scope_id: &ScopeId, role_id: RoleId) -> DispatchResult;
    // role removal
    fn remove_role_from_user(user: AccountId, pallet: IdOrVec, scope_id: &ScopeId, role_id: RoleId) -> DispatchResult;
    // permissions
    fn create_and_set_permissions(pallet: IdOrVec, role: RoleId, permissions: Vec<Vec<u8>>)->
        Result<BoundedVec<PermissionId, Self::MaxPermissionsPerRole>, DispatchError>;
    fn create_permission(pallet: IdOrVec, permissions: Vec<u8>) -> Result<PermissionId, DispatchError>;
    fn set_permission_to_role( pallet: IdOrVec, role: RoleId, permission: PermissionId ) -> DispatchResult;
    fn set_multiple_permissions_to_role(  pallet: IdOrVec, role: RoleId, permission: Vec<PermissionId> )-> DispatchResult;
    // helpers
    fn is_authorized(user: AccountId, pallet: IdOrVec, scope_id: &ScopeId, permission_id: &PermissionId ) -> DispatchResult;
    fn has_role(user: AccountId, pallet: IdOrVec, scope_id: &ScopeId, role_ids: Vec<RoleId>)->DispatchResult;
    fn scope_exists(pallet: IdOrVec, scope_id:&ScopeId) -> DispatchResult;
    fn permission_exists(pallet: IdOrVec, permission_id: &PermissionId)->DispatchResult;
    fn is_role_linked_to_pallet(pallet: IdOrVec, role_id: &RoleId)-> DispatchResult;
    fn is_permission_linked_to_role(pallet: IdOrVec, role_id: &RoleId, permission_id: &PermissionId)-> DispatchResult;
    fn get_role_users_len(pallet: IdOrVec, scope_id:&ScopeId, role_id: &RoleId) -> usize;
    fn to_id(v: Vec<u8>)->[u8;32];

}