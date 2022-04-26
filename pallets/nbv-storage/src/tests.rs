
use crate::{mock::*, Error};
use pallet_identity::{IdentityInfo, Data};
use codec::{Encode};
use frame_support::{
	assert_noop, assert_ok,
	traits::{ConstU32,},
	BoundedVec,
};

fn dummy_identity(name : &[u8] ) -> IdentityInfo<MaxAdditionalFields> {
	IdentityInfo {
		display: Data::Raw(name.to_vec().try_into().unwrap()),
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
	let key = BoundedVec::<u8,ConstU32<32> >::try_from(b"xpub".encode())
				.expect("Error on encoding the xpub key to BoundedVec");
	let mut additional_info = BoundedVec::<(Data, Data),MaxAdditionalFields >::default();
	additional_info.try_push( (Data::Raw(key), Data::None) )
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

fn dummy_xpub() -> BoundedVec<u8,XPubLen >{
	BoundedVec::<u8,XPubLen >::try_from(b"tpubDEMkzn5sBo8Nct35y2BEFhJTqhsa72yeUf5S6ymb85G6LW2okSh1fDkrMhgCtYsrsCAuspm4yVjC63VUA6qrcQ54tVm5TKwhWFBLyyCjabX".encode())
	.expect("Error on encoding the xpub key to BoundedVec")
}

fn dummy_xpub_2() -> BoundedVec<u8,XPubLen >{
	BoundedVec::<u8,XPubLen >::try_from(b"xpub6AHA9hZDN11k2ijHMeS5QqHx2KP9aMBRhTDqANMnwVtdyw2TDYRmF8PjpvwUFcL1Et8Hj59S3gTSMcUQ5gAqTz3Wd8EsMTmF3DChhqPQBnU".encode())
	.expect("Error on encoding the xpub key to BoundedVec")
}


fn dummy_psbt() -> BoundedVec<u8, PSBTMaxLen> {
	BoundedVec::<u8,PSBTMaxLen >::try_from(b"cHNidP8BAK4BAAAAAa5F8SDWH2Hlqgky89rGlhG/4DnKqcbRlL+jQ6F0FBP5AQAAAAD9////AhAnAAAAAAAAR1IhApLkOyyLZWwh3QTadFlmp7x3xt+dPgyL/tQ47r+exVRiIQO7Ut/DRj54BKrR0Kf7c42enyfrbV4TDSpsMiqhfrnQm1KuokkAAAAAAAAiACD0hQx+A3+kUAR7iBY5VjkG2DViANmiP0xOBPixU1x36AAAAAAAAQDqAgAAAAABAecL0e2g6vO11ZpVRcHuBDFZNdXUqcDOmYsg7lK86S3cAAAAAAD+////AlpmwwIAAAAAFgAU370BMJPnoYItIaum9dnKt8LCLI8wdQAAAAAAACIAIILP1EkLWcvTQ15pBdk3paMwDIvglbUG6FQBBon3sRAMAkcwRAIgOYjunqLCM9LhnLS9lVoPSVR6z5Phk9BxodHar/ncgGgCIALhH3N/Q1yD7FxE7SSA9sogkcW3WXH1kxy3BLuMcU1zASECoJ99bEErPxgEAT+Nt7GhfwlgQ24fC//v/3LCUQnpzzBkgSEAAQErMHUAAAAAAAAiACCCz9RJC1nL00NeaQXZN6WjMAyL4JW1BuhUAQaJ97EQDAEFR1IhAip4P8CC/dZji38IFOD6ZjW50Pv3RazsvZExGHoy+MupIQPjlUrnEv00n6ytsa4sIMXdSjKHlXn94P4PBuOifenW51KuIgYCKng/wIL91mOLfwgU4PpmNbnQ+/dFrOy9kTEYejL4y6kMsCkMNwAAAAAAAAAAIgYD45VK5xL9NJ+srbGuLCDF3Uoyh5V5/eD+Dwbjon3p1ucMyqFhVgAAAAAAAAAAAAAiAgMacPy3H41FU/Xw+P81xScxWS/jO5Ny1gGnON1fo+4zbQzKoWFWAQAAAAAAAAAiAgOZ2MtgB/5WFgVoNU56XwjdHdTDuO2TYeQNe8TSV2tq7QywKQw3AQAAAAAAAAAA".encode())
				.expect("Error on encoding the psbt key to BoundedVec")
}


#[test]
fn set_complete_identity_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.

		assert_ok!(NBVStorage::set_complete_identity(Origin::signed(1), Box::new( dummy_identity(b"generic_name") ), dummy_xpub() ));
	});
}

#[test]
fn inserting_same_xpub_should_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!(NBVStorage::set_complete_identity(Origin::signed(1), Box::new( dummy_identity(b"generic_name") ), dummy_xpub() ));
		assert_noop!(NBVStorage::set_complete_identity(Origin::signed(2), Box::new( dummy_identity(b"generic_name") ), dummy_xpub() ),Error::<Test>::XPubAlreadyTaken);
		
	});
}

#[test]
fn removing_xpub_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(NBVStorage::set_complete_identity(Origin::signed(1), Box::new( dummy_identity(b"generic_name") ), dummy_xpub() ));
		assert_ok!(NBVStorage::remove_xpub_from_identity(Origin::signed(1)));
	});
}

#[test]
fn removing_twice_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(NBVStorage::set_complete_identity(Origin::signed(1), Box::new( dummy_identity(b"generic_name") ), dummy_xpub() ));
		assert_ok!(NBVStorage::remove_xpub_from_identity(Origin::signed(1)));
		assert_noop!(NBVStorage::remove_xpub_from_identity(Origin::signed(1)), Error::<Test>::XPubNotFound);
		
	});
}

#[test]
fn updating_identity_should_work() {
	new_test_ext().execute_with(|| {

		assert_ok!(NBVStorage::set_complete_identity(Origin::signed(1), Box::new( dummy_identity(b"generic_name") ), dummy_xpub() ));
		assert_ok!(NBVStorage::set_complete_identity(Origin::signed(1), Box::new( dummy_identity(b"generic_name_dos") ), dummy_xpub_2() ));
		assert_ok!(NBVStorage::set_complete_identity(Origin::signed(1), Box::new( dummy_identity(b"generic_name_tres") ), dummy_xpub_2()));
		
	});
}

#[test]
fn inserting_invalid_field_should_not_work() {
	new_test_ext().execute_with(|| {

		assert_noop!(NBVStorage::set_complete_identity(Origin::signed(1), Box::new( dummy_wrong_identity() ), dummy_xpub() ),
		Error::<Test>::InvalidAdditionalField);
		
	});
}


#[test]
fn setting_psbt_should_work() {
	new_test_ext().execute_with(|| {
		
		assert_ok!(NBVStorage::set_psbt(Origin::signed(1) , dummy_psbt() ) );
		
	});
}
