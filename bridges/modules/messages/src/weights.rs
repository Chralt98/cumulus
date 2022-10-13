// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_bridge_messages`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-20, STEPS: 50, REPEAT: 20
//! LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled
//! CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/millau-bridge-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_bridge_messages
// --extrinsic=*
// --execution=wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --output=./modules/messages/src/weights.rs
// --template=./.maintain/millau-weight-template.hbs

#![allow(clippy::all)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for `pallet_bridge_messages`.
pub trait WeightInfo {
	fn send_minimal_message_worst_case() -> Weight;
	fn send_1_kb_message_worst_case() -> Weight;
	fn send_16_kb_message_worst_case() -> Weight;
	fn maximal_increase_message_fee() -> Weight;
	fn increase_message_fee(i: u32) -> Weight;
	fn receive_single_message_proof() -> Weight;
	fn receive_two_messages_proof() -> Weight;
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight;
	fn receive_single_message_proof_1_kb() -> Weight;
	fn receive_single_message_proof_16_kb() -> Weight;
	fn receive_single_prepaid_message_proof() -> Weight;
	fn receive_delivery_proof_for_single_message() -> Weight;
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight;
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight;
}

/// Weights for `pallet_bridge_messages` using the Millau node and recommended hardware.
pub struct MillauWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for MillauWeight<T> {
	fn send_minimal_message_worst_case() -> Weight {
		(38_822_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
	fn send_1_kb_message_worst_case() -> Weight {
		(39_799_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
	fn send_16_kb_message_worst_case() -> Weight {
		(47_772_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(10 as Weight))
	}
	fn maximal_increase_message_fee() -> Weight {
		(3_081_804_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn increase_message_fee(i: u32) -> Weight {
		(0 as Weight)
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn receive_single_message_proof() -> Weight {
		(26_523_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn receive_two_messages_proof() -> Weight {
		(39_278_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		(32_416_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn receive_single_message_proof_1_kb() -> Weight {
		(27_078_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn receive_single_message_proof_16_kb() -> Weight {
		(78_235_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn receive_single_prepaid_message_proof() -> Weight {
		(27_635_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn receive_delivery_proof_for_single_message() -> Weight {
		(34_576_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		(37_318_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		(41_245_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn send_minimal_message_worst_case() -> Weight {
		(38_822_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(10 as Weight))
	}
	fn send_1_kb_message_worst_case() -> Weight {
		(39_799_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(10 as Weight))
	}
	fn send_16_kb_message_worst_case() -> Weight {
		(47_772_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(10 as Weight))
	}
	fn maximal_increase_message_fee() -> Weight {
		(3_081_804_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn increase_message_fee(i: u32) -> Weight {
		(0 as Weight)
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn receive_single_message_proof() -> Weight {
		(26_523_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn receive_two_messages_proof() -> Weight {
		(39_278_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn receive_single_message_proof_with_outbound_lane_state() -> Weight {
		(32_416_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn receive_single_message_proof_1_kb() -> Weight {
		(27_078_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn receive_single_message_proof_16_kb() -> Weight {
		(78_235_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn receive_single_prepaid_message_proof() -> Weight {
		(27_635_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn receive_delivery_proof_for_single_message() -> Weight {
		(34_576_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn receive_delivery_proof_for_two_messages_by_single_relayer() -> Weight {
		(37_318_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn receive_delivery_proof_for_two_messages_by_two_relayers() -> Weight {
		(41_245_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
}