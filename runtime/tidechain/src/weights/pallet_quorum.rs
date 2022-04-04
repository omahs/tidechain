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

//! Autogenerated weights for `pallet_quorum`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-17, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("lagoon-dev"), DB CACHE: 128

// Executed Command:
// target/release/tidechain
// benchmark
// --chain=lagoon-dev
// --steps=50
// --repeat=20
// --pallet=pallet_quorum
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./FILE_TEMPLATE
// --output=./runtime/lagoon/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_quorum`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_quorum::WeightInfo for WeightInfo<T> {
	// Storage: Quorum Members (r:1 w:0)
	// Storage: Quorum PublicKeys (r:2 w:0)
	// Storage: Security CurrentBlockCount (r:1 w:0)
	// Storage: Security Nonce (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: Quorum Proposals (r:1 w:1)
	fn submit_proposal() -> Weight {
		(54_697_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Quorum Members (r:1 w:0)
	// Storage: Quorum PublicKeys (r:2 w:0)
	// Storage: Security CurrentBlockCount (r:1 w:0)
	// Storage: Quorum Votes (r:1 w:1)
	// Storage: Quorum CounterForVotes (r:1 w:1)
	// Storage: Quorum Threshold (r:1 w:0)
	// Storage: Quorum CounterForMembers (r:1 w:0)
	// Storage: Quorum Proposals (r:1 w:1)
	fn acknowledge_proposal() -> Weight {
		(123_195_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Quorum Members (r:1 w:0)
	// Storage: Quorum PublicKeys (r:2 w:0)
	// Storage: Security CurrentBlockCount (r:1 w:0)
	// Storage: Quorum Votes (r:1 w:1)
	// Storage: Quorum CounterForVotes (r:1 w:1)
	// Storage: Quorum Threshold (r:1 w:0)
	// Storage: Quorum CounterForMembers (r:1 w:0)
	// Storage: Quorum Proposals (r:1 w:1)
	fn reject_proposal() -> Weight {
		(74_553_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Quorum Votes (r:1 w:1)
	// Storage: Security CurrentBlockCount (r:1 w:0)
	// Storage: Quorum Threshold (r:1 w:0)
	// Storage: Quorum CounterForMembers (r:1 w:0)
	fn eval_proposal_state() -> Weight {
		(20_204_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Quorum Members (r:1 w:0)
	// Storage: Quorum PublicKeys (r:0 w:1)
	fn submit_public_keys(k: u32, ) -> Weight {
		(13_828_000 as Weight)
			// Standard Error: 11_000
			.saturating_add((2_888_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(k as Weight)))
	}

	fn acknowledge_burned() -> Weight {
      60_000_300_u64
      .saturating_add(T::DbWeight::get().reads(6_u64))
      .saturating_add(T::DbWeight::get().writes(5_u64))
   }
}
