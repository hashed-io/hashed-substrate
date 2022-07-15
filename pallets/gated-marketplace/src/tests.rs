use crate::{mock::*, Error, types::*, Custodians};
use codec::Encode;
use frame_support::{assert_ok, BoundedVec, traits::{Len, ConstU32}, assert_noop};
use sp_io::hashing::blake2_256;

fn create_label( label : &str ) -> BoundedVec<u8, LabelMaxLen> {
	let s: Vec<u8> = label.as_bytes().into();
	s.try_into().unwrap_or_default()
}

fn dummy_notes() -> BoundedVec<u8, NotesMaxLen> {
	let s: Vec<u8> = "Notes".as_bytes().into();
	s.try_into().unwrap_or_default()
}

fn create_file(name: &str, cid: &str, create_custodian_file: bool) -> ApplicationField {
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
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), None ));
		assert_ok!(GatedMarketplace::apply(Origin::signed(4),m_id, create_application_fields(1), None ));
		let app_id = GatedMarketplace::applications_by_account(3,m_id).unwrap();
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
		assert_ok!(GatedMarketplace::apply(Origin::signed(4),m_id, create_application_fields(1), None ));
		let app_id = GatedMarketplace::applications_by_account(4,m_id).unwrap();
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
		assert_ok!(GatedMarketplace::apply(Origin::signed(3),m_id, create_application_fields(2), None ));

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

//add authorities

#[test]
fn add_authority_appraiser_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Appraiser, m_id));
	});
}

#[test]
fn add_authority_admin_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Admin, m_id));
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
fn remove_authority_appraiser_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Appraiser, m_id));
		assert_ok!(GatedMarketplace::remove_authority(Origin::signed(1), 3, MarketplaceAuthority::Appraiser, m_id));
	});
}

#[test]
fn remove_authority_admin_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Admin, m_id));
		assert_ok!(GatedMarketplace::remove_authority(Origin::signed(1), 3, MarketplaceAuthority::Admin, m_id));
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
		assert_noop!(GatedMarketplace::remove_authority(Origin::signed(3), 3, MarketplaceAuthority::Admin, m_id), Error::<Test>::NegateRemoveAdminItself);
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
fn remove_authority_only_owner_can_remove_admins_work(){
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
		assert_noop!(GatedMarketplace::update_marketplace(Origin::signed(1), m_id_2, create_label("my marketplace 2")), Error::<Test>::MarketplaceNotFound);
	});
	
}

#[test]
fn update_marketplace_user_without_permission_shouldnt_work(){
	new_test_ext().execute_with(|| {
		//user should be an admin or owner
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Appraiser, m_id));
		assert_noop!(GatedMarketplace::update_marketplace(Origin::signed(3), m_id, create_label("my marketplace2")), Error::<Test>::CannotEnroll);
	});
}

#[test]
fn update_label_marketplace_works(){
	new_test_ext().execute_with(|| {
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert!(GatedMarketplace::marketplaces(m_id).is_some());		
		assert_ok!(GatedMarketplace::update_marketplace(Origin::signed(1), m_id, create_label("my marketplace 2")));
		assert!(GatedMarketplace::marketplaces(m_id).is_some() );	
	});
}


//Delete selected marketplace

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
		//user should be an admin or owner
		assert_ok!(GatedMarketplace::create_marketplace(Origin::signed(1),2, create_label("my marketplace") ));
		let m_id = create_label("my marketplace").using_encoded(blake2_256);
		assert_ok!(GatedMarketplace::add_authority(Origin::signed(1), 3, MarketplaceAuthority::Appraiser, m_id));
		assert_noop!(GatedMarketplace::remove_marketplace(Origin::signed(3), m_id), Error::<Test>::CannotEnroll);
	});
}

