use crate as pallet_fruniques;
use frame_support::{construct_runtime, parameter_types, traits::AsEnsureOriginWithArg};
use frame_system::EnsureSigned;
use frame_system::EnsureRoot;
use pallet_balances;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Uniques: pallet_uniques::{Pallet, Call, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Fruniques: pallet_fruniques::{Pallet, Call, Storage, Event<T>},
		RBAC: pallet_rbac::{Pallet, Call, Storage, Event<T>},
	}
);
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const ChildMaxLen: u32 = 10;
	pub const MaxParentsInCollection: u32 = 100;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
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
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_fruniques::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RemoveOrigin = EnsureRoot<Self::AccountId>;
	type ChildMaxLen = ChildMaxLen;
	type MaxParentsInCollection = MaxParentsInCollection;
	type Rbac = RBAC;
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
	type RuntimeEvent = RuntimeEvent;
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
	type RuntimeEvent = RuntimeEvent;
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
// pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
// 	frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
// }
// Build genesis storage according to the mock runtime.


pub fn new_test_ext() -> sp_io::TestExternalities {
	let balance_amount = 1_000_000 as u64;
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, balance_amount), (2, balance_amount), (3, balance_amount)],
	}.assimilate_storage(&mut t).expect("assimilate_storage failed");
	let mut t: sp_io::TestExternalities = t.into();
	t.execute_with(|| Fruniques::do_initial_setup().expect("Error on configuring initial setup"));
	t
}
