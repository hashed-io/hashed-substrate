use crate::{mock::*, Error, types::{ApplicationFile, ApplicationStatus}};
use codec::Encode;
use frame_support::{assert_ok, BoundedVec, traits::{Len, ConstU32}};
use sp_io::hashing::blake2_256;

fn dummy_label() -> BoundedVec<u8, LabelMaxLen> {
	let s: Vec<u8> = "My marketplace".as_bytes().into();
	s.try_into().unwrap_or_default()
}

fn dummy_notes() -> BoundedVec<u8, NotesMaxLen> {
	let s: Vec<u8> = "Notes".as_bytes().into();
	s.try_into().unwrap_or_default()
}

fn create_file(name: &str, cid: &str) -> ApplicationFile<NameMaxLen> {
	let display_name_vec: Vec<u8> = name.as_bytes().into();
	let display_name: BoundedVec<u8, NameMaxLen> = display_name_vec.try_into().unwrap_or_default();
	let cid :BoundedVec<u8, ConstU32<100>> = cid.as_bytes().to_vec().try_into().unwrap_or_default(); 
	ApplicationFile{
		display_name,
		cid
	}
}

fn create_application_files( n_files: u32) -> BoundedVec<ApplicationFile<NameMaxLen>,MaxFiles> {
	let mut files = Vec::<ApplicationFile<NameMaxLen>>::default();
	for i in 0..n_files{
		let file_name = format!("file{}",i.to_string());
		let cid = format!("cid{}",i.to_string());
		files.push(create_file(file_name.as_str(), cid.as_str()));
	}
	BoundedVec::<ApplicationFile<NameMaxLen>,MaxFiles>::try_from( files).unwrap_or_default()
}

#[test]
fn create_marketplace_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),1, dummy_label() ));
		let m_id = dummy_label().using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some() );
		
	});
}

#[test]
fn apply_to_marketplace_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),1, dummy_label() ));
		let m_id = dummy_label().using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(2),m_id, dummy_notes(),create_application_files(2) ));

		assert!( GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Pending).len() ==1);
	});
}

#[test]
fn enroll_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),1, dummy_label() ));
	});
}
