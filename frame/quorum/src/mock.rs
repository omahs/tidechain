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

use crate::pallet as pallet_quorum;
use frame_benchmarking::frame_support::traits::tokens::{DepositConsequence, WithdrawConsequence};
use frame_support::{
  parameter_types,
  traits::{
    fungible::{
      Inspect as FungibleInspect, InspectHold as FungibleInspectHold, Mutate as FungibleMutate,
      MutateHold as FungibleMutateHold, Transfer as FungibleTransfer,
    },
    fungibles::{Inspect, InspectHold, Mutate, MutateHold, Transfer},
    ConstU128, ConstU32, GenesisBuild,
  },
  PalletId,
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
  testing::Header,
  traits::{BlakeTwo256, IdentityLookup},
  DispatchError, DispatchResult, FixedU128,
};
use std::marker::PhantomData;
use system::EnsureRoot;
use tidefi_primitives::{BlockNumber, CurrencyId};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;
type AccountId = u64;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
  pub enum Test where
    Block = Block,
    NodeBlock = Block,
    UncheckedExtrinsic = UncheckedExtrinsic,
  {
    System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
    Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
    Assets: pallet_assets::{Pallet, Call, Storage, Event<T>},
    Quorum: pallet_quorum::{Pallet, Call, Config<T>, Storage, Event<T>},
    Security: pallet_security::{Pallet, Call, Config, Storage, Event<T>},
    Sunrise: pallet_sunrise::{Pallet, Config<T>, Storage, Event<T>},
    AssetRegistry: pallet_asset_registry::{Pallet, Call, Config<T>, Storage, Event<T>},
  }
);

parameter_types! {
  pub const BlockHashCount: u64 = 250;
  pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
  type BaseCallFilter = frame_support::traits::Everything;
  type BlockWeights = ();
  type BlockLength = ();
  type DbWeight = ();
  type Origin = Origin;
  type Call = Call;
  type Index = u64;
  type BlockNumber = u64;
  type Hash = H256;
  type Hashing = BlakeTwo256;
  type AccountId = AccountId;
  type Lookup = IdentityLookup<Self::AccountId>;
  type Header = Header;
  type Event = Event;
  type BlockHashCount = BlockHashCount;
  type Version = ();
  type PalletInfo = PalletInfo;
  type AccountData = pallet_balances::AccountData<Balance>;
  type OnNewAccount = ();
  type OnKilledAccount = ();
  type SystemWeightInfo = ();
  type SS58Prefix = SS58Prefix;
  type OnSetCode = ();
  type MaxConsumers = ConstU32<16>;
}
pub const TDFY: Balance = 1_000_000_000_000;
parameter_types! {
  pub const ExistentialDeposit: Balance = TDFY;
  pub const MaxLocks: u32 = 50;
  pub const MaxReserves: u32 = 50;
  pub const AssetDeposit: u64 = 1;
  pub const ApprovalDeposit: u64 = 1;
  pub const MetadataDepositBase: u64 = 1;
  pub const MetadataDepositPerByte: u64 = 1;

  // Maximum proposals in queue for the quorum, to limit the vector size and optimization
  pub const ProposalsCap: u32 = 1000;
  pub const BurnedCap: u32 = 1000;

  // The lifetime of a proposal by the quorum members
  pub const ProposalLifetime: u64 = 100;
  // The maximum size of a string
  pub const StringLimit: u32 = 255;
  // The number of votes maximum per proposal, should alway be higher than the proposals threshold
  pub const VotesLimit: u32 = 10;
  // The maximum number of account the watchlist can contains
  pub const WatchListLimit: u32 = 10000;
  // The maximum number of pubkey each asset can have, should alway be more more than the current quorum active member set
  pub const PubkeyLimitPerAsset: u32 = 10;
}

impl pallet_assets::Config for Test {
  type Event = Event;
  type Balance = u128;
  type AssetId = u32;
  type Currency = Balances;
  type ForceOrigin = EnsureRoot<Self::AccountId>;
  type AssetDeposit = AssetDeposit;
  type MetadataDepositBase = MetadataDepositBase;
  type MetadataDepositPerByte = MetadataDepositPerByte;
  type ApprovalDeposit = ApprovalDeposit;
  type StringLimit = StringLimit;
  type Freezer = ();
  type Extra = ();
  type WeightInfo = ();
  type AssetAccountDeposit = ConstU128<0>;
}

impl pallet_balances::Config for Test {
  type Balance = Balance;
  type DustRemoval = ();
  type Event = Event;
  type ExistentialDeposit = ExistentialDeposit;
  type AccountStore = frame_system::Pallet<Test>;
  type MaxLocks = MaxLocks;
  type MaxReserves = MaxReserves;
  type ReserveIdentifier = [u8; 8];
  type WeightInfo = ();
}

parameter_types! {
  pub const SunrisePalletId: PalletId = PalletId(*b"sunr*pal");
  pub const TidefiPalletId: PalletId = PalletId(*b"wrpr*pal");
  pub const AssetRegistryPalletId: PalletId = PalletId(*b"asst*pal");
  pub const Cooldown: BlockNumber = 1_296_000; // 90 DAYS
  // max 10k rewards
  pub const MaximumRewardPerSwap: Balance = 10_000_000_000_000_000;
  // 50%
  pub const LeftoverSwapRebates: FixedU128 = FixedU128::from_inner(500_000_000_000_000_000);
}

impl pallet_quorum::Config for Test {
  type Event = Event;
  type WeightInfo = crate::weights::SubstrateWeight<Test>;
  type QuorumPalletId = TidefiPalletId;
  type Security = Security;
  type Sunrise = Sunrise;
  type CurrencyTidefi = Adapter<AccountId>;
  type AssetRegistry = AssetRegistry;
  type ProposalsCap = ProposalsCap;
  type BurnedCap = BurnedCap;
  type ProposalLifetime = ProposalLifetime;
  type StringLimit = StringLimit;
  type VotesLimit = VotesLimit;
  type WatchListLimit = WatchListLimit;
  type PubkeyLimitPerAsset = PubkeyLimitPerAsset;
}

impl pallet_sunrise::Config for Test {
  type Event = Event;
  type Security = Security;
  type SunrisePalletId = SunrisePalletId;
  type CurrencyTidefi = Adapter<AccountId>;
  type Cooldown = Cooldown;
  type MaximumRewardPerSwap = MaximumRewardPerSwap;
  type LeftoverSwapRebates = LeftoverSwapRebates;
}

impl pallet_security::Config for Test {
  type Event = Event;
  type WeightInfo = pallet_security::weights::SubstrateWeight<Test>;
}

impl pallet_asset_registry::Config for Test {
  type Event = Event;
  type WeightInfo = pallet_asset_registry::weights::SubstrateWeight<Test>;
  type AssetRegistryPalletId = AssetRegistryPalletId;
  type CurrencyTidefi = Adapter<AccountId>;
}

// this is only the mock for benchmarking, it's implemented directly in the runtime
pub struct Adapter<AccountId> {
  phantom: PhantomData<AccountId>,
}

impl Inspect<AccountId> for Adapter<AccountId> {
  type AssetId = CurrencyId;
  type Balance = Balance;

  fn total_issuance(asset: Self::AssetId) -> Self::Balance {
    match asset {
      CurrencyId::Tdfy => Balances::total_issuance(),
      CurrencyId::Wrapped(asset_id) => Assets::total_issuance(asset_id),
    }
  }

  fn balance(asset: Self::AssetId, who: &AccountId) -> Self::Balance {
    match asset {
      CurrencyId::Tdfy => Balances::balance(who),
      CurrencyId::Wrapped(asset_id) => Assets::balance(asset_id, who),
    }
  }

  fn minimum_balance(asset: Self::AssetId) -> Self::Balance {
    match asset {
      CurrencyId::Tdfy => Balances::minimum_balance(),
      CurrencyId::Wrapped(asset_id) => Assets::minimum_balance(asset_id),
    }
  }

  fn reducible_balance(asset: Self::AssetId, who: &AccountId, keep_alive: bool) -> Self::Balance {
    match asset {
      CurrencyId::Tdfy => Balances::reducible_balance(who, keep_alive),
      CurrencyId::Wrapped(asset_id) => Assets::reducible_balance(asset_id, who, keep_alive),
    }
  }

  fn can_deposit(
    asset: Self::AssetId,
    who: &AccountId,
    amount: Self::Balance,
    mint: bool,
  ) -> DepositConsequence {
    match asset {
      CurrencyId::Tdfy => Balances::can_deposit(who, amount, mint),
      CurrencyId::Wrapped(asset_id) => Assets::can_deposit(asset_id, who, amount, mint),
    }
  }

  fn can_withdraw(
    asset: Self::AssetId,
    who: &AccountId,
    amount: Self::Balance,
  ) -> WithdrawConsequence<Self::Balance> {
    match asset {
      CurrencyId::Tdfy => Balances::can_withdraw(who, amount),
      CurrencyId::Wrapped(asset_id) => Assets::can_withdraw(asset_id, who, amount),
    }
  }
}

impl Mutate<AccountId> for Adapter<AccountId> {
  fn mint_into(asset: Self::AssetId, who: &AccountId, amount: Self::Balance) -> DispatchResult {
    match asset {
      CurrencyId::Tdfy => Balances::mint_into(who, amount),
      CurrencyId::Wrapped(asset_id) => Assets::mint_into(asset_id, who, amount),
    }
  }

  fn burn_from(
    asset: Self::AssetId,
    who: &AccountId,
    amount: Balance,
  ) -> Result<Balance, DispatchError> {
    match asset {
      CurrencyId::Tdfy => Balances::burn_from(who, amount),
      CurrencyId::Wrapped(asset_id) => Assets::burn_from(asset_id, who, amount),
    }
  }
}

impl InspectHold<AccountId> for Adapter<AccountId> {
  fn balance_on_hold(asset: Self::AssetId, who: &AccountId) -> Self::Balance {
    match asset {
      CurrencyId::Tdfy => Balances::balance_on_hold(who),
      CurrencyId::Wrapped(asset_id) => Assets::balance_on_hold(asset_id, who),
    }
  }
  fn can_hold(asset: Self::AssetId, who: &AccountId, amount: Self::Balance) -> bool {
    match asset {
      CurrencyId::Tdfy => Balances::can_hold(who, amount),
      CurrencyId::Wrapped(asset_id) => Assets::can_hold(asset_id, who, amount),
    }
  }
}

impl MutateHold<AccountId> for Adapter<AccountId> {
  fn hold(asset: CurrencyId, who: &AccountId, amount: Self::Balance) -> DispatchResult {
    match asset {
      CurrencyId::Tdfy => Balances::hold(who, amount),
      CurrencyId::Wrapped(asset_id) => Assets::hold(asset_id, who, amount),
    }
  }

  fn release(
    asset: CurrencyId,
    who: &AccountId,
    amount: Balance,
    best_effort: bool,
  ) -> Result<Balance, DispatchError> {
    match asset {
      CurrencyId::Tdfy => Balances::release(who, amount, best_effort),
      CurrencyId::Wrapped(asset_id) => Assets::release(asset_id, who, amount, best_effort),
    }
  }
  fn transfer_held(
    asset: CurrencyId,
    source: &AccountId,
    dest: &AccountId,
    amount: Balance,
    best_effort: bool,
    on_hold: bool,
  ) -> Result<Balance, DispatchError> {
    match asset {
      CurrencyId::Tdfy => Balances::transfer_held(source, dest, amount, best_effort, on_hold),
      CurrencyId::Wrapped(asset_id) => {
        Assets::transfer_held(asset_id, source, dest, amount, best_effort, on_hold)
      }
    }
  }
}

impl Transfer<AccountId> for Adapter<AccountId>
where
  Assets: Transfer<AccountId>,
{
  fn transfer(
    asset: Self::AssetId,
    source: &AccountId,
    dest: &AccountId,
    amount: Self::Balance,
    keep_alive: bool,
  ) -> Result<Balance, DispatchError> {
    match asset {
      CurrencyId::Tdfy => {
        <Balances as FungibleTransfer<AccountId>>::transfer(source, dest, amount, keep_alive)
      }
      CurrencyId::Wrapped(asset_id) => {
        <Assets as Transfer<AccountId>>::transfer(asset_id, source, dest, amount, keep_alive)
      }
    }
  }
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
  let mut storage = system::GenesisConfig::default()
    .build_storage::<Test>()
    .unwrap();
  pallet_balances::GenesisConfig::<Test>::default()
    .assimilate_storage(&mut storage)
    .unwrap();
  pallet_quorum::GenesisConfig::<Test> {
    enabled: false,
    members: vec![1],
    threshold: 1,
  }
  .assimilate_storage(&mut storage)
  .unwrap();

  let mut ext: sp_io::TestExternalities = storage.into();
  ext.execute_with(|| System::set_block_number(1));
  ext
}
