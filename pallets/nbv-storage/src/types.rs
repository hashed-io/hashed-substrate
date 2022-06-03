//use super::*;
use sp_core::crypto::KeyTypeId;
use frame_support::pallet_prelude::*;
use sp_runtime::{sp_std::vec::Vec};
use frame_system::offchain::{SigningTypes, SignedPayload};
//pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
/*--- Constants section ---*/
//pub const BDK_SERVICES_URL: &[u8] = b"https://bdk.hashed.systems";
pub const UNSIGNED_TXS_PRIORITY: u64 = 100;
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"bdks");

pub const LOCK_BLOCK_EXPIRATION: u32 = 5; // in block number
pub const LOCK_TIMEOUT_EXPIRATION: u64 = 10000; // in milli-seconds

/*--- Crypto module section---*/
pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	// implemented for runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

#[derive(
	Encode,
	Decode,
	Default,
	Eq,
	PartialEq,
	CloneNoBound,
	RuntimeDebugNoBound,
	TypeInfo,
	MaxEncodedLen,
)]
#[scale_info(skip_type_params(MaxLen))]
#[codec(mel_bound())]
pub struct Descriptors<MaxLen: Get<u32>> {
	pub output_descriptor: BoundedVec<u8, MaxLen>,
	pub change_descriptor: Option<BoundedVec<u8, MaxLen>>,
}


	
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[codec(mel_bound())]
pub struct VaultsPayload<Public> {
	pub vaults_payload:Vec<SingleVaultPayload>,
	pub public: Public,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[codec(mel_bound())]
pub struct SingleVaultPayload{
	// Not successful, macros/generics issue
	// descriptors: Descriptors<u8>,
	pub vault_id: [u8;32],
	pub output_descriptor: Vec<u8>,
	pub change_descriptor: Vec<u8>,
}

impl<S: SigningTypes> SignedPayload<S> for VaultsPayload<S::Public> {
	fn public(&self) -> S::Public {
		self.public.clone()
	}
}

/// Struct for requesting a descriptor generation 
#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ProposalRequest<DescriptorMaxLen: Get<u32>, XPubLen: Get<u32>> {
	pub descriptors: Descriptors<DescriptorMaxLen>,
	pub to_address: BoundedVec<u8, XPubLen>,
	pub amount: u64,
	pub fee_sat_per_vb: u32,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[codec(mel_bound())]
pub struct ProposalsPayload<Public> {
	pub proposals_payload:Vec<SingleProposalPayload>,
	pub public: Public,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[codec(mel_bound())]
pub struct SingleProposalPayload{
	pub proposal_id: [u8;32],
	pub psbt: Vec<u8>,
}

impl<S: SigningTypes > SignedPayload<S> for ProposalsPayload<S::Public> {
	fn public(&self) -> S::Public {
		self.public.clone()
	}
}

pub enum XpubStatus {
	Owned,
	Free,
	Taken,
}

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum ProposalStatus {
	Pending,
	Broadcasted,
}

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo,)]
pub enum OffChainStatus<MaxLen: Get<u32> >{
	Pending,
	Done,
	RecoverableError(BoundedVec<u8, MaxLen>),
	IrrecoverableError(BoundedVec<u8, MaxLen>),
}