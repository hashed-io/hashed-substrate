//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{
	account, benchmarks_instance_pallet, whitelist_account, whitelisted_caller,
};

use frame_support::{
	dispatch::UnfilteredDispatchable,
	traits::{EnsureOrigin, Get},
	BoundedVec,
};

use frame_system::RawOrigin as SystemOrigin;
use sp_runtime::traits::Bounded;
use sp_std::prelude::*;

use crate::Pallet as Fruniques;

fn dummy_description() -> BoundedVec<u8, StringLimit> {
	BoundedVec::<u8, StringLimit>::try_from(b"dummy description".to_vec()).unwrap()
}

fn dummy_attributes() -> Vec<(BoundedVec<u8, KeyLimit>, BoundedVec<u8, ValueLimit>)> {
	vec![(
		BoundedVec::<u8, KeyLimit>::try_from(b"dummy key".encode())
			.expect("Error on encoding key to BoundedVec"),
		BoundedVec::<u8, ValueLimit>::try_from(b"dummy value".encode())
			.expect("Error on encoding value to BoundedVec"),
	)]
}

fn dummy_empty_attributes() -> Vec<(BoundedVec<u8, KeyLimit>, BoundedVec<u8, ValueLimit>)> {
	vec![]
}

fn create_collection<T: Config>(
) -> (T::AccountId, AccountIdLookupOf<T>, T::CollectionDescription) {
	let caller: T::AccountId = whitelisted_caller();
	let caller_lookup = T::Lookup::unlookup(caller.clone());
	let collection_description = dummy_description();
	assert!(Fruniques::<T>::create_collection(
		SystemOrigin::Signed(caller.clone()).into(),
		collection_description.clone(),
	)
	.is_ok());
	(caller, caller_lookup, collection_description)
}

fn spawn<T: Config>(
) -> (T::AccountId, AccountIdLookupOf<T>, T::CollectionDescription, Attributes<T>, Some(HierarchicalInfo)) {
	let caller: T::AccountId = whitelisted_caller();
	let caller_lookup = T::Lookup::unlookup(caller.clone());
	let collection_description = dummy_description();
	let attributes = dummy_attributes();
}

benchmarks! {
	do_something {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
