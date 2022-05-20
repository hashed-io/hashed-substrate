use sp_core::crypto::KeyTypeId;
/*--- Constants section ---*/
pub const BDK_SERVICES_URL: &[u8] = b"https://bdk.hashed.systems/";
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
