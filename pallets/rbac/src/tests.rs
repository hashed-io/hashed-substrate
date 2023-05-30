use crate::{
  mock::*,
  types::{IdOrVec, PermissionId, RoleBasedAccessControl, RoleId, ScopeId},
  Config, Error, Permissions, PermissionsByRole,
};
use frame_support::{
  assert_err, assert_noop, assert_ok, pallet_prelude::DispatchResult, BoundedVec,
};
type AccountId = <Test as frame_system::Config>::AccountId;

fn pallet_name() -> IdOrVec {
  IdOrVec::Vec("pallet_test".as_bytes().to_vec())
}

fn pallet_id() -> [u8; 32] {
  pallet_name().to_id()
}

fn create_scope(n: u8) -> ScopeId {
  let scope_id = [n; 32];
  assert_ok!(RBAC::create_scope(pallet_name(), scope_id));
  assert!(RBAC::scopes(pallet_id()).contains(&scope_id));
  scope_id
}

fn gen_roles(n_roles: u32) -> Vec<Vec<u8>> {
  let mut v = Vec::new();
  for i in 0..n_roles {
    v.push(format!("role{}", i).into_bytes().to_vec());
  }
  v
}

fn gen_permissions(n_permissions: u32) -> Vec<Vec<u8>> {
  let mut v = Vec::new();
  for i in 0..n_permissions {
    v.push(format!("permission{}", i).into_bytes().to_vec());
  }
  v
}

fn create_role(role: Vec<u8>) -> RoleId {
  let r_id = RBAC::create_role(role.clone()).unwrap();
  assert_eq!(RBAC::roles(r_id).unwrap().to_vec(), role);
  r_id
}

fn create_and_set_roles(
  roles: Vec<Vec<u8>>,
) -> BoundedVec<RoleId, <Test as Config>::MaxRolesPerPallet> {
  let role_ids = RBAC::create_and_set_roles(pallet_name(), roles).unwrap();
  let inserted_roles_list = RBAC::pallet_roles(pallet_id());
  assert!(role_ids.iter().all(|r_id| inserted_roles_list.contains(r_id)));
  role_ids
}

fn set_role_to_pallet(role_id: RoleId) {
  assert_ok!(RBAC::set_role_to_pallet(pallet_name(), role_id));
}

fn set_multiple_pallet_roles(roles: Vec<RoleId>) {
  assert_ok!(RBAC::set_multiple_pallet_roles(pallet_name(), roles));
}

fn remove_scope(n: u8) {
  assert_ok!(RBAC::remove_scope(pallet_name(), [n; 32]));
  assert!(RBAC::scope_exists(pallet_name(), &[n; 32]).is_err());
}

fn remove_role_from_user(user: AccountId, scope_id: &ScopeId, role_id: RoleId) {
  assert_ok!(RBAC::remove_role_from_user(user, pallet_name(), scope_id, role_id));
  let user_roles = RBAC::roles_by_user((user, pallet_id(), scope_id));
  assert!(!user_roles.contains(&role_id));
  let role_users = RBAC::users_by_scope((pallet_id(), scope_id, role_id));
  assert!(!role_users.contains(&user));
}

fn revoke_permission_from_role(role_id: RoleId, permission_id: PermissionId) {
  assert_ok!(RBAC::revoke_permission_from_role(
    RuntimeOrigin::root(),
    pallet_name(),
    role_id,
    permission_id
  ));
  let permissions = RBAC::permissions_by_role(pallet_id(), role_id);
  assert!(!permissions.contains(&permission_id))
}

fn remove_permission_from_pallet(permission_id: PermissionId) {
  let affected_roles = RBAC::get_roles_that_have_permission(pallet_id(), &permission_id);
  assert_ok!(RBAC::remove_permission_from_pallet(
    RuntimeOrigin::root(),
    pallet_name(),
    permission_id
  ));
  assert!(RBAC::permissions(pallet_id(), permission_id).is_empty());
  affected_roles
    .iter()
    .for_each(|ar| assert!(!RBAC::permissions_by_role(pallet_id(), ar).contains(&permission_id)));
}

fn remove_pallet_storage() {
  assert_ok!(RBAC::remove_pallet_storage(pallet_name()));
  assert!(RBAC::scopes(pallet_id()).is_empty());
  assert!(RBAC::pallet_roles(pallet_id()).is_empty());
  assert_eq!(<Permissions<Test>>::iter_prefix(pallet_id()).count(), 0);
  assert_eq!(<PermissionsByRole<Test>>::iter_prefix(pallet_id()).count(), 0);
}

fn assign_role_to_user(user: AccountId, scope_id: &ScopeId, role_id: RoleId) {
  assert_ok!(RBAC::assign_role_to_user(user, pallet_name(), scope_id, role_id));
  let user_roles = RBAC::roles_by_user((user, pallet_id(), scope_id));
  assert!(user_roles.contains(&role_id));
  let role_users = RBAC::users_by_scope((pallet_id(), scope_id, role_id));
  assert!(role_users.contains(&user));
}

fn create_permission(permission: Vec<u8>) -> PermissionId {
  let permission_id = RBAC::create_permission(pallet_name(), permission.clone()).unwrap();
  assert_eq!(RBAC::permissions(pallet_id(), permission_id).to_vec(), permission);
  permission_id
}

fn set_permission_to_role(role_id: RoleId, permission_id: PermissionId) {
  assert_ok!(RBAC::set_permission_to_role(pallet_name(), role_id, permission_id));
  assert!(RBAC::permissions_by_role(pallet_id(), role_id).contains(&permission_id));
}

fn set_multiple_permissions_to_role(role_id: RoleId, permissions: Vec<PermissionId>) {
  assert_ok!(RBAC::set_multiple_permissions_to_role(pallet_name(), role_id, permissions.clone()));
  let role_permissions = RBAC::permissions_by_role(pallet_id(), role_id);
  assert!(permissions.iter().all(|p| { role_permissions.contains(p) }),);
}

fn create_and_set_permissions(
  role_id: RoleId,
  permissions: Vec<Vec<u8>>,
) -> BoundedVec<PermissionId, <Test as Config>::MaxPermissionsPerRole> {
  let permission_ids =
    RBAC::create_and_set_permissions(pallet_name(), role_id, permissions).unwrap();
  let role_permissions = RBAC::permissions_by_role(pallet_id(), role_id);
  assert!(permission_ids.iter().all(|p| { role_permissions.contains(p) }),);
  permission_ids
}

fn is_authorized(
  user: AccountId,
  scope_id: &ScopeId,
  permission_id: &PermissionId,
) -> DispatchResult {
  RBAC::is_authorized(user, pallet_name(), scope_id, permission_id)
}

fn has_role(user: AccountId, scope_id: &ScopeId, role_ids: Vec<RoleId>) -> DispatchResult {
  RBAC::has_role(user, pallet_name(), scope_id, role_ids)
}

fn scope_exists(scope_id: &ScopeId) -> DispatchResult {
  RBAC::scope_exists(pallet_name(), scope_id)
}

fn permission_exists(permission_id: &PermissionId) -> DispatchResult {
  RBAC::permission_exists(pallet_name(), permission_id)
}

fn is_role_linked_to_pallet(role_id: &RoleId) -> DispatchResult {
  RBAC::is_role_linked_to_pallet(pallet_name(), role_id)
}

fn is_permission_linked_to_role(role_id: &RoleId, permission_id: &PermissionId) -> DispatchResult {
  RBAC::is_permission_linked_to_role(pallet_name(), role_id, permission_id)
}

fn get_role_users_len(scope_id: &ScopeId, role_id: &RoleId) -> usize {
  RBAC::get_role_users_len(pallet_name(), scope_id, role_id)
}

fn does_user_have_any_role_in_scope(
  user: AccountId,
  pallet_id: IdOrVec,
  scope_id: &ScopeId,
) -> bool {
  RBAC::does_user_have_any_role_in_scope(user, pallet_id, scope_id)
}

#[test]
fn create_scope_works() {
  new_test_ext().execute_with(|| {
    create_scope(0);
  });
}

#[test]
fn create_scope_twice_should_fail() {
  new_test_ext().execute_with(|| {
    create_scope(0);
    assert_noop!(RBAC::create_scope(pallet_name(), [0; 32]), Error::<Test>::ScopeAlreadyExists);
  });
}

#[test]
fn exceeding_max_scopes_per_pallet_should_fail() {
  new_test_ext().execute_with(|| {
    for n in 0..<Test as Config>::MaxScopesPerPallet::get() {
      create_scope(n.try_into().unwrap());
    }
    assert_noop!(
      RBAC::create_scope(pallet_name(), [255; 32]),
      Error::<Test>::ExceedMaxScopesPerPallet
    );
  });
}

#[test]
fn remove_scope_works() {
  new_test_ext().execute_with(|| {
    let n_roles = <Test as Config>::MaxRolesPerPallet::get();
    let scope_id = create_scope(0);
    let role_ids = create_and_set_roles(gen_roles(n_roles));
    role_ids.iter().enumerate().for_each(|(i, role_id)| {
      assign_role_to_user(i.try_into().unwrap(), &scope_id, *role_id);
    });
    remove_scope(0);
  });
}

#[test]
fn remove_non_existent_scope_should_fail() {
  new_test_ext().execute_with(|| {
    let n_roles = <Test as Config>::MaxRolesPerPallet::get();
    create_and_set_roles(gen_roles(n_roles));
    assert_noop!(RBAC::remove_scope(pallet_name(), [0; 32]), Error::<Test>::ScopeNotFound);
  });
}

#[test]
fn remove_pallet_storage_works() {
  new_test_ext().execute_with(|| {
    create_scope(0);
    remove_pallet_storage();
  });
}

#[test]
fn create_role_should_work() {
  new_test_ext().execute_with(|| {
    create_role("owner".as_bytes().to_vec());
  });
}

#[test]
fn exceeding_role_max_len_should_fail() {
  new_test_ext().execute_with(|| {
    assert_noop!(
      RBAC::create_role("0123456789A".as_bytes().to_vec()),
      Error::<Test>::ExceedRoleMaxLen
    );
  });
}

#[test]
fn set_role_to_pallet_should_work() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("owner".as_bytes().to_vec());
    set_role_to_pallet(role_id);
  });
}

#[test]
fn set_nonexistent_role_to_pallet_should_fail() {
  new_test_ext().execute_with(|| {
    assert_noop!(RBAC::set_role_to_pallet(pallet_name(), [0; 32]), Error::<Test>::RoleNotFound);
  });
}

#[test]
fn set_role_to_pallet_twice_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("owner".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    assert_noop!(
      RBAC::set_role_to_pallet(pallet_name(), role_id),
      Error::<Test>::RoleAlreadyLinkedToPallet
    );
  });
}

#[test]
fn exceeding_max_roles_per_pallet_should_fail() {
  new_test_ext().execute_with(|| {
    let role_max_len = <Test as Config>::MaxRolesPerPallet::get();
    gen_roles(role_max_len).iter().for_each(|role| {
      let role_id = create_role(role.clone());
      set_role_to_pallet(role_id);
    });
    let role_id = create_role("admin".as_bytes().to_vec());
    assert_noop!(
      RBAC::set_role_to_pallet(pallet_name(), role_id),
      Error::<Test>::ExceedMaxRolesPerPallet
    );
  });
}

#[test]
fn set_multiple_pallet_roles_should_work() {
  new_test_ext().execute_with(|| {
    let n_roles = <Test as Config>::MaxRolesPerPallet::get() - 1;
    let role_ids: Vec<RoleId> =
      gen_roles(n_roles).iter().map(|role| create_role(role.clone())).collect();
    set_multiple_pallet_roles(role_ids);
  });
}

#[test]
fn set_multiple_duplicate_pallet_roles_should_fail() {
  new_test_ext().execute_with(|| {
    let n_roles = <Test as Config>::MaxRolesPerPallet::get() - 1;
    let mut roles = gen_roles(n_roles);
    roles.push("role0".as_bytes().to_vec());
    let role_ids: Vec<RoleId> = roles.iter().map(|role| create_role(role.clone())).collect();
    assert_noop!(
      RBAC::set_multiple_pallet_roles(pallet_name(), role_ids),
      Error::<Test>::DuplicateRole
    );
  });
}

#[test]
fn set_multiple_pallet_roles_twice_should_fail() {
  new_test_ext().execute_with(|| {
    let n_roles = <Test as Config>::MaxRolesPerPallet::get();
    let roles = gen_roles(n_roles);
    let role_ids: Vec<RoleId> = roles.iter().map(|role| create_role(role.clone())).collect();
    set_multiple_pallet_roles(role_ids.clone());
    assert_noop!(
      RBAC::set_multiple_pallet_roles(pallet_name(), role_ids),
      Error::<Test>::RoleAlreadyLinkedToPallet
    );
  });
}

#[test]
fn create_and_set_role_should_work() {
  new_test_ext().execute_with(|| {
    create_and_set_roles(gen_roles(<Test as Config>::MaxRolesPerPallet::get()));
  });
}

#[test]
fn create_and_set_duplicate_role_should_fail() {
  new_test_ext().execute_with(|| {
    let mut roles = gen_roles(<Test as Config>::MaxRolesPerPallet::get() - 1);
    roles.push("role0".as_bytes().to_vec());
    assert_err!(RBAC::create_and_set_roles(pallet_name(), roles), Error::<Test>::DuplicateRole);
  });
}

#[test]
fn exceeding_max_roles_per_pallet_from_create_and_set_role_should_fail() {
  new_test_ext().execute_with(|| {
    let exceed = <Test as Config>::MaxRolesPerPallet::get() + 1;
    assert_err!(
      RBAC::create_and_set_roles(pallet_name(), gen_roles(exceed)),
      Error::<Test>::ExceedMaxRolesPerPallet
    );
  });
}

#[test]
fn assign_role_to_user_should_work() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let role_id = create_role("owner".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    assign_role_to_user(0, &scope_id, role_id);
  });
}

#[test]
fn assign_role_to_user_twice_should_fail() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let role_id = create_role("owner".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    assign_role_to_user(0, &scope_id, role_id);
    assert_noop!(
      RBAC::assign_role_to_user(0, pallet_name(), &scope_id, role_id),
      Error::<Test>::UserAlreadyHasRole
    );
  });
}

#[test]
fn assign_role_to_user_without_scope_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("owner".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    assert_noop!(
      RBAC::assign_role_to_user(0, pallet_name(), &[0; 32], role_id),
      Error::<Test>::ScopeNotFound
    );
  });
}

#[test]
fn exceeding_max_roles_per_user_should_fail() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let n_roles = <Test as Config>::MaxRolesPerUser::get();
    let roles = gen_roles(n_roles);
    let role_ids: Vec<RoleId> = roles.iter().map(|role| create_role(role.clone())).collect();
    set_multiple_pallet_roles(role_ids.clone());
    role_ids.iter().for_each(|role_id| {
      assign_role_to_user(0, &scope_id, *role_id);
    });
    let last_role_id = create_role("owner".as_bytes().to_vec());
    set_role_to_pallet(last_role_id);
    assert_noop!(
      RBAC::assign_role_to_user(0, pallet_name(), &scope_id, last_role_id),
      Error::<Test>::ExceedMaxRolesPerUser
    );
  });
}

#[test]
fn exceeding_max_users_per_role_should_fail() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let role_id = create_role("owner".as_bytes().to_vec());
    let max_users_per_role = <Test as Config>::MaxUsersPerRole::get();
    set_role_to_pallet(role_id);
    for i in 0..max_users_per_role {
      assign_role_to_user(i.into(), &scope_id, role_id)
    }
    // avoiding assert_noop because it checks if the storage mutated
    assert_err!(
      RBAC::assign_role_to_user((max_users_per_role + 1).into(), pallet_name(), &scope_id, role_id),
      Error::<Test>::ExceedMaxUsersPerRole
    );
  });
}

#[test]
fn remove_role_from_user_should_work() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let role_id = create_role("owner".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    assign_role_to_user(0, &scope_id, role_id);
    remove_role_from_user(0, &scope_id, role_id);
  });
}

#[test]
fn remove_non_assigned_role_from_user_should_fail() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    assert_noop!(
      RBAC::remove_role_from_user(0, pallet_name(), &scope_id, [0; 32]),
      Error::<Test>::UserHasNoRoles
    );
  });
}

#[test]
fn remove_non_existent_role_from_user_should_fail() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let role_id = create_role("owner".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    assign_role_to_user(0, &scope_id, role_id);
    assert_noop!(
      RBAC::remove_role_from_user(0, pallet_name(), &scope_id, [0; 32]),
      Error::<Test>::RoleNotFound
    );
  });
}

#[test]
fn create_permission_should_work() {
  new_test_ext().execute_with(|| {
    create_permission("enroll".as_bytes().to_vec());
  });
}

#[test]
fn exceeding_permission_max_len_should_fail() {
  new_test_ext().execute_with(|| {
    assert_noop!(
      RBAC::create_permission(pallet_name(), "0123456789ABCDFG".as_bytes().to_vec()),
      Error::<Test>::ExceedPermissionMaxLen
    );
  });
}

#[test]
fn set_permission_to_role_should_work() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("admin".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    let permission_id = create_permission("enroll".as_bytes().to_vec());
    set_permission_to_role(role_id, permission_id);
  });
}

#[test]
fn set_non_existent_permission_to_role_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("admin".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    assert_noop!(
      RBAC::set_permission_to_role(pallet_name(), role_id, [0; 32]),
      Error::<Test>::PermissionNotFound
    );
  });
}

#[test]
fn set_permission_to_role_twice_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("admin".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    let permission_id = create_permission("enroll".as_bytes().to_vec());
    set_permission_to_role(role_id, permission_id);
    assert_noop!(
      RBAC::set_permission_to_role(pallet_name(), role_id, permission_id),
      Error::<Test>::DuplicatePermission
    );
  });
}

#[test]
fn exceeding_max_permissions_per_role_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("owner".as_bytes().to_vec());
    let max_permissions_per_role = <Test as Config>::MaxPermissionsPerRole::get();
    set_role_to_pallet(role_id);
    gen_permissions(max_permissions_per_role).iter().for_each(|permission| {
      let permission_id = create_permission(permission.clone());
      set_permission_to_role(role_id, permission_id);
    });
    let last_permission_id = create_permission("enroll".as_bytes().to_vec());
    assert_noop!(
      RBAC::set_permission_to_role(pallet_name(), role_id, last_permission_id),
      Error::<Test>::ExceedMaxPermissionsPerRole
    );
  });
}

#[test]
fn set_multiple_permissions_to_role_should_work() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("admin".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    let permissions = gen_permissions(<Test as Config>::MaxPermissionsPerRole::get());
    let permission_ids: Vec<PermissionId> = permissions
      .iter()
      .map(|permission| create_permission(permission.to_vec()))
      .collect();
    set_multiple_permissions_to_role(role_id, permission_ids);
  });
}

#[test]
fn set_multiple_duplicate_permissions_to_role_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("admin".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    let mut permissions = gen_permissions(<Test as Config>::MaxPermissionsPerRole::get() - 1);
    permissions.push("permission0".as_bytes().to_vec());
    let permission_ids: Vec<PermissionId> = permissions
      .iter()
      .map(|permission| create_permission(permission.to_vec()))
      .collect();
    assert_noop!(
      RBAC::set_multiple_permissions_to_role(pallet_name(), role_id, permission_ids),
      Error::<Test>::DuplicatePermission
    );
  });
}

#[test]
fn set_multiple_permissions_to_unlinked_role_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("admin".as_bytes().to_vec());
    let permissions = gen_permissions(<Test as Config>::MaxPermissionsPerRole::get());
    let permission_ids: Vec<PermissionId> = permissions
      .iter()
      .map(|permission| create_permission(permission.to_vec()))
      .collect();
    assert_noop!(
      RBAC::set_multiple_permissions_to_role(pallet_name(), role_id, permission_ids),
      Error::<Test>::RoleNotLinkedToPallet
    );
  });
}

#[test]
fn set_multiple_permissions_to_role_twice_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("admin".as_bytes().to_vec());
    let permissions = gen_permissions(<Test as Config>::MaxPermissionsPerRole::get());
    set_role_to_pallet(role_id);
    let permission_ids: Vec<PermissionId> = permissions
      .iter()
      .map(|permission| create_permission(permission.to_vec()))
      .collect();
    set_multiple_permissions_to_role(role_id, permission_ids.clone());
    assert_noop!(
      RBAC::set_multiple_permissions_to_role(pallet_name(), role_id, permission_ids),
      Error::<Test>::PermissionAlreadyLinkedToRole
    );
  });
}

#[test]
fn exceeding_max_permissions_per_role_from_set_multiple_permissions_to_role_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("admin".as_bytes().to_vec());
    let permissions = gen_permissions(<Test as Config>::MaxPermissionsPerRole::get() + 1);
    set_role_to_pallet(role_id);
    let permission_ids: Vec<PermissionId> = permissions
      .iter()
      .map(|permission| create_permission(permission.to_vec()))
      .collect();
    assert_noop!(
      RBAC::set_multiple_permissions_to_role(pallet_name(), role_id, permission_ids),
      Error::<Test>::ExceedMaxPermissionsPerRole
    );
  });
}

#[test]
fn create_and_set_permissions_should_work() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("admin".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    let permissions = gen_permissions(<Test as Config>::MaxPermissionsPerRole::get());
    create_and_set_permissions(role_id, permissions);
  });
}

#[test]
fn create_set_duplicate_permissions_to_role_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("admin".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    let mut permissions = gen_permissions(<Test as Config>::MaxPermissionsPerRole::get() - 1);
    permissions.push("permission0".as_bytes().to_vec());
    assert_noop!(
      RBAC::create_and_set_permissions(pallet_name(), role_id, permissions),
      Error::<Test>::DuplicatePermission
    );
  });
}

#[test]
fn create_and_set_permissions_to_unlinked_role_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("admin".as_bytes().to_vec());
    let permissions = gen_permissions(<Test as Config>::MaxPermissionsPerRole::get());
    assert_noop!(
      RBAC::create_and_set_permissions(pallet_name(), role_id, permissions),
      Error::<Test>::RoleNotLinkedToPallet
    );
  });
}

#[test]
fn create_and_set_multiple_permissions_to_role_twice_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("admin".as_bytes().to_vec());
    let permissions = gen_permissions(<Test as Config>::MaxPermissionsPerRole::get());
    set_role_to_pallet(role_id);
    create_and_set_permissions(role_id, permissions.clone());
    assert_noop!(
      RBAC::create_and_set_permissions(pallet_name(), role_id, permissions),
      Error::<Test>::PermissionAlreadyLinkedToRole
    );
  });
}

#[test]
fn exceeding_max_permissions_per_role_from_create_and_set_permissions_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("admin".as_bytes().to_vec());
    let permissions = gen_permissions(<Test as Config>::MaxPermissionsPerRole::get() + 1);
    set_role_to_pallet(role_id);
    assert_err!(
      RBAC::create_and_set_permissions(pallet_name(), role_id, permissions),
      Error::<Test>::ExceedMaxPermissionsPerRole
    );
  });
}

#[test]
fn is_authorized_should_work() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let role_ids = create_and_set_roles(["admin".as_bytes().to_vec()].to_vec());
    let mut permission_ids = create_and_set_permissions(
      *role_ids.get(0).unwrap(),
      ["enroll".as_bytes().to_vec()].to_vec(),
    );
    assign_role_to_user(0, &scope_id, *role_ids.get(0).unwrap());
    assert_ok!(is_authorized(0, &scope_id, &permission_ids.pop().unwrap()));
  });
}

#[test]
fn unauthorized_user_should_fail() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let role_ids = create_and_set_roles(["admin".as_bytes().to_vec()].to_vec());
    let mut permission_ids = create_and_set_permissions(
      *role_ids.get(0).unwrap(),
      ["enroll".as_bytes().to_vec()].to_vec(),
    );
    assert_noop!(
      is_authorized(0, &scope_id, &permission_ids.pop().unwrap()),
      Error::<Test>::NotAuthorized
    );
  });
}

#[test]
fn has_role_should_work() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let role_ids = create_and_set_roles(gen_roles(2));
    assign_role_to_user(0, &scope_id, *role_ids.get(0).unwrap());
    assert_ok!(has_role(0, &scope_id, role_ids.to_vec()));
  });
}

#[test]
fn user_that_doesnt_have_role_should_fail() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let role_ids = create_and_set_roles(gen_roles(2));
    assert_noop!(has_role(0, &scope_id, role_ids.to_vec()), Error::<Test>::NotAuthorized);
  });
}

#[test]
fn scope_exists_should_work() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    assert_ok!(scope_exists(&scope_id));
  });
}

#[test]
fn nonexistent_scope_should_fail() {
  new_test_ext().execute_with(|| {
    create_scope(0);
    assert_noop!(scope_exists(&[1; 32]), Error::<Test>::ScopeNotFound);
  });
}

#[test]
fn permission_exists_should_work() {
  new_test_ext().execute_with(|| {
    let permission_id = create_permission("enroll".as_bytes().to_vec());
    assert_ok!(permission_exists(&permission_id));
  });
}

#[test]
fn nonexistent_permission_should_fail() {
  new_test_ext().execute_with(|| {
    create_permission("enroll".as_bytes().to_vec());
    assert_noop!(permission_exists(&[0; 32]), Error::<Test>::PermissionNotFound);
  });
}

#[test]
fn is_role_linked_to_pallet_should_work() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("owner".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    assert_ok!(is_role_linked_to_pallet(&role_id));
  });
}

#[test]
fn unlinked_role_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("owner".as_bytes().to_vec());
    assert_noop!(is_role_linked_to_pallet(&role_id), Error::<Test>::RoleNotLinkedToPallet);
  });
}

#[test]
fn is_permission_linked_to_role_should_work() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("owner".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    let permission_id = create_permission("enroll".as_bytes().to_vec());
    set_permission_to_role(role_id, permission_id);
    assert_ok!(is_permission_linked_to_role(&role_id, &permission_id));
  });
}

#[test]
fn unlinked_permission_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("owner".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    let permission_id = create_permission("enroll".as_bytes().to_vec());
    assert_noop!(
      is_permission_linked_to_role(&role_id, &permission_id),
      Error::<Test>::PermissionNotLinkedToRole
    );
  });
}

#[test]
fn get_role_users_len_should_work() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let role_id = create_role("owner".as_bytes().to_vec());
    set_role_to_pallet(role_id);

    assert_eq!(get_role_users_len(&scope_id, &role_id), 0);

    assign_role_to_user(0, &scope_id, role_id);
    assign_role_to_user(1, &scope_id, role_id);

    assert_eq!(get_role_users_len(&scope_id, &role_id), 2);
  });
}

#[test]
fn tx_create_and_set_roles_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(RBAC::tx_create_and_set_roles(RuntimeOrigin::root(), pallet_name(), gen_roles(2),));
  });
}

#[test]
fn tx_create_and_set_roles_while_not_root_should_fail() {
  new_test_ext().execute_with(|| {
    assert_noop!(
      RBAC::tx_create_and_set_roles(RuntimeOrigin::signed(0), pallet_name(), gen_roles(2),),
      Error::<Test>::NotAuthorized
    );
  });
}

#[test]
fn tx_remove_role_from_user_should_work() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let role_id = create_role("owner".as_bytes().to_vec());
    let pallet_id = pallet_name();
    set_role_to_pallet(role_id);
    assign_role_to_user(0, &scope_id, role_id);
    assert_ok!(RBAC::tx_remove_role_from_user(
      RuntimeOrigin::root(),
      0,
      pallet_id,
      scope_id,
      role_id,
    ));
  });
}

#[test]
fn tx_remove_role_from_user_while_not_root_should_fail() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let role_id = create_role("owner".as_bytes().to_vec());
    let pallet_id = pallet_name();
    set_role_to_pallet(role_id);
    assign_role_to_user(0, &scope_id, role_id);
    assert_noop!(
      RBAC::tx_remove_role_from_user(RuntimeOrigin::signed(0), 0, pallet_id, scope_id, role_id,),
      Error::<Test>::NotAuthorized
    );
  });
}

#[test]
fn tx_create_and_set_permissions_should_work() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("owner".as_bytes().to_vec());
    let pallet_id = pallet_name();
    let permissions = gen_permissions(2);
    set_role_to_pallet(role_id);
    assert_ok!(RBAC::tx_create_and_set_permissions(
      RuntimeOrigin::root(),
      pallet_id,
      role_id,
      permissions,
    ));
  });
}

#[test]
fn tx_create_and_set_permissions_while_not_root_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("owner".as_bytes().to_vec());
    let pallet_id = pallet_name();
    let permissions = gen_permissions(2);
    set_role_to_pallet(role_id);
    assert_noop!(
      RBAC::tx_create_and_set_permissions(
        RuntimeOrigin::signed(0),
        pallet_id,
        role_id,
        permissions,
      ),
      Error::<Test>::NotAuthorized
    );
  });
}

#[test]
fn tx_assing_role_to_user_should_work() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let role_id = create_role("owner".as_bytes().to_vec());
    let pallet_id = pallet_name();
    set_role_to_pallet(role_id);
    assert_ok!(RBAC::tx_assign_role_to_user(
      RuntimeOrigin::root(),
      0,
      pallet_id,
      scope_id,
      role_id,
    ));
  });
}

#[test]
fn tx_assing_role_to_user_while_not_root_should_fail() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let role_id = create_role("owner".as_bytes().to_vec());
    let pallet_id = pallet_name();
    set_role_to_pallet(role_id);
    assert_noop!(
      RBAC::tx_assign_role_to_user(RuntimeOrigin::signed(0), 0, pallet_id, scope_id, role_id,),
      Error::<Test>::NotAuthorized
    );
  });
}

#[test]
fn does_user_have_any_role_in_scope_should_work() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let role_ids = create_and_set_roles(gen_roles(1));
    let pallet_id = pallet_name();
    let role_id = *role_ids.get(0).unwrap();
    assign_role_to_user(0, &scope_id, role_id);
    assert_eq!(does_user_have_any_role_in_scope(0, pallet_id.clone(), &scope_id), true);
  });
}

#[test]
fn user_that_doesnt_have_any_role_in_scope_should_fail() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let role_ids = create_and_set_roles(gen_roles(1));
    let pallet_id = pallet_name();
    let role_id = *role_ids.get(0).unwrap();
    assign_role_to_user(0, &scope_id, role_id);
    remove_role_from_user(0, &scope_id, role_id);
    assert_eq!(does_user_have_any_role_in_scope(0, pallet_id.clone(), &scope_id), false);
  });
}

#[test]
fn user_that_have_any_role_while_on_multiple_scopes_should_work() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let scope_id_2 = create_scope(1);
    let role_ids = create_and_set_roles(gen_roles(1));
    let pallet_id = pallet_name();
    let role_id = *role_ids.get(0).unwrap();
    assign_role_to_user(0, &scope_id, role_id);
    assign_role_to_user(0, &scope_id_2, role_id);
    assert_eq!(does_user_have_any_role_in_scope(0, pallet_id.clone(), &scope_id), true);
  });
}

#[test]
fn user_that_have_any_role_while_not_matching_scope_should_fail() {
  new_test_ext().execute_with(|| {
    let scope_id = create_scope(0);
    let scope_id_2 = create_scope(1);
    let role_ids = create_and_set_roles(gen_roles(1));
    let pallet_id = pallet_name();
    let role_id = *role_ids.get(0).unwrap();
    assign_role_to_user(0, &scope_id, role_id);
    assert_eq!(does_user_have_any_role_in_scope(0, pallet_id.clone(), &scope_id_2), false);
  });
}

#[test]
fn remove_permission_from_role_should_work() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("owner".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    let p = create_and_set_permissions(role_id, gen_permissions(1));
    revoke_permission_from_role(role_id, p.first().unwrap().to_owned())
  });
}

#[test]
fn remove_nonexistent_permission_from_role_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("owner".as_bytes().to_vec());
    let pallet_id = pallet_name();
    set_role_to_pallet(role_id);
    assert_noop!(
      RBAC::revoke_permission_from_role(RuntimeOrigin::root(), pallet_id, role_id, [0; 32]),
      Error::<Test>::PermissionNotFound
    );
  });
}

#[test]
fn remove_permission_from_unlinked_role_should_fail() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("owner".as_bytes().to_vec());
    let pallet_id = pallet_name();
    let p = create_permission(gen_permissions(1).first().unwrap().clone());
    set_role_to_pallet(role_id);
    assert_noop!(
      RBAC::revoke_permission_from_role(RuntimeOrigin::root(), pallet_id, role_id, p),
      Error::<Test>::PermissionNotLinkedToRole
    );
  });
}

#[test]
fn remove_permission_from_pallet_should_work() {
  new_test_ext().execute_with(|| {
    let role_id = create_role("owner".as_bytes().to_vec());
    set_role_to_pallet(role_id);
    let p = create_and_set_permissions(role_id, gen_permissions(2));
    remove_permission_from_pallet(p.first().unwrap().to_owned());
  });
}

#[test]
fn remove_nonexistent_permission_from_pallet_should_fail() {
  new_test_ext().execute_with(|| {
    assert_noop!(
      RBAC::remove_permission_from_pallet(RuntimeOrigin::root(), pallet_name(), [0; 32]),
      Error::<Test>::PermissionNotFound
    );
  });
}
