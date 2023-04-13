use super::*;

use crate::types::*;
use frame_support::sp_io::hashing::blake2_256;
use frame_support::traits::tokens::nonfungibles::Inspect;
use frame_system::pallet_prelude::*;
use scale_info::prelude::string::String;

use pallet_gated_marketplace::types::Marketplace;
use pallet_rbac::types::*;

use frame_support::pallet_prelude::*;
use frame_support::traits::EnsureOriginWithArg;
use frame_support::PalletId;
// use frame_support::traits::OriginTrait;
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::{sp_std::vec::Vec, Permill};

impl<T: Config> Pallet<T> {
	pub fn do_initial_setup(creator: T::AccountId) -> DispatchResult {
		Ok(())
	}
}
