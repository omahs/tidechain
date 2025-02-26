// Copyright 2021-2022 Semantic Network Ltd.
// This file is part of Tidechain.

// Tidechain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tidechain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tidechain.  If not, see <http://www.gnu.org/licenses/>.

use hex_literal::hex;
use itertools::Itertools;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
  traits::{AccountIdConversion, IdentifyAccount, Verify},
  FixedPointNumber, FixedU128, Perbill, Percent,
};
use strum::IntoEnumIterator;
// Tidechain primitives
use tidefi_primitives::{
  assets, AccountId, AssetId, Balance, Block, CurrencyId, OnboardingRebates, Signature,
  StakeCurrencyMeta, SunriseSwapPool,
};

#[cfg(feature = "tidechain-native")]
const TIDECHAIN_STAGING_TELEMETRY_URL: &str = "wss://telemetry.tidefi.io/submit/";

#[cfg(feature = "lagoon-native")]
const LAGOON_STAGING_TELEMETRY_URL: &str = "wss://telemetry.tidefi.io/submit/";

#[cfg(any(feature = "lagoon-native", feature = "tidechain-native"))]
const DEFAULT_PROTOCOL_ID: &str = "tidec0";

const STARTING_BLOCK: u32 = 0;

// We are approximating a month to 30 days
const ONE_MONTH: u32 = 432_000;
const SIX_MONTHS: u32 = 2_592_000;
const ONE_YEAR: u32 = 5_184_000;

const TOTAL_INITIAL_ACCOUNT_ALLOCATION: u128 = 147_248_200 * 1_000_000_000_000;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, sc_chain_spec::ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
  /// Block numbers with known hashes.
  pub fork_blocks: sc_client_api::ForkBlocks<Block>,
  /// Known bad block hashes.
  pub bad_blocks: sc_client_api::BadBlocks<Block>,
  /// Required for Tidechain and Lagoon Runtime, for future light-client implementation.
  pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

#[cfg(feature = "tidechain-native")]
pub type TidechainChainSpec =
  sc_service::GenericChainSpec<tidechain_runtime::GenesisConfig, Extensions>;

#[cfg(feature = "lagoon-native")]
pub type LagoonChainSpec = sc_service::GenericChainSpec<lagoon_runtime::GenesisConfig, Extensions>;

#[cfg(feature = "tidechain-native")]
pub fn tidechain_config() -> Result<TidechainChainSpec, String> {
  TidechainChainSpec::from_json_bytes(&include_bytes!("../res/tidechain.json")[..])
}

#[cfg(feature = "lagoon-native")]
pub fn lagoon_config() -> Result<LagoonChainSpec, String> {
  LagoonChainSpec::from_json_bytes(&include_bytes!("../res/lagoon.json")[..])
}

#[cfg(feature = "lagoon-native")]
fn lagoon_session_keys(
  grandpa: GrandpaId,
  babe: BabeId,
  im_online: ImOnlineId,
  authority_discovery: AuthorityDiscoveryId,
) -> lagoon_runtime::SessionKeys {
  lagoon_runtime::SessionKeys {
    grandpa,
    babe,
    im_online,
    authority_discovery,
  }
}

#[cfg(feature = "tidechain-native")]
fn tidechain_session_keys(
  grandpa: GrandpaId,
  babe: BabeId,
  im_online: ImOnlineId,
  authority_discovery: AuthorityDiscoveryId,
) -> tidechain_runtime::SessionKeys {
  tidechain_runtime::SessionKeys {
    grandpa,
    babe,
    im_online,
    authority_discovery,
  }
}

#[cfg(feature = "lagoon-native")]
fn lagoon_testnet_genesis(
  wasm_binary: &[u8],
  initial_authorities: Vec<(
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
  )>,
  stakeholders: Vec<(CurrencyId, AccountId, Balance)>,
  quorums: Vec<AccountId>,
  oracle: AccountId,
  root: AccountId,
  assets: Vec<(AssetId, Vec<u8>, Vec<u8>, u8)>,
) -> lagoon_runtime::GenesisConfig {
  // 10_500 TDFYs / validators (10_000 stashed)
  const ENDOWMENT: u128 = 10_500 * 1_000_000_000_000;
  const TOTAL_SUPPLY: u128 = 1_000_000_000 * 1_000_000_000_000;
  const STASH: u128 = 10_000 * 1_000_000_000_000;
  const ON_BOARDING_REBATES: u128 = 48_000_000 * 1_000_000_000_000;
  const TRADING_REBATES: u128 = 192_000_000 * 1_000_000_000_000;
  const SUNRISE_POOL: u128 = ON_BOARDING_REBATES + TRADING_REBATES;
  // Treasury Account Id
  let treasury_account: AccountId =
    lagoon_runtime::TreasuryPalletId::get().into_account_truncating();
  // Sunrise Account Id
  let sunrise_account: AccountId = lagoon_runtime::SunrisePalletId::get().into_account_truncating();
  // Get all TDFY from our stakeholders
  let mut claims = helpers::get_tide_from_stakeholders(stakeholders.clone());

  // default threshold set to 60%
  let quorum_threshold = (quorums.len() as f64 * 0.6).ceil() as u16;

  let mut total_claims: u128 = 0;
  for (_, balance) in &claims {
    total_claims += balance;
  }

  // Total funds in treasury
  let mut treasury_funds: u128 = TOTAL_SUPPLY;
  treasury_funds -=
  // remove the fund allocated to the validators and quorums
  helpers::adjust_treasury_balance_for_initial_validators_and_quorums(initial_authorities.len(), quorums.len(), ENDOWMENT)
  // all tokens claimed by the stake holders
  + total_claims
  // 10 TDFY endowed to root
  + 10_000_000_000_000
  // Sunrise pool
  + SUNRISE_POOL;

  // Each initial validator get an endowment of `ENDOWMENT` TDFY.
  let mut inital_validators_endowment = initial_authorities
    .iter()
    .map(|k| (k.0.clone(), ENDOWMENT))
    .collect_vec();

  // Each quorums get an endowment of `ENDOWMENT` TDFY.
  let mut inital_quorums_endowment = quorums.iter().map(|k| (k.clone(), ENDOWMENT)).collect_vec();

  let mut endowed_accounts = vec![
    // Treasury funds
    (treasury_account, treasury_funds),
    // Sunrise pool
    (sunrise_account, SUNRISE_POOL),
    // 10 TDFY to root so he can pay fees
    (root.clone(), 10_000_000_000_000),
  ];

  // Add all stake holders account
  endowed_accounts.append(&mut claims);

  // Endow to validators
  endowed_accounts.append(&mut inital_validators_endowment);

  // Endow to quorums
  endowed_accounts.append(&mut inital_quorums_endowment);

  let mut total_supply: u128 = 0;
  for (_, balance) in &endowed_accounts {
    total_supply += *balance
  }

  assert_eq!(
    total_supply, TOTAL_SUPPLY,
    "Total Supply (endowed_accounts) is not equal to 1 billion"
  );

  let vesting = helpers::get_vesting_terms_lagoon();

  let sunrise = crate::tidefi_sunrise_pool_genesis!(lagoon_runtime);
  let trading_rebates_total: Balance = sunrise
    .swap_pools
    .iter()
    .map(|swap_pool| swap_pool.balance)
    .sum();
  assert_eq!(
    trading_rebates_total, TRADING_REBATES,
    "Sunrise pool trading rebates total is not correct"
  );

  lagoon_runtime::GenesisConfig {
    system: lagoon_runtime::SystemConfig {
      code: wasm_binary.to_vec(),
    },
    balances: lagoon_runtime::BalancesConfig {
      balances: endowed_accounts.clone(),
    },

    indices: lagoon_runtime::IndicesConfig { indices: vec![] },
    session: lagoon_runtime::SessionConfig {
      keys: initial_authorities
        .iter()
        .map(|x| {
          (
            x.0.clone(),
            x.0.clone(),
            lagoon_session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
          )
        })
        .collect::<Vec<_>>(),
    },
    staking: lagoon_runtime::StakingConfig {
      minimum_validator_count: 1,
      validator_count: initial_authorities.len() as u32,
      invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
      stakers: initial_authorities
        .iter()
        .map(|x| {
          (
            x.0.clone(),
            x.1.clone(),
            STASH,
            lagoon_runtime::StakerStatus::Validator,
          )
        })
        .collect(),
      slash_reward_fraction: Perbill::from_percent(10),
      ..Default::default()
    },
    elections: Default::default(),
    democracy: Default::default(),
    council: lagoon_runtime::CouncilConfig {
      members: vec![],
      phantom: Default::default(),
    },
    technical_committee: lagoon_runtime::TechnicalCommitteeConfig {
      members: vec![],
      phantom: Default::default(),
    },

    sudo: lagoon_runtime::SudoConfig {
      key: Some(root.clone()),
    },

    babe: lagoon_runtime::BabeConfig {
      authorities: Default::default(),
      epoch_config: Some(lagoon_runtime::BABE_GENESIS_EPOCH_CONFIG),
    },
    im_online: Default::default(),
    authority_discovery: lagoon_runtime::AuthorityDiscoveryConfig { keys: vec![] },
    grandpa: Default::default(),
    technical_membership: Default::default(),
    treasury: Default::default(),
    // tidefi custom genesis
    quorum: lagoon_runtime::QuorumConfig {
      enabled: true,
      members: quorums,
      threshold: quorum_threshold,
    },
    oracle: lagoon_runtime::OracleConfig {
      enabled: true,
      account: oracle,
      market_makers: vec![
        //5CFsxqm4muZDTZA3vZVE8Pm9ny2XDrKvR8UAZuufxFLGoAwQ
        hex!["0885b880a6305cb19ea441fab8b5ed02cadef5cb5dafe9e9afd7c0be80046636"].into(),
        //5HVXyDbEY3Luroo4aiurP1xLZnKKAsXU4GRxVNBGmH2d2io5
        hex!["f01d04fcd4db7b552a14bec692f6fcb7a9fc4669972cdadc563f2bcb324c9741"].into(),
        //5CLdLM1HrtWdRvYBfH6dcEWQnRD6AeKZWGyuxE4shPBdY2r2
        hex!["0c24b38a7a768577d9e00b8d01f3412bf5121632c855dd4837abc7fe4afd4609"].into(),
        //5HgiTfx31XKS8F74LDVjiXG7VcJ69Q1sWFRjAgyJrK4yXFY1
        hex!["f8a4088e206592cb8eaa5bd73279b552f85a4b4da7761184076ee404df2c906c"].into(),
        //5GL5yZjsYNDLWY12CJt5Vm1jktLfaHTiHXHcZNmsxd13EXf9
        hex!["bcac12e15f80982de85d5667ddc1b6dd49bee80c4edfd371c5ba5d47023fa97b"].into(),
        //5EPTgRuaMcWTH88BfmZQKymiJ41eKJc9goQC7VeRGwGnMGbK
        hex!["66c6683ad9c6b1940d9d74691cdc0cfd4e760357d7427185e73f1c420d2ce464"].into(),
        //5GKxcqHFndxDH8qdpK6311Qco4MLJJZJeY8ZSFjjN6w31goH
        hex!["bc934e6e40cd8207bc9bc72fb8c1c2cb3266ef7caac69f6e18cb5792ab859f62"].into(),
        //5CXeo6fy34CuZgmbkSjy7vjqrv9DojqmQmqCrHwANxwPqC9Q
        hex!["148d51dee87e09b75f8487aaf72aecda9b107f577e184da1d065d14bf02bc542"].into(),
        //5FQyxubtnNEpjcTWWRygJStchrQoSKc9r6ohPUv93WPMechq
        hex!["942bd4d3c1de0dbd822551f572762e194e52664bb94686c96c0679a899147506"].into(),
        //5FKuzgFppRcJqs1bYQvrDJ9DrKZaXqrwKggWBk4DyfpXFvoo
        hex!["904e3dea6bcdc6cb523f52cbdedad53c24bbd95692ec690154b0f2c7f0abc55c"].into(),
      ],
    },
    asset_registry: lagoon_runtime::AssetRegistryConfig {
      // these assets are created on first initialization
      assets: helpers::get_assets_with_stakeholders(stakeholders, assets),
      // FIXME: Is the asset_registry owner should be the same account as root?
      // this is the owner of the wrapped asset on chain and have full authority on them
      // this account can also create new wrapped asset on chain
      account: root,
    },
    security: Default::default(),
    fees: Default::default(),
    vesting: lagoon_runtime::VestingConfig { vesting },
    sunrise,
    tidefi_staking: crate::tidefi_staking_genesis!(
      lagoon_runtime,
      vec![
        (150_u32, Percent::from_parts(1)),
        ((14400_u32 * 15_u32).into(), Percent::from_parts(2)),
        ((14400_u32 * 30_u32).into(), Percent::from_parts(3)),
        ((14400_u32 * 60_u32).into(), Percent::from_parts(4)),
        ((14400_u32 * 90_u32).into(), Percent::from_parts(5)),
      ]
    ),
  }
}

#[cfg(feature = "tidechain-native")]
fn tidechain_testnet_genesis(
  wasm_binary: &[u8],
  initial_authorities: Vec<(
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
  )>,
  stakeholders: Vec<(CurrencyId, AccountId, Balance)>,
  quorums: Vec<AccountId>,
  oracle: AccountId,
  root: AccountId,
  assets: Vec<(AssetId, Vec<u8>, Vec<u8>, u8)>,
) -> tidechain_runtime::GenesisConfig {
  // 10_500 TDFYs / validators (10_000 stashed)
  const ENDOWMENT: u128 = 10_500 * 1_000_000_000_000;
  const TOTAL_SUPPLY: u128 = 1_000_000_000 * 1_000_000_000_000;
  const STASH: u128 = 10_000 * 1_000_000_000_000;
  const ON_BOARDING_REBATES: u128 = 48_000_000 * 1_000_000_000_000;
  const TRADING_REBATES: u128 = 192_000_000 * 1_000_000_000_000;
  const SUNRISE_POOL: u128 = ON_BOARDING_REBATES + TRADING_REBATES;

  const VESTING_TOTAL_FOR_ONE_MONTH_PERIOD: u128 = 2_382_910 * 1_000_000_000_000;
  const VESTING_TOTAL_FOR_SIX_MONTHS_PERIOD: u128 = 2_382_910 * 1_000_000_000_000;
  const VESTING_TOTAL_FOR_THREE_YEARS_PERIOD: u128 = 102_892_380 * 1_000_000_000_000;

  // default threshold set to 60%
  let quorum_threshold = (quorums.len() as f64 * 0.6).ceil() as u16;

  // Treasury Account Id
  let treasury_account: AccountId =
    tidechain_runtime::TreasuryPalletId::get().into_account_truncating();
  // Fees Account Id
  let sunrise_account: AccountId =
    tidechain_runtime::SunrisePalletId::get().into_account_truncating();
  // Asset registry Account Id
  let asset_registry: AccountId =
    tidechain_runtime::AssetRegistryPalletId::get().into_account_truncating();
  // Get all TDFY from our stakeholders
  let mut claims = helpers::get_tide_from_stakeholders(stakeholders.clone());

  let mut total_claims: u128 = 0;
  for (_, balance) in &claims {
    total_claims += balance;
  }

  // Total funds in treasury
  let mut treasury_funds: u128 = TOTAL_SUPPLY;
  treasury_funds -=
  // remove the fund allocated to the validators and quorums
  helpers::adjust_treasury_balance_for_initial_validators_and_quorums(initial_authorities.len(), quorums.len(), ENDOWMENT)
  // all tokens claimed by the stake holders
  + total_claims
  // root
  + 10_000_000_000_000
  // Sunrise pool
  + SUNRISE_POOL;

  // Each initial validator get an endowment of `ENDOWMENT` TDFY.
  let mut inital_validators_endowment = initial_authorities
    .iter()
    .map(|k| (k.0.clone(), ENDOWMENT))
    .collect_vec();

  // Each quorums get an endowment of `ENDOWMENT` TDFY.
  let mut inital_quorums_endowment = quorums.iter().map(|k| (k.clone(), ENDOWMENT)).collect_vec();

  let mut endowed_accounts = vec![
    // Treasury funds
    (treasury_account, treasury_funds),
    // Sunrise pool
    (sunrise_account, SUNRISE_POOL),
    // 10 TDFY to root so he can pay fees
    (root.clone(), 10_000_000_000_000),
  ];

  // Add all stake holders account
  endowed_accounts.append(&mut claims);

  // Endow to validators
  endowed_accounts.append(&mut inital_validators_endowment);

  // Endow to quorums
  endowed_accounts.append(&mut inital_quorums_endowment);

  let mut total_supply: u128 = 0;
  for (_, balance) in &endowed_accounts {
    total_supply += *balance
  }

  assert_eq!(
    total_supply, TOTAL_SUPPLY,
    "Total Supply (endowed_accounts) is not equal to 1 billion"
  );

  let vesting = helpers::get_vesting_terms_tidechain();

  let one_month_vesting_total: Balance = vesting
    .iter()
    .filter(|(_, starting_block, period, period_count, _)| {
      *starting_block == STARTING_BLOCK && *period == ONE_MONTH && *period_count == 1
    })
    .map(|(_, _, _, _, per_period)| per_period)
    .sum();
  let six_months_vesting_total: Balance = vesting
    .iter()
    .filter(|(_, starting_block, period, period_count, _)| {
      *starting_block == STARTING_BLOCK && *period == SIX_MONTHS && *period_count == 1
    })
    .map(|(_, _, _, _, per_period)| per_period)
    .sum();
  let three_years_vesting_total: Balance = vesting
    .iter()
    .filter(|(_, starting_block, period, period_count, _)| {
      *starting_block == STARTING_BLOCK && *period == ONE_YEAR && *period_count == 3
    })
    .map(|(_, _, _, period_count, per_period)| *per_period * Balance::from(*period_count))
    .sum();

  assert_eq!(
    one_month_vesting_total, VESTING_TOTAL_FOR_ONE_MONTH_PERIOD,
    "Total vesting at the end of the first month is not correct"
  );
  assert_eq!(
    six_months_vesting_total, VESTING_TOTAL_FOR_SIX_MONTHS_PERIOD,
    "Total vesting at the end of the first six months is not correct"
  );
  assert_eq!(
    three_years_vesting_total, VESTING_TOTAL_FOR_THREE_YEARS_PERIOD,
    "Total vesting at the end of the three years is not correct"
  );

  let sunrise = crate::tidefi_sunrise_pool_genesis!(tidechain_runtime);
  let trading_rebates_total: Balance = sunrise
    .swap_pools
    .iter()
    .map(|swap_pool| swap_pool.balance)
    .sum();
  assert_eq!(
    trading_rebates_total, TRADING_REBATES,
    "Sunrise pool trading rebates total is not correct"
  );

  tidechain_runtime::GenesisConfig {
    system: tidechain_runtime::SystemConfig {
      code: wasm_binary.to_vec(),
    },
    balances: tidechain_runtime::BalancesConfig {
      balances: endowed_accounts.clone(),
    },

    indices: tidechain_runtime::IndicesConfig { indices: vec![] },
    session: tidechain_runtime::SessionConfig {
      keys: initial_authorities
        .iter()
        .map(|x| {
          (
            x.0.clone(),
            x.0.clone(),
            tidechain_session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
          )
        })
        .collect::<Vec<_>>(),
    },
    staking: tidechain_runtime::StakingConfig {
      minimum_validator_count: 1,
      validator_count: initial_authorities.len() as u32,
      invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
      stakers: initial_authorities
        .iter()
        .map(|x| {
          (
            x.0.clone(),
            x.1.clone(),
            STASH,
            tidechain_runtime::StakerStatus::Validator,
          )
        })
        .collect(),
      slash_reward_fraction: Perbill::from_percent(10),
      ..Default::default()
    },
    elections: Default::default(),
    democracy: Default::default(),

    council: tidechain_runtime::CouncilConfig {
      members: vec![],
      phantom: Default::default(),
    },

    technical_committee: tidechain_runtime::TechnicalCommitteeConfig {
      members: vec![],
      phantom: Default::default(),
    },

    // FIXME: Remove sudo once the staging is completed
    sudo: tidechain_runtime::SudoConfig {
      key: Some(root.clone()),
    },

    babe: tidechain_runtime::BabeConfig {
      authorities: Default::default(),
      epoch_config: Some(tidechain_runtime::BABE_GENESIS_EPOCH_CONFIG),
    },
    im_online: Default::default(),
    authority_discovery: tidechain_runtime::AuthorityDiscoveryConfig { keys: vec![] },
    grandpa: Default::default(),
    technical_membership: Default::default(),
    treasury: Default::default(),
    // tidefi custom genesis
    quorum: tidechain_runtime::QuorumConfig {
      enabled: true,
      members: quorums,
      threshold: quorum_threshold,
    },
    oracle: tidechain_runtime::OracleConfig {
      enabled: true,
      account: oracle,
      market_makers: Vec::new(),
    },
    asset_registry: tidechain_runtime::AssetRegistryConfig {
      // these assets are created on first initialization
      assets: helpers::get_assets_with_stakeholders(stakeholders, assets),
      // FIXME: Not sure if the owner should be the asset registry pallet itself?
      account: asset_registry,
    },
    security: Default::default(),
    fees: Default::default(),
    vesting: tidechain_runtime::VestingConfig { vesting },
    sunrise,
    tidefi_staking: crate::tidefi_staking_genesis!(
      tidechain_runtime,
      vec![
        ((14400_u32 * 15_u32).into(), Percent::from_parts(2)),
        ((14400_u32 * 30_u32).into(), Percent::from_parts(3)),
        ((14400_u32 * 60_u32).into(), Percent::from_parts(4)),
        ((14400_u32 * 90_u32).into(), Percent::from_parts(5)),
      ]
    ),
  }
}

/// Development config (single validator Alice)
#[cfg(feature = "lagoon-native")]
pub fn lagoon_development_config() -> Result<LagoonChainSpec, String> {
  let properties = Some(
    json!({
      "tokenDecimals": 12,
      "tokenSymbol": "TDFY"
    })
    .as_object()
    .expect("Map given; qed")
    .clone(),
  );

  let wasm_binary = lagoon_runtime::WASM_BINARY.ok_or("Lagoon development wasm not available")?;

  Ok(LagoonChainSpec::from_genesis(
    "Development",
    "lagoon_dev",
    ChainType::Development,
    move || lagoon_development_config_genesis(wasm_binary),
    vec![],
    None,
    Some(DEFAULT_PROTOCOL_ID),
    None,
    properties,
    Default::default(),
  ))
}

/// Lagoon local testnet config.
#[cfg(feature = "lagoon-native")]
pub fn lagoon_local_testnet_config() -> Result<LagoonChainSpec, String> {
  let properties = Some(
    json!({
      "tokenDecimals": 12,
      "tokenSymbol": "TDFY"
    })
    .as_object()
    .expect("Map given; qed")
    .clone(),
  );

  let wasm_binary = lagoon_runtime::WASM_BINARY.ok_or("Lagoon development wasm not available")?;

  Ok(LagoonChainSpec::from_genesis(
    "Lagoon Local Testnet",
    "lagoon_local_testnet",
    ChainType::Local,
    move || lagoon_local_testnet_config_genesis(wasm_binary),
    vec![],
    None,
    Some(DEFAULT_PROTOCOL_ID),
    None,
    properties,
    Default::default(),
  ))
}

/// Lagoon staging testnet config.
#[cfg(feature = "lagoon-native")]
pub fn lagoon_staging_testnet_config() -> Result<LagoonChainSpec, String> {
  let properties = Some(
    json!({
      "tokenDecimals": 12,
      "tokenSymbol": "TDFY"
    })
    .as_object()
    .expect("Map given; qed")
    .clone(),
  );

  let wasm_binary = lagoon_runtime::WASM_BINARY.ok_or("Lagoon development wasm not available")?;

  Ok(LagoonChainSpec::from_genesis(
    "Lagoon Staging Testnet",
    "lagoon_staging_testnet",
    ChainType::Live,
    move || lagoon_staging_testnet_config_genesis(wasm_binary),
    vec![],
    Some(
      TelemetryEndpoints::new(vec![(LAGOON_STAGING_TELEMETRY_URL.to_string(), 0)])
        .expect("Discovery Staging telemetry url is valid; qed"),
    ),
    Some(DEFAULT_PROTOCOL_ID),
    None,
    properties,
    Default::default(),
  ))
}

/// Development config (single validator Alice)
#[cfg(feature = "tidechain-native")]
pub fn tidechain_development_config() -> Result<TidechainChainSpec, String> {
  let properties = Some(
    json!({
      "tokenDecimals": 12,
      "tokenSymbol": "TDFY"
    })
    .as_object()
    .expect("Map given; qed")
    .clone(),
  );

  let wasm_binary =
    tidechain_runtime::WASM_BINARY.ok_or("Tidechain development wasm not available")?;

  Ok(TidechainChainSpec::from_genesis(
    "Development",
    "tidechain_dev",
    ChainType::Development,
    move || tidechain_development_config_genesis(wasm_binary),
    vec![],
    None,
    Some(DEFAULT_PROTOCOL_ID),
    None,
    properties,
    Default::default(),
  ))
}

/// Tidechain local testnet config.
#[cfg(feature = "tidechain-native")]
pub fn tidechain_staging_testnet_config() -> Result<TidechainChainSpec, String> {
  let properties = Some(
    json!({
      "tokenDecimals": 12,
      "tokenSymbol": "TDFY"
    })
    .as_object()
    .expect("Map given; qed")
    .clone(),
  );

  let wasm_binary =
    tidechain_runtime::WASM_BINARY.ok_or("Tidechain development wasm not available")?;

  let boot_nodes = vec![];

  Ok(TidechainChainSpec::from_genesis(
    "Tidechain Staging Testnet",
    "tidechain_staging_testnet",
    ChainType::Live,
    move || tidechain_staging_testnet_config_genesis(wasm_binary),
    boot_nodes,
    Some(
      TelemetryEndpoints::new(vec![(TIDECHAIN_STAGING_TELEMETRY_URL.to_string(), 0)])
        .expect("Tidechain Staging telemetry url is valid; qed"),
    ),
    Some(DEFAULT_PROTOCOL_ID),
    None,
    properties,
    Default::default(),
  ))
}

/// Tidechain staging testnet config.
#[cfg(feature = "tidechain-native")]
pub fn tidechain_local_testnet_config() -> Result<TidechainChainSpec, String> {
  let properties = Some(
    json!({
      "tokenDecimals": 12,
      "tokenSymbol": "TDFY"
    })
    .as_object()
    .expect("Map given; qed")
    .clone(),
  );

  let wasm_binary =
    tidechain_runtime::WASM_BINARY.ok_or("Tidechain development wasm not available")?;

  let boot_nodes = vec![];

  Ok(TidechainChainSpec::from_genesis(
    "Local Testnet",
    "tidechain_local_testnet",
    ChainType::Local,
    move || tidechain_local_testnet_config_genesis(wasm_binary),
    boot_nodes,
    None,
    Some(DEFAULT_PROTOCOL_ID),
    None,
    properties,
    Default::default(),
  ))
}

#[cfg(feature = "lagoon-native")]
fn lagoon_development_config_genesis(wasm_binary: &[u8]) -> lagoon_runtime::GenesisConfig {
  lagoon_testnet_genesis(
    wasm_binary,
    vec![helpers::authority_keys_from_seed("Alice")],
    helpers::get_stakeholder_tokens_lagoon(),
    vec![helpers::get_account_id_from_seed::<sr25519::Public>(
      "Charlie",
    )],
    helpers::get_account_id_from_seed::<sr25519::Public>("Ferdie"),
    helpers::get_account_id_from_seed::<sr25519::Public>("Ferdie"),
    helpers::get_all_assets(),
  )
}

#[cfg(feature = "lagoon-native")]
fn lagoon_local_testnet_config_genesis(wasm_binary: &[u8]) -> lagoon_runtime::GenesisConfig {
  lagoon_testnet_genesis(
    wasm_binary,
    vec![
      helpers::authority_keys_from_seed("Alice"),
      helpers::authority_keys_from_seed("Bob"),
    ],
    helpers::get_stakeholder_tokens_lagoon(),
    vec![
      helpers::get_account_id_from_seed::<sr25519::Public>("Charlie"),
      helpers::get_account_id_from_seed::<sr25519::Public>("Dave"),
      helpers::get_account_id_from_seed::<sr25519::Public>("Eve"),
    ],
    helpers::get_account_id_from_seed::<sr25519::Public>("Ferdie"),
    helpers::get_account_id_from_seed::<sr25519::Public>("Ferdie"),
    helpers::get_all_assets(),
  )
}

#[cfg(feature = "lagoon-native")]
fn lagoon_staging_testnet_config_genesis(wasm_binary: &[u8]) -> lagoon_runtime::GenesisConfig {
  let initial_authorities: Vec<(
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
  )> = vec![
    (
      //5EPH4xUAqCbTPw3kfjqewELdrCEQiNp84vuhiVg8hU1A5wX7
      hex!["66a2af6ffba20fa9b410b0cb7e2eb1d99fb3748066210e9e806ba0e824b8de70"].into(),
      //5DF43vX2vamASYjah4u9gNDF95oq3Yb2NPPseua3cfFfTtta
      hex!["3420aaae4b1e1d2ad0abc953083681b9552aec63231d5f8da73d92954a086d2e"].into(),
      //5FN2w1sbZtiJSm4eK6sKfyDSz42uo4g7vN61PhxWBVnCN3KA
      hex!["91ec1590b039fff51d37525cdf599dad78e73349365085f8219bcac459f371da"].unchecked_into(),
      //5H1PrCT66bGdQDzG6CB73ryrxmuCidWBExpeUXGvZ4jzMZ5R
      hex!["daa6cf7287fe8e3bc7feab159409fef2aff2297f64086b0e8fe5a122642b1311"].unchecked_into(),
      //5HTEc4VSMRDcatHFBgrVRZURAJZykSXJ149C7GGBCRHVWjxo
      hex!["ee5c12d9accd02f7b9c41076205b5536e145d87fc4d8e9bb3e59ac5ebf923727"].unchecked_into(),
      //5DJF1CUe6zM3Ux6UvJm895K43ksaomwarhjuX3mVeLm319ci
      hex!["368f462a8a00121449647ad3f7224e61dd4a7a93d678ab739c91910c2f8e2d56"].unchecked_into(),
    ),
    (
      //5Gn8KDvn6ZfcqG4s5WBLbH4bARS77nFrA738tHaNrGkESUb9
      hex!["d08853ecd641605a685c827ead7521b178725a06d80c1ba6131a6fc87b6b9b13"].into(),
      //5Dcq84dRFew2R11X6vXK4Zn6VPbtxzkTtvbpXMMjTmZpyh4k
      hex!["44bc8b430298b88630d178b3d2237879a3202ebaaa65966b3f63a57d5c9f9b24"].into(),
      //5DrLpQS5hd7yLBAcn1pmMHUAQY4m1GAPQh7qs1rrnupac3WQ
      hex!["4f0ab2c75b9b2a2ccdccb1ba566149f06afb7e19bd58af68959f8162452d7385"].unchecked_into(),
      //5FWgrDY4HbGFgnERStbfPLxi12vAmTXbo8aLYjLpnwx2mnSN
      hex!["9885b037ad8e0df9076f69d2a3d501f0c2d0e4a2ed31b65c6a7d437679ec3658"].unchecked_into(),
      //5FHfAgjpQ6pjDG56JCC2sJ58JqGpo23d8CPUVR1dECpyZFwf
      hex!["8e95d661acebbc44fa4fa7824b98ba5cd34b9f4b9f421d1e4e7f248b5e87cc10"].unchecked_into(),
      //5GYitpB3bB4FTTdxN2mBZwxaRa4Na3zAAUqND1nD3x1PxLES
      hex!["c64f4bc3852a7122902c5cacd7777fe1b04b6e16131026788e6ea23f18912c2a"].unchecked_into(),
    ),
    (
      //5Ge8JkHNACxSVR9vNpirDmLNHBVCPqhybEe61BSKsgimEEgr
      hex!["ca6e56414a58383dbc87c281f7e320f3ebdf59233b9544c898294701e9b13f77"].into(),
      //5GxQPxE4D5YrLteyLmVqhtYwpHR5gCx8XHhrrqC7qUB7U53r
      hex!["d85eeacb2f51ac029a9ebcd14e218f869d19ba83f9dac23e8064d809d49f2e20"].into(),
      //5HiusCdwsa28RgCs15ouuJvjPseTyk8RyKyJyhDjhQy8ekNa
      hex!["fa50eb4d60acc5a1a0537b21ae93117063a7148f172265d70f0137caf129aef0"].unchecked_into(),
      //5DvyAENVNcHm5wjqmPVpCPSkdKfGTT8zk8DnxU88KTt3fbsk
      hex!["5292047acf6707b86e8f2c80ec1efa8c40e4a11c01c43cebaf29ab8cf6b6d271"].unchecked_into(),
      //5DPJ9G65fDeoaTFZqwpE8r2X27zHvQ6mNqASkvhSsRDAmywc
      hex!["3a6a116479e5f6c1b6aa4b6e25eff90d30d6d528a9c29bbd72e353930604ae7e"].unchecked_into(),
      //5F4dorWaEpsESMAR1yZijhvQkV5iD1FtBUMcBEBZESnc7mPK
      hex!["84a7126293f444a53e68567d8f259aae4895e1d8991a929900c4866eeb8f8966"].unchecked_into(),
    ),
  ];

  // quorums
  let quorums: Vec<AccountId> = vec![
    //5EFKNPG2kPsyeVK8E5e7i5uiRfYdbQkq8qfhVxeVV42tZfPe
    hex!["60907755938c5ee6561ee929a766cb42cfbce19b19619c3b89adc30cf9cd970b"].into(),
    //5HVb1QTxnzHXpTPLCVT61Ag3Mb4fmyMYAy3kxbXYXMS9KjM6
    hex!["f0273ecee5c89e91c9baee61755498a40885133d0f5ee7ee4b4f035aa1551e53"].into(),
    //5EA2mLbbbdq6cyqDwZuHEGvKPPBVWDNuCS3DwtaetAum9aSe
    hex!["5c88582258ab5c02f342cd3ff37601252953cad2fb04de192cab2e2656788a6e"].into(),
  ];

  lagoon_testnet_genesis(
    wasm_binary,
    initial_authorities,
    helpers::get_stakeholder_tokens_lagoon_staging(),
    quorums,
    //5HKDZMoz5NnX37Np8dMKMAANbNu9N1XuQec15b3tZ8NaBTAR
    hex!["e83e965a0e2c599751184bcea1507d9fe37510d9d75eb37cba3ad8c1a5a1fe12"].into(),
    //5Hp9T9DoHRmLXsZ6j85R7xxqmUxCZ7MS4pfi4C6W6og484G6
    hex!["fe4ee0c4bae7d8a058b478c48bbaeab5e9b9d6adccacc49a45796dfb02bd9908"].into(),
    helpers::get_all_assets(),
  )
}

#[cfg(feature = "tidechain-native")]
fn tidechain_development_config_genesis(wasm_binary: &[u8]) -> tidechain_runtime::GenesisConfig {
  tidechain_testnet_genesis(
    wasm_binary,
    vec![helpers::authority_keys_from_seed("Alice")],
    helpers::get_stakeholder_tokens_tidechain(),
    vec![helpers::get_account_id_from_seed::<sr25519::Public>("Bob")],
    helpers::get_account_id_from_seed::<sr25519::Public>("Charlie"),
    helpers::get_account_id_from_seed::<sr25519::Public>("Ferdie"),
    helpers::get_all_assets(),
  )
}

#[cfg(feature = "tidechain-native")]
fn tidechain_staging_testnet_config_genesis(
  wasm_binary: &[u8],
) -> tidechain_runtime::GenesisConfig {
  let initial_authorities: Vec<(
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
  )> = vec![
    (
      //5EPH4xUAqCbTPw3kfjqewELdrCEQiNp84vuhiVg8hU1A5wX7
      hex!["66a2af6ffba20fa9b410b0cb7e2eb1d99fb3748066210e9e806ba0e824b8de70"].into(),
      //5DF43vX2vamASYjah4u9gNDF95oq3Yb2NPPseua3cfFfTtta
      hex!["3420aaae4b1e1d2ad0abc953083681b9552aec63231d5f8da73d92954a086d2e"].into(),
      //5FN2w1sbZtiJSm4eK6sKfyDSz42uo4g7vN61PhxWBVnCN3KA
      hex!["91ec1590b039fff51d37525cdf599dad78e73349365085f8219bcac459f371da"].unchecked_into(),
      //5H1PrCT66bGdQDzG6CB73ryrxmuCidWBExpeUXGvZ4jzMZ5R
      hex!["daa6cf7287fe8e3bc7feab159409fef2aff2297f64086b0e8fe5a122642b1311"].unchecked_into(),
      //5HTEc4VSMRDcatHFBgrVRZURAJZykSXJ149C7GGBCRHVWjxo
      hex!["ee5c12d9accd02f7b9c41076205b5536e145d87fc4d8e9bb3e59ac5ebf923727"].unchecked_into(),
      //5DJF1CUe6zM3Ux6UvJm895K43ksaomwarhjuX3mVeLm319ci
      hex!["368f462a8a00121449647ad3f7224e61dd4a7a93d678ab739c91910c2f8e2d56"].unchecked_into(),
    ),
    (
      //5Gn8KDvn6ZfcqG4s5WBLbH4bARS77nFrA738tHaNrGkESUb9
      hex!["d08853ecd641605a685c827ead7521b178725a06d80c1ba6131a6fc87b6b9b13"].into(),
      //5Dcq84dRFew2R11X6vXK4Zn6VPbtxzkTtvbpXMMjTmZpyh4k
      hex!["44bc8b430298b88630d178b3d2237879a3202ebaaa65966b3f63a57d5c9f9b24"].into(),
      //5DrLpQS5hd7yLBAcn1pmMHUAQY4m1GAPQh7qs1rrnupac3WQ
      hex!["4f0ab2c75b9b2a2ccdccb1ba566149f06afb7e19bd58af68959f8162452d7385"].unchecked_into(),
      //5FWgrDY4HbGFgnERStbfPLxi12vAmTXbo8aLYjLpnwx2mnSN
      hex!["9885b037ad8e0df9076f69d2a3d501f0c2d0e4a2ed31b65c6a7d437679ec3658"].unchecked_into(),
      //5FHfAgjpQ6pjDG56JCC2sJ58JqGpo23d8CPUVR1dECpyZFwf
      hex!["8e95d661acebbc44fa4fa7824b98ba5cd34b9f4b9f421d1e4e7f248b5e87cc10"].unchecked_into(),
      //5GYitpB3bB4FTTdxN2mBZwxaRa4Na3zAAUqND1nD3x1PxLES
      hex!["c64f4bc3852a7122902c5cacd7777fe1b04b6e16131026788e6ea23f18912c2a"].unchecked_into(),
    ),
    (
      //5Ge8JkHNACxSVR9vNpirDmLNHBVCPqhybEe61BSKsgimEEgr
      hex!["ca6e56414a58383dbc87c281f7e320f3ebdf59233b9544c898294701e9b13f77"].into(),
      //5GxQPxE4D5YrLteyLmVqhtYwpHR5gCx8XHhrrqC7qUB7U53r
      hex!["d85eeacb2f51ac029a9ebcd14e218f869d19ba83f9dac23e8064d809d49f2e20"].into(),
      //5HiusCdwsa28RgCs15ouuJvjPseTyk8RyKyJyhDjhQy8ekNa
      hex!["fa50eb4d60acc5a1a0537b21ae93117063a7148f172265d70f0137caf129aef0"].unchecked_into(),
      //5DvyAENVNcHm5wjqmPVpCPSkdKfGTT8zk8DnxU88KTt3fbsk
      hex!["5292047acf6707b86e8f2c80ec1efa8c40e4a11c01c43cebaf29ab8cf6b6d271"].unchecked_into(),
      //5DPJ9G65fDeoaTFZqwpE8r2X27zHvQ6mNqASkvhSsRDAmywc
      hex!["3a6a116479e5f6c1b6aa4b6e25eff90d30d6d528a9c29bbd72e353930604ae7e"].unchecked_into(),
      //5F4dorWaEpsESMAR1yZijhvQkV5iD1FtBUMcBEBZESnc7mPK
      hex!["84a7126293f444a53e68567d8f259aae4895e1d8991a929900c4866eeb8f8966"].unchecked_into(),
    ),
  ];

  // quorums
  let quorums: Vec<AccountId> = vec![
    //5EFKNPG2kPsyeVK8E5e7i5uiRfYdbQkq8qfhVxeVV42tZfPe
    hex!["60907755938c5ee6561ee929a766cb42cfbce19b19619c3b89adc30cf9cd970b"].into(),
    //5HVb1QTxnzHXpTPLCVT61Ag3Mb4fmyMYAy3kxbXYXMS9KjM6
    hex!["f0273ecee5c89e91c9baee61755498a40885133d0f5ee7ee4b4f035aa1551e53"].into(),
    //5EA2mLbbbdq6cyqDwZuHEGvKPPBVWDNuCS3DwtaetAum9aSe
    hex!["5c88582258ab5c02f342cd3ff37601252953cad2fb04de192cab2e2656788a6e"].into(),
  ];

  tidechain_testnet_genesis(
    wasm_binary,
    initial_authorities,
    helpers::get_stakeholder_tokens_tidechain(),
    quorums,
    //5HKDZMoz5NnX37Np8dMKMAANbNu9N1XuQec15b3tZ8NaBTAR
    hex!["e83e965a0e2c599751184bcea1507d9fe37510d9d75eb37cba3ad8c1a5a1fe12"].into(),
    //fhAwZQ4RZm2rWe6mWyAGwWRASr8t5a4rMVuctVPprubZC6TM6
    hex!["36cac8f87497cbe1eb00b78c923515ae03aabe8e50c1fa1086a884589b25f23f"].into(),
    helpers::get_all_assets(),
  )
}

#[cfg(feature = "tidechain-native")]
fn tidechain_local_testnet_config_genesis(wasm_binary: &[u8]) -> tidechain_runtime::GenesisConfig {
  let initial_authorities: Vec<(
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
  )> = vec![
    (
      //5EPH4xUAqCbTPw3kfjqewELdrCEQiNp84vuhiVg8hU1A5wX7
      hex!["66a2af6ffba20fa9b410b0cb7e2eb1d99fb3748066210e9e806ba0e824b8de70"].into(),
      //5DF43vX2vamASYjah4u9gNDF95oq3Yb2NPPseua3cfFfTtta
      hex!["3420aaae4b1e1d2ad0abc953083681b9552aec63231d5f8da73d92954a086d2e"].into(),
      //5FN2w1sbZtiJSm4eK6sKfyDSz42uo4g7vN61PhxWBVnCN3KA
      hex!["91ec1590b039fff51d37525cdf599dad78e73349365085f8219bcac459f371da"].unchecked_into(),
      //5H1PrCT66bGdQDzG6CB73ryrxmuCidWBExpeUXGvZ4jzMZ5R
      hex!["daa6cf7287fe8e3bc7feab159409fef2aff2297f64086b0e8fe5a122642b1311"].unchecked_into(),
      //5HTEc4VSMRDcatHFBgrVRZURAJZykSXJ149C7GGBCRHVWjxo
      hex!["ee5c12d9accd02f7b9c41076205b5536e145d87fc4d8e9bb3e59ac5ebf923727"].unchecked_into(),
      //5DJF1CUe6zM3Ux6UvJm895K43ksaomwarhjuX3mVeLm319ci
      hex!["368f462a8a00121449647ad3f7224e61dd4a7a93d678ab739c91910c2f8e2d56"].unchecked_into(),
    ),
    (
      //5Gn8KDvn6ZfcqG4s5WBLbH4bARS77nFrA738tHaNrGkESUb9
      hex!["d08853ecd641605a685c827ead7521b178725a06d80c1ba6131a6fc87b6b9b13"].into(),
      //5Dcq84dRFew2R11X6vXK4Zn6VPbtxzkTtvbpXMMjTmZpyh4k
      hex!["44bc8b430298b88630d178b3d2237879a3202ebaaa65966b3f63a57d5c9f9b24"].into(),
      //5DrLpQS5hd7yLBAcn1pmMHUAQY4m1GAPQh7qs1rrnupac3WQ
      hex!["4f0ab2c75b9b2a2ccdccb1ba566149f06afb7e19bd58af68959f8162452d7385"].unchecked_into(),
      //5FWgrDY4HbGFgnERStbfPLxi12vAmTXbo8aLYjLpnwx2mnSN
      hex!["9885b037ad8e0df9076f69d2a3d501f0c2d0e4a2ed31b65c6a7d437679ec3658"].unchecked_into(),
      //5FHfAgjpQ6pjDG56JCC2sJ58JqGpo23d8CPUVR1dECpyZFwf
      hex!["8e95d661acebbc44fa4fa7824b98ba5cd34b9f4b9f421d1e4e7f248b5e87cc10"].unchecked_into(),
      //5GYitpB3bB4FTTdxN2mBZwxaRa4Na3zAAUqND1nD3x1PxLES
      hex!["c64f4bc3852a7122902c5cacd7777fe1b04b6e16131026788e6ea23f18912c2a"].unchecked_into(),
    ),
    (
      //5Ge8JkHNACxSVR9vNpirDmLNHBVCPqhybEe61BSKsgimEEgr
      hex!["ca6e56414a58383dbc87c281f7e320f3ebdf59233b9544c898294701e9b13f77"].into(),
      //5GxQPxE4D5YrLteyLmVqhtYwpHR5gCx8XHhrrqC7qUB7U53r
      hex!["d85eeacb2f51ac029a9ebcd14e218f869d19ba83f9dac23e8064d809d49f2e20"].into(),
      //5HiusCdwsa28RgCs15ouuJvjPseTyk8RyKyJyhDjhQy8ekNa
      hex!["fa50eb4d60acc5a1a0537b21ae93117063a7148f172265d70f0137caf129aef0"].unchecked_into(),
      //5DvyAENVNcHm5wjqmPVpCPSkdKfGTT8zk8DnxU88KTt3fbsk
      hex!["5292047acf6707b86e8f2c80ec1efa8c40e4a11c01c43cebaf29ab8cf6b6d271"].unchecked_into(),
      //5DPJ9G65fDeoaTFZqwpE8r2X27zHvQ6mNqASkvhSsRDAmywc
      hex!["3a6a116479e5f6c1b6aa4b6e25eff90d30d6d528a9c29bbd72e353930604ae7e"].unchecked_into(),
      //5F4dorWaEpsESMAR1yZijhvQkV5iD1FtBUMcBEBZESnc7mPK
      hex!["84a7126293f444a53e68567d8f259aae4895e1d8991a929900c4866eeb8f8966"].unchecked_into(),
    ),
  ];

  // quorums
  let quorums: Vec<AccountId> = vec![
    //5EFKNPG2kPsyeVK8E5e7i5uiRfYdbQkq8qfhVxeVV42tZfPe
    hex!["60907755938c5ee6561ee929a766cb42cfbce19b19619c3b89adc30cf9cd970b"].into(),
    //5HVb1QTxnzHXpTPLCVT61Ag3Mb4fmyMYAy3kxbXYXMS9KjM6
    hex!["f0273ecee5c89e91c9baee61755498a40885133d0f5ee7ee4b4f035aa1551e53"].into(),
    //5EA2mLbbbdq6cyqDwZuHEGvKPPBVWDNuCS3DwtaetAum9aSe
    hex!["5c88582258ab5c02f342cd3ff37601252953cad2fb04de192cab2e2656788a6e"].into(),
  ];

  tidechain_testnet_genesis(
    wasm_binary,
    initial_authorities,
    helpers::get_stakeholder_tokens_tidechain(),
    quorums,
    //5HKDZMoz5NnX37Np8dMKMAANbNu9N1XuQec15b3tZ8NaBTAR
    hex!["e83e965a0e2c599751184bcea1507d9fe37510d9d75eb37cba3ad8c1a5a1fe12"].into(),
    //fhAwZQ4RZm2rWe6mWyAGwWRASr8t5a4rMVuctVPprubZC6TM6
    hex!["36cac8f87497cbe1eb00b78c923515ae03aabe8e50c1fa1086a884589b25f23f"].into(),
    helpers::get_all_assets(),
  )
}

// helpers for our genesis configuration
mod helpers {
  use super::*;
  type AccountPublic = <Signature as Verify>::Signer;

  // syntactic sugar for tidefi staking genesis config.
  #[macro_export]
  macro_rules! tidefi_staking_genesis {
    ($runtime:tt, $staking_periods:expr) => {
      $runtime::TidefiStakingConfig {
        staking_periods: $staking_periods,
        staking_meta: assets::Asset::iter()
          .map(|asset| {
            (
              asset.currency_id(),
              StakeCurrencyMeta {
                minimum_amount: asset.default_minimum_stake_amount(),
                maximum_amount: asset.default_maximum_stake_amount(),
              },
            )
          })
          .collect(),
        unstake_fee: Percent::from_parts(1),
      }
    };
  }

  // syntactic sugar for sunrise pool genesis config.
  #[macro_export]
  macro_rules! tidefi_sunrise_pool_genesis {
    ($runtime:tt) => {
      $runtime::SunriseConfig {
        phantom: Default::default(),
        onboarding_rebates: Some(OnboardingRebates {
          initial_amount: assets::Asset::Tdfy.saturating_mul(48_000_000),
          available_amount: assets::Asset::Tdfy.saturating_mul(48_000_000),
        }),
        swap_pools: vec![
          SunriseSwapPool {
            id: 1,
            minimum_tdfy_value: 0,
            transactions_remaining: 100_000,
            balance: assets::Asset::Tdfy.saturating_mul(67_200_000),
            // 100%
            rebates: FixedU128::saturating_from_rational(100_u32, 100_u32),
          },
          SunriseSwapPool {
            id: 2,
            // 100 TDFY's minimum value
            minimum_tdfy_value: assets::Asset::Tdfy.saturating_mul(100),
            transactions_remaining: 100_000,
            balance: assets::Asset::Tdfy.saturating_mul(57_600_000),
            // 125%
            rebates: FixedU128::saturating_from_rational(125_u32, 100_u32),
          },
          SunriseSwapPool {
            id: 3,
            // 10_000 TDFY's minimum value
            minimum_tdfy_value: assets::Asset::Tdfy.saturating_mul(10_000),
            transactions_remaining: 50_000,
            balance: assets::Asset::Tdfy.saturating_mul(38_400_000),
            // 150%
            rebates: FixedU128::saturating_from_rational(150_u32, 100_u32),
          },
          SunriseSwapPool {
            id: 4,
            // 50_000 TDFY's minimum value
            minimum_tdfy_value: assets::Asset::Tdfy.saturating_mul(50_000),
            transactions_remaining: 10_000,
            balance: assets::Asset::Tdfy.saturating_mul(19_200_000),
            // 200%
            rebates: FixedU128::saturating_from_rational(200_u32, 100_u32),
          },
          SunriseSwapPool {
            id: 5,
            // 100_000 TDFY's minimum value
            minimum_tdfy_value: assets::Asset::Tdfy.saturating_mul(100_000),
            transactions_remaining: 1_000,
            balance: assets::Asset::Tdfy.saturating_mul(9_600_000),
            // 300%
            rebates: FixedU128::saturating_from_rational(300_u32, 100_u32),
          },
        ],
      }
    };
  }

  /// Helper function to generate a crypto pair from seed
  pub(crate) fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
      .expect("static values are valid; qed")
      .public()
  }

  /// Helper function to generate an account ID from seed
  pub(crate) fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
  where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
  {
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
  }

  /// Helper function to generate stash, controller and session key from seed
  pub(crate) fn authority_keys_from_seed(
    seed: &str,
  ) -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
  ) {
    (
      get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
      get_account_id_from_seed::<sr25519::Public>(seed),
      get_from_seed::<GrandpaId>(seed),
      get_from_seed::<BabeId>(seed),
      get_from_seed::<ImOnlineId>(seed),
      get_from_seed::<AuthorityDiscoveryId>(seed),
    )
  }

  pub(crate) fn get_tide_from_stakeholders(
    stakeholders: Vec<(CurrencyId, AccountId, Balance)>,
  ) -> Vec<(AccountId, Balance)> {
    stakeholders
      .into_iter()
      .filter(|(currency_id, _, _)| *currency_id == CurrencyId::Tdfy)
      .map(|(_, account_id, balance)| (account_id, balance))
      .collect()
  }

  pub(crate) fn get_assets_with_stakeholders(
    stakeholders: Vec<(CurrencyId, AccountId, Balance)>,
    assets: Vec<(AssetId, Vec<u8>, Vec<u8>, u8)>,
  ) -> Vec<(CurrencyId, Vec<u8>, Vec<u8>, u8, Vec<(AccountId, Balance)>)> {
    assets
      .iter()
      .map(|(asset_id, asset_name, asset_symbol, decimals)| {
        let all_endowed_accounts = stakeholders
          .clone()
          .into_iter()
          .filter(|(currency_id, _, _)| *currency_id == CurrencyId::Wrapped(*asset_id))
          .map(|(_, account_id, balance)| (account_id, balance))
          .collect();
        (
          CurrencyId::Wrapped(*asset_id),
          asset_name.clone(),
          asset_symbol.clone(),
          *decimals,
          all_endowed_accounts,
        )
      })
      .collect()
  }

  pub fn get_all_assets() -> Vec<(AssetId, Vec<u8>, Vec<u8>, u8)> {
    assets::Asset::iter()
      .filter(|asset| asset.id() != assets::TDFY)
      .map(|asset| {
        (
          asset.id(),
          asset.name().into(),
          asset.symbol().into(),
          asset.exponent(),
        )
      })
      .collect()
  }

  pub(crate) fn adjust_treasury_balance_for_initial_validators_and_quorums(
    initial_validators: usize,
    initial_quorums: usize,
    endowment: u128,
  ) -> u128 {
    // Validators + quorums
    (initial_validators + initial_quorums) as u128 * endowment
  }

  #[cfg(feature = "lagoon-native")]
  pub fn get_stakeholder_tokens_lagoon_staging() -> Vec<(CurrencyId, AccountId, Balance)> {
    vec![
      // faucet
      (
        CurrencyId::Tdfy,
        //5DUeL7kapQZbyP4FCohywPtsN7AfQ8nA1cayoB6P33FL64xQ
        hex!["3e7e404546ac4697dd7026e3837915e60aa2381954803f18cb09eebd7d1aba67"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
    ]
  }

  // SECRET="key" ./scripts/prepare-dev-lagoon.sh
  #[cfg(feature = "lagoon-native")]
  pub fn get_stakeholder_tokens_lagoon() -> Vec<(CurrencyId, AccountId, Balance)> {
    vec![
      // faucet
      (
        CurrencyId::Tdfy,
        //5DUeL7kapQZbyP4FCohywPtsN7AfQ8nA1cayoB6P33FL64xQ
        hex!["3e7e404546ac4697dd7026e3837915e60aa2381954803f18cb09eebd7d1aba67"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
      // investors
      (
        assets::Asset::Tdfy.currency_id(),
        //5CXr49uaCnz6BhJaZvzQ25H7pCQPfRZcP859dN68T5H6nkGQ
        hex!["14b339129926e102774cfcce909dca2b587c7ba3972aa46034c4253f95c51308"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5CXr49uaCnz6BhJaZvzQ25H7pCQPfRZcP859dN68T5H6nkGQ
        hex!["14b339129926e102774cfcce909dca2b587c7ba3972aa46034c4253f95c51308"].into(),
        // 10_000 USDT
        assets::Asset::Tether.saturating_mul(10_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5CXr49uaCnz6BhJaZvzQ25H7pCQPfRZcP859dN68T5H6nkGQ
        hex!["14b339129926e102774cfcce909dca2b587c7ba3972aa46034c4253f95c51308"].into(),
        // 10_000 USDC
        assets::Asset::USDCoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5CXr49uaCnz6BhJaZvzQ25H7pCQPfRZcP859dN68T5H6nkGQ
        hex!["14b339129926e102774cfcce909dca2b587c7ba3972aa46034c4253f95c51308"].into(),
        // 10_000 BTC
        assets::Asset::Bitcoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5CXr49uaCnz6BhJaZvzQ25H7pCQPfRZcP859dN68T5H6nkGQ
        hex!["14b339129926e102774cfcce909dca2b587c7ba3972aa46034c4253f95c51308"].into(),
        // 10_000 ETH
        assets::Asset::Ethereum.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5G6KfjE9SyMHNMp5ivqoxxfNxoiS2zju7HQe9XV5GiGj3oGB
        hex!["b22cbb278194bfece1ecab5207b743424990fd1d320de9f7f589ce84c44b495e"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5G6KfjE9SyMHNMp5ivqoxxfNxoiS2zju7HQe9XV5GiGj3oGB
        hex!["b22cbb278194bfece1ecab5207b743424990fd1d320de9f7f589ce84c44b495e"].into(),
        // 10_000 USDT
        assets::Asset::Tether.saturating_mul(10_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5G6KfjE9SyMHNMp5ivqoxxfNxoiS2zju7HQe9XV5GiGj3oGB
        hex!["b22cbb278194bfece1ecab5207b743424990fd1d320de9f7f589ce84c44b495e"].into(),
        // 10_000 USDC
        assets::Asset::USDCoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5G6KfjE9SyMHNMp5ivqoxxfNxoiS2zju7HQe9XV5GiGj3oGB
        hex!["b22cbb278194bfece1ecab5207b743424990fd1d320de9f7f589ce84c44b495e"].into(),
        // 10_000 BTC
        assets::Asset::Bitcoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5G6KfjE9SyMHNMp5ivqoxxfNxoiS2zju7HQe9XV5GiGj3oGB
        hex!["b22cbb278194bfece1ecab5207b743424990fd1d320de9f7f589ce84c44b495e"].into(),
        // 10_000 ETH
        assets::Asset::Ethereum.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5CDKTdNQo5zQ5Ra3LuPpt1raBCFEchSyhMPeP1i4ewsHPK3x
        hex!["0691d0109120afb6b60e43a625ebadc9bdf8a855d75c91c08d91de6c2e162717"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5CDKTdNQo5zQ5Ra3LuPpt1raBCFEchSyhMPeP1i4ewsHPK3x
        hex!["0691d0109120afb6b60e43a625ebadc9bdf8a855d75c91c08d91de6c2e162717"].into(),
        // 10_000 USDT
        assets::Asset::Tether.saturating_mul(10_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5CDKTdNQo5zQ5Ra3LuPpt1raBCFEchSyhMPeP1i4ewsHPK3x
        hex!["0691d0109120afb6b60e43a625ebadc9bdf8a855d75c91c08d91de6c2e162717"].into(),
        // 10_000 USDC
        assets::Asset::USDCoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5CDKTdNQo5zQ5Ra3LuPpt1raBCFEchSyhMPeP1i4ewsHPK3x
        hex!["0691d0109120afb6b60e43a625ebadc9bdf8a855d75c91c08d91de6c2e162717"].into(),
        // 10_000 BTC
        assets::Asset::Bitcoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5CDKTdNQo5zQ5Ra3LuPpt1raBCFEchSyhMPeP1i4ewsHPK3x
        hex!["0691d0109120afb6b60e43a625ebadc9bdf8a855d75c91c08d91de6c2e162717"].into(),
        // 10_000 ETH
        assets::Asset::Ethereum.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5F9UYvJbwMzNcxiysaCVG6fLLXrKpkvJ6oAAaS2Wb68RznPf
        hex!["885822a596708b07cfb5c9bce3bb28854572ad39bcae0a09062a80edffe6ac45"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5F9UYvJbwMzNcxiysaCVG6fLLXrKpkvJ6oAAaS2Wb68RznPf
        hex!["885822a596708b07cfb5c9bce3bb28854572ad39bcae0a09062a80edffe6ac45"].into(),
        // 10_000 USDT
        assets::Asset::Tether.saturating_mul(10_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5F9UYvJbwMzNcxiysaCVG6fLLXrKpkvJ6oAAaS2Wb68RznPf
        hex!["885822a596708b07cfb5c9bce3bb28854572ad39bcae0a09062a80edffe6ac45"].into(),
        // 10_000 USDC
        assets::Asset::USDCoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5F9UYvJbwMzNcxiysaCVG6fLLXrKpkvJ6oAAaS2Wb68RznPf
        hex!["885822a596708b07cfb5c9bce3bb28854572ad39bcae0a09062a80edffe6ac45"].into(),
        // 10_000 BTC
        assets::Asset::Bitcoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5F9UYvJbwMzNcxiysaCVG6fLLXrKpkvJ6oAAaS2Wb68RznPf
        hex!["885822a596708b07cfb5c9bce3bb28854572ad39bcae0a09062a80edffe6ac45"].into(),
        // 10_000 ETH
        assets::Asset::Ethereum.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5G6T8M1unQRmZwbNs5xyJpiZgmBPTqQo1xrEympG3Y4GMY7A
        hex!["b245d70b5528570848768e4892bb52f3ca4978957c443df6421760f8a72fab5e"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5G6T8M1unQRmZwbNs5xyJpiZgmBPTqQo1xrEympG3Y4GMY7A
        hex!["b245d70b5528570848768e4892bb52f3ca4978957c443df6421760f8a72fab5e"].into(),
        // 10_000 USDT
        assets::Asset::Tether.saturating_mul(10_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5G6T8M1unQRmZwbNs5xyJpiZgmBPTqQo1xrEympG3Y4GMY7A
        hex!["b245d70b5528570848768e4892bb52f3ca4978957c443df6421760f8a72fab5e"].into(),
        // 10_000 USDC
        assets::Asset::USDCoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5G6T8M1unQRmZwbNs5xyJpiZgmBPTqQo1xrEympG3Y4GMY7A
        hex!["b245d70b5528570848768e4892bb52f3ca4978957c443df6421760f8a72fab5e"].into(),
        // 10_000 BTC
        assets::Asset::Bitcoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5G6T8M1unQRmZwbNs5xyJpiZgmBPTqQo1xrEympG3Y4GMY7A
        hex!["b245d70b5528570848768e4892bb52f3ca4978957c443df6421760f8a72fab5e"].into(),
        // 10_000 ETH
        assets::Asset::Ethereum.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5CJtCCvQcJLRmWUYFQbyuRzczKLhqS12aQHEVbsNrMtZ2Eoo
        hex!["0ad03b8cccca0980fb8c0e7469c909f26cf3c36f7a48bd18ffd907728e248434"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5CJtCCvQcJLRmWUYFQbyuRzczKLhqS12aQHEVbsNrMtZ2Eoo
        hex!["0ad03b8cccca0980fb8c0e7469c909f26cf3c36f7a48bd18ffd907728e248434"].into(),
        // 10_000 USDT
        assets::Asset::Tether.saturating_mul(10_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5CJtCCvQcJLRmWUYFQbyuRzczKLhqS12aQHEVbsNrMtZ2Eoo
        hex!["0ad03b8cccca0980fb8c0e7469c909f26cf3c36f7a48bd18ffd907728e248434"].into(),
        // 10_000 USDC
        assets::Asset::USDCoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5CJtCCvQcJLRmWUYFQbyuRzczKLhqS12aQHEVbsNrMtZ2Eoo
        hex!["0ad03b8cccca0980fb8c0e7469c909f26cf3c36f7a48bd18ffd907728e248434"].into(),
        // 10_000 BTC
        assets::Asset::Bitcoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5CJtCCvQcJLRmWUYFQbyuRzczKLhqS12aQHEVbsNrMtZ2Eoo
        hex!["0ad03b8cccca0980fb8c0e7469c909f26cf3c36f7a48bd18ffd907728e248434"].into(),
        // 10_000 ETH
        assets::Asset::Ethereum.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5FA6JuLx8geeWMUvKuTf3LRZNY7sqYwLrtmVZ2wKWRos9nHv
        hex!["88d0823558e2d1784938d2adee2524d311c71396ca014660d52354623283ee65"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5FA6JuLx8geeWMUvKuTf3LRZNY7sqYwLrtmVZ2wKWRos9nHv
        hex!["88d0823558e2d1784938d2adee2524d311c71396ca014660d52354623283ee65"].into(),
        // 10_000 USDT
        assets::Asset::Tether.saturating_mul(10_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5FA6JuLx8geeWMUvKuTf3LRZNY7sqYwLrtmVZ2wKWRos9nHv
        hex!["88d0823558e2d1784938d2adee2524d311c71396ca014660d52354623283ee65"].into(),
        // 10_000 USDC
        assets::Asset::USDCoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5FA6JuLx8geeWMUvKuTf3LRZNY7sqYwLrtmVZ2wKWRos9nHv
        hex!["88d0823558e2d1784938d2adee2524d311c71396ca014660d52354623283ee65"].into(),
        // 10_000 BTC
        assets::Asset::Bitcoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5FA6JuLx8geeWMUvKuTf3LRZNY7sqYwLrtmVZ2wKWRos9nHv
        hex!["88d0823558e2d1784938d2adee2524d311c71396ca014660d52354623283ee65"].into(),
        // 10_000 ETH
        assets::Asset::Ethereum.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5GmiWrcsDvQjbQ4sbKSycRS7dZ3AkxuB12iJC2vn9cZBXSpu
        hex!["d03836376dc9d289ce6b0e01442eec48188a4ea9a00064f95d8fd800f853c111"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5GmiWrcsDvQjbQ4sbKSycRS7dZ3AkxuB12iJC2vn9cZBXSpu
        hex!["d03836376dc9d289ce6b0e01442eec48188a4ea9a00064f95d8fd800f853c111"].into(),
        // 10_000 USDT
        assets::Asset::Tether.saturating_mul(10_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5GmiWrcsDvQjbQ4sbKSycRS7dZ3AkxuB12iJC2vn9cZBXSpu
        hex!["d03836376dc9d289ce6b0e01442eec48188a4ea9a00064f95d8fd800f853c111"].into(),
        // 10_000 USDC
        assets::Asset::USDCoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5GmiWrcsDvQjbQ4sbKSycRS7dZ3AkxuB12iJC2vn9cZBXSpu
        hex!["d03836376dc9d289ce6b0e01442eec48188a4ea9a00064f95d8fd800f853c111"].into(),
        // 10_000 BTC
        assets::Asset::Bitcoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5GmiWrcsDvQjbQ4sbKSycRS7dZ3AkxuB12iJC2vn9cZBXSpu
        hex!["d03836376dc9d289ce6b0e01442eec48188a4ea9a00064f95d8fd800f853c111"].into(),
        // 10_000 ETH
        assets::Asset::Ethereum.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5CJ1ADipRQcLonHR8paMQ3kC7Rs76s7vPfaP7fWea2fBgnTm
        hex!["0a246f9ff97b425735144a1162db84211e8953fad997a4906877a4c7dcb62f22"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5CJ1ADipRQcLonHR8paMQ3kC7Rs76s7vPfaP7fWea2fBgnTm
        hex!["0a246f9ff97b425735144a1162db84211e8953fad997a4906877a4c7dcb62f22"].into(),
        // 10_000 USDT
        assets::Asset::Tether.saturating_mul(10_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5CJ1ADipRQcLonHR8paMQ3kC7Rs76s7vPfaP7fWea2fBgnTm
        hex!["0a246f9ff97b425735144a1162db84211e8953fad997a4906877a4c7dcb62f22"].into(),
        // 10_000 USDC
        assets::Asset::USDCoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5CJ1ADipRQcLonHR8paMQ3kC7Rs76s7vPfaP7fWea2fBgnTm
        hex!["0a246f9ff97b425735144a1162db84211e8953fad997a4906877a4c7dcb62f22"].into(),
        // 10_000 BTC
        assets::Asset::Bitcoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5CJ1ADipRQcLonHR8paMQ3kC7Rs76s7vPfaP7fWea2fBgnTm
        hex!["0a246f9ff97b425735144a1162db84211e8953fad997a4906877a4c7dcb62f22"].into(),
        // 10_000 ETH
        assets::Asset::Ethereum.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5F92rg6xcfyCKxfpbkNNwC9bouURZPSzQRCJWGoX47Myz4PH
        hex!["8801a45cc54e90766bac513b5b40771ffceb96fc45236480b69d7c7ccd01d75e"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5F92rg6xcfyCKxfpbkNNwC9bouURZPSzQRCJWGoX47Myz4PH
        hex!["8801a45cc54e90766bac513b5b40771ffceb96fc45236480b69d7c7ccd01d75e"].into(),
        // 10_000 USDT
        assets::Asset::Tether.saturating_mul(10_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5F92rg6xcfyCKxfpbkNNwC9bouURZPSzQRCJWGoX47Myz4PH
        hex!["8801a45cc54e90766bac513b5b40771ffceb96fc45236480b69d7c7ccd01d75e"].into(),
        // 10_000 USDC
        assets::Asset::USDCoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5F92rg6xcfyCKxfpbkNNwC9bouURZPSzQRCJWGoX47Myz4PH
        hex!["8801a45cc54e90766bac513b5b40771ffceb96fc45236480b69d7c7ccd01d75e"].into(),
        // 10_000 BTC
        assets::Asset::Bitcoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5F92rg6xcfyCKxfpbkNNwC9bouURZPSzQRCJWGoX47Myz4PH
        hex!["8801a45cc54e90766bac513b5b40771ffceb96fc45236480b69d7c7ccd01d75e"].into(),
        // 10_000 ETH
        assets::Asset::Ethereum.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5Dco6bBWGNj62JxoUDSZ5nxFyXv8iEUcXqYzYRRYzBD5KtVB
        hex!["44b5b9d3a8474560f6705eb9a2fa5875fdf3e8bef27352da679814043537d323"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5Dco6bBWGNj62JxoUDSZ5nxFyXv8iEUcXqYzYRRYzBD5KtVB
        hex!["44b5b9d3a8474560f6705eb9a2fa5875fdf3e8bef27352da679814043537d323"].into(),
        // 10_000 USDT
        assets::Asset::Tether.saturating_mul(10_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5Dco6bBWGNj62JxoUDSZ5nxFyXv8iEUcXqYzYRRYzBD5KtVB
        hex!["44b5b9d3a8474560f6705eb9a2fa5875fdf3e8bef27352da679814043537d323"].into(),
        // 10_000 USDC
        assets::Asset::USDCoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5Dco6bBWGNj62JxoUDSZ5nxFyXv8iEUcXqYzYRRYzBD5KtVB
        hex!["44b5b9d3a8474560f6705eb9a2fa5875fdf3e8bef27352da679814043537d323"].into(),
        // 10_000 BTC
        assets::Asset::Bitcoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5Dco6bBWGNj62JxoUDSZ5nxFyXv8iEUcXqYzYRRYzBD5KtVB
        hex!["44b5b9d3a8474560f6705eb9a2fa5875fdf3e8bef27352da679814043537d323"].into(),
        // 10_000 ETH
        assets::Asset::Ethereum.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5FWkciyXUqmvqsv4d8DGJ1rCufcWFqjsbc2ScWSTKJUy3F1n
        hex!["98925ed611afe810efef0e3da011e25ed6704769de1b3b962840c4d04f55933f"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5FWkciyXUqmvqsv4d8DGJ1rCufcWFqjsbc2ScWSTKJUy3F1n
        hex!["98925ed611afe810efef0e3da011e25ed6704769de1b3b962840c4d04f55933f"].into(),
        // 10_000 USDT
        assets::Asset::Tether.saturating_mul(10_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5FWkciyXUqmvqsv4d8DGJ1rCufcWFqjsbc2ScWSTKJUy3F1n
        hex!["98925ed611afe810efef0e3da011e25ed6704769de1b3b962840c4d04f55933f"].into(),
        // 10_000 USDC
        assets::Asset::USDCoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5FWkciyXUqmvqsv4d8DGJ1rCufcWFqjsbc2ScWSTKJUy3F1n
        hex!["98925ed611afe810efef0e3da011e25ed6704769de1b3b962840c4d04f55933f"].into(),
        // 10_000 BTC
        assets::Asset::Bitcoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5FWkciyXUqmvqsv4d8DGJ1rCufcWFqjsbc2ScWSTKJUy3F1n
        hex!["98925ed611afe810efef0e3da011e25ed6704769de1b3b962840c4d04f55933f"].into(),
        // 10_000 ETH
        assets::Asset::Ethereum.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5DSNW8JyKnL8ffQyW5c1rLcNkprkSmRH84ZqHci8vNsRbKrH
        hex!["3cc27afba8905755c9243e61cfdfc46c2bfbc697eb2728d55b7f1b924e947762"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5DSNW8JyKnL8ffQyW5c1rLcNkprkSmRH84ZqHci8vNsRbKrH
        hex!["3cc27afba8905755c9243e61cfdfc46c2bfbc697eb2728d55b7f1b924e947762"].into(),
        // 10_000 USDT
        assets::Asset::Tether.saturating_mul(10_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5DSNW8JyKnL8ffQyW5c1rLcNkprkSmRH84ZqHci8vNsRbKrH
        hex!["3cc27afba8905755c9243e61cfdfc46c2bfbc697eb2728d55b7f1b924e947762"].into(),
        // 10_000 USDC
        assets::Asset::USDCoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5DSNW8JyKnL8ffQyW5c1rLcNkprkSmRH84ZqHci8vNsRbKrH
        hex!["3cc27afba8905755c9243e61cfdfc46c2bfbc697eb2728d55b7f1b924e947762"].into(),
        // 10_000 BTC
        assets::Asset::Bitcoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5DSNW8JyKnL8ffQyW5c1rLcNkprkSmRH84ZqHci8vNsRbKrH
        hex!["3cc27afba8905755c9243e61cfdfc46c2bfbc697eb2728d55b7f1b924e947762"].into(),
        // 10_000 ETH
        assets::Asset::Ethereum.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5HYsxrjNuqE9y4X4ubZgu9heVTSLq3s5xheEEEXgf9PH4xXk
        hex!["f2aa0f922382ad8a828044bb95702a1280d8f38263f28fc15e267ac9481fff5c"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5HYsxrjNuqE9y4X4ubZgu9heVTSLq3s5xheEEEXgf9PH4xXk
        hex!["f2aa0f922382ad8a828044bb95702a1280d8f38263f28fc15e267ac9481fff5c"].into(),
        // 10_000 USDT
        assets::Asset::Tether.saturating_mul(10_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5HYsxrjNuqE9y4X4ubZgu9heVTSLq3s5xheEEEXgf9PH4xXk
        hex!["f2aa0f922382ad8a828044bb95702a1280d8f38263f28fc15e267ac9481fff5c"].into(),
        // 10_000 USDC
        assets::Asset::USDCoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5HYsxrjNuqE9y4X4ubZgu9heVTSLq3s5xheEEEXgf9PH4xXk
        hex!["f2aa0f922382ad8a828044bb95702a1280d8f38263f28fc15e267ac9481fff5c"].into(),
        // 10_000 BTC
        assets::Asset::Bitcoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5HYsxrjNuqE9y4X4ubZgu9heVTSLq3s5xheEEEXgf9PH4xXk
        hex!["f2aa0f922382ad8a828044bb95702a1280d8f38263f28fc15e267ac9481fff5c"].into(),
        // 10_000 ETH
        assets::Asset::Ethereum.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5DLNXC7EiC4Dr7ZpSHrrZrCFwdtdURA6uXg4HxCANa9w1s6x
        hex!["382f11df7a878f1242d98603699b115aed13abc9e6bfa425c5492436336c4c26"].into(),
        // 10_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5DLNXC7EiC4Dr7ZpSHrrZrCFwdtdURA6uXg4HxCANa9w1s6x
        hex!["382f11df7a878f1242d98603699b115aed13abc9e6bfa425c5492436336c4c26"].into(),
        // 10_000 USDT
        assets::Asset::Tether.saturating_mul(10_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5DLNXC7EiC4Dr7ZpSHrrZrCFwdtdURA6uXg4HxCANa9w1s6x
        hex!["382f11df7a878f1242d98603699b115aed13abc9e6bfa425c5492436336c4c26"].into(),
        // 10_000 USDC
        assets::Asset::USDCoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5DLNXC7EiC4Dr7ZpSHrrZrCFwdtdURA6uXg4HxCANa9w1s6x
        hex!["382f11df7a878f1242d98603699b115aed13abc9e6bfa425c5492436336c4c26"].into(),
        // 10_000 BTC
        assets::Asset::Bitcoin.saturating_mul(10_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5DLNXC7EiC4Dr7ZpSHrrZrCFwdtdURA6uXg4HxCANa9w1s6x
        hex!["382f11df7a878f1242d98603699b115aed13abc9e6bfa425c5492436336c4c26"].into(),
        // 10_000 ETH
        assets::Asset::Ethereum.saturating_mul(10_000),
      ),
      // devs
      (
        assets::Asset::Tdfy.currency_id(),
        //5CFsxqm4muZDTZA3vZVE8Pm9ny2XDrKvR8UAZuufxFLGoAwQ
        hex!["0885b880a6305cb19ea441fab8b5ed02cadef5cb5dafe9e9afd7c0be80046636"].into(),
        // 1_000_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5CFsxqm4muZDTZA3vZVE8Pm9ny2XDrKvR8UAZuufxFLGoAwQ
        hex!["0885b880a6305cb19ea441fab8b5ed02cadef5cb5dafe9e9afd7c0be80046636"].into(),
        // 1_000_000 USDT
        assets::Asset::Tether.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5CFsxqm4muZDTZA3vZVE8Pm9ny2XDrKvR8UAZuufxFLGoAwQ
        hex!["0885b880a6305cb19ea441fab8b5ed02cadef5cb5dafe9e9afd7c0be80046636"].into(),
        // 1_000_000 USDC
        assets::Asset::USDCoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5CFsxqm4muZDTZA3vZVE8Pm9ny2XDrKvR8UAZuufxFLGoAwQ
        hex!["0885b880a6305cb19ea441fab8b5ed02cadef5cb5dafe9e9afd7c0be80046636"].into(),
        // 1_000_000 BTC
        assets::Asset::Bitcoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5CFsxqm4muZDTZA3vZVE8Pm9ny2XDrKvR8UAZuufxFLGoAwQ
        hex!["0885b880a6305cb19ea441fab8b5ed02cadef5cb5dafe9e9afd7c0be80046636"].into(),
        // 1_000_000 ETH
        assets::Asset::Ethereum.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5HVXyDbEY3Luroo4aiurP1xLZnKKAsXU4GRxVNBGmH2d2io5
        hex!["f01d04fcd4db7b552a14bec692f6fcb7a9fc4669972cdadc563f2bcb324c9741"].into(),
        // 1_000_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5HVXyDbEY3Luroo4aiurP1xLZnKKAsXU4GRxVNBGmH2d2io5
        hex!["f01d04fcd4db7b552a14bec692f6fcb7a9fc4669972cdadc563f2bcb324c9741"].into(),
        // 1_000_000 USDT
        assets::Asset::Tether.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5HVXyDbEY3Luroo4aiurP1xLZnKKAsXU4GRxVNBGmH2d2io5
        hex!["f01d04fcd4db7b552a14bec692f6fcb7a9fc4669972cdadc563f2bcb324c9741"].into(),
        // 1_000_000 USDC
        assets::Asset::USDCoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5HVXyDbEY3Luroo4aiurP1xLZnKKAsXU4GRxVNBGmH2d2io5
        hex!["f01d04fcd4db7b552a14bec692f6fcb7a9fc4669972cdadc563f2bcb324c9741"].into(),
        // 1_000_000 BTC
        assets::Asset::Bitcoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5HVXyDbEY3Luroo4aiurP1xLZnKKAsXU4GRxVNBGmH2d2io5
        hex!["f01d04fcd4db7b552a14bec692f6fcb7a9fc4669972cdadc563f2bcb324c9741"].into(),
        // 1_000_000 ETH
        assets::Asset::Ethereum.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5CLdLM1HrtWdRvYBfH6dcEWQnRD6AeKZWGyuxE4shPBdY2r2
        hex!["0c24b38a7a768577d9e00b8d01f3412bf5121632c855dd4837abc7fe4afd4609"].into(),
        // 1_000_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5CLdLM1HrtWdRvYBfH6dcEWQnRD6AeKZWGyuxE4shPBdY2r2
        hex!["0c24b38a7a768577d9e00b8d01f3412bf5121632c855dd4837abc7fe4afd4609"].into(),
        // 1_000_000 USDT
        assets::Asset::Tether.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5CLdLM1HrtWdRvYBfH6dcEWQnRD6AeKZWGyuxE4shPBdY2r2
        hex!["0c24b38a7a768577d9e00b8d01f3412bf5121632c855dd4837abc7fe4afd4609"].into(),
        // 1_000_000 USDC
        assets::Asset::USDCoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5CLdLM1HrtWdRvYBfH6dcEWQnRD6AeKZWGyuxE4shPBdY2r2
        hex!["0c24b38a7a768577d9e00b8d01f3412bf5121632c855dd4837abc7fe4afd4609"].into(),
        // 1_000_000 BTC
        assets::Asset::Bitcoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5CLdLM1HrtWdRvYBfH6dcEWQnRD6AeKZWGyuxE4shPBdY2r2
        hex!["0c24b38a7a768577d9e00b8d01f3412bf5121632c855dd4837abc7fe4afd4609"].into(),
        // 1_000_000 ETH
        assets::Asset::Ethereum.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5HgiTfx31XKS8F74LDVjiXG7VcJ69Q1sWFRjAgyJrK4yXFY1
        hex!["f8a4088e206592cb8eaa5bd73279b552f85a4b4da7761184076ee404df2c906c"].into(),
        // 1_000_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5HgiTfx31XKS8F74LDVjiXG7VcJ69Q1sWFRjAgyJrK4yXFY1
        hex!["f8a4088e206592cb8eaa5bd73279b552f85a4b4da7761184076ee404df2c906c"].into(),
        // 1_000_000 USDT
        assets::Asset::Tether.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5HgiTfx31XKS8F74LDVjiXG7VcJ69Q1sWFRjAgyJrK4yXFY1
        hex!["f8a4088e206592cb8eaa5bd73279b552f85a4b4da7761184076ee404df2c906c"].into(),
        // 1_000_000 USDC
        assets::Asset::USDCoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5HgiTfx31XKS8F74LDVjiXG7VcJ69Q1sWFRjAgyJrK4yXFY1
        hex!["f8a4088e206592cb8eaa5bd73279b552f85a4b4da7761184076ee404df2c906c"].into(),
        // 1_000_000 BTC
        assets::Asset::Bitcoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5HgiTfx31XKS8F74LDVjiXG7VcJ69Q1sWFRjAgyJrK4yXFY1
        hex!["f8a4088e206592cb8eaa5bd73279b552f85a4b4da7761184076ee404df2c906c"].into(),
        // 1_000_000 ETH
        assets::Asset::Ethereum.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5GL5yZjsYNDLWY12CJt5Vm1jktLfaHTiHXHcZNmsxd13EXf9
        hex!["bcac12e15f80982de85d5667ddc1b6dd49bee80c4edfd371c5ba5d47023fa97b"].into(),
        // 1_000_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5GL5yZjsYNDLWY12CJt5Vm1jktLfaHTiHXHcZNmsxd13EXf9
        hex!["bcac12e15f80982de85d5667ddc1b6dd49bee80c4edfd371c5ba5d47023fa97b"].into(),
        // 1_000_000 USDT
        assets::Asset::Tether.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5GL5yZjsYNDLWY12CJt5Vm1jktLfaHTiHXHcZNmsxd13EXf9
        hex!["bcac12e15f80982de85d5667ddc1b6dd49bee80c4edfd371c5ba5d47023fa97b"].into(),
        // 1_000_000 USDC
        assets::Asset::USDCoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5GL5yZjsYNDLWY12CJt5Vm1jktLfaHTiHXHcZNmsxd13EXf9
        hex!["bcac12e15f80982de85d5667ddc1b6dd49bee80c4edfd371c5ba5d47023fa97b"].into(),
        // 1_000_000 BTC
        assets::Asset::Bitcoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5GL5yZjsYNDLWY12CJt5Vm1jktLfaHTiHXHcZNmsxd13EXf9
        hex!["bcac12e15f80982de85d5667ddc1b6dd49bee80c4edfd371c5ba5d47023fa97b"].into(),
        // 1_000_000 ETH
        assets::Asset::Ethereum.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5EPTgRuaMcWTH88BfmZQKymiJ41eKJc9goQC7VeRGwGnMGbK
        hex!["66c6683ad9c6b1940d9d74691cdc0cfd4e760357d7427185e73f1c420d2ce464"].into(),
        // 1_000_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5EPTgRuaMcWTH88BfmZQKymiJ41eKJc9goQC7VeRGwGnMGbK
        hex!["66c6683ad9c6b1940d9d74691cdc0cfd4e760357d7427185e73f1c420d2ce464"].into(),
        // 1_000_000 USDT
        assets::Asset::Tether.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5EPTgRuaMcWTH88BfmZQKymiJ41eKJc9goQC7VeRGwGnMGbK
        hex!["66c6683ad9c6b1940d9d74691cdc0cfd4e760357d7427185e73f1c420d2ce464"].into(),
        // 1_000_000 USDC
        assets::Asset::USDCoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5EPTgRuaMcWTH88BfmZQKymiJ41eKJc9goQC7VeRGwGnMGbK
        hex!["66c6683ad9c6b1940d9d74691cdc0cfd4e760357d7427185e73f1c420d2ce464"].into(),
        // 1_000_000 BTC
        assets::Asset::Bitcoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5EPTgRuaMcWTH88BfmZQKymiJ41eKJc9goQC7VeRGwGnMGbK
        hex!["66c6683ad9c6b1940d9d74691cdc0cfd4e760357d7427185e73f1c420d2ce464"].into(),
        // 1_000_000 ETH
        assets::Asset::Ethereum.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5GKxcqHFndxDH8qdpK6311Qco4MLJJZJeY8ZSFjjN6w31goH
        hex!["bc934e6e40cd8207bc9bc72fb8c1c2cb3266ef7caac69f6e18cb5792ab859f62"].into(),
        // 1_000_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5GKxcqHFndxDH8qdpK6311Qco4MLJJZJeY8ZSFjjN6w31goH
        hex!["bc934e6e40cd8207bc9bc72fb8c1c2cb3266ef7caac69f6e18cb5792ab859f62"].into(),
        // 1_000_000 USDT
        assets::Asset::Tether.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5GKxcqHFndxDH8qdpK6311Qco4MLJJZJeY8ZSFjjN6w31goH
        hex!["bc934e6e40cd8207bc9bc72fb8c1c2cb3266ef7caac69f6e18cb5792ab859f62"].into(),
        // 1_000_000 USDC
        assets::Asset::USDCoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5GKxcqHFndxDH8qdpK6311Qco4MLJJZJeY8ZSFjjN6w31goH
        hex!["bc934e6e40cd8207bc9bc72fb8c1c2cb3266ef7caac69f6e18cb5792ab859f62"].into(),
        // 1_000_000 BTC
        assets::Asset::Bitcoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5GKxcqHFndxDH8qdpK6311Qco4MLJJZJeY8ZSFjjN6w31goH
        hex!["bc934e6e40cd8207bc9bc72fb8c1c2cb3266ef7caac69f6e18cb5792ab859f62"].into(),
        // 1_000_000 ETH
        assets::Asset::Ethereum.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5CXeo6fy34CuZgmbkSjy7vjqrv9DojqmQmqCrHwANxwPqC9Q
        hex!["148d51dee87e09b75f8487aaf72aecda9b107f577e184da1d065d14bf02bc542"].into(),
        // 1_000_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5CXeo6fy34CuZgmbkSjy7vjqrv9DojqmQmqCrHwANxwPqC9Q
        hex!["148d51dee87e09b75f8487aaf72aecda9b107f577e184da1d065d14bf02bc542"].into(),
        // 1_000_000 USDT
        assets::Asset::Tether.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5CXeo6fy34CuZgmbkSjy7vjqrv9DojqmQmqCrHwANxwPqC9Q
        hex!["148d51dee87e09b75f8487aaf72aecda9b107f577e184da1d065d14bf02bc542"].into(),
        // 1_000_000 USDC
        assets::Asset::USDCoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5CXeo6fy34CuZgmbkSjy7vjqrv9DojqmQmqCrHwANxwPqC9Q
        hex!["148d51dee87e09b75f8487aaf72aecda9b107f577e184da1d065d14bf02bc542"].into(),
        // 1_000_000 BTC
        assets::Asset::Bitcoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5CXeo6fy34CuZgmbkSjy7vjqrv9DojqmQmqCrHwANxwPqC9Q
        hex!["148d51dee87e09b75f8487aaf72aecda9b107f577e184da1d065d14bf02bc542"].into(),
        // 1_000_000 ETH
        assets::Asset::Ethereum.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5FQyxubtnNEpjcTWWRygJStchrQoSKc9r6ohPUv93WPMechq
        hex!["942bd4d3c1de0dbd822551f572762e194e52664bb94686c96c0679a899147506"].into(),
        // 1_000_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5FQyxubtnNEpjcTWWRygJStchrQoSKc9r6ohPUv93WPMechq
        hex!["942bd4d3c1de0dbd822551f572762e194e52664bb94686c96c0679a899147506"].into(),
        // 1_000_000 USDT
        assets::Asset::Tether.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5FQyxubtnNEpjcTWWRygJStchrQoSKc9r6ohPUv93WPMechq
        hex!["942bd4d3c1de0dbd822551f572762e194e52664bb94686c96c0679a899147506"].into(),
        // 1_000_000 USDC
        assets::Asset::USDCoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5FQyxubtnNEpjcTWWRygJStchrQoSKc9r6ohPUv93WPMechq
        hex!["942bd4d3c1de0dbd822551f572762e194e52664bb94686c96c0679a899147506"].into(),
        // 1_000_000 BTC
        assets::Asset::Bitcoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5FQyxubtnNEpjcTWWRygJStchrQoSKc9r6ohPUv93WPMechq
        hex!["942bd4d3c1de0dbd822551f572762e194e52664bb94686c96c0679a899147506"].into(),
        // 1_000_000 ETH
        assets::Asset::Ethereum.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tdfy.currency_id(),
        //5FKuzgFppRcJqs1bYQvrDJ9DrKZaXqrwKggWBk4DyfpXFvoo
        hex!["904e3dea6bcdc6cb523f52cbdedad53c24bbd95692ec690154b0f2c7f0abc55c"].into(),
        // 1_000_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Tether.currency_id(),
        //5FKuzgFppRcJqs1bYQvrDJ9DrKZaXqrwKggWBk4DyfpXFvoo
        hex!["904e3dea6bcdc6cb523f52cbdedad53c24bbd95692ec690154b0f2c7f0abc55c"].into(),
        // 1_000_000 USDT
        assets::Asset::Tether.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::USDCoin.currency_id(),
        //5FKuzgFppRcJqs1bYQvrDJ9DrKZaXqrwKggWBk4DyfpXFvoo
        hex!["904e3dea6bcdc6cb523f52cbdedad53c24bbd95692ec690154b0f2c7f0abc55c"].into(),
        // 1_000_000 USDC
        assets::Asset::USDCoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Bitcoin.currency_id(),
        //5FKuzgFppRcJqs1bYQvrDJ9DrKZaXqrwKggWBk4DyfpXFvoo
        hex!["904e3dea6bcdc6cb523f52cbdedad53c24bbd95692ec690154b0f2c7f0abc55c"].into(),
        // 1_000_000 BTC
        assets::Asset::Bitcoin.saturating_mul(1_000_000),
      ),
      (
        assets::Asset::Ethereum.currency_id(),
        //5FKuzgFppRcJqs1bYQvrDJ9DrKZaXqrwKggWBk4DyfpXFvoo
        hex!["904e3dea6bcdc6cb523f52cbdedad53c24bbd95692ec690154b0f2c7f0abc55c"].into(),
        // 1_000_000 ETH
        assets::Asset::Ethereum.saturating_mul(1_000_000),
      ),
    ]
  }

  // SECRET="key" ./scripts/prepare-dev-tidechain.sh
  #[cfg(feature = "tidechain-native")]
  pub fn get_stakeholder_tokens_tidechain() -> Vec<(CurrencyId, AccountId, Balance)> {
    let stakeholder_tokens_tidechain = vec![
      (
        CurrencyId::Tdfy,
        //Lending for Validators
        hex!["49a114a2e284aead1fac808e7678969feaa203ab13573bdf531aac7593e79fa2"].into(),
        // 5_000_000 TDFY
        assets::Asset::Tdfy.saturating_mul(5_000_000),
      ),
      (
        CurrencyId::Tdfy,
        //Market Maker
        hex!["c4a1869429a17efc98db6c2c664d5230e8d74e842ef40235507d119ef705e532"].into(),
        // 10_000_000 TDFY
        assets::Asset::Tdfy.saturating_mul(10_000_000),
      ),
      (
        CurrencyId::Tdfy,
        //Tidefi development pool
        hex!["8e14d1ac896ea00e18d855588ee13103449cc6e41d4b0173d917cabea84bdb08"].into(),
        // 80_000_000 TDFY
        assets::Asset::Tdfy.saturating_mul(80_000_000),
      ),
      // team
      (
        CurrencyId::Tdfy,
        //A
        hex!["542e6df326b7ea609ae7d1b3f30b5fbc9e473415d8094e236ebdbc0f5204cc25"].into(),
        // 1_720_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_720_000),
      ),
      (
        CurrencyId::Tdfy,
        //B
        hex!["264bf1d4d2e4fe38912bce61c7fcdc35d83147fdb164bd761bcfd951265c9e47"].into(),
        // 2_230_000 TDFY
        assets::Asset::Tdfy.saturating_mul(2_230_000),
      ),
      (
        CurrencyId::Tdfy,
        //C
        hex!["56d24b8bd664c8f986139c943764033ff87d75f070be15a5756fd8a1461d925a"].into(),
        // 11_760_000 TDFY
        assets::Asset::Tdfy.saturating_mul(11_760_000),
      ),
      (
        CurrencyId::Tdfy,
        //D
        hex!["7c37ae52b566e906c12d692c6491f6bf53c7b8aac48a5508a19ead28cbb63a6b"].into(),
        // 1_885_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_885_000),
      ),
      (
        CurrencyId::Tdfy,
        //E
        hex!["640bc73182d4aff458d28e15d6747d51aedd0be5c663c0cf09ac292727a4022b"].into(),
        // 2_066_900 TDFY
        assets::Asset::Tdfy.saturating_mul(2_066_900),
      ),
      (
        CurrencyId::Tdfy,
        //F
        hex!["1073d449bf500cfdb1d6bb58bd3ed056ebcbf35023b7a749e35d7b69b8cb3229"].into(),
        // 105_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_653_800),
      ),
      (
        CurrencyId::Tdfy,
        //G
        hex!["0e17ae7783c5eabf6a99ae2de893c51ea418645283a41aa888e9d52af00e5e44"].into(),
        // 3_043_000 TDFY
        assets::Asset::Tdfy.saturating_mul(3_043_000),
      ),
      (
        CurrencyId::Tdfy,
        //H
        hex!["fed36467731f9f952aac5fe2f3d23601aea3057cf9a9e37baea4afd752928654"].into(),
        // 4_355_200 TDFY
        assets::Asset::Tdfy.saturating_mul(4_355_200),
      ),
      (
        CurrencyId::Tdfy,
        //I
        hex!["9e0726eaac26eb8c8d7a9c75bd5f074464a3c974c16467d193769dacd7743c31"].into(),
        // 1_994_800 TDFY
        assets::Asset::Tdfy.saturating_mul(1_994_800),
      ),
      (
        CurrencyId::Tdfy,
        //J
        hex!["d263a88080f51fb484804b9e2e744f0a81ab820a5255d666424c5a8aa5e82418"].into(),
        // 2_215_500 TDFY
        assets::Asset::Tdfy.saturating_mul(2_215_500),
      ),
      (
        CurrencyId::Tdfy,
        //K
        hex!["1661a2075b7be54a942b52ba10e924469e3bdf8abbe9037d38f357ee78f86d79"].into(),
        // 1_113_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_113_000),
      ),
      (
        CurrencyId::Tdfy,
        //L
        hex!["40a4ee2d7304c4ef7cf00732425c57a9d58e4202bf43472f2dcb162a748f606d"].into(),
        // 2_825_000 TDFY
        assets::Asset::Tdfy.saturating_mul(2_825_000),
      ),
      (
        CurrencyId::Tdfy,
        //M
        hex!["5e7d7e4a8af71774f2928de7cd8b201fafff6bd6c96b5601d687da560cf00916"].into(),
        // 2_175_000 TDFY
        assets::Asset::Tdfy.saturating_mul(2_175_000),
      ),
      (
        CurrencyId::Tdfy,
        //N
        hex!["d0fbfacb97883cb2556460da5e6308c11b836c216714255a03f28ab1106aff41"].into(),
        // 590_000 TDFY
        assets::Asset::Tdfy.saturating_mul(590_000),
      ),
      (
        CurrencyId::Tdfy,
        //O
        hex!["1a6ccfd2da09c82181721b0d819a0bb4a7fd5fefccda52281d5ba6508fbb6c64"].into(),
        // 2_451_000 TDFY
        assets::Asset::Tdfy.saturating_mul(2_451_000),
      ),
      (
        CurrencyId::Tdfy,
        //P
        hex!["a61f1f9724052d89026286f60158fb096d191bbb9d657332964ad20503effc31"].into(),
        // 1_845_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_845_000),
      ),
      (
        CurrencyId::Tdfy,
        //Q
        hex!["ece719a6a53971d93350f7df82fad72be842ebc15fd6fda6a3c9a01f74819e74"].into(),
        // 1_515_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_515_000),
      ),
      (
        CurrencyId::Tdfy,
        //R
        hex!["d683b0f3706345f4ff1bbb1fd9a54454c4edf050e20cb177d5e82b0945a55b3f"].into(),
        // 3_360_000 TDFY
        assets::Asset::Tdfy.saturating_mul(3_360_000),
      ),
      (
        CurrencyId::Tdfy,
        //S
        hex!["043c68b39d9b45788ca53ad830979e221c4034f86398672775c6a9d35424b878"].into(),
        // 240_000 TDFY
        assets::Asset::Tdfy.saturating_mul(240_000),
      ),
      (
        CurrencyId::Tdfy,
        //T
        hex!["f49304cc009eb5eb37e762ff829db11b86b3f639048da93f638ea9fd65828a72"].into(),
        // 1_865_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_865_000),
      ),
      (
        CurrencyId::Tdfy,
        //U
        hex!["7a6df5d223accd21b5672ada2ed68b790b5be0cafd84dc18d30f0b525122617e"].into(),
        // 1_345_000 TDFY
        assets::Asset::Tdfy.saturating_mul(1_345_000),
      ),
    ];

    let total_initial_account_allocation: Balance = stakeholder_tokens_tidechain
      .iter()
      .filter(|(currency_id, _, _)| *currency_id == CurrencyId::Tdfy)
      .map(|(_, _, balance)| *balance)
      .sum();

    assert_eq!(
      total_initial_account_allocation, TOTAL_INITIAL_ACCOUNT_ALLOCATION,
      "Initial account allocation total is not correct"
    );

    stakeholder_tokens_tidechain
  }

  #[cfg(feature = "lagoon-native")]
  pub fn get_vesting_terms_lagoon() -> Vec<(AccountId, u32, u32, u32, Balance)> {
    vec![]
  }

  #[cfg(feature = "tidechain-native")]
  pub fn get_vesting_terms_tidechain() -> Vec<(AccountId, u32, u32, u32, Balance)> {
    vec![
      //Tidefi development pool
      (
        hex!["8e14d1ac896ea00e18d855588ee13103449cc6e41d4b0173d917cabea84bdb08"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(20_000_000),
      ),
      //A
      (
        hex!["542e6df326b7ea609ae7d1b3f30b5fbc9e473415d8094e236ebdbc0f5204cc25"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(80_000),
      ),
      (
        hex!["542e6df326b7ea609ae7d1b3f30b5fbc9e473415d8094e236ebdbc0f5204cc25"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(80_000),
      ),
      (
        hex!["542e6df326b7ea609ae7d1b3f30b5fbc9e473415d8094e236ebdbc0f5204cc25"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(480_000),
      ),
      //B
      (
        hex!["264bf1d4d2e4fe38912bce61c7fcdc35d83147fdb164bd761bcfd951265c9e47"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(95_000),
      ),
      (
        hex!["264bf1d4d2e4fe38912bce61c7fcdc35d83147fdb164bd761bcfd951265c9e47"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(95_000),
      ),
      (
        hex!["264bf1d4d2e4fe38912bce61c7fcdc35d83147fdb164bd761bcfd951265c9e47"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(570_000),
      ),
      //C
      (
        hex!["56d24b8bd664c8f986139c943764033ff87d75f070be15a5756fd8a1461d925a"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(552_000),
      ),
      (
        hex!["56d24b8bd664c8f986139c943764033ff87d75f070be15a5756fd8a1461d925a"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(552_000),
      ),
      (
        hex!["56d24b8bd664c8f986139c943764033ff87d75f070be15a5756fd8a1461d925a"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(3_312_000),
      ),
      //D
      (
        hex!["7c37ae52b566e906c12d692c6491f6bf53c7b8aac48a5508a19ead28cbb63a6b"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(80_000),
      ),
      (
        hex!["7c37ae52b566e906c12d692c6491f6bf53c7b8aac48a5508a19ead28cbb63a6b"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(80_000),
      ),
      (
        hex!["7c37ae52b566e906c12d692c6491f6bf53c7b8aac48a5508a19ead28cbb63a6b"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(480_000),
      ),
      //E
      (
        hex!["640bc73182d4aff458d28e15d6747d51aedd0be5c663c0cf09ac292727a4022b"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(89_095),
      ),
      (
        hex!["640bc73182d4aff458d28e15d6747d51aedd0be5c663c0cf09ac292727a4022b"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(89_095),
      ),
      (
        hex!["640bc73182d4aff458d28e15d6747d51aedd0be5c663c0cf09ac292727a4022b"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(534_570),
      ),
      //F
      (
        hex!["1073d449bf500cfdb1d6bb58bd3ed056ebcbf35023b7a749e35d7b69b8cb3229"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(77_440),
      ),
      (
        hex!["1073d449bf500cfdb1d6bb58bd3ed056ebcbf35023b7a749e35d7b69b8cb3229"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(77_440),
      ),
      (
        hex!["1073d449bf500cfdb1d6bb58bd3ed056ebcbf35023b7a749e35d7b69b8cb3229"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(464_640),
      ),
      //G
      (
        hex!["0e17ae7783c5eabf6a99ae2de893c51ea418645283a41aa888e9d52af00e5e44"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(136_400),
      ),
      (
        hex!["0e17ae7783c5eabf6a99ae2de893c51ea418645283a41aa888e9d52af00e5e44"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(136_400),
      ),
      (
        hex!["0e17ae7783c5eabf6a99ae2de893c51ea418645283a41aa888e9d52af00e5e44"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(818_400),
      ),
      //H
      (
        hex!["fed36467731f9f952aac5fe2f3d23601aea3057cf9a9e37baea4afd752928654"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(199_760),
      ),
      (
        hex!["fed36467731f9f952aac5fe2f3d23601aea3057cf9a9e37baea4afd752928654"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(199_760),
      ),
      (
        hex!["fed36467731f9f952aac5fe2f3d23601aea3057cf9a9e37baea4afd752928654"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(1_198_560),
      ),
      //I
      (
        hex!["9e0726eaac26eb8c8d7a9c75bd5f074464a3c974c16467d193769dacd7743c31"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(86_240),
      ),
      (
        hex!["9e0726eaac26eb8c8d7a9c75bd5f074464a3c974c16467d193769dacd7743c31"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(86_240),
      ),
      (
        hex!["9e0726eaac26eb8c8d7a9c75bd5f074464a3c974c16467d193769dacd7743c31"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(517_440),
      ),
      //J
      (
        hex!["d263a88080f51fb484804b9e2e744f0a81ab820a5255d666424c5a8aa5e82418"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(96_525),
      ),
      (
        hex!["d263a88080f51fb484804b9e2e744f0a81ab820a5255d666424c5a8aa5e82418"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(96_525),
      ),
      (
        hex!["d263a88080f51fb484804b9e2e744f0a81ab820a5255d666424c5a8aa5e82418"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(579_150),
      ),
      //K
      (
        hex!["1661a2075b7be54a942b52ba10e924469e3bdf8abbe9037d38f357ee78f86d79"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(45_150),
      ),
      (
        hex!["1661a2075b7be54a942b52ba10e924469e3bdf8abbe9037d38f357ee78f86d79"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(45_150),
      ),
      (
        hex!["1661a2075b7be54a942b52ba10e924469e3bdf8abbe9037d38f357ee78f86d79"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(270_900),
      ),
      //L
      (
        hex!["40a4ee2d7304c4ef7cf00732425c57a9d58e4202bf43472f2dcb162a748f606d"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(130_000),
      ),
      (
        hex!["40a4ee2d7304c4ef7cf00732425c57a9d58e4202bf43472f2dcb162a748f606d"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(130_000),
      ),
      (
        hex!["40a4ee2d7304c4ef7cf00732425c57a9d58e4202bf43472f2dcb162a748f606d"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(780_000),
      ),
      //M
      (
        hex!["5e7d7e4a8af71774f2928de7cd8b201fafff6bd6c96b5601d687da560cf00916"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(105_000),
      ),
      (
        hex!["5e7d7e4a8af71774f2928de7cd8b201fafff6bd6c96b5601d687da560cf00916"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(105_000),
      ),
      (
        hex!["5e7d7e4a8af71774f2928de7cd8b201fafff6bd6c96b5601d687da560cf00916"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(630_000),
      ),
      //N
      (
        hex!["d0fbfacb97883cb2556460da5e6308c11b836c216714255a03f28ab1106aff41"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(25_000),
      ),
      (
        hex!["d0fbfacb97883cb2556460da5e6308c11b836c216714255a03f28ab1106aff41"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(25_000),
      ),
      (
        hex!["d0fbfacb97883cb2556460da5e6308c11b836c216714255a03f28ab1106aff41"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(150_000),
      ),
      //O
      (
        hex!["1a6ccfd2da09c82181721b0d819a0bb4a7fd5fefccda52281d5ba6508fbb6c64"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(118_800),
      ),
      (
        hex!["1a6ccfd2da09c82181721b0d819a0bb4a7fd5fefccda52281d5ba6508fbb6c64"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(118_800),
      ),
      (
        hex!["1a6ccfd2da09c82181721b0d819a0bb4a7fd5fefccda52281d5ba6508fbb6c64"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(712_800),
      ),
      //P
      (
        hex!["a61f1f9724052d89026286f60158fb096d191bbb9d657332964ad20503effc31"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(88_500),
      ),
      (
        hex!["a61f1f9724052d89026286f60158fb096d191bbb9d657332964ad20503effc31"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(88_500),
      ),
      (
        hex!["a61f1f9724052d89026286f60158fb096d191bbb9d657332964ad20503effc31"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(531_000),
      ),
      //Q
      (
        hex!["ece719a6a53971d93350f7df82fad72be842ebc15fd6fda6a3c9a01f74819e74"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(72_000),
      ),
      (
        hex!["ece719a6a53971d93350f7df82fad72be842ebc15fd6fda6a3c9a01f74819e74"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(72_000),
      ),
      (
        hex!["ece719a6a53971d93350f7df82fad72be842ebc15fd6fda6a3c9a01f74819e74"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(432_000),
      ),
      //R
      (
        hex!["d683b0f3706345f4ff1bbb1fd9a54454c4edf050e20cb177d5e82b0945a55b3f"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(150_000),
      ),
      (
        hex!["d683b0f3706345f4ff1bbb1fd9a54454c4edf050e20cb177d5e82b0945a55b3f"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(150_000),
      ),
      (
        hex!["d683b0f3706345f4ff1bbb1fd9a54454c4edf050e20cb177d5e82b0945a55b3f"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(900_000),
      ),
      //S has no vesting terms
      //T
      (
        hex!["f49304cc009eb5eb37e762ff829db11b86b3f639048da93f638ea9fd65828a72"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(91_000),
      ),
      (
        hex!["f49304cc009eb5eb37e762ff829db11b86b3f639048da93f638ea9fd65828a72"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(91_000),
      ),
      (
        hex!["f49304cc009eb5eb37e762ff829db11b86b3f639048da93f638ea9fd65828a72"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(546_000),
      ),
      //U
      (
        hex!["7a6df5d223accd21b5672ada2ed68b790b5be0cafd84dc18d30f0b525122617e"].into(),
        STARTING_BLOCK,
        ONE_MONTH,
        1,
        assets::Asset::Tdfy.saturating_mul(65_000),
      ),
      (
        hex!["7a6df5d223accd21b5672ada2ed68b790b5be0cafd84dc18d30f0b525122617e"].into(),
        STARTING_BLOCK,
        SIX_MONTHS,
        1,
        assets::Asset::Tdfy.saturating_mul(65_000),
      ),
      (
        hex!["7a6df5d223accd21b5672ada2ed68b790b5be0cafd84dc18d30f0b525122617e"].into(),
        STARTING_BLOCK,
        ONE_YEAR,
        3,
        assets::Asset::Tdfy.saturating_mul(390_000),
      ),
    ]
  }
}
