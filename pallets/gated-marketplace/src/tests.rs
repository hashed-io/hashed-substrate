use crate::{mock::*, Error, types::*, Custodians};
use std::vec;
use sp_runtime::sp_std::vec::Vec;
use codec::Encode;
use frame_support::{assert_ok, BoundedVec, traits::{Len, ConstU32}, assert_noop};
use sp_io::hashing::blake2_256;

fn create_label( label : &str ) -> BoundedVec<u8, LabelMaxLen> {
	let s: Vec<u8> = label.as_bytes().into();
	s.try_into().unwrap_or_default()
}

fn default_feedback() -> BoundedVec<u8, MaxFeedbackLen> {
	let s: Vec<u8> = "No feedback".as_bytes().into();
	s.try_into().unwrap_or_default()
}

fn feedback(message: &str) -> BoundedVec<u8, MaxFeedbackLen> {
	let s: Vec<u8> = message.as_bytes().into();
	s.try_into().unwrap_or_default()
}

fn boundedvec_to_string(boundedvec: &BoundedVec<u8, MaxFeedbackLen>) -> String {
	let mut s = String::new();
	for b in boundedvec.iter() {
		s.push(*b as char);
	}
	s
}

fn _create_file(name: &str, cid: &str, create_custodian_file: bool) -> ApplicationField {
	let display_name_vec: Vec<u8> = name.as_bytes().into();
	let display_name: BoundedVec<u8, ConstU32<100>> = display_name_vec.try_into().unwrap_or_default();
	let cid :BoundedVec<u8, ConstU32<100>> = cid.as_bytes().to_vec().try_into().unwrap_or_default();
	let custodian_cid = match create_custodian_file{ true => Some(cid.clone()), false=> None};
	ApplicationField{
		display_name,
		cid,
		custodian_cid,
	}
}

// due to encoding problems with polkadot-js, the custodians_cid generation will be done in another function
fn create_application_fields( n_files: u32) -> 
		BoundedVec<(BoundedVec<u8,ConstU32<100> >,BoundedVec<u8,ConstU32<100>> ), MaxFiles> {
	let mut files = Vec::<(BoundedVec<u8,ConstU32<100> >,BoundedVec<u8,ConstU32<100>> )>::default();
	for i in 0..n_files{
		let file_name = format!("file{}",i.to_string());
		let cid = format!("cid{}",i.to_string());
		files.push( (file_name.encode().try_into().unwrap_or_default(), cid.encode().try_into().unwrap_or_default()) );
	}
	BoundedVec::<(BoundedVec<u8,ConstU32<100> >,BoundedVec<u8,ConstU32<100>> ), MaxFiles>::try_from( files).unwrap_or_default()
}

fn create_custiodian_fields( custodian_account: u64, n_files: u32, ) ->
	Option<( u64,BoundedVec<BoundedVec<u8,ConstU32<100>>, MaxFiles>) >{
	let cids: Vec<BoundedVec<u8,ConstU32<100>>> = (0..n_files).map(|n|{
		let cid = format!("cid_custodian{}",n.to_string());
		cid.as_bytes().to_vec().try_into().unwrap_or_default()

	}).collect();

	Some( 
		(custodian_account,BoundedVec::<BoundedVec<u8,ConstU32<100>>, MaxFiles>::try_from(cids).unwrap_or_default())
	)
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
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), None ));

		assert!( GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Pending).len() ==1);
	});
}

#[test]
fn apply_with_custodian_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), create_custiodian_fields(4,2) ));

		assert!( GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Pending).len() ==1);
		assert!(GatedMarketplace::custodians(4, m_id).pop().is_some() );
	});
}

#[test]
fn apply_with_same_account_as_custodian_shouldnt_work() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_noop!(
			GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), create_custiodian_fields(3,2) ),
			Error::<Test>::ApplicantCannotBeCustodian
		);
	});
}

#[test]
fn exceeding_max_applications_per_custodian_shouldnt_work() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), create_custiodian_fields(6,2) ));
		assert_ok!(GatedMarketplace::apply(Origin::signed(4),m_id, create_application_fields(2), create_custiodian_fields(6,2) ));
		assert_noop!(
			GatedMarketplace::apply(Origin::signed(5),m_id, create_application_fields(2), create_custiodian_fields(6,2) ),
			Error::<Test>::ExceedMaxApplicationsPerCustodian
		);
	});
}



#[test]
fn apply_to_nonexistent_marketplace_shouldnt_work() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		// No such marletplace exists:
		let m_id = create_label("false marketplace").using_encoded(blake2_256);
		assert_noop!(GatedMarketplace::apply(Origin::signed(3),m_id,create_application_fields(2), None ), Error::<Test>::MarketplaceNotFound);
	});
}

#[test]
fn apply_twice_shouldnt_work() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(3), m_id, create_application_fields(2), None ));
		assert_noop!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), None ), Error::<Test>::AlreadyApplied );
	});
}

#[test]
fn exceeding_max_applicants_shouldnt_work() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), None ));
		assert_ok!(GatedMarketplace::apply(Origin::signed(4),m_id, create_application_fields(3), None ));
		assert_ok!(GatedMarketplace::apply(Origin::signed(5),m_id, create_application_fields(3), None ));
		assert_noop!(GatedMarketplace::apply(Origin::signed(6),m_id, create_application_fields(1), None ), Error::<Test>::ExceedMaxApplicants );
	});
}

#[test]
fn enroll_works() {
	new_test_ext().execute_with(|| {

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id,create_application_fields(2), None ));
		assert_ok!(GatedMarketplace::apply(Origin::signed(4),m_id,create_application_fields(1), None));
		let app_id = GatedMarketplace::applications_by_account(3,m_id).unwrap();
		// enroll with account
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Account(3), true, default_feedback()));
		// enroll with application
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application(app_id), true, default_feedback()));
	});
}

#[test]
fn enroll_rejected_works() {
	new_test_ext().execute_with(|| {

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), None ));
		assert_ok!(GatedMarketplace::apply(Origin::signed(4),m_id, create_application_fields(1), None ));
		let app_id = GatedMarketplace::applications_by_account(3,m_id).unwrap();
		// reject with account
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Account(3), false, default_feedback()));
		// reject with application
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application(app_id), false, default_feedback()));
	});
}

#[test]
fn enroll_rejected_has_feedback_works() {
	new_test_ext().execute_with(|| {

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), None ));
		assert_ok!(GatedMarketplace::apply(Origin::signed(4),m_id, create_application_fields(1), None ));
		let app_id = GatedMarketplace::applications_by_account(3,m_id).unwrap();
		// reject with account
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Account(3), false, feedback("We need to reject this application")));
		// reject with application
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application(app_id), false, feedback("We need to reject this application")));

		assert_eq!(boundedvec_to_string(&GatedMarketplace::applications(app_id).unwrap().feedback), String::from("We need to reject this application"));
	});
}

#[test]
fn enroll_approved_has_feedback_works() {
	new_test_ext().execute_with(|| {

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), None ));
		assert_ok!(GatedMarketplace::apply(Origin::signed(4),m_id, create_application_fields(1), None ));
		let app_id = GatedMarketplace::applications_by_account(3,m_id).unwrap();
		// reject with account
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Account(3), true, feedback("We've accepted your publication")));
		// reject with application
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application(app_id), true, feedback("We've accepted your publication")));

		assert_eq!(boundedvec_to_string(&GatedMarketplace::applications(app_id).unwrap().feedback), String::from("We've accepted your publication"));
	});
}

#[test]
fn change_enroll_status_works() {
	new_test_ext().execute_with(|| {

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(4),m_id, create_application_fields(1), None ));
		let app_id = GatedMarketplace::applications_by_account(4,m_id).unwrap();
		// reject an account
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Account(4), false, default_feedback()));
		// and then change it to "accepted"
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application(app_id), true, default_feedback()));
	});
}

#[test]
fn non_authorized_user_enroll_shouldnt_work() {
	new_test_ext().execute_with(|| {

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), None ));

		// external user tries to enroll someone
		assert_noop!(GatedMarketplace::enroll(Origin::signed(4), m_id , AccountOrApplication::Account(3), true, default_feedback()), Error::<Test>::CannotEnroll);
	});
}

#[test]
fn enroll_nonexistent_application_shouldnt_work() {
	new_test_ext().execute_with(|| {

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		// accept nonexisten application throws error (account version)
		assert_noop!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Account(3), true, default_feedback()), Error::<Test>::ApplicationNotFound);
		// accept nonexisten application throws error (application id version)
		assert_noop!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application([0;32]), true, default_feedback()), Error::<Test>::ApplicationNotFound);
	});
}

//add authorities

#[test]
fn add_authority_appraiser_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Appraiser, m_id));
		assert!(GatedMarketplace::marketplaces_by_authority(3, m_id) == vec![MarketplaceAuthority::Appraiser]);
	});
}

#[test]
fn add_authority_admin_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Admin, m_id));
		assert!(GatedMarketplace::marketplaces_by_authority(3, m_id) == vec![MarketplaceAuthority::Admin]);
	});
}

#[test]
fn add_authority_redenmption_specialist_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::RedemptionSpecialist, m_id));
		assert!(GatedMarketplace::marketplaces_by_authority(3, m_id) == vec![MarketplaceAuthority::RedemptionSpecialist]);
	});
}

#[test]
fn add_authority_owner_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_noop!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Owner, m_id), Error::<Test>::OnlyOneOwnerIsAllowed);
	});
}

#[test]
fn add_authority_cant_apply_twice_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Appraiser, m_id));
		assert_noop!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Appraiser, m_id), Error::<Test>::AlreadyApplied);
	});
}

//remove authorities


#[test]
fn remove_authority_appraiser_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Appraiser, m_id));
		assert_ok!(GatedMarketplace::remove_authority(Origin::signed(1), 3, MarketplaceAuthority::Appraiser, m_id));
		assert!(GatedMarketplace::marketplaces_by_authority(3, m_id) == vec![]);
	});
}

#[test]
fn remove_authority_admin_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Admin, m_id));
		assert_ok!(GatedMarketplace::remove_authority(Origin::signed(1), 3, MarketplaceAuthority::Admin, m_id));
		assert!(GatedMarketplace::marketplaces_by_authority(3, m_id) == vec![]);
	});
}

#[test]
fn remove_authority_redemption_specialist_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::RedemptionSpecialist, m_id));
		assert_ok!(GatedMarketplace::remove_authority(Origin::signed(1), 3, MarketplaceAuthority::RedemptionSpecialist, m_id));
		assert!(GatedMarketplace::marketplaces_by_authority(3, m_id) == vec![]);
	});
}

#[test]
fn remove_authority_owner_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_noop!(GatedMarketplace::remove_authority(Origin::signed(1), 1 , MarketplaceAuthority::Owner, m_id), Error::<Test>::CantRemoveOwner);
	});
}



#[test]
fn remove_authority_admin_by_admin_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Admin, m_id));
		assert_noop!(GatedMarketplace::remove_authority(Origin::signed(3), 3, MarketplaceAuthority::Admin, m_id), Error::<Test>::AdminCannotRemoveItself);
	});
}

#[test]
fn remove_authority_user_tries_to_remove_non_existent_role_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Appraiser, m_id));
		assert_noop!(GatedMarketplace::remove_authority(Origin::signed(1), 3, MarketplaceAuthority::Admin, m_id), Error::<Test>::AuthorityNotFoundForUser);
	});
}

#[test]
fn remove_authority_user_is_not_admin_or_owner_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Appraiser, m_id));
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 4, MarketplaceAuthority::Admin, m_id));
		assert_noop!(GatedMarketplace::remove_authority(Origin::signed(3), 4,MarketplaceAuthority::Appraiser, m_id), Error::<Test>::CannotEnroll);
	});
}

#[test]
fn remove_authority_only_owner_can_remove_admins_works(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Admin, m_id));
		assert_ok!(GatedMarketplace::remove_authority(Origin::signed(1), 3, MarketplaceAuthority::Admin, m_id));
	});
}

// Update marketplace's label

#[test]
fn update_marketplace_marketplace_not_found_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());
		let m_id_2 = create_label("not the first marketplace").using_encoded(blake2_256);	
		assert_noop!(GatedMarketplace::update_label_marketplace(Origin::signed(1), m_id_2, create_label("my marketplace 2")), Error::<Test>::MarketplaceNotFound);
	});
	
}

#[test]
fn update_marketplace_user_without_permission_shouldnt_work(){
	new_test_ext().execute_with(|| {
		//user should be an admin or owner
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Appraiser, m_id));
		assert_noop!(GatedMarketplace::update_label_marketplace(Origin::signed(3), m_id, create_label("my marketplace2")), Error::<Test>::CannotEnroll);
	});
}

#[test]
fn update_label_marketplace_works(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());		
		assert_ok!(GatedMarketplace::update_label_marketplace(Origin::signed(1), m_id, create_label("my marketplace 2")));
		assert!(GatedMarketplace::marketplaces(m_id).is_some() );	
	});
}


//Delete the selected marketplace

#[test]
fn remove_marketplace_marketplace_not_found_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());
		let m_id_2 = create_label("not the first marketplace").using_encoded(blake2_256);
		assert_noop!(GatedMarketplace::remove_marketplace(Origin::signed(1), m_id_2), Error::<Test>::MarketplaceNotFound);
	});
	 
}


#[test]
fn remove_marketplace_user_without_permission_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Appraiser, m_id));
		assert_noop!(GatedMarketplace::remove_marketplace(Origin::signed(3), m_id), Error::<Test>::CannotEnroll);
	});
}

#[test]
fn remove_marketplace_deletes_storage_from_marketplaces_works(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());
		assert_ok!(GatedMarketplace::remove_marketplace(Origin::signed(1), m_id));
		assert!(GatedMarketplace::marketplaces(m_id).is_none());
	});
}

#[test]
fn remove_marketplace_deletes_storage_from_marketplaces_by_authority_works(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());

		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Appraiser, m_id));
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 4, MarketplaceAuthority::RedemptionSpecialist, m_id));
		
		assert!(GatedMarketplace::marketplaces_by_authority(1, m_id) == vec![MarketplaceAuthority::Owner]);
		assert!(GatedMarketplace::marketplaces_by_authority(2, m_id) == vec![MarketplaceAuthority::Admin]);
		assert!(GatedMarketplace::marketplaces_by_authority(3, m_id) == vec![MarketplaceAuthority::Appraiser]);
		assert!(GatedMarketplace::marketplaces_by_authority(4, m_id) == vec![MarketplaceAuthority::RedemptionSpecialist]);

		assert_ok!(GatedMarketplace::remove_marketplace(Origin::signed(1), m_id));

		assert!(GatedMarketplace::marketplaces_by_authority(1, m_id) == vec![]);
		assert!(GatedMarketplace::marketplaces_by_authority(2, m_id) == vec![]);
		assert!(GatedMarketplace::marketplaces_by_authority(3, m_id) == vec![]);
		assert!(GatedMarketplace::marketplaces_by_authority(4, m_id) == vec![]);
		assert!(GatedMarketplace::marketplaces(m_id).is_none());
	});
}

#[test]
fn remove_marketplace_deletes_storage_from_authorities_by_marketplace_works(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());

		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Appraiser, m_id));
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 4, MarketplaceAuthority::RedemptionSpecialist, m_id));

		assert!(GatedMarketplace::authorities_by_marketplace(m_id, MarketplaceAuthority::Owner) == vec![1]);
		assert!(GatedMarketplace::authorities_by_marketplace(m_id, MarketplaceAuthority::Admin) == vec![2]);
		assert!(GatedMarketplace::authorities_by_marketplace(m_id, MarketplaceAuthority::Appraiser) == vec![3]);
		assert!(GatedMarketplace::authorities_by_marketplace(m_id, MarketplaceAuthority::RedemptionSpecialist) == vec![4]);

		assert_ok!(GatedMarketplace::remove_marketplace(Origin::signed(1), m_id));

		assert!(GatedMarketplace::authorities_by_marketplace(m_id, MarketplaceAuthority::Owner) == vec![]);
		assert!(GatedMarketplace::authorities_by_marketplace(m_id, MarketplaceAuthority::Admin) == vec![]);
		assert!(GatedMarketplace::authorities_by_marketplace(m_id, MarketplaceAuthority::Appraiser) == vec![]);
		assert!(GatedMarketplace::authorities_by_marketplace(m_id, MarketplaceAuthority::RedemptionSpecialist) == vec![]);
		assert!(GatedMarketplace::marketplaces(m_id).is_none());
	});
}

#[test]
fn remove_marketplace_deletes_storage_from_custodians_works(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());

		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), create_custiodian_fields(4,2) ));
		assert!(GatedMarketplace::custodians(4, m_id) == vec![3]);

		assert_ok!(GatedMarketplace::remove_marketplace(Origin::signed(1), m_id));

		assert!(GatedMarketplace::custodians(4, m_id) == vec![]);
		assert!(GatedMarketplace::marketplaces(m_id).is_none());

	});
}


#[test]
fn remove_marketplace_deletes_storage_from_applicants_by_marketplace_status_pending_works(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());

		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), None ));
		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Pending) == vec![3]);
		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Approved) == vec![]);
		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Rejected) == vec![]);

		assert_ok!(GatedMarketplace::remove_marketplace(Origin::signed(1), m_id));

		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Pending) == vec![]);
		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Approved) == vec![]);
		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Rejected) == vec![]);
		assert!(GatedMarketplace::marketplaces(m_id).is_none());
	});
}

#[test]
fn remove_marketplace_deletes_storage_from_applicants_by_marketplace_status_approved_works(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());

		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), None ));
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Account(3), true, default_feedback()));

		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Pending) == vec![]);
		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Approved) == vec![3]);
		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Rejected) == vec![]);

		assert_ok!(GatedMarketplace::remove_marketplace(Origin::signed(1), m_id));

		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Pending) == vec![]);
		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Approved) == vec![]);
		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Rejected) == vec![]);
		assert!(GatedMarketplace::marketplaces(m_id).is_none());
	});
}

#[test]
fn remove_marketplace_deletes_storage_from_applicants_by_marketplace_status_rejected_works(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());

		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), None ));
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Account(3), false, default_feedback()));

		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Pending) == vec![]);
		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Approved) == vec![]);
		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Rejected) == vec![3]);

		assert_ok!(GatedMarketplace::remove_marketplace(Origin::signed(1), m_id));

		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Pending) == vec![]);
		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Approved) == vec![]);
		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Rejected) == vec![]);
		assert!(GatedMarketplace::marketplaces(m_id).is_none());
	});
}

#[test]
fn remove_marketplace_deletes_storage_from_applicantions_by_account_works(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());

		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), None ));
		let app_id = GatedMarketplace::applications_by_account(3,m_id).unwrap();
		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Pending) == vec![3]);
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Account(3), true, default_feedback()));
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application(app_id), true, default_feedback()));
		assert!(GatedMarketplace::applications_by_account(3, m_id).is_some());

		assert_ok!(GatedMarketplace::remove_marketplace(Origin::signed(1), m_id));
		assert!(GatedMarketplace::applications_by_account(3, m_id).is_none());
		assert!(GatedMarketplace::marketplaces(m_id).is_none());

	});
}


#[test]
fn remove_marketplace_deletes_storage_from_applications_works(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());

		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), None ));
		let app_id = GatedMarketplace::applications_by_account(3,m_id).unwrap();

		assert!(GatedMarketplace::applicants_by_marketplace(m_id, ApplicationStatus::Pending) == vec![3]);
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Account(3), true,default_feedback()));
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application(app_id), true, default_feedback()));
		assert!(GatedMarketplace::applications(app_id).is_some());

		assert_ok!(GatedMarketplace::remove_marketplace(Origin::signed(1), m_id));
		assert!(GatedMarketplace::applications(app_id).is_none());
		assert!(GatedMarketplace::marketplaces(m_id).is_none());
	});
}

//reapply
#[test]
fn reapply_user_has_never_applied_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());
		assert_noop!(GatedMarketplace::reapply(Origin::signed(3), m_id, create_application_fields(2), None), Error::<Test>::ApplicationIdNotFound);
	});
}

#[test]
fn reapply_with_wrong_marketplace_id_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		let m_id2 = create_label("my marketplace2").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());
		assert_noop!(GatedMarketplace::reapply(Origin::signed(1), m_id2, create_application_fields(2), None), Error::<Test>::ApplicationIdNotFound);
	});
}

#[test]
fn reapply_status_application_is_still_pendding_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());

		assert_ok!(GatedMarketplace::apply(Origin::signed(3), m_id, create_application_fields(2), None));
		let app_id = GatedMarketplace::applications_by_account(3,m_id).unwrap();
		assert_eq!(GatedMarketplace::applications(app_id).unwrap().status, ApplicationStatus::Pending);

		assert_noop!(GatedMarketplace::reapply(Origin::signed(3), m_id, create_application_fields(2), None), Error::<Test>::ApplicationStatusStillPending);
	});
}

#[test]
fn reapply_status_application_is_already_approved_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());

		assert_ok!(GatedMarketplace::apply(Origin::signed(3), m_id, create_application_fields(2), None));
		let app_id = GatedMarketplace::applications_by_account(3,m_id).unwrap();
		assert_eq!(GatedMarketplace::applications(app_id).unwrap().status, ApplicationStatus::Pending);

		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application(app_id), true, default_feedback()));
		assert_eq!(GatedMarketplace::applications(app_id).unwrap().status, ApplicationStatus::Approved);
		assert_noop!(GatedMarketplace::reapply(Origin::signed(3), m_id, create_application_fields(2), None), Error::<Test>::ApplicationHasAlreadyBeenApproved);
	});
}

#[test]
fn reapply_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());

		assert_ok!(GatedMarketplace::apply(Origin::signed(3), m_id, create_application_fields(2), None));
		let app_id = GatedMarketplace::applications_by_account(3, m_id).unwrap();
		assert_eq!(GatedMarketplace::applications(app_id).unwrap().status, ApplicationStatus::Pending);
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application(app_id), false, default_feedback()));
		assert_ok!(GatedMarketplace::reapply(Origin::signed(3), m_id, create_application_fields(2), None));

		assert_eq!(GatedMarketplace::applications(app_id).unwrap().status, ApplicationStatus::Pending);

	});
}

