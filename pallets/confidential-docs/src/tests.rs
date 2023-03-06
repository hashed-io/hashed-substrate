use crate::{mock::*, types::*, Error, Event};
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

fn generate_group_name(id: &str) -> GroupName<Test> {
	format!("group name:{}", id).encode().try_into().unwrap()
}

fn generate_owned_doc(id: &str, owner: <Test as system::Config>::AccountId) -> OwnedDoc<Test> {
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
		RuntimeOrigin::signed(who),
		generate_user_id(id),
		generate_public_key(id),
		generate_cid(id)
	));
}

fn setup_group(
	creator: <Test as system::Config>::AccountId,
	group: <Test as system::Config>::AccountId,
) {
	let id = &group.to_string();
	let group_name: GroupName<Test> = generate_group_name(id);
	let public_key = generate_public_key(id);
	let cid = generate_cid(id);
	assert_ok!(ConfidentialDocs::create_group(
		RuntimeOrigin::signed(creator),
		group,
		group_name.clone(),
		public_key,
		cid.clone()
	));
}

fn add_group_member(
	creator: <Test as system::Config>::AccountId,
	group: <Test as system::Config>::AccountId,
	member: <Test as system::Config>::AccountId,
	role: GroupRole,
) -> GroupMember<Test> {
	let id = &member.to_string();

	let group_member =
		GroupMember { authorizer: creator, cid: generate_cid(id), group, member, role };
	assert_ok!(ConfidentialDocs::add_group_member(
		RuntimeOrigin::signed(creator),
		group_member.clone()
	));
	assert_eq!(ConfidentialDocs::group_members(group, member), Some(group_member.clone()));
	group_member
}

fn setup_owned_doc(id: &str, owner: <Test as system::Config>::AccountId) -> OwnedDoc<Test> {
	let doc = generate_owned_doc(id, owner);
	assert_ok!(ConfidentialDocs::set_owned_document(RuntimeOrigin::signed(owner), doc.clone()));
	assert_owned_doc(&doc);
	doc
}

fn setup_shared_doc(
	id: &str,
	from: <Test as system::Config>::AccountId,
	to: <Test as system::Config>::AccountId,
) -> SharedDoc<Test> {
	let doc = generate_shared_doc(id, from, to);
	assert_ok!(ConfidentialDocs::share_document(RuntimeOrigin::signed(from), doc.clone()));
	assert_shared_doc(&doc);
	doc
}

fn assert_owned_doc(doc: &OwnedDoc<Test>) {
	assert_eq!(ConfidentialDocs::owned_docs(&doc.cid), Some(doc.clone()));
	let owned_docs = ConfidentialDocs::owned_docs_by_owner(doc.owner);
	assert_eq!(owned_docs.contains(&doc.cid), true);
}

fn assert_owned_doc_not_exists(cid: &CID, owner: <Test as system::Config>::AccountId) {
	assert_eq!(ConfidentialDocs::owned_docs(cid), None);
	let owned_docs = ConfidentialDocs::owned_docs_by_owner(owner);
	assert_eq!(owned_docs.contains(cid), false);
}

fn assert_shared_doc(doc: &SharedDoc<Test>) {
	let SharedDoc { from, to, .. } = doc;
	assert_eq!(ConfidentialDocs::shared_docs(&doc.cid), Some(doc.clone()));
	assert_eq!(ConfidentialDocs::shared_docs_by_to(to).contains(&doc.cid), true);
	assert_eq!(ConfidentialDocs::shared_docs_by_from(from).contains(&doc.cid), true);
}

fn assert_shared_doc_not_exists(doc: &SharedDoc<Test>) {
	let SharedDoc { from, to, .. } = doc;
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
		assert_ok!(ConfidentialDocs::set_vault(
			RuntimeOrigin::signed(owner),
			user_id,
			public_key,
			cid.clone()
		));
		// Read pallet storage and assert an expected result.
		let vault = Vault { cid, owner };
		assert_eq!(ConfidentialDocs::vaults(user_id), Some(vault.clone()));
		assert_eq!(ConfidentialDocs::public_keys(owner), Some(public_key));
		System::assert_has_event(Event::<Test>::VaultStored(user_id, public_key, vault).into());

		let public_key = generate_public_key("2");
		let cid = generate_cid("2");
		assert_ok!(ConfidentialDocs::set_vault(
			RuntimeOrigin::signed(owner),
			user_id,
			public_key,
			cid.clone()
		));
		// Read pallet storage and assert an expected result.
		let vault = Vault { cid, owner };
		assert_eq!(ConfidentialDocs::vaults(user_id), Some(vault.clone()));
		assert_eq!(ConfidentialDocs::public_keys(owner), Some(public_key));
		System::assert_has_event(Event::<Test>::VaultStored(user_id, public_key, vault).into());
	});
}

#[test]
fn set_vault_should_fail_for_empty_cid() {
	new_test_ext().execute_with(|| {
		let user_id = generate_user_id("1");
		let public_key = generate_public_key("1");
		let cid: CID = Vec::new().try_into().unwrap();
		assert_noop!(
			ConfidentialDocs::set_vault(RuntimeOrigin::signed(1), user_id, public_key, cid),
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
		assert_ok!(ConfidentialDocs::set_vault(
			RuntimeOrigin::signed(1),
			user_id,
			public_key,
			cid.clone()
		));
		assert_noop!(
			ConfidentialDocs::set_vault(
				RuntimeOrigin::signed(1),
				generate_user_id("2"),
				public_key,
				cid
			),
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
		assert_ok!(ConfidentialDocs::set_vault(
			RuntimeOrigin::signed(1),
			user_id,
			public_key,
			cid.clone()
		));
		assert_noop!(
			ConfidentialDocs::set_vault(RuntimeOrigin::signed(2), user_id, public_key, cid),
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
		assert_ok!(ConfidentialDocs::set_owned_document(
			RuntimeOrigin::signed(owner),
			doc1.clone()
		));
		assert_eq!(ConfidentialDocs::owned_docs(&doc1.cid), Some(doc1.clone()));
		let owned_docs = ConfidentialDocs::owned_docs_by_owner(owner);
		let expected_cid_vec = vec![doc1.cid.clone()];
		assert_eq!(owned_docs.into_inner(), expected_cid_vec);
		System::assert_has_event(Event::<Test>::OwnedDocStored(doc1.clone()).into());

		doc1.name = generate_doc_name("2");
		doc1.description = generate_doc_desc("2");
		assert_ok!(ConfidentialDocs::set_owned_document(
			RuntimeOrigin::signed(owner),
			doc1.clone()
		));
		assert_eq!(ConfidentialDocs::owned_docs(&doc1.cid), Some(doc1.clone()));
		let owned_docs = ConfidentialDocs::owned_docs_by_owner(owner);
		assert_eq!(owned_docs.into_inner(), expected_cid_vec);
		System::assert_has_event(Event::<Test>::OwnedDocStored(doc1).into());
	});
}

#[test]
fn set_owned_document_should_fail_for_updating_non_owned_doc() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		setup_vault(owner);
		let mut doc1 = generate_owned_doc("1", owner);
		assert_ok!(ConfidentialDocs::set_owned_document(
			RuntimeOrigin::signed(owner),
			doc1.clone()
		));
		doc1.name = generate_doc_name("2");
		let owner = 2;
		setup_vault(owner);
		assert_noop!(
			ConfidentialDocs::set_owned_document(RuntimeOrigin::signed(owner), doc1.clone()),
			Error::<Test>::NotDocOwner
		);
	});
}

#[test]
fn set_owned_document_should_fail_for_empty_cid() {
	new_test_ext().execute_with(|| {
		let mut doc = generate_owned_doc("1", 1);
		doc.cid = Vec::new().try_into().unwrap();
		assert_noop!(
			ConfidentialDocs::set_owned_document(RuntimeOrigin::signed(1), doc.clone()),
			Error::<Test>::CIDNoneValue
		);
	});
}

#[test]
fn set_owned_document_should_fail_for_name_too_short() {
	new_test_ext().execute_with(|| {
		let mut doc = generate_owned_doc("1", 1);
		doc.name = "as".encode().try_into().unwrap();
		assert_noop!(
			ConfidentialDocs::set_owned_document(RuntimeOrigin::signed(1), doc.clone()),
			Error::<Test>::DocNameTooShort
		);
	});
}

#[test]
fn set_owned_document_should_fail_for_description_too_short() {
	new_test_ext().execute_with(|| {
		let mut doc = generate_owned_doc("1", 1);
		doc.description = "des".encode().try_into().unwrap();
		assert_noop!(
			ConfidentialDocs::set_owned_document(RuntimeOrigin::signed(1), doc.clone()),
			Error::<Test>::DocDescTooShort
		);
	});
}

#[test]
fn set_owned_document_should_fail_for_owner_with_no_public_key() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		let doc = generate_owned_doc("1", owner);
		assert_noop!(
			ConfidentialDocs::set_owned_document(RuntimeOrigin::signed(owner), doc.clone()),
			Error::<Test>::AccountHasNoPublicKey
		);
	});
}

#[test]
fn remove_owned_document_works() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		setup_vault(owner);
		let doc1 = setup_owned_doc("1", owner);
		let doc2 = setup_owned_doc("2", owner);
		assert_ok!(ConfidentialDocs::remove_owned_document(
			RuntimeOrigin::signed(owner),
			doc1.cid.clone()
		));

		assert_owned_doc_not_exists(&doc1.cid, owner);
		System::assert_has_event(Event::<Test>::OwnedDocRemoved(doc1).into());

		assert_owned_doc(&doc2);
		assert_ok!(ConfidentialDocs::remove_owned_document(
			RuntimeOrigin::signed(owner),
			doc2.cid.clone()
		));
		assert_owned_doc_not_exists(&doc2.cid, owner);
		System::assert_has_event(Event::<Test>::OwnedDocRemoved(doc2).into());
	});
}

#[test]
fn remove_owned_document_should_fail_for_non_existant_document() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		let doc1 = generate_owned_doc("1", owner);
		assert_noop!(
			ConfidentialDocs::remove_owned_document(RuntimeOrigin::signed(owner), doc1.cid.clone()),
			Error::<Test>::DocNotFound
		);
	});
}

#[test]
fn remove_owned_document_should_fail_for_not_document_owner() {
	new_test_ext().execute_with(|| {
		let owner = 1;
		setup_vault(owner);
		let doc1 = setup_owned_doc("1", owner);
		let not_owner = 2;
		assert_noop!(
			ConfidentialDocs::remove_owned_document(
				RuntimeOrigin::signed(not_owner),
				doc1.cid.clone()
			),
			Error::<Test>::NotDocOwner
		);
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
		assert_ok!(ConfidentialDocs::share_document(
			RuntimeOrigin::signed(from),
			shared_doc1.clone()
		));
		// Read pallet storage and assert an expected result.
		assert_eq!(ConfidentialDocs::shared_docs(&shared_doc1.cid), Some(shared_doc1.clone()));
		let shared_docs_to = ConfidentialDocs::shared_docs_by_to(to);
		let mut expected_cid_to_vec = vec![shared_doc1.cid.clone()];
		assert_eq!(shared_docs_to.into_inner(), expected_cid_to_vec);
		let expected_cid_from_vec = vec![shared_doc1.cid.clone()];
		let shared_docs_from = ConfidentialDocs::shared_docs_by_from(from);
		assert_eq!(shared_docs_from.into_inner(), expected_cid_from_vec);
		System::assert_has_event(Event::<Test>::SharedDocStored(shared_doc1).into());

		let from = 3;
		setup_vault(3);
		let shared_doc2 = generate_shared_doc("2", from, to);
		assert_ok!(ConfidentialDocs::share_document(
			RuntimeOrigin::signed(from),
			shared_doc2.clone()
		));
		assert_eq!(ConfidentialDocs::shared_docs(&shared_doc2.cid), Some(shared_doc2.clone()));
		let shared_docs_to = ConfidentialDocs::shared_docs_by_to(to);
		expected_cid_to_vec.push(shared_doc2.cid.clone());
		assert_eq!(shared_docs_to.into_inner(), expected_cid_to_vec);
		let expected_cid_from_vec = vec![shared_doc2.cid.clone()];
		let shared_docs_from = ConfidentialDocs::shared_docs_by_from(from);
		assert_eq!(shared_docs_from.into_inner(), expected_cid_from_vec);
		System::assert_has_event(Event::<Test>::SharedDocStored(shared_doc2).into());
	});
}

#[test]
fn share_document_should_fail_for_empty_cid() {
	new_test_ext().execute_with(|| {
		let mut shared_doc = generate_shared_doc("1", 1, 2);
		shared_doc.cid = Vec::new().try_into().unwrap();
		assert_noop!(
			ConfidentialDocs::share_document(RuntimeOrigin::signed(1), shared_doc.clone()),
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
			ConfidentialDocs::share_document(RuntimeOrigin::signed(1), shared_doc.clone()),
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
			ConfidentialDocs::share_document(RuntimeOrigin::signed(1), shared_doc.clone()),
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
			ConfidentialDocs::share_document(RuntimeOrigin::signed(from), shared_doc.clone()),
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
		assert_ok!(ConfidentialDocs::share_document(
			RuntimeOrigin::signed(from),
			shared_doc.clone()
		));
		assert_noop!(
			ConfidentialDocs::share_document(RuntimeOrigin::signed(from), shared_doc.clone()),
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
			ConfidentialDocs::share_document(RuntimeOrigin::signed(from), shared_doc.clone()),
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
			ConfidentialDocs::share_document(RuntimeOrigin::signed(from), shared_doc.clone()),
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
		assert_ok!(ConfidentialDocs::update_shared_document_metadata(
			RuntimeOrigin::signed(to),
			shared_doc1.clone()
		));
		assert_shared_doc(&shared_doc1);
		System::assert_has_event(Event::<Test>::SharedDocUpdated(shared_doc1).into());

	});
}

#[test]
fn update_shared_document_metadata_should_fail_for_non_existant_doc() {
	new_test_ext().execute_with(|| {
		let to = 1;
		let from = 2;
		let shared_doc1 = generate_shared_doc("1", from, to);
		assert_noop!(
			ConfidentialDocs::update_shared_document_metadata(
				RuntimeOrigin::signed(to),
				shared_doc1.clone()
			),
			Error::<Test>::DocNotFound
		);
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
		assert_noop!(
			ConfidentialDocs::update_shared_document_metadata(
				RuntimeOrigin::signed(to2),
				shared_doc1.clone()
			),
			Error::<Test>::NotDocSharee
		);
		assert_noop!(
			ConfidentialDocs::update_shared_document_metadata(
				RuntimeOrigin::signed(from),
				shared_doc1.clone()
			),
			Error::<Test>::NotDocSharee
		);
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
		assert_ok!(ConfidentialDocs::remove_shared_document(
			RuntimeOrigin::signed(to1),
			shared_doc1.cid.clone()
		));
		assert_shared_doc_not_exists(&shared_doc1);
		assert_shared_doc(&shared_doc2);
		assert_ok!(ConfidentialDocs::remove_shared_document(
			RuntimeOrigin::signed(to2),
			shared_doc2.cid.clone()
		));
		assert_shared_doc_not_exists(&shared_doc2);
		System::assert_has_event(Event::<Test>::SharedDocRemoved(shared_doc1).into());
	});
}

#[test]
fn remove_shared_document_should_fail_for_non_existant_doc() {
	new_test_ext().execute_with(|| {
		let to = 1;
		let from = 2;
		let shared_doc1 = generate_shared_doc("1", from, to);
		assert_noop!(
			ConfidentialDocs::remove_shared_document(
				RuntimeOrigin::signed(to),
				shared_doc1.cid.clone()
			),
			Error::<Test>::DocNotFound
		);
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
		assert_noop!(
			ConfidentialDocs::remove_shared_document(
				RuntimeOrigin::signed(to2),
				shared_doc1.cid.clone()
			),
			Error::<Test>::NotDocSharee
		);
		assert_noop!(
			ConfidentialDocs::remove_shared_document(
				RuntimeOrigin::signed(from),
				shared_doc1.cid.clone()
			),
			Error::<Test>::NotDocSharee
		);
	});
}

#[test]
fn create_group_works() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let group_name = generate_group_name("1");
		let group_id = 2;
		let public_key = generate_public_key("2");
		let cid = generate_cid("2");
		assert_ok!(ConfidentialDocs::create_group(
			RuntimeOrigin::signed(creator),
			group_id,
			group_name.clone(),
			public_key,
			cid.clone()
		));
		let group = Group { group: group_id, creator, name: group_name };
		assert_eq!(ConfidentialDocs::groups(group_id), Some(group.clone()));
		assert_eq!(
			ConfidentialDocs::group_members(group_id, creator),
			Some(GroupMember {
				group: group_id,
				authorizer: creator,
				member: creator,
				role: GroupRole::Owner,
				cid,
			})
		);
		let expected_member_groups = vec![group_id];
		assert_eq!(ConfidentialDocs::member_groups(creator).into_inner(), expected_member_groups);
		assert_eq!(ConfidentialDocs::public_keys(group_id), Some(public_key));
		System::assert_has_event(Event::<Test>::GroupCreated(group).into());
	});
}

#[test]
fn create_group_should_fail_for_creator_without_vault() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		let group_name = generate_group_name("1");
		let group_id = 2;
		let public_key = generate_public_key("2");
		let cid = generate_cid("2");
		assert_noop!(
			ConfidentialDocs::create_group(
				RuntimeOrigin::signed(creator),
				group_id,
				group_name.clone(),
				public_key,
				cid.clone()
			),
			Error::<Test>::AccountHasNoPublicKey
		);
	});
}

#[test]
fn create_group_should_fail_for_group_name_too_short() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let group_name: GroupName<Test> = "g".encode().try_into().unwrap();
		let group_id = 2;
		let public_key = generate_public_key("2");
		let cid = generate_cid("2");
		assert_noop!(
			ConfidentialDocs::create_group(
				RuntimeOrigin::signed(creator),
				group_id,
				group_name.clone(),
				public_key,
				cid.clone()
			),
			Error::<Test>::GroupNameTooShort
		);
	});
}

#[test]
fn create_group_should_fail_for_empty_cid() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let group_name: GroupName<Test> = generate_group_name("1");
		let group_id = 2;
		let public_key = generate_public_key("2");
		let cid: CID = Vec::new().try_into().unwrap();
		assert_noop!(
			ConfidentialDocs::create_group(
				RuntimeOrigin::signed(creator),
				group_id,
				group_name.clone(),
				public_key,
				cid.clone()
			),
			Error::<Test>::CIDNoneValue
		);
	});
}

#[test]
fn add_group_member_works() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let group = 2;
		setup_group(creator, group);
		let member = 3;
		setup_vault(member);
		let group_member = GroupMember {
			authorizer: creator,
			cid: generate_cid("3"),
			group,
			member,
			role: GroupRole::Admin,
		};
		assert_ok!(ConfidentialDocs::add_group_member(
			RuntimeOrigin::signed(creator),
			group_member.clone()
		));
		assert_eq!(ConfidentialDocs::group_members(group, member), Some(group_member.clone()));
		let expected_member_groups = vec![group];
		assert_eq!(ConfidentialDocs::member_groups(member).into_inner(), expected_member_groups);

		System::assert_has_event(Event::<Test>::GroupMemberAdded(group_member).into());
	});
}

#[test]
fn add_group_member_should_fail_for_non_existant_group() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let group = 2;
		let member = 3;
		setup_vault(member);
		let group_member = GroupMember {
			authorizer: creator,
			cid: generate_cid("3"),
			group,
			member,
			role: GroupRole::Admin,
		};
		assert_noop!(
			ConfidentialDocs::add_group_member(
				RuntimeOrigin::signed(creator),
				group_member.clone()
			),
			Error::<Test>::GroupDoesNotExist
		);
	});
}

#[test]
fn add_group_member_should_fail_for_empty_cid() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let group = 2;
		setup_group(creator, group);
		let member = 3;
		setup_vault(member);
		let group_member = GroupMember {
			authorizer: creator,
			cid: Vec::new().try_into().unwrap(),
			group,
			member,
			role: GroupRole::Admin,
		};
		assert_noop!(
			ConfidentialDocs::add_group_member(
				RuntimeOrigin::signed(creator),
				group_member.clone()
			),
			Error::<Test>::CIDNoneValue
		);
	});
}

#[test]
fn add_group_member_should_fail_for_member_without_public_key() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let group = 2;
		setup_group(creator, group);
		let member = 3;
		let group_member = GroupMember {
			authorizer: creator,
			cid: generate_cid("3"),
			group,
			member,
			role: GroupRole::Admin,
		};
		assert_noop!(
			ConfidentialDocs::add_group_member(
				RuntimeOrigin::signed(creator),
				group_member.clone()
			),
			Error::<Test>::AccountHasNoPublicKey
		);
	});
}

#[test]
fn add_group_member_should_fail_for_adding_member_as_owner() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let group = 2;
		setup_group(creator, group);
		let member = 3;
		setup_vault(member);
		let group_member = GroupMember {
			authorizer: creator,
			cid: generate_cid("3"),
			group,
			member,
			role: GroupRole::Owner,
		};
		assert_noop!(
			ConfidentialDocs::add_group_member(
				RuntimeOrigin::signed(creator),
				group_member.clone()
			),
			Error::<Test>::CanNotAddMemberAsGroupOwner
		);
	});
}

#[test]
fn add_group_member_should_fail_for_authorizer_not_member_of_group() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let non_creator = 4;
		setup_vault(non_creator);
		let group = 2;
		setup_group(creator, group);
		let member = 3;
		setup_vault(member);
		let group_member = GroupMember {
			authorizer: non_creator,
			cid: generate_cid("3"),
			group,
			member,
			role: GroupRole::Admin,
		};
		assert_noop!(
			ConfidentialDocs::add_group_member(
				RuntimeOrigin::signed(non_creator),
				group_member.clone()
			),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn add_group_member_should_fail_for_authorizer_not_admin() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let non_admin = 4;
		setup_vault(non_admin);
		let group = 2;
		setup_group(creator, group);
		add_group_member(creator, group, non_admin, GroupRole::Member);
		let member = 3;
		setup_vault(member);
		let group_member = GroupMember {
			authorizer: non_admin,
			cid: generate_cid("3"),
			group,
			member,
			role: GroupRole::Admin,
		};
		assert_noop!(
			ConfidentialDocs::add_group_member(
				RuntimeOrigin::signed(non_admin),
				group_member.clone()
			),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn remove_group_member_works() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let group = 2;
		setup_group(creator, group);
		let member = 3;
		setup_vault(member);
		let group_member = add_group_member(creator, group, member, GroupRole::Admin);

		assert_ok!(ConfidentialDocs::remove_group_member(
			RuntimeOrigin::signed(creator),
			group,
			member
		));
		assert_eq!(ConfidentialDocs::group_members(group, member), None);
		let expected_member_groups = Vec::<<Test as system::Config>::AccountId>::new();
		assert_eq!(ConfidentialDocs::member_groups(member).into_inner(), expected_member_groups);
		System::assert_has_event(Event::<Test>::GroupMemberRemoved(group_member).into());
	});
}

#[test]
fn remove_group_member_works_for_admin_removing_member_he_added() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let group = 2;
		setup_group(creator, group);
		let member_authorizer = 3;
		setup_vault(member_authorizer);
		add_group_member(creator, group, member_authorizer, GroupRole::Admin);
		let member_to_remove = 4;
		setup_vault(member_to_remove);
		let group_member = add_group_member(member_authorizer, group, member_to_remove, GroupRole::Member);

		assert_ok!(ConfidentialDocs::remove_group_member(
			RuntimeOrigin::signed(member_authorizer),
			group,
			member_to_remove
		));
		assert_eq!(ConfidentialDocs::group_members(group, member_to_remove), None);
		let expected_member_groups = Vec::<<Test as system::Config>::AccountId>::new();
		assert_eq!(ConfidentialDocs::member_groups(member_to_remove).into_inner(), expected_member_groups);
		System::assert_has_event(Event::<Test>::GroupMemberRemoved(group_member).into());
	});
}

#[test]
fn remove_group_member_should_fail_for_non_existant_group() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let group = 2;
		let member = 3;
		setup_vault(member);

		assert_noop!(
			ConfidentialDocs::remove_group_member(RuntimeOrigin::signed(creator), group, member),
			Error::<Test>::GroupDoesNotExist
		);
	});
}

#[test]
fn remove_group_member_should_fail_for_trying_to_remove_non_member() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let group = 2;
		setup_group(creator, group);
		let non_member = 3;

		assert_noop!(
			ConfidentialDocs::remove_group_member(
				RuntimeOrigin::signed(creator),
				group,
				non_member
			),
			Error::<Test>::GroupMemberDoesNotExist
		);
	});
}

#[test]
fn remove_group_member_should_fail_for_trying_to_remove_owner() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let group = 2;
		setup_group(creator, group);

		assert_noop!(
			ConfidentialDocs::remove_group_member(
				RuntimeOrigin::signed(creator),
				group,
				creator
			),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn remove_group_member_should_fail_for_role_member_as_authorizer() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let group = 2;
		setup_group(creator, group);
		let member_authorizer = 3;
		setup_vault(member_authorizer);
		add_group_member(creator, group, member_authorizer, GroupRole::Member);
		let member_to_remove = 4;
		setup_vault(member_to_remove);
		add_group_member(creator, group, member_to_remove, GroupRole::Member);

		assert_noop!(
			ConfidentialDocs::remove_group_member(
				RuntimeOrigin::signed(member_authorizer),
				group,
				member_to_remove
			),
			Error::<Test>::NoPermission
		);
	});
}


#[test]
fn remove_group_member_should_fail_for_admin_removing_member_he_did_not_add() {
	new_test_ext().execute_with(|| {
		let creator = 1;
		setup_vault(creator);
		let group = 2;
		setup_group(creator, group);
		let member_authorizer = 3;
		setup_vault(member_authorizer);
		add_group_member(creator, group, member_authorizer, GroupRole::Admin);
		let member_to_remove = 4;
		setup_vault(member_to_remove);
		add_group_member(creator, group, member_to_remove, GroupRole::Member);

		assert_noop!(
			ConfidentialDocs::remove_group_member(
				RuntimeOrigin::signed(member_authorizer),
				group,
				member_to_remove
			),
			Error::<Test>::NoPermission
		);
	});
}


