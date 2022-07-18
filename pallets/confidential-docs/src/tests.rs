use crate::{mock::*, Error, types::*};
use codec::Encode;
use frame_support::{assert_noop, assert_ok, sp_io::hashing::blake2_256};


fn generate_user_id(id: &str) -> UserId {
	format!("user id: {}", id).using_encoded(blake2_256)
}

fn generate_public_key(id: &str) -> PublicKey {
	format!("public key: {}", id).using_encoded(blake2_256)
}

fn generate_cid(id: &str) -> CID {
	format!("cid{}", id).encode().try_into().unwrap()
}

fn generate_document(id: &str) -> Document<Test>{
	Document{
		name: format!("doc name:{}", id).encode().try_into().unwrap(),
		description: format!("doc desc:{}", id).encode().try_into().unwrap()
	}
}



#[test]
fn set_vault_works() {
	new_test_ext().execute_with(|| {
		let user_id = generate_user_id("1");
		let public_key = generate_public_key("1");
		let cid = generate_cid("1");
		assert_ok!(ConfidentialDocs::set_vault(Origin::signed(1), user_id, public_key, cid.clone()));
		// Read pallet storage and assert an expected result.
		assert_eq!(ConfidentialDocs::vaults(user_id), Some(Vault{
			cid,
			owner: 1
		}));
		assert_eq!(ConfidentialDocs::public_keys(1), Some(public_key));
	});
}

#[test]
fn set_vault_should_fail_for_empty_cid() {
	new_test_ext().execute_with(|| {
		let user_id = generate_user_id("1");
		let public_key = generate_public_key("1");
		let cid: CID = Vec::new().try_into().unwrap();
		assert_noop!(ConfidentialDocs::set_vault(Origin::signed(1), user_id, public_key, cid), Error::<Test>::CIDNoneValue);		
	});
}

#[test]
fn set_vault_should_fail_for_user_with_vault() {
	new_test_ext().execute_with(|| {
		let user_id = generate_user_id("1");
		let public_key = generate_public_key("1");
		let cid = generate_cid("1");
		assert_ok!(ConfidentialDocs::set_vault(Origin::signed(1), user_id, public_key, cid.clone()));
		assert_noop!(ConfidentialDocs::set_vault(Origin::signed(1), user_id, public_key, cid), Error::<Test>::UserAlreadyHasVault);		
	});
}

#[test]
fn set_vault_should_fail_for_account_with_public_key() {
	new_test_ext().execute_with(|| {
		let public_key = generate_public_key("1");
		let cid = generate_cid("1");
		assert_ok!(ConfidentialDocs::set_vault(Origin::signed(1), generate_user_id("1"), public_key, cid.clone()));
		assert_noop!(ConfidentialDocs::set_vault(Origin::signed(1), generate_user_id("2"), public_key, cid), Error::<Test>::AccountAlreadyHasPublicKey);		
	});
}

#[test]
fn set_document_works() {
	new_test_ext().execute_with(|| {
		let cid = generate_cid("1");
		let document = generate_document("1");
		assert_ok!(ConfidentialDocs::set_document(Origin::signed(1), cid.clone(), document.name.clone(), document.description.clone()));
		// Read pallet storage and assert an expected result.
		assert_eq!(ConfidentialDocs::documents(1, cid), Some(document));
	});
}

#[test]
fn set_document_should_fail_for_empty_cid() {
	new_test_ext().execute_with(|| {
		let cid: CID = Vec::new().try_into().unwrap();
		let document = generate_document("1");
		assert_noop!(ConfidentialDocs::set_document(Origin::signed(1), cid.clone(), document.name.clone(), document.description.clone()), Error::<Test>::CIDNoneValue);		
	});
}

#[test]
fn set_document_should_fail_for_name_too_short() {
	new_test_ext().execute_with(|| {
		let cid: CID = Vec::new().try_into().unwrap();
		let document = generate_document("1");
		assert_noop!(ConfidentialDocs::set_document(Origin::signed(1), cid.clone(), "as".encode().try_into().unwrap(), document.description.clone()), Error::<Test>::CIDNoneValue);		
	});
}

#[test]
fn set_document_should_fail_for_description_too_short() {
	new_test_ext().execute_with(|| {
		let cid: CID = Vec::new().try_into().unwrap();
		let document = generate_document("1");
		assert_noop!(ConfidentialDocs::set_document(Origin::signed(1), cid.clone(), document.name.clone(), "desc".encode().try_into().unwrap()), Error::<Test>::CIDNoneValue);		
	});
}
