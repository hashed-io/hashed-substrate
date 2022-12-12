//! Migrations to version [`2.0.0`], as denoted by the changelog.

use crate::{Config, Pallet};
use codec::{Decode, Encode, FullCodec};
use frame_support::{
	pallet_prelude::ValueQuery, traits::StorageVersion, weights::Weight, RuntimeDebug, Twox64Concat,
};
use sp_std::prelude::*;



/// Trait to implement to give information about types used for migration
pub trait V1ToV2 {
	/// System config account id
	type AccountId: 'static + FullCodec;

	/// Elections-phragmen currency balance.
	type Balance: 'static + FullCodec + Copy;
}

#[frame_support::storage_alias]
type MyBytesVal<T: Config> = MyBytesVal<
	Pallet<T>,
	ValueQuery,
>;

