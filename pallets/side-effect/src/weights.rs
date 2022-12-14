
//! Autogenerated weights for `pallet_side_effect`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-31, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `sangeet`, CPU: `AMD Ryzen 5 4500U with Radeon Graphics`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_side_effect
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// pallets/side-effect/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_side_effect`.
pub trait WeightInfo {
	#[rustfmt::skip]
	fn commit_side_effect() -> Weight;
	#[rustfmt::skip]
	fn revert_side_effect() -> Weight;
	
}

pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: SideEffects SideEffectStorage (r:1 w:1)
	fn commit_side_effect() -> Weight {
		Weight::from_ref_time(23_155_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: SideEffects SideEffectStorage (r:1 w:1)
	fn revert_side_effect() -> Weight {
		Weight::from_ref_time(22_874_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
