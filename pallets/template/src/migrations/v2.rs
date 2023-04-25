//! Migrations to version [`2.0.0`], as denoted by the changelog.

use frame_support::{
	traits::{Get, StorageVersion},
	weights::Weight,
};
#[warn(unused_imports)]
use frame_support::sp_runtime::traits::Zero;

/// The old prefix.
pub const OLD_PREFIX: &[u8] = b"Template";

/// Some checks prior to migration. This can be linked to
/// [`frame_support::traits::OnRuntimeUpgrade::pre_upgrade`] for further testing.
///
/// Panics if anything goes wrong.
pub fn pre_migration<T: crate::Config, N: AsRef<str>>(new: N) {
	let new = new.as_ref();
	log::info!("pre-migration for the template pallet v2 = {}", new);

	// ensure storage version is 3.
	assert_eq!(StorageVersion::get::<crate::Pallet<T>>(), 1);
}

/// Migrate the entire storage of this pallet to a new prefix.
///
/// This new prefix must be the same as the one set in construct_runtime. For safety, use
/// `PalletInfo` to get it, as:
/// `<Runtime as frame_system::Config>::PalletInfo::name::<Template>`.
///
/// The old storage prefix, `Template` is hardcoded in the migration code.
pub fn migrate<T: crate::Config, N: AsRef<str>>(new_pallet_name: N) -> Weight {
	if new_pallet_name.as_ref().as_bytes() == OLD_PREFIX {
		log::info!(
			target: "runtime::template",
			"New pallet name is equal to the old prefix. No migration needs to be done.",
		);
		return Weight::zero()
	}
	let storage_version = StorageVersion::get::<crate::Pallet<T>>();
	log::info!(
		target: "runtime::template",
		"Running migration to v2 for template with storage version {:?}",
		storage_version,
	);

	if storage_version <= 1 {
		log::info!("new prefix: {}", new_pallet_name.as_ref());
		frame_support::storage::migration::move_pallet(
			OLD_PREFIX,
			new_pallet_name.as_ref().as_bytes(),
		);

		StorageVersion::new(2).put::<crate::Pallet<T>>();

		<T as frame_system::Config>::BlockWeights::get().max_block
	} else {
		log::warn!(
			target: "runtime::template",
			"Attempted to apply migration to v2 but failed because storage version is {:?}",
			storage_version,
		);
		Weight::zero()
	}
}


/// Some checks for after migration. This can be linked to
/// [`frame_support::traits::OnRuntimeUpgrade::post_upgrade`] for further testing.
///
/// Panics if anything goes wrong.
pub fn post_migration<T: crate::Config>() {
	log::info!("post-migration for the template pallet v2");
	// ensure we've been updated to v4 by the automatic write of crate version -> storage version.
	assert_eq!(StorageVersion::get::<crate::Pallet<T>>(), 2);
}
