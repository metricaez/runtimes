
//! Autogenerated weights for `pallet_bridge_messages`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `svyatonik-benchmarking`, CPU: `Intel(R) Xeon(R) CPU @ 2.80GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("bh-kusama-local-raw.json")`, DB CACHE: 1024

// Executed Command:
// ../polkadot-sdk/target/production/polkadot-parachain-benchmarks
// benchmark
// pallet
// --chain
// bh-kusama-local-raw.json
// --pallet
// pallet-bridge-messages
// --extrinsic
// *
// --output=system-parachains/bridge-hubs/bridge-hub-kusama/src/weights
// --no-median-slopes
// --no-min-squares

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_bridge_messages`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_messages::WeightInfo for WeightInfo<T> {
	/// Storage: `BridgePolkadotMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgePolkadotParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgePolkadotParachains::ImportedParaHeads` (`max_values`: Some(600), `max_size`: Some(196), added: 1681, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_single_message_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `470`
		//  Estimated: `52645`
		// Minimum execution time: 73_258_000 picoseconds.
		Weight::from_parts(75_198_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgePolkadotMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgePolkadotParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgePolkadotParachains::ImportedParaHeads` (`max_values`: Some(600), `max_size`: Some(196), added: 1681, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_two_messages_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `470`
		//  Estimated: `52645`
		// Minimum execution time: 92_110_000 picoseconds.
		Weight::from_parts(94_813_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgePolkadotMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgePolkadotParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgePolkadotParachains::ImportedParaHeads` (`max_values`: Some(600), `max_size`: Some(196), added: 1681, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `470`
		//  Estimated: `52645`
		// Minimum execution time: 81_696_000 picoseconds.
		Weight::from_parts(84_047_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgePolkadotMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgePolkadotParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgePolkadotParachains::ImportedParaHeads` (`max_values`: Some(600), `max_size`: Some(196), added: 1681, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	fn receive_single_message_proof_1_kb() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `438`
		//  Estimated: `52645`
		// Minimum execution time: 71_530_000 picoseconds.
		Weight::from_parts(73_127_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgePolkadotMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:0)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgePolkadotParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgePolkadotParachains::ImportedParaHeads` (`max_values`: Some(600), `max_size`: Some(196), added: 1681, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	fn receive_single_message_proof_16_kb() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `438`
		//  Estimated: `52645`
		// Minimum execution time: 112_482_000 picoseconds.
		Weight::from_parts(115_496_000, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `BridgePolkadotMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgePolkadotParachains::ImportedParaHeads` (`max_values`: Some(600), `max_size`: Some(196), added: 1681, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotMessages::OutboundLanes` (`max_values`: Some(1), `max_size`: Some(44), added: 539, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:1 w:1)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_single_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `3808`
		// Minimum execution time: 40_730_000 picoseconds.
		Weight::from_parts(41_879_000, 0)
			.saturating_add(Weight::from_parts(0, 3808))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `BridgePolkadotMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgePolkadotParachains::ImportedParaHeads` (`max_values`: Some(600), `max_size`: Some(196), added: 1681, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotMessages::OutboundLanes` (`max_values`: Some(1), `max_size`: Some(44), added: 539, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:1 w:1)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `3808`
		// Minimum execution time: 40_765_000 picoseconds.
		Weight::from_parts(41_895_000, 0)
			.saturating_add(Weight::from_parts(0, 3808))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `BridgePolkadotMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgePolkadotParachains::ImportedParaHeads` (`max_values`: Some(600), `max_size`: Some(196), added: 1681, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotMessages::OutboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotMessages::OutboundLanes` (`max_values`: Some(1), `max_size`: Some(44), added: 539, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x6e0a18b62a1de81c5f519181cc611e18` (r:1 w:0)
	/// Storage: `BridgeRelayers::RelayerRewards` (r:2 w:2)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `6086`
		// Minimum execution time: 44_120_000 picoseconds.
		Weight::from_parts(44_844_000, 0)
			.saturating_add(Weight::from_parts(0, 6086))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `BridgePolkadotMessages::PalletOperatingMode` (r:1 w:0)
	/// Proof: `BridgePolkadotMessages::PalletOperatingMode` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::OutboundXcmpStatus` (r:1 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpStatus` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `BridgePolkadotParachains::ImportedParaHeads` (r:1 w:0)
	/// Proof: `BridgePolkadotParachains::ImportedParaHeads` (`max_values`: Some(600), `max_size`: Some(196), added: 1681, mode: `MaxEncodedLen`)
	/// Storage: `BridgePolkadotMessages::InboundLanes` (r:1 w:1)
	/// Proof: `BridgePolkadotMessages::InboundLanes` (`max_values`: None, `max_size`: Some(49180), added: 51655, mode: `MaxEncodedLen`)
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `XcmpQueue::DeliveryFeeFactor` (r:1 w:0)
	/// Proof: `XcmpQueue::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::RelevantMessagingState` (r:1 w:0)
	/// Proof: `ParachainSystem::RelevantMessagingState` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmpQueue::OutboundXcmpMessages` (r:0 w:1)
	/// Proof: `XcmpQueue::OutboundXcmpMessages` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `i` is `[128, 2048]`.
	fn receive_single_message_proof_with_dispatch(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `601`
		//  Estimated: `52645`
		// Minimum execution time: 86_315_000 picoseconds.
		Weight::from_parts(87_602_840, 0)
			.saturating_add(Weight::from_parts(0, 52645))
			// Standard Error: 123
			.saturating_add(Weight::from_parts(7_611, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
