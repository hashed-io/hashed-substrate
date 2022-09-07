use crate as pallet_gated_marketplace;
use frame_support::{parameter_types, traits::AsEnsureOriginWithArg};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
use frame_system::EnsureRoot;
use system::EnsureSigned;
// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		GatedMarketplace: pallet_gated_marketplace::{Pallet, Call, Storage, Event<T>},
		Uniques: pallet_uniques::{Pallet, Call, Storage, Event<T>},
		// Fruniques: pallet_fruniques::{Pallet, Call, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		RBAC: pallet_rbac::{Pallet, Call, Storage, Event<T>},
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
	type AccountData = pallet_balances::AccountData<u64>;
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
	pub const MaxApplicationsPerCustodian: u32 = 2;
	pub const MaxMarketsPerItem: u32 = 10;
	pub const MaxOffersPerMarket: u32 = 100;
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
	type MaxOffersPerMarket = MaxOffersPerMarket;
	type MaxMarketsPerItem = MaxMarketsPerItem;
	type Timestamp = Timestamp;
	type Moment = u64;
	//type LocalCurrency = Balances;
	type Rbac = RBAC;
}

impl pallet_fruniques::Config for Test {
	type Event = Event;
}

parameter_types! {
	pub const ClassDeposit: u64 = 2;
	pub const InstanceDeposit: u64 = 1;
	pub const KeyLimit: u32 = 50;
	pub const ValueLimit: u32 = 50;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: u64 = 1;
	pub const AttributeDepositBase: u64 = 1;
	pub const MetadataDepositPerByte: u64 = 1;
}

impl pallet_uniques::Config for Test {
	type Event = Event;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type CollectionDeposit = ClassDeposit;
	type ItemDeposit = InstanceDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type AttributeDepositBase = MetadataDepositBase;
	type DepositPerByte = MetadataDepositPerByte;
	type StringLimit = StringLimit;
	type KeyLimit = KeyLimit;
	type ValueLimit = ValueLimit;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<Self::AccountId>>;
	type Locker = ();
	
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
	type Balance = u64;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
}


parameter_types! {
	pub const MaxScopesPerPallet: u32 = 2;
	pub const MaxRolesPerPallet: u32 = 6;
	pub const RoleMaxLen: u32 = 25;
	pub const PermissionMaxLen: u32 = 25;
	pub const MaxPermissionsPerRole: u32 = 11;
	pub const MaxRolesPerUser: u32 = 2;
	pub const MaxUsersPerRole: u32 = 2;
}
impl pallet_rbac::Config for Test {
	type Event = Event;
	type MaxScopesPerPallet = MaxScopesPerPallet;
	type MaxRolesPerPallet = MaxRolesPerPallet;
	type RoleMaxLen = RoleMaxLen;
	type PermissionMaxLen = PermissionMaxLen;
	type MaxPermissionsPerRole = MaxPermissionsPerRole;
	type MaxRolesPerUser = MaxRolesPerUser;
	type MaxUsersPerRole = MaxUsersPerRole;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	// TODO: get initial conf?
	let mut t: sp_io::TestExternalities = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
	t.execute_with(|| GatedMarketplace::do_initial_setup().expect("Error on configuring initial setup"));
	t
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ();
	type WeightInfo = ();
}