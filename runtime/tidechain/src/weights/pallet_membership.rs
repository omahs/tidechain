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

//! Autogenerated weights for `pallet_membership`
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

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// The range of component `m` is `[1, 99]`.
	fn add_member(m: u32, ) -> Weight {
		(21_630_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((75_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:1 w:0)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// The range of component `m` is `[2, 100]`.
	fn remove_member(m: u32, ) -> Weight {
		(25_503_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((78_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:1 w:0)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// The range of component `m` is `[2, 100]`.
	fn swap_member(m: u32, ) -> Weight {
		(25_570_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((90_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:1 w:0)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	fn reset_member(m: u32, ) -> Weight {
		(25_396_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((254_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	fn change_key(m: u32, ) -> Weight {
		(26_610_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((87_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: TechnicalMembership Prime (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	fn set_prime(m: u32, ) -> Weight {
		(6_663_000 as Weight)
			// Standard Error: 0
			.saturating_add((38_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: TechnicalMembership Prime (r:0 w:1)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	fn clear_prime(m: u32, ) -> Weight {
		(2_277_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
