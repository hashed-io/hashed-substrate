use crate::mock::*;
use frame_support::assert_ok;

pub struct ExtBuilder;

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {}
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		pallet_balances::GenesisConfig::<Test> {
			balances: vec![(1, 10), (2, 20), (3, 30), (4, 40), (5, 50), (6, 60)],
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
		assert_ok!(Fruniques::create(Origin::signed(1), 1, 42, 1));
	});
}

#[test]
fn divide_extrinsic_notyetimplemented() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert!(
			Fruniques::divide(Origin::signed(1), 1, 42, 43, 4).is_err(),
			"divide is not yet implemented"
		);
	});
}
