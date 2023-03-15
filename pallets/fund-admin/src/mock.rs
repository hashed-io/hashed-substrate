use crate as pallet_fund_admin;
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
		FundAdmin: pallet_fund_admin::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		RBAC: pallet_rbac::{Pallet, Call, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
	type Balance = u64;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
}

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
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type AccountData = pallet_balances::AccountData<u64>;
}

parameter_types! {
	pub const MaxDocuments:u32 = 5;
	pub const MaxProjectsPerUser:u32 = 10;
	pub const MaxUserPerProject:u32 = 2000; // should be the sum of the max number of builders, investors, issuers, regional centers
	pub const MaxBuildersPerProject:u32 = 500;
	pub const MaxInvestorsPerProject:u32 = 500;
	pub const MaxIssuersPerProject:u32 = 500;
	pub const MaxRegionalCenterPerProject:u32 = 500;
	pub const MaxProjectsPerInvestor:u32 = 1;
	pub const MaxDrawdownsPerProject:u32 = 1000;
	pub const MaxTransactionsPerDrawdown:u32 = 500;
	pub const MaxRegistrationsAtTime:u32 = 50;
	pub const MaxExpendituresPerProject:u32 = 1000;
	pub const MaxBanksPerProject:u32 = 200;
	pub const MaxJobEligiblesByProject:u32 = 1000;
	pub const MaxRevenuesByProject:u32 = 1000;
	pub const MaxTransactionsPerRevenue:u32 = 500;
	pub const MaxStatusChangesPerDrawdown:u32 = 100;
	pub const MaxStatusChangesPerRevenue:u32 = 100;
	pub const MinAdminBalance:u64 = 10;
	pub const TransferAmount:u64 = 10;
	pub const InitialAdminBalance:u64 = 1_000_000;
}

impl pallet_fund_admin::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RemoveOrigin = EnsureRoot<Self::AccountId>;
	type Timestamp = Timestamp;
	type Moment = u64;
	type Rbac = RBAC;
	type Currency = Balances;

	type MaxDocuments = MaxDocuments;
	type MaxProjectsPerUser = MaxProjectsPerUser;
	type MaxUserPerProject = MaxUserPerProject;
	type MaxBuildersPerProject = MaxBuildersPerProject;
	type MaxInvestorsPerProject = MaxInvestorsPerProject;
	type MaxIssuersPerProject = MaxIssuersPerProject;
	type MaxRegionalCenterPerProject = MaxRegionalCenterPerProject;
	type MaxDrawdownsPerProject = MaxDrawdownsPerProject;
	type MaxTransactionsPerDrawdown = MaxTransactionsPerDrawdown;
	type MaxRegistrationsAtTime = MaxRegistrationsAtTime;
	type MaxExpendituresPerProject = MaxExpendituresPerProject;
	type MaxProjectsPerInvestor = MaxProjectsPerInvestor;
	type MaxBanksPerProject = MaxBanksPerProject;
	type MaxJobEligiblesByProject = MaxJobEligiblesByProject;
	type MaxRevenuesByProject = MaxRevenuesByProject;
	type MaxTransactionsPerRevenue = MaxTransactionsPerRevenue;
	type MaxStatusChangesPerDrawdown = MaxStatusChangesPerDrawdown;
	type MaxStatusChangesPerRevenue = MaxStatusChangesPerRevenue;
	type MinAdminBalance = MinAdminBalance;
	type TransferAmount = TransferAmount;
}


impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ();
	type WeightInfo = ();
}

parameter_types! {
	pub const MaxScopesPerPallet: u32 = 1000;
	pub const MaxRolesPerPallet: u32 = 50;
	pub const RoleMaxLen: u32 = 50;
	pub const PermissionMaxLen: u32 = 50;
	pub const MaxPermissionsPerRole: u32 = 100;
	pub const MaxRolesPerUser: u32 = 10;
	pub const MaxUsersPerRole: u32 = 2500;
}
impl pallet_rbac::Config for Test {
	type RuntimeEvent = RuntimeEvent;
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
	let balance_amount = InitialAdminBalance::get();
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, balance_amount)],
	}.assimilate_storage(&mut t).expect("assimilate_storage failed");
	let mut t: sp_io::TestExternalities = t.into();
	t.execute_with(|| FundAdmin::do_initial_setup().expect("Error on configuring initial setup"));
	t
}
