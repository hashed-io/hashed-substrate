use crate::{mock::*, types::*, Error};
use codec::Encode;
use frame_support::{assert_noop, assert_ok, sp_io::hashing::blake2_256};
use frame_system as system;

fn generate_user_id(id: &str) -> UserId {
	format!("user id: {}", id).using_encoded(blake2_256)
}

fn generate_public_key(id: &str) -> PublicKey {
	format!("public key: {}", id).using_encoded(blake2_256)
}

fn generate_cid(id: &str) -> CID {
	format!("cid: {}", id).encode().try_into().unwrap()
}

fn generate_doc_name(id: &str) -> DocName<Test> {
	format!("doc name:{}", id).encode().try_into().unwrap()
}

fn generate_doc_desc(id: &str) -> DocDesc<Test> {
	format!("doc desc:{}", id).encode().try_into().unwrap()
}

fn generate_owned_doc(
	id: &str,
	owner: <Test as system::Config>::AccountId,
) -> OwnedDoc<Test> {
	OwnedDoc {
		cid: generate_cid(id),
		name: generate_doc_name(id),
		description: generate_doc_desc(id),
		owner,
	}
}

fn generate_shared_doc(
	id: &str,
	from: <Test as system::Config>::AccountId,
	to: <Test as system::Config>::AccountId,
) -> SharedDoc<Test> {
	SharedDoc {
		cid: generate_cid(id),
		name: generate_doc_name(id),
		description: generate_doc_desc(id),
		from,
		to,
	}
}

fn setup_vault(who: <Test as system::Config>::AccountId) {
	let id = &who.to_string();
	assert_ok!(ConfidentialDocs::set_vault(
		Origin::signed(who),
		generate_user_id(id),
		generate_public_key(id),
		generate_cid(id)
	));
}

fn setup_owned_doc(id: &str, owner: <Test as system::Config>::AccountId) -> OwnedDoc<Test> {
	let doc = generate_owned_doc(id, owner);
	assert_ok!(ConfidentialDocs::set_owned_document(Origin::signed(owner), doc.clone()));
	assert_owned_doc(&doc);
	doc
}

fn setup_shared_doc(id: &str, from: <Test as system::Config>::AccountId, to: <Test as system::Config>::AccountId) -> SharedDoc<Test> {
	let doc = generate_shared_doc(id, from, to);
	assert_ok!(ConfidentialDocs::share_document(Origin::signed(from), doc.clone()));
	assert_shared_doc(&doc);
	doc
}

fn assert_owned_doc(doc: &OwnedDoc<Test>){
	assert_eq!(ConfidentialDocs::owned_docs(&doc.cid), Some(doc.clone()));
	let owned_docs = ConfidentialDocs::owned_docs_by_owner(doc.owner);
	assert_eq!(owned_docs.contains(&doc.cid), true);
}

fn assert_owned_doc_not_exists(cid: &CID, owner: <Test as system::Config>::AccountId){
	assert_eq!(ConfidentialDocs::owned_docs(cid), None);
	let owned_docs = ConfidentialDocs::owned_docs_by_owner(owner);
	assert_eq!(owned_docs.contains(cid), false);
}


fn assert_shared_doc(doc: &SharedDoc<Test>){
	let SharedDoc {
		from,
		to,
		..
	} = doc;
	assert_eq!(ConfidentialDocs::shared_docs(&doc.cid), Some(doc.clone()));
	assert_eq!(ConfidentialDocs::shared_docs_by_to(to).contains(&doc.cid), true);
	assert_eq!(ConfidentialDocs::shared_docs_by_from(from).contains(&doc.cid), true);
}

fn assert_shared_doc_not_exists(doc: &SharedDoc<Test>){
	let SharedDoc {
		from,
		to,
		..
	} = doc;
	assert_eq!(ConfidentialDocs::shared_docs(&doc.cid), None);
	assert_eq!(ConfidentialDocs::shared_docs_by_to(to).contains(&doc.cid), false);
	assert_eq!(ConfidentialDocs::shared_docs_by_from(from).contains(&doc.cid), false);
}

#[test]
fn set_vault_works() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		let user_id = generate_user_id("1");
		let public_key = generate_public_key("1");
		let cid = generate_cid("1");
		assert_ok!(ConfidentialDocs::set_vault(Origin::signed(owner), user_id, public_key, cid.clone()));
		// Read pallet storage and assert an expected result.
		let vault = Vault { cid, owner };
		assert_eq!(ConfidentialDocs::vaults(user_id), Some(vault));
		assert_eq!(ConfidentialDocs::public_keys(owner), Some(public_key));

		let public_key = generate_public_key("2");
		let cid = generate_cid("2");
		assert_ok!(ConfidentialDocs::set_vault(Origin::signed(owner), user_id, public_key, cid.clone()));
		// Read pallet storage and assert an expected result.
		let vault = Vault { cid, owner };
		assert_eq!(ConfidentialDocs::vaults(user_id), Some(vault));
		assert_eq!(ConfidentialDocs::public_keys(owner), Some(public_key));
		// assert_eq!(last_event(), Event::VaultStored(user_id, public_key, ))
	});
}

#[test]
fn set_vault_should_fail_for_empty_cid() {
	new_test_ext().execute_with(|| {
		let user_id = generate_user_id("1");
		let public_key = generate_public_key("1");
		let cid: CID = Vec::new().try_into().unwrap();
		assert_noop!(
			ConfidentialDocs::set_vault(Origin::signed(1), user_id, public_key, cid),
			Error::<Test>::CIDNoneValue
		);
	});
}

#[test]
fn set_vault_should_fail_for_origin_not_owner_of_user_id() {
	new_test_ext().execute_with(|| {
		let user_id = generate_user_id("1");
		let public_key = generate_public_key("1");
		let cid = generate_cid("1");
		assert_ok!(ConfidentialDocs::set_vault(Origin::signed(1), user_id, public_key, cid.clone()));
		assert_noop!(
			ConfidentialDocs::set_vault(Origin::signed(1), generate_user_id("2"), public_key, cid),
			Error::<Test>::NotOwnerOfUserId
		);
	});
}

#[test]
fn set_vault_should_fail_for_origin_not_owner_of_vault() {
	new_test_ext().execute_with(|| {
		let user_id = generate_user_id("1");
		let public_key = generate_public_key("1");
		let cid = generate_cid("1");
		assert_ok!(ConfidentialDocs::set_vault(Origin::signed(1), user_id, public_key, cid.clone()));
		assert_noop!(
			ConfidentialDocs::set_vault(Origin::signed(2), user_id, public_key, cid),
			Error::<Test>::NotOwnerOfVault
		);
	});
}


#[test]
fn set_owned_document_works() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		setup_vault(owner);
		let mut doc1 = generate_owned_doc("1", owner);
		assert_ok!(ConfidentialDocs::set_owned_document(Origin::signed(owner), doc1.clone()));
		assert_eq!(ConfidentialDocs::owned_docs(&doc1.cid), Some(doc1.clone()));
		let owned_docs = ConfidentialDocs::owned_docs_by_owner(owner);
		let expected_cid_vec = vec!(doc1.cid.clone());
		assert_eq!(owned_docs.into_inner(), expected_cid_vec);
		doc1.name = generate_doc_name("2");
		doc1.description = generate_doc_desc("2");
		assert_ok!(ConfidentialDocs::set_owned_document(Origin::signed(owner), doc1.clone()));
		assert_eq!(ConfidentialDocs::owned_docs(&doc1.cid), Some(doc1.clone()));
		let owned_docs = ConfidentialDocs::owned_docs_by_owner(owner);
		assert_eq!(owned_docs.into_inner(), expected_cid_vec);
	});
}

#[test]
fn set_owned_document_should_fail_for_updating_non_owned_doc() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		setup_vault(owner);
		let mut doc1 = generate_owned_doc("1", owner);
		assert_ok!(ConfidentialDocs::set_owned_document(Origin::signed(owner), doc1.clone()));
		doc1.name = generate_doc_name("2");
		let owner = 2;
		setup_vault(owner);
		assert_noop!(ConfidentialDocs::set_owned_document(Origin::signed(owner), doc1.clone()), Error::<Test>::NotDocOwner);
	});
}

#[test]
fn set_owned_document_should_fail_for_empty_cid() {
	new_test_ext().execute_with(|| {
		let mut doc = generate_owned_doc("1", 1);
		doc.cid = Vec::new().try_into().unwrap();
		assert_noop!(ConfidentialDocs::set_owned_document(Origin::signed(1), doc.clone()), Error::<Test>::CIDNoneValue);
	});
}

#[test]
fn set_owned_document_should_fail_for_name_too_short() {
	new_test_ext().execute_with(|| {
		let mut doc = generate_owned_doc("1", 1);
		doc.name = "as".encode().try_into().unwrap();
		assert_noop!(ConfidentialDocs::set_owned_document(Origin::signed(1), doc.clone()), Error::<Test>::DocNameTooShort);
	});
}

#[test]
fn set_owned_document_should_fail_for_description_too_short() {
	new_test_ext().execute_with(|| {
		let mut doc = generate_owned_doc("1", 1);
		doc.description = "des".encode().try_into().unwrap();
		assert_noop!(ConfidentialDocs::set_owned_document(Origin::signed(1), doc.clone()), Error::<Test>::DocDescTooShort);
	});
}

#[test]
fn set_owned_document_should_fail_for_owner_with_no_public_key() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		let doc = generate_owned_doc("1", owner);
		assert_noop!(ConfidentialDocs::set_owned_document(Origin::signed(owner), doc.clone()), Error::<Test>::AccountHasNoPublicKey);
	});
}

#[test]
fn remove_owned_document_works() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		setup_vault(owner);
		let doc1 = setup_owned_doc("1", owner);
		let doc2 = setup_owned_doc("2", owner);
		assert_ok!(ConfidentialDocs::remove_owned_document(Origin::signed(owner), doc1.cid.clone()));
		assert_owned_doc_not_exists(&doc1.cid, owner);
		assert_owned_doc(&doc2);
		assert_ok!(ConfidentialDocs::remove_owned_document(Origin::signed(owner), doc2.cid.clone()));
		assert_owned_doc_not_exists(&doc2.cid, owner);
	});
}

#[test]
fn remove_owned_document_should_fail_for_non_existant_document() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		let doc1 = generate_owned_doc("1", owner);
		assert_noop!(ConfidentialDocs::remove_owned_document(Origin::signed(owner), doc1.cid.clone()), Error::<Test>::DocNotFound);
	});
}

#[test]
fn remove_owned_document_should_fail_for_not_document_owner() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		setup_vault(owner);
		let doc1 = setup_owned_doc("1", owner);
		let not_owner = 2;
		assert_noop!(ConfidentialDocs::remove_owned_document(Origin::signed(not_owner), doc1.cid.clone()), Error::<Test>::NotDocOwner);
	});
}

#[test]
fn share_document_works() {
	new_test_ext().execute_with(|| {
		let to = 1;
		let from = 2;
		setup_vault(to);
		setup_vault(from);
		let shared_doc1 = generate_shared_doc("1", from, to);
		assert_ok!(ConfidentialDocs::share_document(Origin::signed(from), shared_doc1.clone()));
		// Read pallet storage and assert an expected result.
		assert_eq!(ConfidentialDocs::shared_docs(&shared_doc1.cid), Some(shared_doc1.clone()));
		let shared_docs_to = ConfidentialDocs::shared_docs_by_to(to);
		let mut expected_cid_to_vec = vec!(shared_doc1.cid.clone());
		assert_eq!(shared_docs_to.into_inner(), expected_cid_to_vec);
		let expected_cid_from_vec = vec!(shared_doc1.cid.clone());
		let shared_docs_from = ConfidentialDocs::shared_docs_by_from(from);
		assert_eq!(shared_docs_from.into_inner(), expected_cid_from_vec);

		let from = 3;
		setup_vault(3);
		let shared_doc2 = generate_shared_doc("2", from, to);
		assert_ok!(ConfidentialDocs::share_document(Origin::signed(from), shared_doc2.clone()));
		assert_eq!(ConfidentialDocs::shared_docs(&shared_doc2.cid), Some(shared_doc2.clone()));
		let shared_docs_to = ConfidentialDocs::shared_docs_by_to(to);
		expected_cid_to_vec.push(shared_doc2.cid.clone());
		assert_eq!(shared_docs_to.into_inner(), expected_cid_to_vec);
		let expected_cid_from_vec = vec!(shared_doc2.cid.clone());
		let shared_docs_from = ConfidentialDocs::shared_docs_by_from(from);
		assert_eq!(shared_docs_from.into_inner(), expected_cid_from_vec);
	});
}

#[test]
fn share_document_should_fail_for_empty_cid() {
	new_test_ext().execute_with(|| {
		let mut shared_doc = generate_shared_doc("1", 1, 2);
		shared_doc.cid = Vec::new().try_into().unwrap();
		assert_noop!(
			ConfidentialDocs::share_document(Origin::signed(1), shared_doc.clone()),
			Error::<Test>::CIDNoneValue
		);
	});
}

#[test]
fn share_document_should_fail_for_name_too_short() {
	new_test_ext().execute_with(|| {
		let mut shared_doc = generate_shared_doc("1", 1, 2);
		shared_doc.name = "as".encode().try_into().unwrap();
		assert_noop!(
			ConfidentialDocs::share_document(Origin::signed(1), shared_doc.clone()),
			Error::<Test>::DocNameTooShort
		);
	});
}

#[test]
fn share_document_should_fail_for_desc_too_short() {
	new_test_ext().execute_with(|| {
		let mut shared_doc = generate_shared_doc("1", 1, 2);
		shared_doc.description = "des".encode().try_into().unwrap();
		assert_noop!(
			ConfidentialDocs::share_document(Origin::signed(1), shared_doc.clone()),
			Error::<Test>::DocDescTooShort
		);
	});
}

#[test]
fn share_document_should_fail_for_share_to_self() {
	new_test_ext().execute_with(|| {
		let to = 1;
		let from = 1;
		setup_vault(to);
		let shared_doc = generate_shared_doc("1", from, to);
		assert_noop!(
			ConfidentialDocs::share_document(Origin::signed(from), shared_doc.clone()),
			Error::<Test>::DocSharedWithSelf
		);
	});
}

#[test]
fn share_document_should_fail_for_doc_already_shared() {
	new_test_ext().execute_with(|| {
		let to = 1;
		let from = 2;
		setup_vault(to);
		setup_vault(from);
		let shared_doc = generate_shared_doc("1", from, to);
		assert_ok!(ConfidentialDocs::share_document(Origin::signed(from), shared_doc.clone()));
		assert_noop!(
			ConfidentialDocs::share_document(Origin::signed(from), shared_doc.clone()),
			Error::<Test>::DocAlreadyShared
		);
	});
}

#[test]
fn share_document_should_fail_for_from_with_no_public_key() {
	new_test_ext().execute_with(|| {
		let to = 1;
		let from = 2;
		setup_vault(to);
		let shared_doc = generate_shared_doc("1", from, to);
		assert_noop!(
			ConfidentialDocs::share_document(Origin::signed(from), shared_doc.clone()),
			Error::<Test>::AccountHasNoPublicKey
		);
	});
}

#[test]
fn share_document_should_fail_for_to_with_no_public_key() {
	new_test_ext().execute_with(|| {
		let to = 1;
		let from = 2;
		setup_vault(from);
		let shared_doc = generate_shared_doc("1", from, to);
		assert_noop!(
			ConfidentialDocs::share_document(Origin::signed(from), shared_doc.clone()),
			Error::<Test>::AccountHasNoPublicKey
		);
	});
}
#[test]
fn update_shared_document_metadata_works() {
	new_test_ext().execute_with(|| {
		let to = 1;
		let from = 2;
		setup_vault(to);
		setup_vault(from);
		let mut shared_doc1 = setup_shared_doc("1", from, to);
		shared_doc1.name = generate_doc_name("2");
		shared_doc1.description = generate_doc_desc("2");
		assert_ok!(ConfidentialDocs::update_shared_document_metadata(Origin::signed(to), shared_doc1.clone()));
		assert_shared_doc(&shared_doc1);
	});
}

#[test]
fn update_shared_document_metadata_should_fail_for_non_existant_doc() {
	new_test_ext().execute_with(|| {
		let to = 1;
		let from = 2;
		let shared_doc1 = generate_shared_doc("1", from, to);
		assert_noop!(ConfidentialDocs::update_shared_document_metadata(Origin::signed(to), shared_doc1.clone()), Error::<Test>::DocNotFound);
	});
}

#[test]
fn update_shared_document_metadata_should_fail_for_not_doc_sharee() {
	new_test_ext().execute_with(|| {
		let to1 = 1;
		let to2 = 2;
		let from = 3;
		setup_vault(to1);
		setup_vault(to2);
		setup_vault(from);
		let shared_doc1 = setup_shared_doc("1", from, to1);
		assert_noop!(ConfidentialDocs::update_shared_document_metadata(Origin::signed(to2), shared_doc1.clone()), Error::<Test>::NotDocSharee);
		assert_noop!(ConfidentialDocs::update_shared_document_metadata(Origin::signed(from), shared_doc1.clone()), Error::<Test>::NotDocSharee);
	});
}

#[test]
fn remove_shared_document_works() {
	new_test_ext().execute_with(|| {
		let to1 = 1;
		let to2 = 2;
		let from = 3;
		setup_vault(to1);
		setup_vault(to2);
		setup_vault(from);
		let shared_doc1 = setup_shared_doc("1", from, to1);
		let shared_doc2 = setup_shared_doc("2", from, to2);
		assert_ok!(ConfidentialDocs::remove_shared_document(Origin::signed(to1), shared_doc1.cid.clone()));
		assert_shared_doc_not_exists(&shared_doc1);
		assert_shared_doc(&shared_doc2);
		assert_ok!(ConfidentialDocs::remove_shared_document(Origin::signed(to2), shared_doc2.cid.clone()));
		assert_shared_doc_not_exists(&shared_doc2);
	});
}

#[test]
fn remove_shared_document_should_fail_for_non_existant_doc() {
	new_test_ext().execute_with(|| {
		let to = 1;
		let from = 2;
		let shared_doc1 = generate_shared_doc("1", from, to);
		assert_noop!(ConfidentialDocs::remove_shared_document(Origin::signed(to), shared_doc1.cid.clone()), Error::<Test>::DocNotFound);
	});
}

#[test]
fn remove_shared_document_should_fail_for_not_doc_sharee() {
	new_test_ext().execute_with(|| {
		let to1 = 1;
		let to2 = 2;
		let from = 3;
		setup_vault(to1);
		setup_vault(to2);
		setup_vault(from);
		let shared_doc1 = setup_shared_doc("1", from, to1);
		assert_noop!(ConfidentialDocs::remove_shared_document(Origin::signed(to2), shared_doc1.cid.clone()), Error::<Test>::NotDocSharee);
		assert_noop!(ConfidentialDocs::remove_shared_document(Origin::signed(from), shared_doc1.cid.clone()), Error::<Test>::NotDocSharee);
	});
}

