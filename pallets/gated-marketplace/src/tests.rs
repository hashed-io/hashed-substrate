use crate::{mock::*, Error, types::*, Config};
use std::vec;
use sp_runtime::sp_std::vec::Vec;
use codec::Encode;
use frame_support::{assert_ok, BoundedVec, traits::{Len, ConstU32, Currency}, assert_noop};
use pallet_rbac::types::RoleBasedAccessControl;
use sp_io::hashing::blake2_256;

type RbacErr = pallet_rbac::Error<Test>;

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

fn _find_id(vec_tor: BoundedVec<[u8;32], ConstU32<100>>, id:[u8;32]) -> bool {
	vec_tor.iter().find(|&x| *x == id).ok_or(Error::<Test>::OfferNotFound).is_ok()
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
fn exceeding_max_roles_per_auth_shouldnt_work() {
	new_test_ext().execute_with(|| {
		let m_label =  create_label("my marketplace");
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, m_label.clone() ));
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 2, MarketplaceRole::Appraiser, m_label.using_encoded(blake2_256)));
		assert_noop!(
			GatedMarketplace::add_authority(Origin::signed(1), 2, MarketplaceRole::RedemptionSpecialist, m_label.using_encoded(blake2_256) ), 
			RbacErr::ExceedMaxRolesPerUser 
		);

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
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application(app_id), false, default_feedback()));
	});
}

#[test]
fn enroll_rejected_works() {
	new_test_ext().execute_with(|| {

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), None ));
		assert_ok!(GatedMarketplace::apply(Origin::signed(4),m_id, create_application_fields(1), None ));
		let app_id = GatedMarketplace::applications_by_account(4,m_id).unwrap();
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
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Account(3), true, feedback("We need to accept this application")));
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
		let app_id = GatedMarketplace::applications_by_account(4,m_id).unwrap();
		// reject with account
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Account(3), true, feedback("We've rejected your publication")));
		// reject with application
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application(app_id), true, feedback("We've rejected your publication")));

		assert_eq!(boundedvec_to_string(&GatedMarketplace::applications(app_id).unwrap().feedback), String::from("We've rejected your publication"));
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
		assert_noop!(GatedMarketplace::enroll(Origin::signed(4), m_id , AccountOrApplication::Account(3), true, default_feedback()), RbacErr::NotAuthorized);
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
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::Appraiser, m_id));
		//assert!(GatedMarketplace::marketplaces_by_authority(3, m_id) == vec![MarketplaceRole::Appraiser]);
		assert!(RBAC::roles_by_user((3, GatedMarketplace::pallet_id(), m_id)).contains(&MarketplaceRole::Appraiser.id()));
	});
}

#[test]
fn add_authority_admin_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::Admin, m_id));
		//assert!(GatedMarketplace::marketplaces_by_authority(3, m_id) == vec![MarketplaceRole::Admin]);
		assert!(RBAC::roles_by_user((3, GatedMarketplace::pallet_id(), m_id)).contains(&MarketplaceRole::Admin.id()));
	});
}

#[test]
fn add_authority_redenmption_specialist_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::RedemptionSpecialist, m_id));
		//assert!(GatedMarketplace::marketplaces_by_authority(3, m_id) == vec![MarketplaceRole::RedemptionSpecialist]);
		assert!(RBAC::roles_by_user((3, GatedMarketplace::pallet_id(), m_id)).contains(&MarketplaceRole::RedemptionSpecialist.id()));
	});
}

#[test]
fn add_authority_owner_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_noop!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::Owner, m_id), Error::<Test>::OnlyOneOwnerIsAllowed);
		let n_owners = <Test as Config>::Rbac::get_role_users_len(GatedMarketplace::pallet_id(), &m_id,  &MarketplaceRole::Owner.id());
		assert_eq!(n_owners, 1);
	});
}

#[test]
fn add_authority_cant_apply_twice_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::Appraiser, m_id));
		assert_noop!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::Appraiser, m_id), RbacErr::UserAlreadyHasRole);
	});
}

//remove authorities


#[test]
fn remove_authority_appraiser_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::Appraiser, m_id));
		assert_ok!(GatedMarketplace::remove_authority(Origin::signed(1), 3, MarketplaceRole::Appraiser, m_id));
		//assert!(GatedMarketplace::marketplaces_by_authority(3, m_id) == vec![]);
		assert!(RBAC::roles_by_user((3, GatedMarketplace::pallet_id(), m_id)).is_empty());
	});
}

#[test]
fn remove_authority_admin_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::Admin, m_id));
		assert_ok!(GatedMarketplace::remove_authority(Origin::signed(1), 3, MarketplaceRole::Admin, m_id));
		//assert!(GatedMarketplace::marketplaces_by_authority(3, m_id) == vec![]);
		assert!(RBAC::roles_by_user((3, GatedMarketplace::pallet_id(), m_id)).is_empty());
	});
}

#[test]
fn remove_authority_redemption_specialist_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::RedemptionSpecialist, m_id));
		assert_ok!(GatedMarketplace::remove_authority(Origin::signed(1), 3, MarketplaceRole::RedemptionSpecialist, m_id));
		//assert!(GatedMarketplace::marketplaces_by_authority(3, m_id) == vec![]);
		assert!(RBAC::roles_by_user((3, GatedMarketplace::pallet_id(), m_id)).is_empty());
	});
}

#[test]
fn remove_authority_owner_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_noop!(GatedMarketplace::remove_authority(Origin::signed(1), 1 , MarketplaceRole::Owner, m_id), Error::<Test>::CantRemoveOwner);
	});
}



#[test]
fn remove_authority_admin_by_admin_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::Admin, m_id));
		assert_noop!(GatedMarketplace::remove_authority(Origin::signed(3), 3, MarketplaceRole::Admin, m_id), Error::<Test>::AdminCannotRemoveItself);
	});
}

#[test]
fn remove_authority_user_tries_to_remove_non_existent_role_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::Appraiser, m_id));
		assert_noop!(GatedMarketplace::remove_authority(Origin::signed(1), 3, MarketplaceRole::Admin, m_id), RbacErr::RoleNotFound);
	});
}

#[test]
fn remove_authority_user_is_not_admin_or_owner_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::Appraiser, m_id));
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 4, MarketplaceRole::Admin, m_id));
		assert_noop!(GatedMarketplace::remove_authority(Origin::signed(3), 4,MarketplaceRole::Appraiser, m_id), RbacErr::NotAuthorized);
	});
}

#[test]
fn remove_authority_only_owner_can_remove_admins_works(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::Admin, m_id));
		assert_ok!(GatedMarketplace::remove_authority(Origin::signed(1), 3, MarketplaceRole::Admin, m_id));
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
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::Appraiser, m_id));
		assert_noop!(GatedMarketplace::update_label_marketplace(Origin::signed(3), m_id, create_label("my marketplace2")), RbacErr::NotAuthorized);
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
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::Appraiser, m_id));
		assert_noop!(GatedMarketplace::remove_marketplace(Origin::signed(3), m_id), RbacErr::NotAuthorized);
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

		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::Appraiser, m_id));
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 4, MarketplaceRole::RedemptionSpecialist, m_id));
		
		//assert!(GatedMarketplace::marketplaces_by_authority(1, m_id) == vec![MarketplaceRole::Owner]);
		assert_ok!(<Test as Config>::Rbac::has_role(1, GatedMarketplace::pallet_id(), &m_id, vec![MarketplaceRole::Owner.id()]));
		//assert!(GatedMarketplace::marketplaces_by_authority(2, m_id) == vec![MarketplaceRole::Admin]);
		assert_ok!(<Test as Config>::Rbac::has_role(2, GatedMarketplace::pallet_id(), &m_id, vec![MarketplaceRole::Admin.id()]));
		//assert!(GatedMarketplace::marketplaces_by_authority(3, m_id) == vec![MarketplaceRole::Appraiser]);
		assert_ok!(<Test as Config>::Rbac::has_role(3, GatedMarketplace::pallet_id(), &m_id, vec![MarketplaceRole::Appraiser.id()]));
		//assert!(GatedMarketplace::marketplaces_by_authority(4, m_id) == vec![MarketplaceRole::RedemptionSpecialist]);
		assert_ok!(<Test as Config>::Rbac::has_role(4, GatedMarketplace::pallet_id(), &m_id, vec![MarketplaceRole::RedemptionSpecialist.id()]));
		assert_ok!(GatedMarketplace::remove_marketplace(Origin::signed(1), m_id));

		//assert!(GatedMarketplace::marketplaces_by_authority(1, m_id) == vec![]);
		assert!(RBAC::roles_by_user((1, GatedMarketplace::pallet_id(), m_id)).is_empty());
		//assert!(GatedMarketplace::marketplaces_by_authority(2, m_id) == vec![]);
		assert!(RBAC::roles_by_user((2, GatedMarketplace::pallet_id(), m_id)).is_empty());
		//assert!(GatedMarketplace::marketplaces_by_authority(3, m_id) == vec![]);
		assert!(RBAC::roles_by_user((3, GatedMarketplace::pallet_id(), m_id)).is_empty());
		//assert!(GatedMarketplace::marketplaces_by_authority(4, m_id) == vec![]);
		assert!(RBAC::roles_by_user((4, GatedMarketplace::pallet_id(), m_id)).is_empty());

		assert!(GatedMarketplace::marketplaces(m_id).is_none());
	});
}

#[test]
fn remove_marketplace_deletes_storage_from_authorities_by_marketplace_works(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());

		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceRole::Appraiser, m_id));
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 4, MarketplaceRole::RedemptionSpecialist, m_id));

		//assert!(GatedMarketplace::authorities_by_marketplace(m_id, MarketplaceRole::Owner) == vec![1]);
		assert!(RBAC::users_by_scope((GatedMarketplace::pallet_id(), m_id, MarketplaceRole::Owner.id())).contains(&1));
		//assert!(GatedMarketplace::authorities_by_marketplace(m_id, MarketplaceRole::Admin) == vec![2]);
		assert!(RBAC::users_by_scope((GatedMarketplace::pallet_id(), m_id, MarketplaceRole::Admin.id())).contains(&2));
		//assert!(GatedMarketplace::authorities_by_marketplace(m_id, MarketplaceRole::Appraiser) == vec![3]);
		assert!(RBAC::users_by_scope((GatedMarketplace::pallet_id(), m_id, MarketplaceRole::Appraiser.id())).contains(&3));
		//assert!(GatedMarketplace::authorities_by_marketplace(m_id, MarketplaceRole::RedemptionSpecialist) == vec![4]);
		assert!(RBAC::users_by_scope((GatedMarketplace::pallet_id(), m_id, MarketplaceRole::RedemptionSpecialist.id())).contains(&4));

		assert_ok!(GatedMarketplace::remove_marketplace(Origin::signed(1), m_id));

		//assert!(GatedMarketplace::authorities_by_marketplace(m_id, MarketplaceRole::Owner) == vec![]);
		assert!(RBAC::users_by_scope((GatedMarketplace::pallet_id(), m_id, MarketplaceRole::Owner.id())).is_empty());
		//assert!(GatedMarketplace::authorities_by_marketplace(m_id, MarketplaceRole::Admin) == vec![]);
		assert!(RBAC::users_by_scope((GatedMarketplace::pallet_id(), m_id, MarketplaceRole::Admin.id())).is_empty());
		//assert!(GatedMarketplace::authorities_by_marketplace(m_id, MarketplaceRole::Appraiser) == vec![]);
		assert!(RBAC::users_by_scope((GatedMarketplace::pallet_id(), m_id, MarketplaceRole::Appraiser.id())).is_empty());
		//assert!(GatedMarketplace::authorities_by_marketplace(m_id, MarketplaceRole::RedemptionSpecialist) == vec![]);
		assert!(RBAC::users_by_scope((GatedMarketplace::pallet_id(), m_id, MarketplaceRole::RedemptionSpecialist.id())).is_empty());
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
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application(app_id), false, default_feedback()));
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
		assert_ok!(GatedMarketplace::enroll(Origin::signed(1), m_id , AccountOrApplication::Application(app_id), false, default_feedback()));
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

//Offers
#[test]
fn enlist_sell_offer_works() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 10000)); 
		let offer_id = GatedMarketplace::offers_by_item(0, 0).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());
		assert_eq!(GatedMarketplace::offers_info(offer_id).unwrap().offer_type, OfferType::SellOrder);

	});
}


#[test]
fn enlist_sell_offer_item_does_not_exist_shouldnt_work() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);

		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);

		assert_noop!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 1, 10000), Error::<Test>::CollectionNotFound);
	});
}


#[test]
fn enlist_sell_offer_item_already_enlisted_shouldnt_work() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 10000)); 
		let offer_id = GatedMarketplace::offers_by_item(0, 0).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());
		assert_noop!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 10000), Error::<Test>::OfferAlreadyExists);
	});
}

#[test]
fn enlist_sell_offer_not_owner_tries_to_enlist_shouldnt_work() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 100);

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_noop!(GatedMarketplace::enlist_sell_offer(Origin::signed(2), m_id, 0, 0, 10000), Error::<Test>::NotOwner);
	});
}

#[test]
fn enlist_sell_offer_price_must_greater_than_zero_shouldnt_work() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_noop!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 0), Error::<Test>::PriceMustBeGreaterThanZero);
	});
}

#[test]
fn enlist_sell_offer_price_must_greater_than_minimun_amount_works() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
	
		let minimum_amount = 1001;	
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, minimum_amount));
		let offer_id = GatedMarketplace::offers_by_item(0, 0).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());
	});
}

#[test]
fn enlist_sell_offer_is_properly_stored_works() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 10000));
		let offer_id = GatedMarketplace::offers_by_item(0, 0).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());
		assert_eq!(GatedMarketplace::offers_by_account(1).iter().next().unwrap().clone(), offer_id);
		assert_eq!(GatedMarketplace::offers_by_marketplace(m_id).iter().next().unwrap().clone(), offer_id);

	});
}


#[test]
fn enlist_sell_offer_two_marketplaces(){
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace2")));
		let m_id2 = create_label("my marketplace2").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 10000)); 
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id2, 0, 0, 11000)); 

		assert_eq!(GatedMarketplace::offers_by_item(0, 0).len(), 2);
		assert_eq!(GatedMarketplace::offers_by_account(1).len(), 2);
		assert_eq!(GatedMarketplace::offers_by_marketplace(m_id).len(), 1);
		assert_eq!(GatedMarketplace::offers_by_marketplace(m_id2).len(), 1);

	});
}

#[test]
fn enlist_buy_offer_works() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1100);

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 1001)); 
		let offer_id = GatedMarketplace::offers_by_account(1).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());

		assert_ok!(GatedMarketplace::enlist_buy_offer(Origin::signed(2), m_id, 0, 0, 1100));
		let offer_id2 = GatedMarketplace::offers_by_account(2).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id2).is_some());
		assert_eq!(GatedMarketplace::offers_info(offer_id2).unwrap().offer_type, OfferType::BuyOrder);
	});
}

#[test]
fn enlist_buy_offer_item_is_not_for_sale_shouldnt_work() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1100);

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 1001)); 
		let offer_id = GatedMarketplace::offers_by_account(1).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());

		assert_noop!(GatedMarketplace::enlist_buy_offer(Origin::signed(2), m_id, 0, 1, 1100), Error::<Test>::ItemNotForSale);
	});
}


#[test]
fn enlist_buy_offer_owner_cannnot_create_buy_offers_for_their_own_items_shouldnt_work() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1100);

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 1001)); 
		let offer_id = GatedMarketplace::offers_by_account(1).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());
		
		assert_noop!(GatedMarketplace::enlist_buy_offer(Origin::signed(1), m_id, 0, 0, 1100), Error::<Test>::CannotCreateOffer);
	});
}

#[test] 
fn enlist_buy_offer_user_does_not_have_enough_balance_shouldnt_work() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 100);


		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 10000)); 
		let offer_id = GatedMarketplace::offers_by_account(1).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());

		assert_noop!(GatedMarketplace::enlist_buy_offer(Origin::signed(2), m_id, 0, 0, 10000), Error::<Test>::NotEnoughBalance);

	});
}

#[test]
fn enlist_buy_offer_price_must_greater_than_zero_shouldnt_work() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1100);

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 10000)); 
		let offer_id = GatedMarketplace::offers_by_account(1).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());

		assert_noop!(GatedMarketplace::enlist_buy_offer(Origin::signed(2), m_id, 0, 0, 0), Error::<Test>::PriceMustBeGreaterThanZero);
	});
}



#[test]
fn enlist_buy_offer_an_item_can_receive_multiple_buy_offers(){
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1101);
		Balances::make_free_balance_be(&3, 1201);

		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 10000)); 
		let offer_id = GatedMarketplace::offers_by_account(1).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());

		assert_ok!(GatedMarketplace::enlist_buy_offer(Origin::signed(2), m_id, 0, 0, 1100));
		let offer_id2 = GatedMarketplace::offers_by_account(2).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id2).is_some());

		assert_ok!(GatedMarketplace::enlist_buy_offer(Origin::signed(3), m_id, 0, 0, 1200));
		let offer_id3 = GatedMarketplace::offers_by_account(3).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id3).is_some());

		assert_eq!(GatedMarketplace::offers_by_item(0, 0).len(), 3);

	});

}

#[test]
fn take_sell_offer_works(){
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1300);
	
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 1200)); 
		let offer_id = GatedMarketplace::offers_by_item(0, 0).iter().next().unwrap().clone();

		assert_ok!(GatedMarketplace::take_sell_offer(Origin::signed(2), offer_id, m_id, 0, 0));
		assert_eq!(GatedMarketplace::offers_by_item(0, 0).len(), 0);
		assert_eq!(GatedMarketplace::offers_info(offer_id).unwrap().status, OfferStatus::Closed);

	});
}

#[test]
fn take_sell_offer_owner_cannnot_be_the_buyer_shouldnt_work() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1300);
	
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 1200)); 
		let offer_id = GatedMarketplace::offers_by_item(0, 0).iter().next().unwrap().clone();

		assert_noop!(GatedMarketplace::take_sell_offer(Origin::signed(1), offer_id, m_id, 0, 0), Error::<Test>::CannotTakeOffer);
	});
}

#[test]
fn take_sell_offer_id_does_not_exist_shouldnt_work(){
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1300);
	
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 1200)); 
		let offer_id = GatedMarketplace::offers_by_item(0, 0).iter().next().unwrap().clone();
		let offer_id2 = offer_id.using_encoded(blake2_256);

		assert_noop!(GatedMarketplace::take_sell_offer(Origin::signed(2), offer_id2, m_id, 0, 0), Error::<Test>::OfferNotFound);
	});
}


#[test]
fn take_sell_offer_buyer_does_not_have_enough_balance_shouldnt_work(){
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1100);
	
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 1200)); 
		let offer_id = GatedMarketplace::offers_by_item(0, 0).iter().next().unwrap().clone();

		assert_noop!(GatedMarketplace::take_sell_offer(Origin::signed(2), offer_id, m_id, 0, 0), Error::<Test>::NotEnoughBalance);
	});
}

#[test]
fn take_buy_offer_works(){
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1300);
	
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);

		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 1001)); 
		let offer_id = GatedMarketplace::offers_by_account(1).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());
		
		assert_ok!(GatedMarketplace::enlist_buy_offer(Origin::signed(2), m_id, 0, 0, 1200)); 
		let offer_id2 = GatedMarketplace::offers_by_account(2).iter().next().unwrap().clone();
		assert_eq!(GatedMarketplace::offers_info(offer_id2).unwrap().offer_type, OfferType::BuyOrder);
		
		assert_ok!(GatedMarketplace::take_buy_offer(Origin::signed(1), offer_id2, m_id, 0, 0));
		assert_eq!(GatedMarketplace::offers_by_item(0, 0).len(), 0);
		assert_eq!(GatedMarketplace::offers_info(offer_id).unwrap().status, OfferStatus::Closed);
	});
}

#[test]
fn take_buy_offer_only_owner_can_accept_buy_offers_shouldnt_work(){
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1300);
	
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		

		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 1001)); 
		let offer_id = GatedMarketplace::offers_by_account(1).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());
		
		assert_ok!(GatedMarketplace::enlist_buy_offer(Origin::signed(2), m_id, 0, 0, 1200)); 
		let offer_id2 = GatedMarketplace::offers_by_account(2).iter().next().unwrap().clone();
		assert_eq!(GatedMarketplace::offers_info(offer_id2).unwrap().offer_type, OfferType::BuyOrder);

		assert_noop!(GatedMarketplace::take_buy_offer(Origin::signed(2), offer_id2, m_id, 0, 0), Error::<Test>::NotOwner);
	});
}

#[test]
fn take_buy_offer_id_does_not_exist_shouldnt_work(){
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1300);
	
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 1001)); 
		let offer_id = GatedMarketplace::offers_by_account(1).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());
		
		assert_ok!(GatedMarketplace::enlist_buy_offer(Origin::signed(2), m_id, 0, 0, 1200)); 
		let offer_id2 = GatedMarketplace::offers_by_account(2).iter().next().unwrap().clone();
		assert_eq!(GatedMarketplace::offers_info(offer_id2).unwrap().offer_type, OfferType::BuyOrder);

		let offer_id3 = offer_id2.using_encoded(blake2_256);
		assert_noop!(GatedMarketplace::take_buy_offer(Origin::signed(1), offer_id3, m_id, 0, 0), Error::<Test>::OfferNotFound);

	});
}

#[test]
fn take_buy_offer_user_does_not_have_enough_balance_shouldnt_work(){
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1300);
	
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 1001)); 
		let offer_id = GatedMarketplace::offers_by_account(1).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());
		
		assert_ok!(GatedMarketplace::enlist_buy_offer(Origin::signed(2), m_id, 0, 0, 1200)); 
		let offer_id2 = GatedMarketplace::offers_by_account(2).iter().next().unwrap().clone();
		assert_eq!(GatedMarketplace::offers_info(offer_id2).unwrap().offer_type, OfferType::BuyOrder);
		
		Balances::make_free_balance_be(&2, 0);
		assert_noop!(GatedMarketplace::take_buy_offer(Origin::signed(1), offer_id2, m_id, 0, 0), Error::<Test>::NotEnoughBalance);
	});
}

#[test]
fn remove_sell_offer_works(){
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1300);
	
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 1001)); 
		let offer_id = GatedMarketplace::offers_by_account(1).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());
		
		assert_ok!(GatedMarketplace::remove_offer(Origin::signed(1), offer_id, m_id, 0, 0));
		assert_eq!(GatedMarketplace::offers_by_account(1).len(), 0);
		assert!(GatedMarketplace::offers_info(offer_id).is_none());
	});
}

#[test]
fn remove_buy_offer_works(){
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1300);
	
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 1001)); 
		let offer_id = GatedMarketplace::offers_by_account(1).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());

		assert_ok!(GatedMarketplace::enlist_buy_offer(Origin::signed(2), m_id, 0, 0, 1001)); 
		let offer_id2 = GatedMarketplace::offers_by_account(2).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id2).is_some());
		
		assert_ok!(GatedMarketplace::remove_offer(Origin::signed(2), offer_id2, m_id, 0, 0));
		assert_eq!(GatedMarketplace::offers_by_account(2).len(), 0);
		assert!(GatedMarketplace::offers_info(offer_id2).is_none());
	});
}

#[test]
fn remove_offer_id_does_not_exist_sholdnt_work(){
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1300);
	
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 1001)); 
		let offer_id = GatedMarketplace::offers_by_account(1).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());
		
		let offer_id2 = offer_id.using_encoded(blake2_256);
		assert_noop!(GatedMarketplace::remove_offer(Origin::signed(2), offer_id2, m_id, 0, 0), Error::<Test>::OfferNotFound);
	});
}

#[test]
fn remove_offer_creator_doesnt_match_sholdnt_work(){
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1300);
	
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);
		
		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 1001)); 
		let offer_id = GatedMarketplace::offers_by_account(1).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());
		
		assert_noop!(GatedMarketplace::remove_offer(Origin::signed(2), offer_id, m_id, 0, 0), Error::<Test>::CannotRemoveOffer);
	});
}

#[test]
fn remove_offer_status_is_closed_shouldnt_work(){
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 1300);
	
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1), 2, create_label("my marketplace")));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		
		assert_ok!(Uniques::create(Origin::signed(1), 0, 1));
		assert_ok!(Uniques::mint(Origin::signed(1), 0, 0, 1));
		assert_eq!(Uniques::owner(0, 0).unwrap(), 1);

		assert_ok!(GatedMarketplace::enlist_sell_offer(Origin::signed(1), m_id, 0, 0, 1001)); 
		let offer_id = GatedMarketplace::offers_by_account(1).iter().next().unwrap().clone();
		assert!(GatedMarketplace::offers_info(offer_id).is_some());
		
		assert_ok!(GatedMarketplace::enlist_buy_offer(Origin::signed(2), m_id, 0, 0, 1200)); 
		let offer_id2 = GatedMarketplace::offers_by_account(2).iter().next().unwrap().clone();
		assert_eq!(GatedMarketplace::offers_info(offer_id2).unwrap().offer_type, OfferType::BuyOrder);
		
		assert_ok!(GatedMarketplace::take_buy_offer(Origin::signed(1), offer_id2, m_id, 0, 0));
		assert_eq!(GatedMarketplace::offers_by_item(0, 0).len(), 0);
		assert_eq!(GatedMarketplace::offers_info(offer_id).unwrap().status, OfferStatus::Closed);

		assert_noop!(GatedMarketplace::remove_offer(Origin::signed(2), offer_id2, m_id, 0, 0), Error::<Test>::CannotDeleteOffer);
	});
	
}