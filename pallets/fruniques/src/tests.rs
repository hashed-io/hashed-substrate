use crate::{mock::*, Error};

use frame_support::{assert_err, assert_noop, assert_ok};
use sp_runtime::Permill;

pub struct ExtBuilder;

// helper function to set BoundedVec
macro_rules! bvec {
	($( $x:tt )*) => {
		vec![$( $x )*].try_into().unwrap()
	}
}

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

#[test]
fn create_frunique_works() {
	// Create a frunique
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(Fruniques::create(Origin::signed(1), 1, 0, Some(Permill::from_percent(50)), 1));
	});
}

#[test]
fn create_frunique_with_attributes_should_work() {
	// Create a frunique with attributes
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			Fruniques::create_with_attributes(
				Origin::signed(1),
				1,
				0,
				Some(Permill::from_percent(50)),
				1,
				vec![]
			),
			Error::<Test>::AttributesEmpty
		);

		assert_ok!(Fruniques::create_with_attributes(
			Origin::signed(1),
			1,
			0,
			Some(Permill::from_percent(50)),
			1,
			vec![(bvec![0], bvec![0])],
		));
	});
}

// this test is failing for some reason...
/*---- tests::spawn_extrinsic_works stdout ----
thread 'tests::spawn_extrinsic_works' panicked at 'Expected Ok(_). Got Err(
		Module(
				ModuleError {
						index: 1,
						error: [
								1,
								0,
								0,
								0,
						],
						message: Some(
								"UnknownCollection",
						),
				},
		),
)', pallets/fruniques/src/tests.rs:41:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/
#[test]
fn spawn_extrinsic_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(Fruniques::create(Origin::signed(1), 1, 0, Some(Permill::from_percent(50)), 1));
		assert_ok!(Fruniques::spawn(
			Origin::signed(1),
			1,
			255,
			true,
			Permill::from_float(20.525),
			1
		));
		//Fruniques::spawn(Origin::signed(1),1,255,true,Permill::from_float(20.525),1 );
		assert_ok!(Fruniques::spawn(Origin::signed(1), 1, 1, true, Permill::from_float(20.525), 1));
		assert_ok!(Fruniques::instance_exists(Origin::signed(1), 1, 1));
	});
}

#[test]
fn set_attributes_should_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(Fruniques::create(Origin::signed(1), 0, 0, Some(Permill::from_percent(50)), 1));
		assert_noop!(
			Fruniques::set_attributes(Origin::signed(1), 0, 0, vec![]),
			Error::<Test>::AttributesEmpty
		);
		assert_ok!(Fruniques::set_attributes(Origin::signed(1), 0, 0, vec![(bvec![0], bvec![0])]));
	});
}
