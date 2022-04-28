// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
pub type Balance = u128;
/// The block number type used by Polkadot.
/// 32-bits will allow for 136 years of blocks assuming 1 block per second.
pub type BlockNumber = u32;

/// An instant or duration in time.
pub type Moment = u64;

pub const EXISTENTIAL_DEPOSIT: Balance = 1 * CENTS;

pub const UNITS: Balance = 1_000_000_000_000;
pub const CENTS: Balance = UNITS / 30_000;
pub const GRAND: Balance = CENTS * 100_000;
pub const MILLICENTS: Balance = CENTS / 1_000;
pub const DOLLARS: Balance = 100 * CENTS; // 0x0000_0000_0000_0000_0000_5af3_107a_4000u128

pub const fn deposit(items: u32, bytes: u32) -> Balance {
	items as Balance * 2_000 * CENTS + (bytes as Balance) * 100 * MILLICENTS
}

pub const MILLISECS_PER_BLOCK: Moment = 6000;
pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;
pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = 1 * HOURS;

// These time units are defined in number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;
pub const WEEKS: BlockNumber = DAYS * 7;

// 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

pub const XPUB_LEN: u32 = 166;
// #![cfg_attr(not(feature = "std"), no_std)]
// Money matters.
// pub mod currency {
// 	// use primitives::v0::Balance;

// 	/// The existential deposit.
// 	pub const EXISTENTIAL_DEPOSIT: Balance = 1 * CENTS;

// 	pub const UNITS: Balance = 1_000_000_000_000;
// 	pub const CENTS: Balance = UNITS / 30_000;
// 	pub const GRAND: Balance = CENTS * 100_000;
// 	pub const MILLICENTS: Balance = CENTS / 1_000;
// 	pub const DOLLARS: Balance = 100 * CENTS; // 0x0000_0000_0000_0000_0000_5af3_107a_4000u128

// 	pub const fn deposit(items: u32, bytes: u32) -> Balance {
// 		items as Balance * 2_000 * CENTS + (bytes as Balance) * 100 * MILLICENTS
// 	}
// }

// Time and blocks.
// pub mod time {
// 	// use primitives::v0::{BlockNumber, Moment};
// 	pub const MILLISECS_PER_BLOCK: Moment = 6000;
// 	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;
// 	pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = 1 * HOURS;

// 	// These time units are defined in number of blocks.
// 	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
// 	pub const HOURS: BlockNumber = MINUTES * 60;
// 	pub const DAYS: BlockNumber = HOURS * 24;
// 	pub const WEEKS: BlockNumber = DAYS * 7;

// 	// 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
// 	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
// }
