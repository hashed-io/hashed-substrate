use crate::{mock::*, Error, types::{RoleBasedAccessControl, RoleId, ScopeId}, Config};
use frame_support::{assert_noop, assert_ok, assert_err};

type AccountId = <Test as frame_system::Config>::AccountId;
const PALLET_ID : u64 = 1;

fn create_scope(n: u8)->ScopeId{
	let scope_id = [n;32];
	assert_ok!(RBAC::create_scope(PALLET_ID, scope_id));
	assert!(RBAC::scopes(1).contains(&scope_id));
	scope_id
}

fn gen_roles(n_roles: u32)-> Vec<Vec<u8>>{
	let mut v = Vec::new();
	for i in 0..n_roles{
		v.push(format!("role{}",i).into_bytes().to_vec());
	}
	v
}

fn create_role(role: Vec<u8>)->RoleId{
	let r_id = RBAC::create_role(role.clone()).unwrap();
	assert_eq!(RBAC::roles(r_id).unwrap().to_vec(), role);
	r_id
}

fn set_role_to_pallet(role_id: RoleId){
	assert_ok!(RBAC::set_role_to_pallet(PALLET_ID, role_id));
}

fn set_multiple_pallet_roles(roles: Vec<RoleId>){
	assert_ok!(RBAC::set_multiple_pallet_roles(PALLET_ID, roles));
}

fn remove_scope(n: u8){
	assert_ok!(RBAC::remove_scope(PALLET_ID, [n;32]));
	assert!(!RBAC::scopes(PALLET_ID).contains(&[n;32]));
	// TODO check that other storage maps were removed too
}

fn remove_role_from_user(user: AccountId, scope_id: &ScopeId, role_id: RoleId){
	assert_ok!(RBAC::remove_role_from_user(user, PALLET_ID, scope_id, role_id));
	let user_roles = RBAC::roles_by_user((user, PALLET_ID, scope_id));
	assert!(!user_roles.contains(&role_id));
	let role_users = RBAC::users_by_scope((PALLET_ID, scope_id, role_id));
	assert!(!role_users.contains(&user));
}

fn remove_pallet_storage(){
	assert_ok!(RBAC::remove_pallet_storage(PALLET_ID));
	// TODO: Check that other storage maps were removed too
}

fn assign_role_to_user(user: AccountId, scope_id : &ScopeId, role_id: RoleId){
	assert_ok!(
		RBAC::assign_role_to_user(user, PALLET_ID, scope_id, role_id)
	);
	let user_roles = RBAC::roles_by_user((user,PALLET_ID, scope_id));
	assert!(user_roles.contains(&role_id));
	let role_users = RBAC::users_by_scope((PALLET_ID, scope_id, role_id));
	assert!(role_users.contains(&user));
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
		assert_noop!(RBAC::create_scope(1, [0;32]), Error::<Test>::ScopeAlreadyExists);
	});
}

#[test]
fn exceeding_max_scopes_per_pallet_should_fail() {
	new_test_ext().execute_with(|| {
		for n in 0..<Test as Config>::MaxScopesPerPallet::get(){
			create_scope(n.try_into().unwrap());
		}
		assert_noop!(RBAC::create_scope(1, [255;32]), Error::<Test>::ExceedMaxScopesPerPallet);
	});
}

#[test]
fn remove_scope_works() {
	new_test_ext().execute_with(|| {
		create_scope(0);
		// TODO: add roles to pallet and users to scope
		remove_scope(0);
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
		assert_noop!(
			RBAC::set_role_to_pallet(PALLET_ID, [0;32]),
			Error::<Test>::RoleNotFound
		);
	});
}

#[test]
fn set_role_to_pallet_twice_should_fail() {
	new_test_ext().execute_with(|| {
		let role_id = create_role("owner".as_bytes().to_vec());
		set_role_to_pallet(role_id);
		assert_noop!(
			RBAC::set_role_to_pallet(PALLET_ID, role_id),
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
			RBAC::set_role_to_pallet(PALLET_ID, role_id),
			Error::<Test>::ExceedMaxRolesPerPallet
		);
	});
}

#[test]
fn set_multiple_pallet_roles_should_work() {
	new_test_ext().execute_with(|| {
		let n_roles = <Test as Config>::MaxRolesPerPallet::get()-1;
		let role_ids: Vec<RoleId> = gen_roles(n_roles).iter().map(|role|{
			create_role(role.clone())
		}).collect();
		set_multiple_pallet_roles(role_ids);
	});
}

#[test]
fn set_multiple_duplicate_pallet_roles_should_fail() {
	new_test_ext().execute_with(|| {
		let n_roles = <Test as Config>::MaxRolesPerPallet::get()-1;
		let mut roles = gen_roles(n_roles);
		roles.push("role0".as_bytes().to_vec());
		let role_ids: Vec<RoleId> = roles.iter().map(|role|{
			create_role(role.clone())
		}).collect();
		assert_noop!(
			RBAC::set_multiple_pallet_roles(PALLET_ID, role_ids),
			Error::<Test>::DuplicateRole
		);
	});
}

#[test]
fn set_multiple_pallet_roles_twice_should_fail() {
	new_test_ext().execute_with(|| {
		let n_roles = <Test as Config>::MaxRolesPerPallet::get();
		let roles = gen_roles(n_roles);
		let role_ids: Vec<RoleId> = roles.iter().map(|role|{
			create_role(role.clone())
		}).collect();
		set_multiple_pallet_roles(role_ids.clone());
		assert_noop!(
			RBAC::set_multiple_pallet_roles(PALLET_ID, role_ids),
			Error::<Test>::RoleAlreadyLinkedToPallet
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
fn assign_role_to_user_without_scope_should_fail() {
	new_test_ext().execute_with(|| {
		let role_id = create_role("owner".as_bytes().to_vec());
		set_role_to_pallet(role_id);
		assert_noop!(
			RBAC::assign_role_to_user(0, PALLET_ID, &[0;32], role_id),
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
		let role_ids: Vec<RoleId> = roles.iter().map(|role|{
			create_role(role.clone())
		}).collect();
		set_multiple_pallet_roles(role_ids.clone());
		role_ids.iter().for_each(|role_id|{
			assign_role_to_user(0, &scope_id, *role_id);
		});
		let last_role_id = create_role("owner".as_bytes().to_vec());
		set_role_to_pallet(last_role_id);
		assert_noop!(
			RBAC::assign_role_to_user(0, PALLET_ID, &scope_id, last_role_id),
			Error::<Test>::ExceedMaxRolesPerUser
		);
	});
}

#[test]
fn exceeding_max_users_per_role_should_fail() {
	new_test_ext().execute_with(|| {
		let scope_id = create_scope(0);
		let role_id = create_role("owner".as_bytes().to_vec());
		let max_users_per_role =  <Test as Config>::MaxUsersPerRole::get();
		set_role_to_pallet(role_id);
		for i in 0..max_users_per_role{
			assign_role_to_user(i.into(), &scope_id, role_id)
		}
		// avoiding assert_noop because it checks if the storage mutated 
		assert_err!(
			RBAC::assign_role_to_user((max_users_per_role+1).into(), PALLET_ID, &scope_id, role_id),
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
			RBAC::remove_role_from_user(0, PALLET_ID, &scope_id, [0;32]),
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
			RBAC::remove_role_from_user(0, PALLET_ID, &scope_id, [0;32]),
			Error::<Test>::RoleNotFound
		);
	});
}

