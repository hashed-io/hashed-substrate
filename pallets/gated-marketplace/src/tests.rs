use crate::{mock::*, Error, types::*};
use codec::Encode;
use frame_support::{assert_ok, BoundedVec, traits::{Len, ConstU32}, assert_noop, assert_err};
use sp_io::hashing::blake2_256;

fn create_label( label : &str ) -> BoundedVec<u8, LabelMaxLen> {
	let s: Vec<u8> = label.as_bytes().into();
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
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some() );
		
	});
}

#[test]
fn duplicate_marketplaces_shouldnt_work() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		assert_noop!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ), Error::<Test>::MarketplaceAlreadyExists);
	});
}

#[test]
fn exceeding_max_markets_per_auth_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		assert_noop!(GatedMarketplace::create_marketplace(Origin::signed(3),3, create_label("my marketplace 2")), Error::<Test>::ExceedMaxRolesPerAuth );

		// TODO: test ExceedMaxMarketsPerAuth when its possible to add new authorities
	});
}

#[test]
fn apply_to_marketplace_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, dummy_notes(),create_application_files(2) ));

		assert!( GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Pending).len() ==1);
	});
}

#[test]
fn apply_to_nonexistent_marketplace_shouldnt_work() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		// No such marletplace exists:
		let m_id = create_label("false marketplace").using_encoded(blake2_256);
		assert_noop!(GatedMarketplace::apply(Origin::signed(3),m_id, dummy_notes(),create_application_files(2) ), Error::<Test>::MarketplaceNotFound);
	});
}

#[test]
fn apply_twice_shouldnt_work() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, dummy_notes(),create_application_files(2) ));
		assert_noop!(GatedMarketplace::apply(Origin::signed(3),m_id, dummy_notes(),create_application_files(2) ), Error::<Test>::AlreadyApplied );
	});
}

#[test]
fn exceeding_max_applicants_shouldnt_work() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, dummy_notes(),create_application_files(2) ));
		assert_ok!(GatedMarketplace::apply(Origin::signed(4),m_id, dummy_notes(),create_application_files(3) ));
		assert_noop!(GatedMarketplace::apply(Origin::signed(5),m_id, dummy_notes(),create_application_files(1) ), Error::<Test>::ExceedMaxApplicants );
	});
}

#[test]
fn enroll_works() {
	new_test_ext().execute_with(|| {

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, dummy_notes(),create_application_files(2) ));
		assert_ok!(GatedMarketplace::apply(Origin::signed(4),m_id, dummy_notes(),create_application_files(1) ));
		let app_id = Application::<Test>{
			status : ApplicationStatus::Pending ,
			notes : dummy_notes(),
			files: create_application_files(1),
		}.using_encoded(blake2_256);
		// enroll with account
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Account(3), true));
		// enroll with application
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application(app_id), true));
	});
}

#[test]
fn enroll_reject_work() {
	new_test_ext().execute_with(|| {

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, dummy_notes(),create_application_files(2) ));
		assert_ok!(GatedMarketplace::apply(Origin::signed(4),m_id, dummy_notes(),create_application_files(1) ));
		let app_id = Application::<Test>{
			status : ApplicationStatus::Pending ,
			notes : dummy_notes(),
			files: create_application_files(1),
		}.using_encoded(blake2_256);
		// reject with account
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Account(3), false));
		// reject with application
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application(app_id), false));
	});
}

#[test]
fn change_enroll_status_work() {
	new_test_ext().execute_with(|| {

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(4),m_id, dummy_notes(),create_application_files(1) ));
		let app_id = Application::<Test>{
			status : ApplicationStatus::Pending ,
			notes : dummy_notes(),
			files: create_application_files(1),
		}.using_encoded(blake2_256);
		// reject an account
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Account(4), false));
		// and then change it to "accepted"
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application(app_id), true));
	});
}

#[test]
fn non_authorized_user_enroll_shouldnt_work() {
	new_test_ext().execute_with(|| {

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, dummy_notes(),create_application_files(2) ));

		// external user tries to enroll someone
		assert_noop!(GatedMarketplace::enroll(Origin::signed(4), m_id , AccountOrApplication::Account(3), true), Error::<Test>::CannotEnroll);
	});
}

#[test]
fn enroll_nonexistent_application_shouldnt_work() {
	new_test_ext().execute_with(|| {

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		// accept nonexisten application throws error (account version)
		assert_noop!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Account(3), true), Error::<Test>::ApplicationNotFound);
		// accept nonexisten application throws error (application id version)
		assert_noop!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application([0;32]), true), Error::<Test>::ApplicationNotFound);
	});
}