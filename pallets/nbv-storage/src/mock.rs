use crate as pallet_nbv_storage;
use frame_support::{parameter_types, traits::ConstU32};
use frame_support::traits::{ConstU64};
use frame_system::EnsureRoot;
//use frame_system as system;
use pallet_balances;
use sp_core::H256;
//use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
use sp_runtime::{
	testing::{Header,TestXt},
	traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Extrinsic as ExtrinsicT, Verify},
	//RuntimeAppPublic,
};
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
//use sp_runtime::generic::SignedPayload;
use sp_core::sr25519::{Signature,};

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		NBVStorage: pallet_nbv_storage::{Pallet, Call, Storage, Event<T>, ValidateUnsigned},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
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
	pub const XPubLen: u32 = 166;
	pub const PSBTMaxLen: u32  = 2048;
	pub const MaxVaultsPerUser: u32 = 2;
	pub const MaxCosignersPerVault: u32 =3;
	pub const VaultDescriptionMaxLen: u32 = 200;
	pub const OutputDescriptorMaxLen: u32 = 2048;
	pub const MaxProposalsPerVault : u32 = 2;
}

impl pallet_nbv_storage::Config for Test {
	type AuthorityId = pallet_nbv_storage::types::crypto::TestAuthId;
	type Event = Event;
	type ChangeBDKOrigin = EnsureRoot<AccountId>;
	type XPubLen = XPubLen;
	type PSBTMaxLen = PSBTMaxLen;
	type MaxVaultsPerUser = MaxVaultsPerUser;
	type MaxCosignersPerVault = MaxCosignersPerVault;
	type VaultDescriptionMaxLen =VaultDescriptionMaxLen;
	type OutputDescriptorMaxLen = OutputDescriptorMaxLen;
	type MaxProposalsPerVault = MaxProposalsPerVault;
}

type Extrinsic = TestXt<Call, ()>;
type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

impl frame_system::offchain::SigningTypes for Test {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		_public: <Signature as Verify>::Signer,
		_account: AccountId,
		nonce: u64,
	) -> Option<(Call, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
		Some((call, (nonce, ())))
	}
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
	type AccountId = sp_core::sr25519::Public;
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

pub fn test_pub(n : u8) -> sp_core::sr25519::Public {
	sp_core::sr25519::Public::from_raw([n; 32])
}

// Build genesis storage according to the mock runtime.
 pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(test_pub(1), 10000), (test_pub(2), 1000), (test_pub(3), 1000)],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
	 
 }
