use crate::{mock::*, Error};
use codec::Encode;
use core::convert::TryFrom;

use frame_support::{assert_noop, assert_ok, BoundedVec};
use crate::types::ParentInfoCall;
pub struct ExtBuilder;

// helper function to set BoundedVec
// macro_rules! bvec {
// 	($( $x:tt )*) => {
// 		vec![$( $x )*].try_into().unwrap()
// 	}
// }

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {}
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		pallet_balances::GenesisConfig::<Test> {
			balances: vec![(1, 100), (2, 20), (3, 30), (4, 40), (5, 50), (6, 60)],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

fn dummy_description() -> BoundedVec<u8, StringLimit> {
	BoundedVec::<u8, StringLimit>::try_from(b"dummy description".to_vec()).unwrap()
}

fn dummy_attributes() -> Vec<(BoundedVec<u8, KeyLimit>, BoundedVec<u8, ValueLimit>)> {
	vec![(
		BoundedVec::<u8, KeyLimit>::try_from(b"dummy key".encode())
			.expect("Error on encoding key to BoundedVec"),
		BoundedVec::<u8, ValueLimit>::try_from(b"dummy value".encode())
			.expect("Error on encoding value to BoundedVec"),
	)]
}

fn dummy_empty_attributes() -> Vec<(BoundedVec<u8, KeyLimit>, BoundedVec<u8, ValueLimit>)> {
	vec![]
}

fn dummy_parent(collection_id: u32, parent_id: u32) -> ParentInfoCall<Test> {
	ParentInfoCall {
		collection_id,
		parent_id,
		parent_percentage: 10,
		is_hierarchical: true
	}
}

#[test]
fn create_collection_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(Fruniques::create_collection(RuntimeOrigin::signed(1), dummy_description()));
	})
}

// SBP-M2 review: This test is failing please update it
#[test]
fn spawn_extrinsic_works() {
	new_test_ext().execute_with(|| {
		// A collection must be created before spawning an NFT
		assert_noop!(
			Fruniques::spawn(RuntimeOrigin::signed(1), 0, dummy_description(), None, None),
			Error::<Test>::CollectionNotFound
		);

		// Create a collection
		assert_ok!(Fruniques::create_collection(RuntimeOrigin::signed(1), dummy_description()));

		// The first item can not be a child
		assert_noop!(
			Fruniques::spawn(RuntimeOrigin::signed(1), 0, dummy_description(), None, Some(dummy_parent(0, 10))),
			Error::<Test>::ParentNotFound
		);

		// A NFT can be created with empty data
		assert_ok!(Fruniques::spawn(RuntimeOrigin::signed(1), 0, dummy_description(), None, None));
		// A NFT can be created with attributes
		assert_ok!(Fruniques::spawn(
			RuntimeOrigin::signed(1),
			0,
			dummy_description(),
			Some(dummy_attributes()),
			None
		));
		// A NFT can be hierarchical
		assert_ok!(Fruniques::spawn(
			RuntimeOrigin::signed(1),
			0,
			dummy_description(),
			None,
			Some(dummy_parent(0, 0))
		));
		// The parent must exist
		assert_noop!(
			Fruniques::spawn(
				RuntimeOrigin::signed(1),
				0,
				dummy_description(),
				None,
				Some(dummy_parent(0, 10))
			),
			Error::<Test>::ParentNotFound
		);
	})
}

#[test]
fn set_attributes_works() {
	new_test_ext().execute_with(|| {
		// A collection must be created before spawning an NFT
		assert_noop!(
			Fruniques::spawn(RuntimeOrigin::signed(1), 0, dummy_description(), None, None),
			Error::<Test>::CollectionNotFound
		);

		// Create a collection
		assert_ok!(Fruniques::create_collection(RuntimeOrigin::signed(1), dummy_description()));
		// Attributes can be added only to existing NFTs
		assert_noop!(
			Fruniques::set_attributes(RuntimeOrigin::signed(1), 0, 0, dummy_attributes()),
			Error::<Test>::FruniqueNotFound
		);
		// A NFT can be created with empty data
		assert_ok!(Fruniques::spawn(RuntimeOrigin::signed(1), 0, dummy_description(), None, None));
		// Attributes can not be empty
		assert_noop!(
			Fruniques::set_attributes(RuntimeOrigin::signed(1), 0, 0, dummy_empty_attributes()),
			Error::<Test>::AttributesEmpty
		);
	})
}

#[test]
fn invite_collaborator_works() {
	new_test_ext().execute_with(|| {
		// Create a collection
		assert_ok!(Fruniques::create_collection(RuntimeOrigin::signed(1), dummy_description()));
		assert_ok!(Fruniques::invite(
			RuntimeOrigin::signed(1),
			0,
			2
		));
	});
}

#[test]
fn verify_by_admin_works() {
	new_test_ext().execute_with(|| {
		// Create a collection
		assert_ok!(Fruniques::create_collection(RuntimeOrigin::signed(1), dummy_description()));
		// Spawn an NFT
		assert_ok!(Fruniques::spawn(RuntimeOrigin::signed(1), 0, dummy_description(), None, None));
		// Add admin to the collection
		assert_ok!(Fruniques::insert_auth_in_frunique_collection(2, 0, crate::types::FruniqueRole::Admin));
		// Verify
		assert_ok!(Fruniques::verify(
			RuntimeOrigin::signed(2),
			0,
			0
		));
	});
}

#[test]
fn verify_by_owner_works() {
	new_test_ext().execute_with(|| {
		// Create a collection
		assert_ok!(Fruniques::create_collection(RuntimeOrigin::signed(1), dummy_description()));
		// Spawn an NFT
		assert_ok!(Fruniques::spawn(RuntimeOrigin::signed(1), 0, dummy_description(), None, None));
		// Verify
		assert_ok!(Fruniques::verify(
			RuntimeOrigin::signed(1),
			0,
			0
		));
	});
}

#[test]
fn verify_by_neither_admin_nor_owner_fails() {
	new_test_ext().execute_with(|| {
		// Create a collection
		assert_ok!(Fruniques::create_collection(RuntimeOrigin::signed(1), dummy_description()));
		// Spawn an NFT
		assert_ok!(Fruniques::spawn(RuntimeOrigin::signed(1), 0, dummy_description(), None, None));
		// Invite collaborator
		assert_ok!(Fruniques::invite(
			RuntimeOrigin::signed(1),
			0,
			2
		));
		// Verify
		assert_noop!(
			Fruniques::verify(
				RuntimeOrigin::signed(3),
				0,
				0
			),
			Error::<Test>::NotAuthorized
		);
	});
}

#[test]
fn verify_already_verified_fails() {
	new_test_ext().execute_with(|| {
		// Create a collection
		assert_ok!(Fruniques::create_collection(RuntimeOrigin::signed(1), dummy_description()));
		// Spawn an NFT
		assert_ok!(Fruniques::spawn(RuntimeOrigin::signed(1), 0, dummy_description(), None, None));
		// Add admin to the collection
		assert_ok!(Fruniques::insert_auth_in_frunique_collection(2, 0, crate::types::FruniqueRole::Admin));
		// Verify
		assert_ok!(Fruniques::verify(
			RuntimeOrigin::signed(2),
			0,
			0
		));
		// Verify again
		assert_noop!(
			Fruniques::verify(
				RuntimeOrigin::signed(2),
				0,
				0
			),
			Error::<Test>::FruniqueAlreadyVerified
		);
	});
}
