use crate::{mock::*, Error, types::*, Config};
use frame_support::{assert_ok, BoundedVec, traits::ConstU32, assert_noop};
use pallet_rbac::types::RoleBasedAccessControl;
use sp_io::hashing::blake2_256;
use std::vec;

type RbacErr = pallet_rbac::Error<Test>;

fn pallet_id() ->[u8;32]{
	FundAdmin::pallet_id().to_id()
}

fn pallet_name()-> pallet_rbac::types::IdOrVec {
	pallet_rbac::types::IdOrVec::Vec(
		"pallet_test".as_bytes().to_vec()
	)
}

fn return_field_name(name: &str) -> FieldName {
    let name: BoundedVec<u8, ConstU32<100>> = name.as_bytes().to_vec().try_into().unwrap_or_default();
    name
} 
// U S E R S
// -----------------------------------------------------------------------------------------
#[test]
fn register_admin_works() {
    new_test_ext().execute_with(|| {
        let admin = 1;
    });
}

// #[test]
// fn sudo_register_administrator_account_works() {
//     new_test_ext().execute_with(|| {
//         // let alice_name = return_field_name("Alice Keys");
//         // assert_ok!(FundAdmin::sudo_add_administrator(
//         //     Origin::signed(1),
//         //     1,
//         //     alice_name.clone()
//         // ));

//     });
// }

