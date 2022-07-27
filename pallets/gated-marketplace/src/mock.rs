use crate as pallet_gated_marketplace;
use frame_support::parameter_types;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
use frame_system::EnsureRoot;
// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		GatedMarketplace: pallet_gated_marketplace::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const LabelMaxLen: u32 = 32;
	pub const MaxAuthsPerMarket: u32 = 3;
	pub const MaxRolesPerAuth : u32 = 1;
	pub const MaxApplicants: u32 = 3;
	pub const NotesMaxLen: u32 = 256;
	pub const MaxFeedbackLen: u32 = 256;
	pub const NameMaxLen: u32 = 100;
	pub const MaxFiles: u32 = 10;
	pub const  MaxApplicationsPerCustodian: u32 = 2;
}

impl pallet_gated_marketplace::Config for Test {
	type Event = Event;
	type RemoveOrigin = EnsureRoot<Self::AccountId>;
	type MaxAuthsPerMarket = MaxAuthsPerMarket;
	type MaxRolesPerAuth = MaxRolesPerAuth;
	type MaxApplicants = MaxApplicants;
	type LabelMaxLen = LabelMaxLen;
	type NotesMaxLen = NotesMaxLen;
	type MaxFeedbackLen = MaxFeedbackLen;
	type NameMaxLen = NameMaxLen;
	type MaxFiles = MaxFiles;
	type MaxApplicationsPerCustodian = MaxApplicationsPerCustodian;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
