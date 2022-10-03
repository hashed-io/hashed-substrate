use crate as pallet_proxy;
use frame_support::parameter_types;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use frame_system::EnsureRoot;


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
		Proxy: pallet_proxy::{Pallet, Call, Storage, Event<T>},
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
	pub const ProjectNameMaxLen:u32 = 32;
	pub const ProjectDescMaxLen:u32 = 256;
	pub const MaxDocuments:u32 = 5;
	pub const MaxAccountsPerTransaction:u32 = 5;
	pub const MaxProjectsPerUser:u32 = 10;
	pub const CIDMaxLen:u32 = 100;	
	pub const MaxUserPerProject:u32 = 50;
	pub const MaxDevelopersPerProject:u32 = 1;
	pub const MaxInvestorsPerProject:u32 = 50;
	pub const MaxIssuersPerProject:u32 = 1;
	pub const MaxRegionalCenterPerProject:u32 = 1;
	pub const MaxBoundedVecs:u32 = 1;
	pub const MaxExpendituresPerProject:u32 = 1000;
	pub const MaxBudgetsPerProject:u32 = 1000;
	pub const MaxDrawdownsPerProject:u32 = 1000;

}

impl pallet_proxy::Config for Test {
	type Event = Event;
	type RemoveOrigin = EnsureRoot<Self::AccountId>;
	type ProjectNameMaxLen = ProjectNameMaxLen;
	type ProjectDescMaxLen = ProjectDescMaxLen;
	type MaxDocuments = MaxDocuments;
	type MaxAccountsPerTransaction = MaxAccountsPerTransaction;
	type MaxProjectsPerUser = MaxProjectsPerUser;
	type CIDMaxLen = CIDMaxLen;
	type MaxUserPerProject = MaxUserPerProject;
	type MaxDevelopersPerProject = MaxDevelopersPerProject;
	type MaxInvestorsPerProject = MaxInvestorsPerProject;
	type MaxIssuersPerProject = MaxIssuersPerProject;
	type MaxRegionalCenterPerProject = MaxRegionalCenterPerProject;
	type MaxBoundedVecs = MaxBoundedVecs;
	type MaxExpendituresPerProject = MaxExpendituresPerProject;
	type MaxBudgetsPerProject = MaxBudgetsPerProject; 
	type MaxDrawdownsPerProject = MaxDrawdownsPerProject;

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
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
