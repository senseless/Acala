// This file is part of Acala.

// Copyright (C) 2020-2023 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_incentives
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-46-22`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/karura/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_incentives.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_incentives::WeightInfo for WeightInfo<T> {
	// Storage: `EmergencyShutdown::IsShutdown` (r:1 w:0)
	// Proof: `EmergencyShutdown::IsShutdown` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	// Storage: `Rewards::PoolInfos` (r:5 w:0)
	// Proof: `Rewards::PoolInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Incentives::IncentiveRewardAmounts` (r:8 w:0)
	// Proof: `Incentives::IncentiveRewardAmounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:0)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 4]`.
	fn on_initialize(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1254 + c * (198 ±0)`
		//  Estimated: `4747 + c * (5027 ±0)`
		// Minimum execution time: 9_774 nanoseconds.
		Weight::from_parts(13_822_109, 4747)
			// Standard Error: 131_552
			.saturating_add(Weight::from_parts(17_192_919, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 5027).saturating_mul(c.into()))
	}
	// Storage: `Tokens::Accounts` (r:2 w:2)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Rewards::PoolInfos` (r:1 w:1)
	// Proof: `Rewards::PoolInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Rewards::SharesAndWithdrawnRewards` (r:1 w:1)
	// Proof: `Rewards::SharesAndWithdrawnRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn deposit_dex_share() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1808`
		//  Estimated: `6234`
		// Minimum execution time: 69_777 nanoseconds.
		Weight::from_parts(70_933_000, 6234)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: `Rewards::SharesAndWithdrawnRewards` (r:1 w:1)
	// Proof: `Rewards::SharesAndWithdrawnRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:2 w:2)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:0)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Rewards::PoolInfos` (r:1 w:1)
	// Proof: `Rewards::PoolInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn withdraw_dex_share() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1848`
		//  Estimated: `6234`
		// Minimum execution time: 68_246 nanoseconds.
		Weight::from_parts(69_154_000, 6234)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: `Rewards::SharesAndWithdrawnRewards` (r:1 w:1)
	// Proof: `Rewards::SharesAndWithdrawnRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Rewards::PoolInfos` (r:1 w:1)
	// Proof: `Rewards::PoolInfos` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Incentives::PendingMultiRewards` (r:1 w:1)
	// Proof: `Incentives::PendingMultiRewards` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Incentives::ClaimRewardDeductionRates` (r:1 w:0)
	// Proof: `Incentives::ClaimRewardDeductionRates` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	fn claim_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1860`
		//  Estimated: `5325`
		// Minimum execution time: 97_474 nanoseconds.
		Weight::from_parts(99_456_000, 5325)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: `Incentives::IncentiveRewardAmounts` (r:4 w:4)
	// Proof: `Incentives::IncentiveRewardAmounts` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[0, 4]`.
	fn update_incentive_rewards(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `751 + c * (85 ±0)`
		//  Estimated: `1997 + c * (2475 ±0)`
		// Minimum execution time: 6_087 nanoseconds.
		Weight::from_parts(9_144_336, 1997)
			// Standard Error: 98_707
			.saturating_add(Weight::from_parts(8_623_945, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2475).saturating_mul(c.into()))
	}
	// Storage: `Incentives::ClaimRewardDeductionRates` (r:4 w:4)
	// Proof: `Incentives::ClaimRewardDeductionRates` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[0, 4]`.
	fn update_claim_reward_deduction_rates(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `676 + c * (10 ±0)`
		//  Estimated: `1698 + c * (2475 ±0)`
		// Minimum execution time: 5_892 nanoseconds.
		Weight::from_parts(6_706_753, 1698)
			// Standard Error: 18_072
			.saturating_add(Weight::from_parts(2_130_283, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2475).saturating_mul(c.into()))
	}
}
