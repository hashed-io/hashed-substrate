use super::*;
use sp_core::crypto::KeyTypeId;
use frame_support::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
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


// Struct for holding Vaults information.
#[derive(
	Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen,
)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Vault<T : Config> {
	pub owner: T::AccountId,
	pub threshold: u32,
	pub description: BoundedVec<u8, T::VaultDescriptionMaxLen>,
	pub cosigners: BoundedVec<T::AccountId, T::MaxCosignersPerVault>,
	pub descriptors: Descriptors<T::OutputDescriptorMaxLen>,
	pub offchain_status: BDKStatus<T::VaultDescriptionMaxLen>,
}

impl<T: Config> Vault<T>{

	pub fn is_vault_member(&self, account: &T::AccountId)->bool{
		Self::get_vault_members(self).contains(account)
	}

	pub fn get_vault_members(&self) -> Vec<T::AccountId>{
		let mut members = [self.cosigners.clone().as_slice(),&[self.owner.clone()],].concat();
		members.sort();
        members.dedup();
		members
	}

	pub fn signers_are_unique(&self)-> bool {
		let mut filtered_signers = self.cosigners.clone().to_vec();
		filtered_signers.sort();
		filtered_signers.dedup();
		self.cosigners.len() == filtered_signers.len()
	}

	/// A vault must have valid descriptors in order to produce psbt's 
	pub fn is_valid(&self) -> bool{
		self.offchain_status.eq(&BDKStatus::Valid) && self.descriptors.are_not_empty()
	}
}

impl<T: Config> PartialEq for Vault<T>{
	fn eq(&self, other: &Self) -> bool{
		self.using_encoded(blake2_256) == other.using_encoded(blake2_256)
	}
}

impl<T: Config> Clone for Vault<T> {
	fn clone(&self) -> Self {
		Vault {
			owner: self.owner.clone(),
			threshold: self.threshold.clone(),
			cosigners: self.cosigners.clone(),
			description: self.description.clone(),
			descriptors: self.descriptors.clone(),
			offchain_status: self.offchain_status.clone(),
		}
	}
}

#[derive(Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct ProposalSignatures<T: Config> {
	pub signer: T::AccountId,
	pub signature: BoundedVec<u8, T::PSBTMaxLen>,
}

impl<T: Config> Clone for ProposalSignatures<T>{
	fn clone(&self) -> Self {
		Self{
			signer: self.signer.clone(),
			signature: self.signature.clone(),
		}
	}
}
// Struct for holding Proposal information.
#[derive(Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Proposal<T: Config> {
	pub proposer: T::AccountId,
	pub vault_id: [u8; 32],
	pub status: ProposalStatus,
	pub offchain_status: BDKStatus<T::VaultDescriptionMaxLen>,
	pub to_address: BoundedVec<u8, T::XPubLen>,
	pub amount: u64,
	pub fee_sat_per_vb: u32,
	pub description: BoundedVec<u8, T::VaultDescriptionMaxLen>,
	pub tx_id: Option< BoundedVec<u8, T::VaultDescriptionMaxLen> >,
	pub psbt: BoundedVec<u8, T::PSBTMaxLen>,
	pub signed_psbts: BoundedVec<ProposalSignatures<T>, T::MaxCosignersPerVault>,
}

impl<T: Config> Proposal<T>{
	pub fn can_be_finalized(&self) -> bool {
		self.status.is_ready_to_finalize() && self.offchain_status.eq(&BDKStatus::Valid)
	}

	// pub fn can_be_broadcasted(&self) -> bool {
	// 	self.status.eq(&ProposalStatus::ReadyToBroadcast) && self.offchain_status.eq(&BDKStatus::Valid)
	// }
}

impl<T: Config> Clone for Proposal<T>{
	fn clone(&self) -> Self {
		Self{
			proposer: self.proposer.clone(),
			vault_id: self.vault_id.clone(),
			status: self.status.clone(),
			offchain_status: self.offchain_status.clone(),
			to_address: self.to_address.clone(),
			amount: self.amount.clone(),
			fee_sat_per_vb: self.fee_sat_per_vb.clone(),
			description: self.description.clone(),
			tx_id: self.tx_id.clone(),
			psbt: self.psbt.clone(),
			signed_psbts: self.signed_psbts.clone(),
		}
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

impl <MaxLen: Get<u32>> Descriptors<MaxLen>{
	pub fn are_not_empty(&self)->bool{
		!self.output_descriptor.is_empty() && self.change_descriptor.is_some()
	}
}
	
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[codec(mel_bound())]
pub struct VaultsPayload<Public > {
	pub vaults_payload:Vec<SingleVaultPayload >,
	pub public: Public,
}

#[derive(Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(MaxLen))]
#[codec(mel_bound())]
pub struct SingleVaultPayload{
	// Not successful, macros/generics issue
	// descriptors: Descriptors<u8>,
	pub vault_id: [u8;32],
	pub output_descriptor: Vec<u8>,
	pub change_descriptor: Vec<u8>,
	pub status: OffchainStatus
}

impl Clone for SingleVaultPayload{
	fn clone(&self) -> Self {
		Self { 
			vault_id: self.vault_id.clone(), 
			output_descriptor: self.output_descriptor.clone(), 
			change_descriptor: self.change_descriptor.clone(), 
			status: self.status.clone() 
		}
    }
}
 
impl<S: SigningTypes > SignedPayload<S> for VaultsPayload<S::Public> {
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
	pub status: OffchainStatus,
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
	ReadyToFinalize(bool), //bool is the flag to broadcast automatically once finalized
	Finalized,
	//ReadyToBroadcast,
	Broadcasted,
}

impl ProposalStatus{
	pub fn is_ready_to_finalize(&self) ->bool{
		match *self{
			ProposalStatus::ReadyToFinalize(_) => true,
			_ => false,
		}
	}
	
	pub fn next_status(&self) -> Self {
		use ProposalStatus::*;
        match *self {
            Pending => ReadyToFinalize(false),
			ReadyToFinalize(false) => Finalized, // it will be finalized but the broadcast is still pending
			ReadyToFinalize(true) => Broadcasted, // the "true" flag value will finalize and broadcast it 
			Finalized => ReadyToFinalize(true),  // this will broadcast the tx
			//ReadyToBroadcast => Broadcasted, // not used, but not discarded 
			Broadcasted => Broadcasted
        }
    }
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo)]
pub enum OffchainStatus{
	Pending,
	Valid,
	RecoverableError(Vec<u8>),
	IrrecoverableError(Vec<u8>),
}


//Default macro didnt work
impl Default for OffchainStatus{
	fn default() -> Self {
		OffchainStatus::Pending
	}
}

impl<MaxLen: Get<u32> > From<OffchainStatus> for BDKStatus<MaxLen>{
    fn from( status: OffchainStatus) -> Self {
        match status {
			OffchainStatus::Pending => BDKStatus::Pending,
			OffchainStatus::Valid => BDKStatus::Valid,
			OffchainStatus::RecoverableError(msj) => BDKStatus::RecoverableError(
				BoundedVec::<u8,MaxLen>::try_from(msj).unwrap_or_default()
			),
			OffchainStatus::IrrecoverableError(msj) => BDKStatus::IrrecoverableError(
				BoundedVec::<u8, MaxLen>::try_from(msj).unwrap_or_default()
			),
		}
    }
}


#[derive(Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(MaxLen))]
#[codec(mel_bound())]
pub enum BDKStatus<MaxLen: Get<u32> >{
	Pending,
	Valid,
	RecoverableError(BoundedVec<u8, MaxLen>),
	IrrecoverableError(BoundedVec<u8, MaxLen>),
}
impl<MaxLen: Get<u32> > Default for BDKStatus<MaxLen>{
	fn default() -> Self {
		BDKStatus::Pending
	}
}
// Clone macro didnt work
impl<MaxLen: Get<u32> >  Clone for BDKStatus<MaxLen>{
    fn clone(&self) -> Self {
        match self {
            Self::Pending => Self::Pending,
            Self::Valid => Self::Valid,
            Self::RecoverableError(arg0) => Self::RecoverableError(arg0.clone()),
            Self::IrrecoverableError(arg0) => Self::IrrecoverableError(arg0.clone()),
        }
    }
}

impl<MaxLen: Get<u32> >  PartialEq for BDKStatus<MaxLen> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::RecoverableError(_), Self::RecoverableError(_)) => true,
            (Self::IrrecoverableError(l0), Self::IrrecoverableError(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }

    fn ne(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::RecoverableError(l0), Self::RecoverableError(r0)) => l0 == r0,
            (Self::IrrecoverableError(l0), Self::IrrecoverableError(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}