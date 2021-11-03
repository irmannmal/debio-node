use debio_runtime::{
	currency::DBIO, opaque::Block, opaque::SessionKeys, AccountId, Balance, Signature,
	WASM_BINARY, BABE_GENESIS_EPOCH_CONFIG,
	GenesisConfig, BabeConfig, BalancesConfig,
	OctopusAppchainConfig, OctopusLposConfig, SessionConfig, SudoConfig, SystemConfig,
	OrdersConfig, RewardsConfig, LabsConfig, ServiceRequestConfig,
};
use beefy_primitives::crypto::AuthorityId as BeefyId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_octopus_appchain::AuthorityId as OctopusId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::{ChainType, Properties};
use serde::{Deserialize, Serialize};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

use hex_literal::hex;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	beefy: BeefyId,
	octopus: OctopusId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online, beefy, octopus }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
	seed: &str,
) -> (AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId) {
	(
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<BeefyId>(seed),
		get_from_seed::<OctopusId>(seed),
	)
}

/// Helper function to generate an properties
pub fn get_properties(symbol: &str, decimals: u32, ss58format: u32) -> Properties {
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), symbol.into());
	properties.insert("tokenDecimals".into(), decimals.into());
	properties.insert("ss58Format".into(), ss58format.into());

	properties
}

/// Helper function to generate appchain config
pub fn appchain_config(
	relay_contract: &str,
	asset_id_by_name: &str,
	premined_amount: Balance,
	era_payout: Balance,
) -> (String, String, Balance, Balance) {
	(
		relay_contract.to_string(),
		asset_id_by_name.to_string(),
		premined_amount,
		era_payout,
	)
}

pub fn testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("DBIO", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"DeBio Testnet",
		// ID
		"debio_testnet",
		ChainType::Live,
		move || {
			genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5FR73HBVwSpPjnPsBZPDVyuHQS1KE8jvSL3pSud6F6HZcuBA
				hex!["9443a63297b9f5b4e2569ee17225011db11a537066bce62d018acbcfda88f947"].into(),
				// Initial PoA authorities
				vec![],
				// Pre-funded accounts
				vec![],
				// Appchain config
				appchain_config(
					// Appchain Relay Contract
					"",
					// Appchain Asset Id by Name
					"usdc.testnet",
					// Premined Amount
					0,
					// Era Payout
					1024,
				),
				// API admin account
				// 5FpcRYvUMB3bNRdbj5YDwKeGHKVeWmdjnzY45RdTJSoSGcKN
				hex!["a63135764844b7b889f0447cc5127c4aa1b78fb998878549bf66ed7b0ee49753"].into(),
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("debio-testnet"),
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

pub fn staging_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("DBIO", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"Debio Staging Testnet",
		// ID
		"debio_staging_testnet",
		ChainType::Live,
		move || {
			genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5CB5udaxY6zFqApVHWPQTGTW5FszotkXKAUD48fvi5Y7FSR2
				hex!["04ddb3f730857ed801327da2242dff4d4d85e25b33c43db6f328d55904247f40"].into(),
				// Initial PoA authorities
				vec![
					(
						// 5DWyDncRWXBuQHwJkwndcxD8EpiNjC5aUpkvQvH5pKWW31kS
						hex!["4044558c867f510c90406c029d4132552cff769af982df767536607126f20b3e"].into(),
						// 5DWyDncRWXBuQHwJkwndcxD8EpiNjC5aUpkvQvH5pKWW31kS
						hex!["4044558c867f510c90406c029d4132552cff769af982df767536607126f20b3e"].unchecked_into(),
						// 5F3w32CUTSdx6tVtKqtmX5ySxv2EWtf5ALozHXdm1sRmDRyn
						hex!["841dd15656fe6f518d7f834be42c8ebac03856b973fe0cfe884d3bf63f54044e"].unchecked_into(),
						// 5DWyDncRWXBuQHwJkwndcxD8EpiNjC5aUpkvQvH5pKWW31kS
						hex!["4044558c867f510c90406c029d4132552cff769af982df767536607126f20b3e"].unchecked_into(),
						// KWE8SLkhBFQDYC6mp9BXGFvtHHoXWgVspsnrHhXXwGxdd52J1
						hex!["03ef93c4f8f2b34f0945ac57f666555f9a6a211cbb7f21118dfc8049100347162d"].unchecked_into(),
						// 5DWyDncRWXBuQHwJkwndcxD8EpiNjC5aUpkvQvH5pKWW31kS
						hex!["4044558c867f510c90406c029d4132552cff769af982df767536607126f20b3e"].unchecked_into(),
					),
					(
						// 5CaJm3bpWi3ieWYHcbz4xd7MrF8Njma4p7tGTBwemRbYnknT
						hex!["16939c61baa637549e3a90277790655b5c5ce0e60ea9688559f9da587b2cb419"].into(),
						// 5CaJm3bpWi3ieWYHcbz4xd7MrF8Njma4p7tGTBwemRbYnknT
						hex!["16939c61baa637549e3a90277790655b5c5ce0e60ea9688559f9da587b2cb419"].unchecked_into(),
						// 5Ca1whan2BfjBU4JYkcK2HwkkJxcgUvgkXHd1q3sn1kMM2iX
						hex!["165b008e2e3181f869893b237406e847daa043d90bf80e62bb8a8442281afac0"].unchecked_into(),
						// 5CaJm3bpWi3ieWYHcbz4xd7MrF8Njma4p7tGTBwemRbYnknT
						hex!["16939c61baa637549e3a90277790655b5c5ce0e60ea9688559f9da587b2cb419"].unchecked_into(),
						// KW6TMKohWcZBna5xBkcthEuGX1p5R7gn6z8eqCMBDjEWBLUbA
						hex!["029c1ead8e295430573bb984b8b38c9479b7a9a236725d7c2090182fd38bf4d9b5"].unchecked_into(),
						// 5CaJm3bpWi3ieWYHcbz4xd7MrF8Njma4p7tGTBwemRbYnknT
						hex!["16939c61baa637549e3a90277790655b5c5ce0e60ea9688559f9da587b2cb419"].unchecked_into(),
					),
				],
				// Pre-funded accounts
				vec![
					// Sudo account
					// 5CB5udaxY6zFqApVHWPQTGTW5FszotkXKAUD48fvi5Y7FSR2
					hex!["04ddb3f730857ed801327da2242dff4d4d85e25b33c43db6f328d55904247f40"].into(),
					// API admin account
					// 5ELYNFhFz9tauMxfjgTGhd6sRbnndddEXqh3UxWsPi6Rjajg
					hex!["648c728f7fcf0ae26a44410cf0ba4ea15b27b3169a4f809a14097680b8d0bc53"].into(),
				],
				// Appchain config
				appchain_config(
					// Appchain Relay Contract
					"",
					// Appchain Asset Id by Name
					"usdc.testnet",
					// Premined Amount
					0,
					// Era Payout
					1024,
				),
				// API admin account
				// 5ELYNFhFz9tauMxfjgTGhd6sRbnndddEXqh3UxWsPi6Rjajg
				hex!["648c728f7fcf0ae26a44410cf0ba4ea15b27b3169a4f809a14097680b8d0bc53"].into(),
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("debio-staging-testnet"),
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

pub fn development_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("DBIO", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"DeBio Development Testnet",
		// ID
		"debio_development_testnet",
		ChainType::Live,
		move || {
			genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5G3nLeySH5sFzD9WPKt2kB3KNVnazsZykaFfotouvjf1RZWY
				hex!["b03cc727c3c98eab988e5acfa815f6e6ed1939060471adaa78d2e39bbb1fc50b"].into(),
				// Initial PoA authorities
				vec![
					(
						// 5FNUtTJn1hhx1JEBrtWz9yaGx7M19hGhWZonxaJFHFu6SQ6C
						hex!["92437599810542e6c9e435290225920cb7b8174a949ed8f67b3413c6435ad76c"].into(),
						// 5FNUtTJn1hhx1JEBrtWz9yaGx7M19hGhWZonxaJFHFu6SQ6C
						hex!["92437599810542e6c9e435290225920cb7b8174a949ed8f67b3413c6435ad76c"].unchecked_into(),
						// 5DZQ8hkpX2STvCDKxnisDS4M3wKr8T4irH7Kb6pi1opYWicR
						hex!["421eaffb5d5601b080f546fa8be621d26085a2743b4d935d2b8dd83c2cecaa39"].unchecked_into(),
						// 5FNUtTJn1hhx1JEBrtWz9yaGx7M19hGhWZonxaJFHFu6SQ6C
						hex!["92437599810542e6c9e435290225920cb7b8174a949ed8f67b3413c6435ad76c"].unchecked_into(),
						// KW39i1yj3MYMcCaF5QZUbk8FBPbEzbrn1E6A3Xdmw4beErUGT
						hex!["0209f537ca85f50055cf9553d72c8a594516a915b6c040109ed5450da0185c3ff1"].unchecked_into(),
						// 5FNUtTJn1hhx1JEBrtWz9yaGx7M19hGhWZonxaJFHFu6SQ6C
						hex!["92437599810542e6c9e435290225920cb7b8174a949ed8f67b3413c6435ad76c"].unchecked_into(),
					),
					(
						// 5DF6RP41YxxgE8yemXAH47aJo9313TG7pVvx1utM4a9WnKk5
						hex!["3428a50b8746e28304b67a2a8dfd5fc40c0ee17c28ce129c5db1ac42c4e9905a"].into(),
						// 5DF6RP41YxxgE8yemXAH47aJo9313TG7pVvx1utM4a9WnKk5
						hex!["3428a50b8746e28304b67a2a8dfd5fc40c0ee17c28ce129c5db1ac42c4e9905a"].unchecked_into(),
						// 5CetaryC3UwJEwSJvo8GzLVM4kxejioSfjmoZyAX4TKPSNuq
						hex!["1a1274a58903a684d89cd926735137961a795d798b250926f7c8867b487549d8"].unchecked_into(),
						// 5DF6RP41YxxgE8yemXAH47aJo9313TG7pVvx1utM4a9WnKk5
						hex!["3428a50b8746e28304b67a2a8dfd5fc40c0ee17c28ce129c5db1ac42c4e9905a"].unchecked_into(),
						// KW2ywDzHXAmvvcCZu14szXHdsXka9Xuez4Q1RPuXMkw2VTZYk
						hex!["020281390b3b2a5f25dcda82477a2da7a00a2570724b24d60e82446a63f81db4c7"].unchecked_into(),
						// 5DF6RP41YxxgE8yemXAH47aJo9313TG7pVvx1utM4a9WnKk5
						hex!["3428a50b8746e28304b67a2a8dfd5fc40c0ee17c28ce129c5db1ac42c4e9905a"].unchecked_into(),
					),
				],
				// Pre-funded accounts
				vec![
					// Sudo account
					// 5G3nLeySH5sFzD9WPKt2kB3KNVnazsZykaFfotouvjf1RZWY
					hex!["b03cc727c3c98eab988e5acfa815f6e6ed1939060471adaa78d2e39bbb1fc50b"].into(),
					// API admin account
					// C8KpmHUFT7HJbNLv74cXrtT1w9LF1W3WduN8nVGQUySSJTF
					hex!["02c2cffef38fbf56b32d6a49eeeecc0e3345a1e0549cd8817d52f6cf2e414152"].into(),
				],
				// Appchain config
				appchain_config(
					// Appchain Relay Contract
					"",
					// Appchain Asset Id by Name
					"usdc.testnet",
					// Premined Amount
					0,
					// Era Payout
					1024,
				),
				// API admin account
				// C8KpmHUFT7HJbNLv74cXrtT1w9LF1W3WduN8nVGQUySSJTF
				hex!["02c2cffef38fbf56b32d6a49eeeecc0e3345a1e0549cd8817d52f6cf2e414152"].into(),
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("debio-development-testnet"),
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

pub fn local_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("DBIO", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"DeBio Local",
		// ID
		"debio_local",
		ChainType::Local,
		move || {
			genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial PoA authorities
				vec![
					authority_keys_from_seed("Alice"),
					authority_keys_from_seed("Bob"),
				],
				// Pre-funded accounts
				vec![
					// Sudo account and API admin account
					// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					// Additionals
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				],
				// Appchain config
				appchain_config(
					// Appchain Relay Contract
					"",
					// Appchain Asset Id by Name
					"usdc.testnet",
					// Premined Amount
					0,
					// Era Payout
					1024,
				),
				// API admin account
				// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				get_account_id_from_seed::<sr25519::Public>("Alice"),
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("debio-local"),
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM not available".to_string())?;
	let properties = get_properties("DBIO", 18, 42);

	Ok(ChainSpec::from_genesis(
		// Name
		"DeBio Development",
		// ID
		"debio_development",
		ChainType::Development,
		move || {
			genesis(
				// WASM Binary
				wasm_binary,
				// Sudo account
				// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Pre-funded accounts
				vec![
					// Sudo account and API admin account
					// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					// Additionals
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				],
				// Appchain config
				appchain_config(
					// Appchain Relay Contract
					"",
					// Appchain Asset Id by Name
					"usdc.testnet",
					// Premined Amount
					0,
					// Era Payout
					1024,
				),
				// API admin account
				// 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				get_account_id_from_seed::<sr25519::Public>("Alice"),
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some("debio-development"),
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

/// Configure initial storage state for FRAME modules.
fn genesis(
	wasm_binary: &[u8],
	root_key: AccountId,
	initial_authorities: Vec<(AccountId, BabeId, GrandpaId, ImOnlineId, BeefyId, OctopusId)>,
	endowed_accounts: Vec<AccountId>,
	appchain_config: (String, String, Balance, Balance),
	api_admin_key: AccountId,
) -> GenesisConfig {
	const ENDOWMENT: Balance = 1_000_000 * DBIO;
	const STASH: Balance = 100 * DBIO;

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).collect(),
		},
		sudo: SudoConfig { key: root_key },
		babe: BabeConfig { authorities: Default::default(), epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG) },
		grandpa: Default::default(),
		im_online: Default::default(),
		beefy: Default::default(),
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(
							x.1.clone(),
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
						),
					)
				})
				.collect(),
		},
		assets: Default::default(),
		octopus_appchain: OctopusAppchainConfig {
			anchor_contract: appchain_config.0,
			asset_id_by_name: vec![(appchain_config.1, 0)],
			premined_amount: appchain_config.2,
			validators: initial_authorities.iter().map(|x| (x.0.clone(), STASH)).collect(),
		},
		octopus_lpos: OctopusLposConfig { era_payout: appchain_config.3, ..Default::default() },
		orders: OrdersConfig {
			escrow_key: api_admin_key.clone(),
		},
        rewards: RewardsConfig {
            rewarder_key: api_admin_key.clone(),
        },
        labs: LabsConfig {
            lab_verifier_key: api_admin_key.clone(),
        },
		service_request: ServiceRequestConfig {
            admin_key: api_admin_key.clone(),
        },
	}
}
