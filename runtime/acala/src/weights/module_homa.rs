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

//! Autogenerated weights for module_homa
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ip-172-31-47-187`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=acala-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_homa.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_homa::WeightInfo for WeightInfo<T> {
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::LastEraBumpedBlock` (r:1 w:0)
	// Proof: `Homa::LastEraBumpedBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::BumpEraFrequency` (r:1 w:0)
	// Proof: `Homa::BumpEraFrequency` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `828`
		//  Estimated: `2313`
		// Minimum execution time: 7_109 nanoseconds.
		Weight::from_parts(7_437_000, 2313)
			.saturating_add(T::DbWeight::get().reads(3))
	}
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::LastEraBumpedBlock` (r:1 w:1)
	// Proof: `Homa::LastEraBumpedBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::BumpEraFrequency` (r:1 w:0)
	// Proof: `Homa::BumpEraFrequency` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::RelayChainCurrentEra` (r:1 w:1)
	// Proof: `Homa::RelayChainCurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::EstimatedRewardRatePerEra` (r:1 w:0)
	// Proof: `Homa::EstimatedRewardRatePerEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::StakingLedgers` (r:2 w:1)
	// Proof: `Homa::StakingLedgers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::TotalStakingBonded` (r:1 w:1)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::CommissionRate` (r:1 w:0)
	// Proof: `Homa::CommissionRate` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::TotalIssuance` (r:2 w:2)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:3 w:3)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `XcmInterface::XcmDestWeightAndFee` (r:4 w:0)
	// Proof: `XcmInterface::XcmDestWeightAndFee` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::UnclaimedRedemption` (r:1 w:1)
	// Proof: `Homa::UnclaimedRedemption` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::ToBondPool` (r:1 w:1)
	// Proof: `Homa::ToBondPool` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::SoftBondedCapPerSubAccount` (r:1 w:0)
	// Proof: `Homa::SoftBondedCapPerSubAccount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `EVM::XcmOrigin` (r:1 w:1)
	// Proof: `EVM::XcmOrigin` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `UnknownTokens::ConcreteFungibleBalances` (r:1 w:0)
	// Proof: `UnknownTokens::ConcreteFungibleBalances` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::RedeemRequests` (r:2 w:1)
	// Proof: `Homa::RedeemRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::Unbondings` (r:1 w:1)
	// Proof: `Homa::Unbondings` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::TotalVoidLiquid` (r:0 w:1)
	// Proof: `Homa::TotalVoidLiquid` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_initialize_with_bump_era() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2966`
		//  Estimated: `13856`
		// Minimum execution time: 343_951 nanoseconds.
		Weight::from_parts(355_942_000, 13856)
			.saturating_add(T::DbWeight::get().reads(34))
			.saturating_add(T::DbWeight::get().writes(19))
	}
	// Storage: `Homa::TotalStakingBonded` (r:1 w:0)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::ToBondPool` (r:1 w:1)
	// Proof: `Homa::ToBondPool` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::SoftBondedCapPerSubAccount` (r:1 w:0)
	// Proof: `Homa::SoftBondedCapPerSubAccount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:3 w:3)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalVoidLiquid` (r:1 w:1)
	// Proof: `Homa::TotalVoidLiquid` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::EstimatedRewardRatePerEra` (r:1 w:0)
	// Proof: `Homa::EstimatedRewardRatePerEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1706`
		//  Estimated: `8856`
		// Minimum execution time: 82_662 nanoseconds.
		Weight::from_parts(85_326_000, 8856)
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: `Homa::RedeemRequests` (r:1 w:1)
	// Proof: `Homa::RedeemRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:2 w:2)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn request_redeem() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1615`
		//  Estimated: `6234`
		// Minimum execution time: 53_949 nanoseconds.
		Weight::from_parts(55_033_000, 6234)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: `Homa::RedeemRequests` (r:50 w:50)
	// Proof: `Homa::RedeemRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::ToBondPool` (r:1 w:1)
	// Proof: `Homa::ToBondPool` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::TotalStakingBonded` (r:1 w:0)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	// Storage: `Homa::TotalVoidLiquid` (r:1 w:0)
	// Proof: `Homa::TotalVoidLiquid` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::FastMatchFeeRate` (r:1 w:0)
	// Proof: `Homa::FastMatchFeeRate` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:52 w:52)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:51 w:51)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 50]`.
	fn fast_match_redeems(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1955 + n * (295 ±0)`
		//  Estimated: `6234 + n * (2770 ±0)`
		// Minimum execution time: 94_862 nanoseconds.
		Weight::from_parts(33_414_349, 6234)
			// Standard Error: 27_635
			.saturating_add(Weight::from_parts(55_154_284, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2770).saturating_mul(n.into()))
	}
	// Storage: `Homa::RelayChainCurrentEra` (r:1 w:0)
	// Proof: `Homa::RelayChainCurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::Unbondings` (r:2 w:1)
	// Proof: `Homa::Unbondings` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::UnclaimedRedemption` (r:1 w:1)
	// Proof: `Homa::UnclaimedRedemption` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:2 w:2)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `EvmAccounts::EvmAddresses` (r:1 w:0)
	// Proof: `EvmAccounts::EvmAddresses` (`max_values`: None, `max_size`: Some(60), added: 2535, mode: `MaxEncodedLen`)
	fn claim_redemption() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1825`
		//  Estimated: `7765`
		// Minimum execution time: 77_054 nanoseconds.
		Weight::from_parts(78_713_000, 7765)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: `Homa::EstimatedRewardRatePerEra` (r:1 w:1)
	// Proof: `Homa::EstimatedRewardRatePerEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::CommissionRate` (r:1 w:1)
	// Proof: `Homa::CommissionRate` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::FastMatchFeeRate` (r:1 w:1)
	// Proof: `Homa::FastMatchFeeRate` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::SoftBondedCapPerSubAccount` (r:0 w:1)
	// Proof: `Homa::SoftBondedCapPerSubAccount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn update_homa_params() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1144`
		//  Estimated: `2629`
		// Minimum execution time: 33_669 nanoseconds.
		Weight::from_parts(34_293_000, 2629)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::LastEraBumpedBlock` (r:0 w:1)
	// Proof: `Homa::LastEraBumpedBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Homa::BumpEraFrequency` (r:0 w:1)
	// Proof: `Homa::BumpEraFrequency` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn update_bump_era_params() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1211`
		//  Estimated: `2696`
		// Minimum execution time: 26_269 nanoseconds.
		Weight::from_parts(27_067_000, 2696)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: `Homa::StakingLedgers` (r:10 w:10)
	// Proof: `Homa::StakingLedgers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Homa::TotalStakingBonded` (r:1 w:1)
	// Proof: `Homa::TotalStakingBonded` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `n` is `[0, 10]`.
	fn reset_ledgers(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1144`
		//  Estimated: `2629 + n * (2475 ±0)`
		// Minimum execution time: 5_839 nanoseconds.
		Weight::from_parts(13_190_648, 2629)
			// Standard Error: 43_123
			.saturating_add(Weight::from_parts(9_777_272, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2475).saturating_mul(n.into()))
	}
	// Storage: `Homa::RelayChainCurrentEra` (r:1 w:1)
	// Proof: `Homa::RelayChainCurrentEra` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn reset_current_era() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1144`
		//  Estimated: `2629`
		// Minimum execution time: 18_810 nanoseconds.
		Weight::from_parts(19_427_000, 2629)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
