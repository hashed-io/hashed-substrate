use crate::{mock::*, Error, types::RoleBasedAccessControl, Config};
use frame_support::{assert_noop, assert_ok};


const PALLET_ID : u64 = 1;

fn create_scope(n: u8){
	assert_ok!(RBAC::create_scope(PALLET_ID, [n;32]));
	assert!(RBAC::scopes(1).contains(&[n;32]));
}

fn create_role(role: &str){
	let r_id = RBAC::create_role(role.as_bytes().to_vec());
	assert_ok!(RBAC::create_role(role.as_bytes().to_vec()));
}

fn remove_scope(n: u8){
	assert_ok!(RBAC::remove_scope(PALLET_ID, [n;32]));
	assert!(!RBAC::scopes(PALLET_ID).contains(&[n;32]));
	// TODO check that other storage maps were removed too
}

fn remove_pallet_storage(){
	assert_ok!(RBAC::remove_pallet_storage(PALLET_ID));
	// TODO: Check that other storage maps were removed too
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
fn create_role_should_work() {
	new_test_ext().execute_with(|| {
		
	});
}

#[test]
fn exceeding_max_scopes_per_pallet_should_fail() {
	new_test_ext().execute_with(|| {
		for n in 0..<Test as Config>::MaxScopesPerPallet::get(){
			create_scope(n.try_into().unwrap())
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