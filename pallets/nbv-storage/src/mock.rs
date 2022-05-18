use crate as pallet_nbv_storage;
use crate::mock::system::EnsureRoot;
use frame_support::ord_parameter_types;
use frame_support::{parameter_types, traits::ConstU32};
use frame_support::traits::{ConstU64, EnsureOneOf};
use frame_system as system;
use pallet_balances;
use pallet_preimage;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use system::EnsureSignedBy;
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
		NBVStorage: pallet_nbv_storage::{Pallet, Call, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Preimage: pallet_preimage::{Pallet, Call, Storage, Event<T>},
		Identity: pallet_identity::{Pallet, Call, Storage, Event<T>},
	}
);

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u64;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
}


parameter_types! {
	// Taken from Polkadot as reference.
	pub const PreimageMaxSize: u32 = 4096 * 1024;
}

impl pallet_preimage::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<Self::AccountId>;
	type MaxSize = PreimageMaxSize;
	type BaseDeposit = ConstU64<1>;
	type ByteDeposit = ConstU64<1>;
	type WeightInfo = ();
}

parameter_types! {
	pub const MaxAdditionalFields: u32 = 1;
	pub const MaxRegistrars: u32 = 20;
}
ord_parameter_types! {
	pub const One: u64 = 1;
	pub const Two: u64 = 2;
}
type EnsureOneOrRoot = EnsureOneOf<EnsureRoot<u64>, EnsureSignedBy<One, u64>>;
type EnsureTwoOrRoot = EnsureOneOf<EnsureRoot<u64>, EnsureSignedBy<Two, u64>>;
impl pallet_identity::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type Slashed = ();
	type BasicDeposit = ConstU64<10>;
	type FieldDeposit = ConstU64<10>;
	type SubAccountDeposit = ConstU64<10>;
	type MaxSubAccounts = ConstU32<2>;
	type MaxAdditionalFields = MaxAdditionalFields;
	type MaxRegistrars = MaxRegistrars;
	type RegistrarOrigin = EnsureOneOrRoot;
	type ForceOrigin = EnsureTwoOrRoot;
	type WeightInfo = ();
}


parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Call = Call;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub const XPubLen: u32 = 166;
	pub const PSBTMaxLen: u32  = 2048;
	pub const MaxVaultsPerUser: u32 = 10;
	pub const MaxCosignersPerVault: u32 = 5;
	pub const VaultDescriptionMaxLen: u32 = 200;
	pub const OutputDescriptorMaxLen: u32 = 2048;
}

impl pallet_nbv_storage::Config for Test {
	type Event = Event;
	type XPubLen = XPubLen;
	type PSBTMaxLen = PSBTMaxLen;
	type MaxVaultsPerUser = MaxVaultsPerUser;
	type MaxCosignersPerVault = MaxCosignersPerVault;
	type VaultDescriptionMaxLen =VaultDescriptionMaxLen;
	type OutputDescriptorMaxLen = OutputDescriptorMaxLen;
}
// Build genesis storage according to the mock runtime.
 pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 1000000000), (2000000000, 10), (3, 10), (10, 100), (20, 100), (30, 100)],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
	 
 }
