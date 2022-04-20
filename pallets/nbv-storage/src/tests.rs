
use crate::{mock::*, Error};
use pallet_identity::{IdentityInfo, Data};
use codec::{Decode, Encode};
use frame_support::{
	assert_noop, assert_ok, ord_parameter_types, parameter_types,
	traits::{ConstU32, ConstU64, EnsureOneOf},
	BoundedVec, assert_err, assert_err_ignore_postinfo,
};
use frame_system::{EnsureRoot, EnsureSignedBy};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BadOrigin, BlakeTwo256, IdentityLookup},
};

/*
test:
same xpub (should fail)
preimage too large (should fail and deploy error) done
insert additional fields
not enough tokens
*/

fn dummy_identity() -> IdentityInfo<MaxAdditionalFields> {
	IdentityInfo {
		display: Data::Raw(b"ten".to_vec().try_into().unwrap()),
		legal: Data::Raw(b"The Right Ordinal Ten, Esq.".to_vec().try_into().unwrap()),
		web: Data::None,
		riot: Data::None,
		email: Data::None,
		image: Data::None,
		twitter: Data::None,
		additional: Default::default(),
		pgp_fingerprint: Default::default(),
	}
}

fn dummy_wrong_identity() -> IdentityInfo<MaxAdditionalFields> {
	let mut additional_info = BoundedVec::<(Data, Data),MaxAdditionalFields >::default();
	additional_info.try_push( (Data::None, Data::None) )
		.expect("Error pushing additional info while building dummy info");
	IdentityInfo {
		display: Data::Raw(b"ten".to_vec().try_into().unwrap()),
		legal: Data::Raw(b"The Right Ordinal Ten, Esq.".to_vec().try_into().unwrap()),
		web: Data::None,
		riot: Data::None,
		email: Data::None,
		image: Data::None,
		twitter: Data::None,
		additional: additional_info,
		pgp_fingerprint: Default::default(),
	}
}

#[test]
fn set_complete_identity_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let xpub = BoundedVec::<u8,XPubLen >::try_from(b"generic_xpub".encode())
				.expect("Error on encoding the xpub key to BoundedVec");
		assert_ok!(NBVStorage::set_complete_identity(Origin::signed(1), Box::new( dummy_identity() ), xpub ));
	});
}

#[test]
fn inserting_same_xpub_should_fail() {
	new_test_ext().execute_with(|| {
		let xpub = BoundedVec::<u8,XPubLen >::try_from(b"generic_xpub".encode())
				.expect("Error on encoding the xpub key to BoundedVec");
		assert_ok!(NBVStorage::set_complete_identity(Origin::signed(1), Box::new( dummy_identity() ), xpub.clone() ));
		assert_noop!(NBVStorage::set_complete_identity(Origin::signed(2), Box::new( dummy_identity() ), xpub ),Error::<Test>::XPubAlreadyTaken);
		
	});
}

#[test]
fn removing_xpub_should_work() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let xpub = BoundedVec::<u8,XPubLen >::try_from(b"generic_xpub".encode())
				.expect("Error on encoding the xpub key to BoundedVec");
		assert_ok!(NBVStorage::set_complete_identity(Origin::signed(1), Box::new( dummy_identity() ), xpub ));
		assert_ok!(NBVStorage::remove_xpub_from_identity(Origin::signed(1)));
	});
}

#[test]
fn set_psbt_not_implemented() {
	new_test_ext().execute_with(|| {
		assert_noop!(NBVStorage::set_psbt(Origin::signed(1)),Error::<Test>::NotYetImplemented);
		
	});
}

// #[test]
// fn inserting_too_many_fields_should_fail() {
// 	new_test_ext().execute_with(|| {
// 		let xpub = BoundedVec::<u8,XPubLen >::try_from(b"generic_xpub".encode())
// 				.expect("Error on encoding the xpub key to BoundedVec");
// 		assert_err_ignore_postinfo!(NBVStorage::set_complete_identity(Origin::signed(1), Box::new( dummy_wrong_identity() ), xpub ),
// 			"Module(ModuleError { index: 4, error: 11, message: Some(\"TooManyFields\") }) }"
// 		);
// 		Identity::error_metadata()
// 	});
// }

// #[test]
// fn inserting_too_long_xpub_should_fail() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		let xpub = BoundedVec::<u8,ConstU32<113> >::
// 		try_from(b"tpubDEMkzn5sBo8Nct35y2BEFhJTqhsa72yeUf5S6ymb85G6LW2okSh1fDkrMhgCtYsrsCAuspm4yVjC63VUA6qrcQ54tVm5TKwhWFBLyyCjabX12".encode())
// 		  .expect("Error on encoding the xpub key to BoundedVec");
// 		  assert_ok!(NBVStorage::set_complete_identity(Origin::signed(1), Box::new( dummy_identity() ), xpub ));
// 	});
// }