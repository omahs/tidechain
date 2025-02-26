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

//! Autogenerated weights for `pallet_tidefi`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-12, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ns3206536`, CPU: `Intel(R) Xeon(R) D-2141I CPU @ 2.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("tidechain-dev"), DB CACHE: 1024

// Executed Command:
// target/production/tidechain
// benchmark
// pallet
// --chain=tidechain-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./FILE_TEMPLATE
// --output=./runtime/tidechain/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_tidefi`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tidefi::WeightInfo for WeightInfo<T> {
	// Storage: Security ChainStatus (r:1 w:0)
	// Storage: Quorum QuorumStatus (r:1 w:0)
	// Storage: Assets Asset (r:1 w:0)
	// Storage: Assets Account (r:1 w:0)
	// Storage: Security Nonce (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: Security CurrentBlockCount (r:1 w:0)
	// Storage: Quorum Proposals (r:1 w:1)
	fn withdrawal() -> Weight {
		(49_309_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Security ChainStatus (r:1 w:0)
	// Storage: Oracle OracleStatus (r:1 w:0)
	// Storage: Assets Asset (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: System ExtrinsicData (r:1 w:0)
	// Storage: Oracle MarketMakers (r:1 w:0)
	// Storage: Security Nonce (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: Oracle AccountSwaps (r:1 w:1)
	// Storage: Oracle CounterForAccountSwaps (r:1 w:1)
	// Storage: Oracle Swaps (r:0 w:1)
	fn swap() -> Weight {
		(80_239_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn transfer() -> Weight {
		(16_273_000 as Weight)
	}
	// Storage: Fees ActiveEra (r:1 w:0)
	// Storage: Security CurrentBlockCount (r:1 w:0)
	// Storage: Sunrise Rewards (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn claim_sunrise_rewards() -> Weight {
		(64_182_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
