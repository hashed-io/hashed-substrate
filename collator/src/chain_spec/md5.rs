use sc_service::ChainType;
use sp_core::{crypto::UncheckedInto, sr25519};
use hex_literal::hex;

use super::{
	get_account_id_from_seed, session_keys, SAFE_XCM_VERSION, Extensions,
};

use cumulus_primitives_core::ParaId;
use hashed_parachain_runtime::{AccountId, AuraId, EXISTENTIAL_DEPOSIT};

/// Specialized `ChainSpec` for MD5 Network.
pub type Md5ChainSpec =
	sc_service::GenericChainSpec<hashed_parachain_runtime::GenesisConfig, Extensions>;

/// Gen MD5 chain specification 
pub fn get_chain_spec() -> Md5ChainSpec {

    let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "MD5".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 5000.into());

	Md5ChainSpec::from_genesis(
		"MD5 Network",
		"md5",
		ChainType::Live,
		move || {
			md5_genesis(
				// initial collators.
				vec![
					(
                        // 5HgAxuAcEybo448w5BZdoceCuHMAbEW9AetBKsj9s5GEBZT3
                        hex!["f83a0218e100ce3ede12c5d403116ef034124c62b181fff6935403cea9396d2f"].into(),                    						
                        hex!["f83a0218e100ce3ede12c5d403116ef034124c62b181fff6935403cea9396d2f"].unchecked_into(),
					),
					(
                        // 5DkJvQp2gqHraWZU1BNCDxEKTQHezn2Qy7z5hLPksUdjtEG9
                        hex!["4a70d789b0f0897e0880e8d3d532187ac77cbda04228cfadf8bededdd0b1005e"].into(),                    						
                        hex!["4a70d789b0f0897e0880e8d3d532187ac77cbda04228cfadf8bededdd0b1005e"].unchecked_into(),
					),
				],
				vec![
                    // 5HgAxuAcEybo448w5BZdoceCuHMAbEW9AetBKsj9s5GEBZT3
                    hex!["f83a0218e100ce3ede12c5d403116ef034124c62b181fff6935403cea9396d2f"].into(),   
                    // 5DkJvQp2gqHraWZU1BNCDxEKTQHezn2Qy7z5hLPksUdjtEG9                 
                    hex!["4a70d789b0f0897e0880e8d3d532187ac77cbda04228cfadf8bededdd0b1005e"].into(),
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				4088.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions {
			relay_chain: "rococo".into(), // You MUST set this to the correct network!
			para_id: 4088,
		},
	)
}

fn md5_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> hashed_parachain_runtime::GenesisConfig {
	hashed_parachain_runtime::GenesisConfig {
		system: hashed_parachain_runtime::SystemConfig {
			code: hashed_parachain_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: hashed_parachain_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		parachain_info: hashed_parachain_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: hashed_parachain_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: hashed_parachain_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
						session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: hashed_parachain_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
	}
}

