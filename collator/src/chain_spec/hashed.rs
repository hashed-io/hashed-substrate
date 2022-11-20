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
	properties.insert("tokenSymbol".into(), "HASH".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 9072.into());
	properties.insert("prefix".into(), 9072.into());
	properties.insert("network".into(), "hashed".into());
	properties.insert("displayName".into(), "Hashed Network".into());
	properties.insert("standardAccount".into(),"*25519".into());
	properties.insert("website".into(), "https://hashed.network".into());

    // REVIEW: this is where the Hashed genesis is customized, for now,
    //  it is just a duplicate of the development configuration
	HashedChainSpec::from_genesis(
		"Hashed Network",
		"hashed",
		ChainType::Live,
		move || {
			hashed_genesis(
				// initial collators.
				vec![
					(
						// JA
                        // haZmb2DUWLEjv3z7NDn5nArSGoMgaN6xeJdAr9TDaeQYpxe6g
                        hex!["90150e105b07c3357d43ed5c727efb9be347699cb2b5a41a26423b559615b222"].into(),                    						
                        hex!["90150e105b07c3357d43ed5c727efb9be347699cb2b5a41a26423b559615b222"].unchecked_into(),
					),
					(
						// MdL
                        // haWa1d98tnLtvNCLSrdWmSo1DYHQDiKBoM9s5xNYtScTUwJ74
                        hex!["028c3a5c8890c3e98023b35f99a5d904b170612b78b6b9fdd8f60cbf24ab9f59"].into(),                    						
                        hex!["028c3a5c8890c3e98023b35f99a5d904b170612b78b6b9fdd8f60cbf24ab9f59"].unchecked_into(),
					),
				],
				vec![
					// GW
					// haWZRqBVdT9vDAM1Wpga7B1GZ99WDWnXsLyzCBtpwrjPLaZUa
                    hex!["021a78fcc3ec988411388ec2f8ab25fbb79a7eaacead997d13f211ebe34ce359"].into(),
				],
				hex!["021a78fcc3ec988411388ec2f8ab25fbb79a7eaacead997d13f211ebe34ce359"].into(), 
				2093.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions {
			relay_chain: "polkadot".into(), // You MUST set this to the correct network!
			para_id: 2093,
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

