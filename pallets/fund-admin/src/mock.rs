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
	pub const MaxDocuments:u32 = 5;
	pub const MaxProjectsPerUser:u32 = 10;
	pub const MaxUserPerProject:u32 = 50;
	pub const MaxBuildersPerProject:u32 = 1;
	pub const MaxInvestorsPerProject:u32 = 50;
	pub const MaxIssuersPerProject:u32 = 1;
	pub const MaxRegionalCenterPerProject:u32 = 1;
	pub const MaxBoundedVecs:u32 = 1;
	pub const MaxDrawdownsPerProject:u32 = 1000;
	pub const MaxTransactionsPerDrawdown:u32 = 500;
	pub const MaxRegistrationsAtTime:u32 = 50;
	pub const MaxExpendituresPerProject:u32 = 1000;

}

impl pallet_fund_admin::Config for Test {
	type Event = Event;
	type RemoveOrigin = EnsureRoot<Self::AccountId>;
	type MaxDocuments = MaxDocuments;
	type MaxProjectsPerUser = MaxProjectsPerUser;
	type MaxUserPerProject = MaxUserPerProject;
	type MaxBuildersPerProject = MaxBuildersPerProject;
	type MaxInvestorsPerProject = MaxInvestorsPerProject;
	type MaxIssuersPerProject = MaxIssuersPerProject;
	type MaxRegionalCenterPerProject = MaxRegionalCenterPerProject;
	type MaxBoundedVecs = MaxBoundedVecs;
	type MaxDrawdownsPerProject = MaxDrawdownsPerProject;
	type MaxTransactionsPerDrawdown = MaxTransactionsPerDrawdown;
	type MaxRegistrationsAtTime = MaxRegistrationsAtTime;
	type MaxExpendituresPerProject = MaxExpendituresPerProject;


	type Timestamp = Timestamp;
	type Moment = u64;
	type Rbac = RBAC;
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
	let mut t: sp_io::TestExternalities = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
	t.execute_with(|| FundAdmin::do_initial_setup().expect("Error on configuring initial setup"));
	t
}