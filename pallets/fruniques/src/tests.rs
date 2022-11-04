use crate::{mock::*, Error};
use core::convert::TryFrom;
use codec::{Encode};

use frame_support::{assert_noop, assert_ok, BoundedVec};

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
			BoundedVec::<u8, KeyLimit>::try_from(b"dummy key".encode()).expect("Error on encoding key to BoundedVec"),
			BoundedVec::<u8, ValueLimit>::try_from(b"dummy value".encode()).expect("Error on encoding value to BoundedVec"),
		)]
}

fn dummy_empty_attributes() -> Vec<(BoundedVec<u8, KeyLimit>, BoundedVec<u8, ValueLimit>)> {
	vec![]
}


#[test]
fn create_collection_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(Fruniques::create_collection(Origin::signed(1), dummy_description()));
	})
}

#[test]
fn spawn_extrinsic_works() {
	ExtBuilder::default().build().execute_with(|| {
		// A collection must be created before spawning an NFT
		assert_noop!(Fruniques::spawn(Origin::signed(1), 0, None, dummy_description(), None), Error::<Test>::CollectionNotFound);

		// Create a collection
		assert_ok!(Fruniques::create_collection(Origin::signed(1), dummy_description()));

		// The first item can not be a child
		assert_noop!(Fruniques::spawn(Origin::signed(1), 0, Some((0, false, 10)), dummy_description(), None), Error::<Test>::ParentNotFound);

		// A NFT can be created with empty data
		assert_ok!(Fruniques::spawn(Origin::signed(1), 0, None, dummy_description(), None));
		// A NFT can be created with attributes
		assert_ok!(Fruniques::spawn(Origin::signed(1), 0, None, dummy_description(), Some(dummy_attributes())));
		// A NFT can be hierarchical
		assert_ok!(Fruniques::spawn(Origin::signed(1), 0, Some((0, false, 10)), dummy_description(), None));
		// The parent must exist
		assert_noop!(Fruniques::spawn(Origin::signed(1), 0, Some((100, false, 10)), dummy_description(), None), Error::<Test>::ParentNotFound);

	})
}

#[test]
fn set_attributes_works() {
	ExtBuilder::default().build().execute_with(|| {
		// A collection must be created before spawning an NFT
		assert_noop!(Fruniques::spawn(Origin::signed(1), 0, None, dummy_description(), None), Error::<Test>::CollectionNotFound);

		// Create a collection
		assert_ok!(Fruniques::create_collection(Origin::signed(1), dummy_description()));
		// Attributes can be added only to existing NFTs
		assert_noop!(Fruniques::set_attributes(Origin::signed(1), 0, 0, dummy_attributes()), Error::<Test>::FruniqueNotFound);
		// A NFT can be created with empty data
		assert_ok!(Fruniques::spawn(Origin::signed(1), 0, None, dummy_description(), None));
		// Attributes can not be empty
		assert_noop!(Fruniques::set_attributes(Origin::signed(1), 0, 0, dummy_empty_attributes()), Error::<Test>::AttributesEmpty);

	})
}
