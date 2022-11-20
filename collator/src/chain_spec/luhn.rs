use sc_service::ChainType;
// use sp_core::sr25519;
use sp_core::{crypto::UncheckedInto};

use hex_literal::hex;

use super::{
	/*get_account_id_from_seed, get_collator_keys_from_seed,*/ session_keys, SAFE_XCM_VERSION, Extensions,
};

use cumulus_primitives_core::ParaId;
use hashed_parachain_runtime::{AccountId, AuraId, SudoConfig, EXISTENTIAL_DEPOSIT};

/// Specialized `ChainSpec` for Hashed Network
pub type HashedChainSpec =
	sc_service::GenericChainSpec<hashed_parachain_runtime::GenesisConfig, Extensions>;

/// Gen HASH chain specification 
pub fn get_chain_spec() -> HashedChainSpec {

    let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "LUHN".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 11486.into());
	properties.insert("prefix".into(), 11486.into());
	properties.insert("network".into(), "luhn".into());
	properties.insert("displayName".into(), "Luhn Network".into());
	properties.insert("standardAccount".into(),"*25519".into());
	properties.insert("website".into(), "https://luhn.network".into());

	HashedChainSpec::from_genesis(
		"Luhn Network",
		"luhn",
		ChainType::Live,
		move || {
			hashed_genesis(
				// initial collators.
				vec![
					(
						// Collator #1
                        // uhsPQGuXYwjnLvoJWWttQ6FEVtztHSvsjE7UFzxS8mSfoSmts
                        hex!["1cfc7e49e91696b84bf8e931c16375ea634c3997b36155657faf7dc4716e273e"].into(),                    						
                        hex!["1cfc7e49e91696b84bf8e931c16375ea634c3997b36155657faf7dc4716e273e"].unchecked_into(),
					),
					(
						// Collator #2
                        // uhujXWvqSqGrY3K62qeamwCGQd7tTo1hm1s9ZYrgxAZFBLyLv
                        hex!["84ce3f0bc9ae73d8497c6161927e9e04f39f4bc54579689532d048188c10a77c"].into(),                    						
                        hex!["84ce3f0bc9ae73d8497c6161927e9e04f39f4bc54579689532d048188c10a77c"].unchecked_into(),
					),
				],
				vec![
					// PH
					// uhtqJBJ9ZeKguyAG4GJ2S7cme5FvJ661P5NVdHTYKQgvDEQAR
                    hex!["5cf8957922e4058a953281f82fdced2e4d389fe37c77f41a0fd2379df0caf877"].into(),
				],
				// uhtqJBJ9ZeKguyAG4GJ2S7cme5FvJ661P5NVdHTYKQgvDEQAR
				hex!["5cf8957922e4058a953281f82fdced2e4d389fe37c77f41a0fd2379df0caf877"].into(), 
				2232.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions {
			relay_chain: "kusama".into(), 
			para_id: 2232,
		},
	)
}

fn hashed_genesis(
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

