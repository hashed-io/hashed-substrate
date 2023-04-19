use sc_service::ChainType;
use sp_core::{crypto::UncheckedInto};
use hex_literal::hex;

use super::{
	session_keys, SAFE_XCM_VERSION, Extensions,
};

use cumulus_primitives_core::ParaId;
use hashed_parachain_runtime::{AccountId, AuraId, CouncilConfig, SudoConfig, EXISTENTIAL_DEPOSIT};

/// Specialized `ChainSpec` for MD5 Network.
pub type Md5ChainSpec =
	sc_service::GenericChainSpec<hashed_parachain_runtime::GenesisConfig, Extensions>;

/// Gen MD5 chain specification
pub fn get_chain_spec() -> Md5ChainSpec {

    let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "MD5".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 5000.into());
	properties.insert("prefix".into(), 5000.into());
	properties.insert("network".into(), "md5".into());
	properties.insert("displayName".into(), "MD5 Network".into());
	properties.insert("standardAccount".into(),"*25519".into());
	properties.insert("website".into(), "https://hashed.network".into());

	Md5ChainSpec::from_genesis(
		"MD5 Network",
		"md5",
		ChainType::Live,
		move || {
			md5_genesis(
				// initial collators.
				vec![
					(

                        // 5GuMRvQ38upeanVE9gZTY6yPGXQbi1K9cmX51qnxQzpyPdsQ
                        hex!["d60b3240b281079b932d36e549e3d0d66df3d44824a268195be6ab2daaf8855e"].into(),
                        hex!["d60b3240b281079b932d36e549e3d0d66df3d44824a268195be6ab2daaf8855e"].unchecked_into(),
					),
					(

                        // 5Gpc7qF343nKSr68PKPvnXUWFrfeDJjpCs7WBgovWzEx2JZx
                        hex!["d26c69f5c67860a3d5a555ebc1092f47a68d4f8455e22051519b3ea81153c448"].into(),
                        hex!["d26c69f5c67860a3d5a555ebc1092f47a68d4f8455e22051519b3ea81153c448"].unchecked_into(),
					),
				],
				vec![
                    // 5HMo3Dmbw7fZaFYdFZNKGdJ63WrY9Mv8itAVHsJUM3hhWvML
                    hex!["ea35c8785e0711bfe4698116f68033981e431a64a294529b7019c568df5bb82c"].into(),
				],
				// 5HMo3Dmbw7fZaFYdFZNKGdJ63WrY9Mv8itAVHsJUM3hhWvML
				hex!["ea35c8785e0711bfe4698116f68033981e431a64a294529b7019c568df5bb82c"].into(),
				2093.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions {
			relay_chain: "rococo".into(), // You MUST set this to the correct network!
			para_id: 2093,
		},
	)
}

fn md5_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	root_key: AccountId,
	id: ParaId,
) -> hashed_parachain_runtime::GenesisConfig {
	hashed_parachain_runtime::GenesisConfig {
		system: hashed_parachain_runtime::SystemConfig {
			code: hashed_parachain_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: hashed_parachain_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1000000000000000000000000000)).collect(),
		},
		sudo: SudoConfig { key: Some(root_key) },
		council: Default::default(),
		treasury: Default::default(),
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

