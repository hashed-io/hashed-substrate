use crate as pallet_afloat;
use frame_support::{
  parameter_types,
  traits::{AsEnsureOriginWithArg, ConstU32, ConstU64, Currency},
};
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

use crate::types::CreateAsset;
use frame_system::RawOrigin;
type AssetId = u32;

parameter_types! {
  pub const BlockHashCount: u64 = 250;
  pub const SS58Prefix: u8 = 42;
}

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
    Fruniques: pallet_fruniques::{Pallet, Call, Storage, Event<T>},
    Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
    Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
    RBAC: pallet_rbac::{Pallet, Call, Storage, Event<T>},
    Assets: pallet_mapped_assets::{Pallet, Call, Storage, Event<T>},
    Afloat: pallet_afloat::{Pallet, Call, Storage, Event<T>},
  }
);

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

impl pallet_afloat::Config for Test {
  type RuntimeEvent = RuntimeEvent;
  type TimeProvider = pallet_timestamp::Pallet<Self>;
  //type RemoveOrigin = frame_system::EnsureSigned<Self::AccountId>;
  type Currency = pallet_balances::Pallet<Self>;
  type Rbac = RBAC;
  type ItemId = u32;
}

parameter_types! {
  pub const LabelMaxLen: u32 = 32;
  pub const MaxAuthsPerMarket: u32 = 3;
  pub const MaxRolesPerAuth : u32 = 1;
  pub const MaxApplicants: u32 = 3;
  pub const MaxBlockedUsersPerMarket: u32 = 100;
  pub const NotesMaxLen: u32 = 256;
  pub const MaxFeedbackLen: u32 = 256;
  pub const NameMaxLen: u32 = 100;
  pub const MaxFiles: u32 = 10;
  pub const MaxApplicationsPerCustodian: u32 = 2;
  pub const MaxMarketsPerItem: u32 = 10;
  pub const MaxOffersPerMarket: u32 = 100;
}

impl pallet_gated_marketplace::Config for Test {
  type RuntimeEvent = RuntimeEvent;
  type MaxAuthsPerMarket = MaxAuthsPerMarket;
  type MaxRolesPerAuth = MaxRolesPerAuth;
  type MaxApplicants = MaxApplicants;
  type MaxBlockedUsersPerMarket = MaxBlockedUsersPerMarket;
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
parameter_types! {
  pub const ChildMaxLen: u32 = 10;
  pub const MaxParentsInCollection: u32 = 10;
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
  pub const MaxPermissionsPerRole: u32 = 30;
  pub const MaxRolesPerUser: u32 = 2;
  pub const MaxUsersPerRole: u32 = 2;
}
impl pallet_rbac::Config for Test {
  type RuntimeEvent = RuntimeEvent;
  type RemoveOrigin = EnsureRoot<Self::AccountId>;
  type MaxScopesPerPallet = MaxScopesPerPallet;
  type MaxRolesPerPallet = MaxRolesPerPallet;
  type RoleMaxLen = RoleMaxLen;
  type PermissionMaxLen = PermissionMaxLen;
  type MaxPermissionsPerRole = MaxPermissionsPerRole;
  type MaxRolesPerUser = MaxRolesPerUser;
  type MaxUsersPerRole = MaxUsersPerRole;
}

impl pallet_timestamp::Config for Test {
  type Moment = u64;
  type OnTimestampSet = ();
  type MinimumPeriod = ();
  type WeightInfo = ();
}

pub trait AssetsCallback<AssetId, AccountId> {
  /// Indicates that asset with `id` was successfully created by the `owner`
  fn created(_id: &AssetId, _owner: &AccountId) {}

  /// Indicates that asset with `id` has just been destroyed
  fn destroyed(_id: &AssetId) {}
}

pub struct AssetsCallbackHandle;
impl pallet_mapped_assets::AssetsCallback<u32, u64> for AssetsCallbackHandle {
  fn created(_id: &AssetId, _owner: &u64) {}

  fn destroyed(_id: &AssetId) {}
}

impl pallet_mapped_assets::Config for Test {
  type RuntimeEvent = RuntimeEvent;
  type Balance = u64;
  type AssetId = u32;
  type AssetIdParameter = u32;
  type Currency = Balances;
  type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<u64>>;
  type ForceOrigin = frame_system::EnsureRoot<u64>;
  type AssetDeposit = ConstU64<1>;
  type AssetAccountDeposit = ConstU64<10>;
  type MetadataDepositBase = ConstU64<1>;
  type MetadataDepositPerByte = ConstU64<1>;
  type ApprovalDeposit = ConstU64<1>;
  type StringLimit = ConstU32<50>;
  type Freezer = ();
  type WeightInfo = ();
  type CallbackHandle = AssetsCallbackHandle;
  type Extra = ();
  type RemoveItemsLimit = ConstU32<5>;
  type MaxReserves = MaxReserves;
  type ReserveIdentifier = u32;
  type Rbac = RBAC;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
  // TODO: get initial conf?
  let mut t: sp_io::TestExternalities =
    frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
  t.execute_with(|| {
    Balances::make_free_balance_be(&1, 100);
    Balances::make_free_balance_be(&2, 100);
    Afloat::initial_setup(
      RawOrigin::Root.into(),
      1,
      2,
      CreateAsset::New { asset_id: 0, min_balance: 1 },
    )
    .expect("Error on GatedMarketplace configuring initial setup");
  });
  t
}
