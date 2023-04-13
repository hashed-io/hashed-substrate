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
	// pub fn string_to_short_s

	pub fn do_initial_setup(creator: T::AccountId) -> DispatchResult {
		// let marketplace = Marketplace {
		// 	label: BoundedVec<u8, T::LabelMaxLen>::try_from(b"afloat".to_vec()).unwrap(),
		// 	buy_fee: Permill::from_percent(5),
		// 	sell_fee: Permill::from_percent(5),
		// 	creator,
		// };

		// let marketplace = Marketp
		Ok(())
	}
}
