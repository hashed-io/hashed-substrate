use super::*;
// use crate::types::*;
use frame_support::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::offchain::{http, Duration};
use sp_runtime::sp_std::vec::Vec;

impl<T: Config> Pallet<T> {
	pub fn bytes_to_u32(input: Vec<u8>) -> u32 {
		u32::from_ne_bytes(input.try_into().unwrap())
	}

}
