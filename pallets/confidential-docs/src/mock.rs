use crate as pallet_confidential_docs;
use frame_support::parameter_types;
use frame_system as system;
use frame_system::EnsureRoot;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		ConfidentialDocs: pallet_confidential_docs::{Pallet, Call, Storage, Event<T>},
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
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
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
	pub const MaxOwnedDocs: u32 = 100;
	pub const MaxSharedToDocs: u32 = 100;
	pub const MaxSharedFromDocs: u32 = 100;
	pub const DocNameMinLen: u32 = 4;
	pub const DocNameMaxLen: u32 = 30;
	pub const DocDescMinLen: u32 = 5;
	pub const DocDescMaxLen: u32 = 100;
}


impl pallet_confidential_docs::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RemoveOrigin = EnsureRoot<Self::AccountId>;
	type MaxOwnedDocs = MaxOwnedDocs;
	type MaxSharedToDocs = MaxSharedToDocs;
	type MaxSharedFromDocs = MaxSharedFromDocs;
	type DocNameMinLen = DocNameMinLen;
	type DocNameMaxLen = DocNameMaxLen;
	type DocDescMinLen = DocDescMinLen;
	type DocDescMaxLen = DocDescMaxLen;
	
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
