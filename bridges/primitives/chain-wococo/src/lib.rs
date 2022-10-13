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

#![cfg_attr(not(feature = "std"), no_std)]
// RuntimeApi generated functions
#![allow(clippy::too_many_arguments)]

use bp_messages::{
	InboundMessageDetails, LaneId, MessageNonce, MessagePayload, OutboundMessageDetails,
};
use sp_runtime::FixedU128;
use sp_std::prelude::*;

pub use bp_polkadot_core::*;
// Rococo runtime = Wococo runtime
pub use bp_rococo::{
	SS58Prefix, WeightToFee, EXISTENTIAL_DEPOSIT, PARAS_PALLET_NAME,
	PAY_INBOUND_DISPATCH_FEE_WEIGHT, VERSION,
};
use bp_runtime::decl_bridge_runtime_apis;

/// Wococo Chain
pub type Wococo = PolkadotLike;

/// The target length of a session (how often authorities change) on Wococo measured in of number
/// of blocks.
///
/// Note that since this is a target sessions may change before/after this time depending on network
/// conditions.
pub const SESSION_LENGTH: BlockNumber = time_units::MINUTES;

/// Name of the With-Wococo GRANDPA pallet instance that is deployed at bridged chains.
pub const WITH_WOCOCO_GRANDPA_PALLET_NAME: &str = "BridgeWococoGrandpa";
/// Name of the With-Wococo messages pallet instance that is deployed at bridged chains.
pub const WITH_WOCOCO_MESSAGES_PALLET_NAME: &str = "BridgeWococoMessages";

decl_bridge_runtime_apis!(wococo);