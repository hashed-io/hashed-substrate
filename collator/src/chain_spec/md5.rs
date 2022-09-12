use sc_service::ChainType;
use sp_core::{crypto::UncheckedInto, sr25519};
use hex_literal::hex;

use super::{
	get_account_id_from_seed, session_keys, SAFE_XCM_VERSION, Extensions,
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
					(
						// 5Dw5KkHeoh6umBurwqh4CHoXYBY6rpxtQampp8bFkuLgFz4L
						hex!["52a6c480ff5cff0a2b889016e71de5fd8c3ab6d7d7220e543e8d0f8d60142517"].into(),                    						
						hex!["52a6c480ff5cff0a2b889016e71de5fd8c3ab6d7d7220e543e8d0f8d60142517"].unchecked_into(),
					),
					(
						// 5GwwAKFomhgd4AHXZLUBVK3B792DvgQUnoHTtQNkwmt5h17k
						hex!["d8033c4d04a502901d24a789da32940085c62eba881c4701a73411288445cc46"].into(),                    						
						hex!["d8033c4d04a502901d24a789da32940085c62eba881c4701a73411288445cc46"].unchecked_into(),
					),
					(
						// 5DU84E1JYAhftyimxYd1MUaQ82GBKxNVFhDJSUSGU1ULpg1C
						hex!["3e1856f529530d07ec86f8ba00d9ef6a05520e9317d8025c8380b94670f90022"].into(),                    						
						hex!["3e1856f529530d07ec86f8ba00d9ef6a05520e9317d8025c8380b94670f90022"].unchecked_into(),
					),
					(
						// 5HBZ2CSDcRAjE6AKMKzuJ1w5c5iB6XaSn9h5eeAcGwcykKnz
						hex!["e266243731bf69fff27133f3cbb8def28c6fd26d688d14fee34ab6950351aa0f"].into(),                    						
						hex!["e266243731bf69fff27133f3cbb8def28c6fd26d688d14fee34ab6950351aa0f"].unchecked_into(),
					),
					(
						// 5Ft1pwMVeLRdRFiZNTtfxvnn1W8vPp71u215uoU4eDWixCok
						hex!["a8c9ba30f906cb94594c4d884e708064d5e173f5ee84eca771166542cb74f06c"].into(),                    						
						hex!["a8c9ba30f906cb94594c4d884e708064d5e173f5ee84eca771166542cb74f06c"].unchecked_into(),
					),
				],
				vec![
                    // 5HgAxuAcEybo448w5BZdoceCuHMAbEW9AetBKsj9s5GEBZT3
                    hex!["f83a0218e100ce3ede12c5d403116ef034124c62b181fff6935403cea9396d2f"].into(),   
                    // 5DkJvQp2gqHraWZU1BNCDxEKTQHezn2Qy7z5hLPksUdjtEG9                 
                    hex!["4a70d789b0f0897e0880e8d3d532187ac77cbda04228cfadf8bededdd0b1005e"].into(),
					// 5Dw5KkHeoh6umBurwqh4CHoXYBY6rpxtQampp8bFkuLgFz4L        
                    hex!["52a6c480ff5cff0a2b889016e71de5fd8c3ab6d7d7220e543e8d0f8d60142517"].into(),
					// 5GwwAKFomhgd4AHXZLUBVK3B792DvgQUnoHTtQNkwmt5h17k                
					hex!["d8033c4d04a502901d24a789da32940085c62eba881c4701a73411288445cc46"].into(),
					// 5DU84E1JYAhftyimxYd1MUaQ82GBKxNVFhDJSUSGU1ULpg1C               
					hex!["3e1856f529530d07ec86f8ba00d9ef6a05520e9317d8025c8380b94670f90022"].into(),
					// 5HBZ2CSDcRAjE6AKMKzuJ1w5c5iB6XaSn9h5eeAcGwcykKnz                  
					hex!["e266243731bf69fff27133f3cbb8def28c6fd26d688d14fee34ab6950351aa0f"].into(),
					// 5Ft1pwMVeLRdRFiZNTtfxvnn1W8vPp71u215uoU4eDWixCok                
					hex!["a8c9ba30f906cb94594c4d884e708064d5e173f5ee84eca771166542cb74f06c"].into(),
				],
				// 5HgAxuAcEybo448w5BZdoceCuHMAbEW9AetBKsj9s5GEBZT3
				hex!["f83a0218e100ce3ede12c5d403116ef034124c62b181fff6935403cea9396d2f"].into(), 
				2089.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		Some(properties),
		Extensions {
			relay_chain: "rococo".into(), // You MUST set this to the correct network!
			para_id: 2089,
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
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		sudo: SudoConfig { key: Some(root_key) },
		treasury: Default::default(),
		council: CouncilConfig {
			members: endowed_accounts,
			// vec![
			// 	hex!["f83a0218e100ce3ede12c5d403116ef034124c62b181fff6935403cea9396d2f"].into(),
			// 	hex!["4a70d789b0f0897e0880e8d3d532187ac77cbda04228cfadf8bededdd0b1005e"].into(),
			// ],
			phantom: Default::default(),
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

