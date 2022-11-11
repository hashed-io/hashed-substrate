use crate::{mock::*, Error, types::*, Config};
use frame_support::{assert_ok, BoundedVec, traits::ConstU32, assert_noop, error::BadOrigin, bounded_vec, assert_err};
use pallet_rbac::types::RoleBasedAccessControl;
use sp_io::hashing::blake2_256;
use sp_runtime::DispatchResult;
use std::vec;
use codec::Encode;


type RbacErr = pallet_rbac::Error<Test>;

#[allow(dead_code)]
fn pallet_id() ->[u8;32]{
	FundAdmin::pallet_id().to_id()
}

#[allow(dead_code)]
fn pallet_name()-> pallet_rbac::types::IdOrVec {
	pallet_rbac::types::IdOrVec::Vec(
		"pallet_test".as_bytes().to_vec()
	)
}

fn return_field_name(name: &str) -> FieldName {
    let name: BoundedVec<u8, ConstU32<100>> = name.as_bytes().to_vec().try_into().unwrap_or_default();
    name
} 

fn return_field_description(description: &str) -> FieldDescription {
    let description: BoundedVec<u8, ConstU32<400>> = description.as_bytes().to_vec().try_into().unwrap_or_default();
    description
}

fn register_administrator() -> DispatchResult {
    FundAdmin::sudo_add_administrator(
        Origin::root(),
        1,
        return_field_name("Administrator"),
        ).map_err(|_| Error::<Test>::UserAlreadyRegistered
    )?;
    Ok(())
}

fn return_user(user_account:u64, user_name: Option<&str>, user_role: Option<ProxyRole>, action: CUDAction) -> BoundedVec<(u64, Option<BoundedVec<FieldName, MaxBoundedVecs>>,
    Option<ProxyRole>,
    CUDAction), MaxRegistrationsAtTime> {
    let mut users: BoundedVec<(u64, Option<BoundedVec<FieldName, MaxBoundedVecs>>,
        Option<ProxyRole>,
        CUDAction), MaxRegistrationsAtTime> = bounded_vec![];
    let field_name: BoundedVec<FieldName, MaxBoundedVecs> = BoundedVec::try_from(vec![return_field_name(user_name.unwrap_or_default())]).unwrap_or_default();
    users.try_push((user_account, Some(field_name), user_role, action)).unwrap_or_default();
    users
}

fn field_name_to_string(boundedvec: &BoundedVec<u8, ConstU32<100>>) -> String {
	let mut s = String::new();
	for b in boundedvec.iter() {
		s.push(*b as char);
	}
	s
}

#[allow(dead_code)]
fn field_description_to_string(boundedvec: &BoundedVec<u8, ConstU32<400>>) -> String {
	let mut s = String::new();
	for b in boundedvec.iter() {
		s.push(*b as char);
	}
	s
}

fn create_documents_field(n_files: u32) -> 
	BoundedVec<(BoundedVec<u8,ConstU32<100> >,BoundedVec<u8,ConstU32<100>> ), MaxDocuments> {
	let mut files = Vec::<(BoundedVec<u8,ConstU32<100> >,BoundedVec<u8,ConstU32<100>> )>::default();
	for i in 0..n_files{
		let file_name = format!("file{}",i.to_string());
		let cid = format!("cid{}",i.to_string());
		files.push( (file_name.encode().try_into().unwrap_or_default(), cid.encode().try_into().unwrap_or_default()) );
	}
	BoundedVec::<(BoundedVec<u8,ConstU32<100> >,BoundedVec<u8,ConstU32<100>> ), MaxDocuments>::try_from( files).unwrap_or_default()
}

fn return_edit_field_name(name: &str) -> BoundedVec<FieldName, MaxBoundedVecs> {
    let name: BoundedVec<u8, ConstU32<100>> = name.as_bytes().to_vec().try_into().unwrap_or_default();
    let mut field_name: BoundedVec<FieldName, MaxBoundedVecs> = bounded_vec![];
    field_name.try_push(name).unwrap_or_default();
    field_name
}

fn return_edit_field_description(description: &str) -> BoundedVec<FieldDescription, MaxBoundedVecs> {
    let description: BoundedVec<u8, ConstU32<400>> = description.as_bytes().to_vec().try_into().unwrap_or_default();
    let mut field_description: BoundedVec<FieldDescription, MaxBoundedVecs> = bounded_vec![];
    field_description.try_push(description).unwrap_or_default();
    field_description
}

fn generate_expenditures(expenditures: Vec<(ExpenditureType, CUDAction, Option<[u8;32]>)>) -> 
BoundedVec<(
    Option<BoundedVec<FieldName, MaxBoundedVecs>>,
    Option<ExpenditureType>,
    Option<u64>,
    Option<BoundedVec<FieldDescription, MaxBoundedVecs>>,
    Option<u32>,
    CUDAction,
    Option<[u8;32]>,
), MaxRegistrationsAtTime> {
    let mut expenditures_vec: BoundedVec<(
        Option<BoundedVec<FieldName, MaxBoundedVecs>>,
        Option<ExpenditureType>,
        Option<u64>,
        Option<BoundedVec<FieldDescription, MaxBoundedVecs>>,
        Option<u32>,
        CUDAction,
        Option<[u8;32]>,
    ), MaxRegistrationsAtTime> = bounded_vec![];

    for i in 0..expenditures.len() {
        let expenditure = expenditures[i].clone();
        let expenditure_name = format!("expenditure: {}", i.to_string());
        let expenditure_description = format!("naics_code: {}", i.to_string());
        let expenditure_field_name = return_edit_field_name(&expenditure_name);
        let expenditure_field_description = return_edit_field_description(&expenditure_description);
        expenditures_vec.try_push((
            Some(expenditure_field_name),
            Some(expenditure.0),
            Some(1000),
            Some(expenditure_field_description),
            Some(1258),
            expenditure.1,
            expenditure.2,
        )).unwrap_or_default();
    }
    expenditures_vec
}

fn generate_users_assignment(users: Vec<(u64, ProxyRole, AssignAction)>) -> 
BoundedVec<(u64, ProxyRole, AssignAction), MaxRegistrationsAtTime> {
    let mut users_vec: BoundedVec<(u64, ProxyRole, AssignAction), MaxRegistrationsAtTime> = bounded_vec![];
    for i in 0..users.len() {
        let user = users[i].clone();
        users_vec.try_push((user.0, user.1, user.2)).unwrap_or_default();
    }
    users_vec
}

fn generate_users_registration(users: Vec<(u64, Option<&str>, Option<ProxyRole>, CUDAction)>) ->
BoundedVec<(u64, Option<BoundedVec<FieldName, MaxBoundedVecs>>, Option<ProxyRole>, CUDAction), MaxRegistrationsAtTime> {
    let mut users_vec: BoundedVec<(
        u64,
        Option<BoundedVec<FieldName, MaxBoundedVecs>>,
        Option<ProxyRole>,
        CUDAction),
        MaxRegistrationsAtTime> = bounded_vec![];
    for i in 0..users.len() {
        let user = users[i].clone();
        let user_name = return_edit_field_name(user.1.unwrap_or_default());
        users_vec.try_push((user.0, Some(user_name), user.2, user.3)).unwrap_or_default();
    }
    users_vec
}

// I N I T I A L 
// -----------------------------------------------------------------------------------------
#[test]
fn cannon_initialize_pallet_twice_shouldnt_work() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            FundAdmin::initial_setup(
                Origin::root()),
            RbacErr::ScopeAlreadyExists);
    });
}

#[test]
fn sudo_register_administrator_account_works() {
    new_test_ext().execute_with(|| {
        let alice_name = return_field_name("Alice Keys");
        assert_ok!(FundAdmin::sudo_add_administrator(
            Origin::root(),
            2,
            alice_name.clone()
        ));
        assert!(FundAdmin::users_info(2).is_some());
    });
}

#[test]
fn sudo_a_non_sudo_user_cannot_register_administrator_account_shouldnt_work() {
    new_test_ext().execute_with(|| {
        let alice_name = return_field_name("Alice Keys");
        assert_noop!(
            FundAdmin::sudo_add_administrator(Origin::signed(1), 2, alice_name.clone()),
            BadOrigin
        );
    });
}

#[test]
fn sudo_cannot_register_an_administrator_account_twice_shouldnt_work() {
    new_test_ext().execute_with(|| {
        let alice_name = return_field_name("Alice Keys");
        assert_ok!(FundAdmin::sudo_add_administrator(
            Origin::root(),
            2,
            alice_name.clone()
        ));
        assert_noop!(
            FundAdmin::sudo_add_administrator(Origin::root(), 2, alice_name.clone()),
            Error::<Test>::UserAlreadyRegistered
        );
    });
}

#[test]
fn sudo_delete_administrator_account_works() {
    new_test_ext().execute_with(|| {
        let alice_name = return_field_name("Alice Keys");
        assert_ok!(FundAdmin::sudo_add_administrator(
            Origin::root(),
            2,
            alice_name.clone()
        ));
        assert!(FundAdmin::users_info(2).is_some());

        assert_ok!(FundAdmin::sudo_remove_administrator(
            Origin::root(),
            2,
        ));
        assert!(FundAdmin::users_info(2).is_none());
    });
}

#[test]
fn sudo_cannot_delete_an_administrator_account_that_doesnt_exist_shouldnt_work() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            FundAdmin::sudo_remove_administrator(Origin::root(), 2),
            Error::<Test>::UserNotRegistered
        );
    });
}

#[test]
fn sudo_administrator_can_remove_another_administrator_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(FundAdmin::sudo_add_administrator(
            Origin::root(),
            2,
            return_field_name("Alice Keys")
        ));
        assert!(FundAdmin::users_info(2).is_some());

        assert_ok!(FundAdmin::sudo_add_administrator(
            Origin::root(),
            3,
            return_field_name("Bob Keys")
        ));
        assert!(FundAdmin::users_info(3).is_some());

        assert_ok!(FundAdmin::sudo_remove_administrator(
            Origin::root(),
            2,
        ));
        assert!(FundAdmin::users_info(2).is_none());
    });
}

#[test]
fn sudo_administrator_can_remove_themselves_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(FundAdmin::sudo_add_administrator(
            Origin::root(),
            2,
            return_field_name("Alice Keys")
        ));
        assert!(FundAdmin::users_info(2).is_some());

        assert_ok!(FundAdmin::sudo_remove_administrator(
            Origin::root(),
            2,
        ));
        assert!(FundAdmin::users_info(2).is_none());
    });
}

// U S E R S
// -----------------------------------------------------------------------------------------
#[test]
fn users_register_administrator_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            return_user(2, Some("Alice Administrator"), Some(ProxyRole::Administrator), CUDAction::Create)
        ));

        assert!(FundAdmin::users_info(2).is_some());
    });
}

#[test]
fn users_register_builder_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            return_user(2, Some("Alice Builder"), Some(ProxyRole::Builder), CUDAction::Create)
        ));

        assert!(FundAdmin::users_info(2).is_some());
    });
}

#[test]
fn users_register_investor_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            return_user(2, Some("Alice Investor"), Some(ProxyRole::Investor), CUDAction::Create)
        ));

        assert!(FundAdmin::users_info(2).is_some());
    });
}

#[test]
fn users_register_issuer_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            return_user(2, Some("Alice Issuer"), Some(ProxyRole::Issuer), CUDAction::Create)
        ));

        assert!(FundAdmin::users_info(2).is_some());
    });
}

#[test]
fn users_register_regional_center_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            return_user(2, Some("Alice Regional Center"), Some(ProxyRole::RegionalCenter), CUDAction::Create)
        ));

        assert!(FundAdmin::users_info(2).is_some());
    });
}

#[test]
fn users_a_non_registered_admin_tries_to_register_an_account_shouldnt_work() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            FundAdmin::users(
                Origin::signed(1),
                return_user(2, Some("Alice Regional Center"), Some(ProxyRole::RegionalCenter), CUDAction::Create)
            ),
            RbacErr::NotAuthorized
        );
    });
}

#[test]
fn users_a_registered_admin_tries_to_register_an_account_twice_shouldnt_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            return_user(2, Some("Alice Regional Center"), Some(ProxyRole::RegionalCenter), CUDAction::Create)
        ));

        assert_noop!(
            FundAdmin::users(
                Origin::signed(1),
                return_user(2, Some("Alice Regional Center"), Some(ProxyRole::RegionalCenter), CUDAction::Create)
            ),
            Error::<Test>::UserAlreadyRegistered
        );
    });
}

#[test]
fn users_update_a_registered_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            return_user(2, Some("Alice Regional Center"), Some(ProxyRole::RegionalCenter), CUDAction::Create)
        ));

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            return_user(2, Some("Alice Regional Center Updated"), None, CUDAction::Update)
        ));

        assert_eq!(field_name_to_string(&FundAdmin::users_info(2).unwrap().name), String::from("Alice Regional Center Updated"));
    });
}

#[test]
fn users_update_role_of_a_registered_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            return_user(2, Some("Alice Regional Center"), Some(ProxyRole::RegionalCenter), CUDAction::Create)
        ));

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            return_user(2, Some("Alice Investor"), Some(ProxyRole::Investor), CUDAction::Update)
        ));

        assert_eq!(FundAdmin::users_info(2).unwrap().role, ProxyRole::Investor);
        assert_eq!(FundAdmin::users_info(2).unwrap().name, return_field_name("Alice Investor"));00
    });
}


#[test]
fn users_update_a_non_registered_account_shouldnt_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_noop!(
            FundAdmin::users(
                Origin::signed(1),
                return_user(2, Some("Alice Regional Center"), Some(ProxyRole::RegionalCenter), CUDAction::Update)
            ),
            Error::<Test>::UserNotRegistered
        );
    });
}

//TODO: cannot update a registered users if the user has assigned projects

#[test]
fn users_delete_a_registered_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            return_user(2, Some("Alice Regional Center"), Some(ProxyRole::RegionalCenter), CUDAction::Create)
        ));

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            return_user(2, None, None, CUDAction::Delete)
        ));

        assert!(FundAdmin::users_info(2).is_none());
    });
}

#[test]
fn users_delete_a_non_registered_account_shouldnt_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_noop!(
            FundAdmin::users(
                Origin::signed(1),
                return_user(2, None, None, CUDAction::Delete)
            ),
            Error::<Test>::UserNotRegistered
        );
    });
}

#[test]
fn edit_user_user_edits_their_own_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            return_user(2, Some("Bob Regional Center"), Some(ProxyRole::RegionalCenter), CUDAction::Create)
        ));

        assert_ok!(FundAdmin::users_edit_user(
            Origin::signed(2),
            Some(return_edit_field_name("Bob Regional Center Updated")),
            Some(return_edit_field_name("QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg")),
            Some(return_edit_field_name("bob@testing.com")),
            None,
        ));

        assert_eq!(field_name_to_string(&FundAdmin::users_info(2).unwrap().name), String::from("Bob Regional Center Updated"));
        assert_eq!(field_name_to_string(&FundAdmin::users_info(2).unwrap().image), String::from("QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg"));
        assert_eq!(field_name_to_string(&FundAdmin::users_info(2).unwrap().email), String::from("bob@testing.com"));
    });
}

#[test]
fn edit_user_investor_user_uploads_their_documentation_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            return_user(2, Some("Alice Investor Center"), Some(ProxyRole::Investor), CUDAction::Create)
        ));

        assert_ok!(FundAdmin::users_edit_user(
            Origin::signed(2),
            Some(return_edit_field_name("Alice Investor Updated")),
            Some(return_edit_field_name("QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg")),
            Some(return_edit_field_name("alice@testing.com")),
            Some(create_documents_field(1)),
        ));

        assert_eq!(field_name_to_string(&FundAdmin::users_info(2).unwrap().name), String::from("Alice Investor Updated"));
        assert_eq!(field_name_to_string(&FundAdmin::users_info(2).unwrap().image), String::from("QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg"));
        assert_eq!(field_name_to_string(&FundAdmin::users_info(2).unwrap().email), String::from("alice@testing.com"));
        assert_eq!(FundAdmin::users_info(2).unwrap().documents, Some(create_documents_field(1)));
    });
}

#[test]
fn edit_user_no_investor_user_tries_to_upload_documentos_to_their_profile(){
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            return_user(2, Some("Alice Regional Center"), Some(ProxyRole::RegionalCenter), CUDAction::Create)
        ));

        assert_noop!(
            FundAdmin::users_edit_user(
                Origin::signed(2),
                Some(return_edit_field_name("Alice Regional Center Updated")),
                Some(return_edit_field_name("QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg")),
                Some(return_edit_field_name("alice@testing.com")),
                Some(create_documents_field(1)),
            ), Error::<Test>::UserIsNotAnInvestor
        );

    });
}

#[test]
fn edit_user_no_registered_user_tries_to_edit_their_profile(){
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_noop!(
            FundAdmin::users_edit_user(
                Origin::signed(2),
                Some(return_edit_field_name("Alice Regional Center Updated")),
                Some(return_edit_field_name("QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg")),
                Some(return_edit_field_name("noregistered@testing.com")),
                None
            ), Error::<Test>::UserNotRegistered
        );
    });
}

//TODO: cannot delete a registered users if the user has assigned projects

// P R O J E C T S
// ------------------------------------------------------------------------------------------------

#[test]
fn projects_create_project_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::projects_create_project(
            Origin::signed(1),
            return_field_name("Project 1"),
            return_field_description("Project 1 description"),
            return_field_name("QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg"),
            return_field_name("New York City"),
            1662867646,
            1694403646,
            generate_expenditures([
                (ExpenditureType::HardCost, CUDAction::Create, None),
                (ExpenditureType::SoftCost, CUDAction::Create, None),
                (ExpenditureType::Operational, CUDAction::Create, None),
                (ExpenditureType::Others, CUDAction::Create, None)].to_vec()),
                None,
        ));

        let project_id = return_field_name("Project 1").using_encoded(blake2_256);

        assert!(FundAdmin::projects_info(project_id).is_some());
             
    });
}

#[test]
fn projects_create_project_and_assign_users_works(){
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            generate_users_registration([
                (2, Some("Bob Regional Center"), Some(ProxyRole::RegionalCenter), CUDAction::Create),
                (3, Some("Alice Investor"), Some(ProxyRole::Investor), CUDAction::Create),
                (4, Some("Charlie Builder"), Some(ProxyRole::Builder), CUDAction::Create),
                (5, Some("Eve Issuer"), Some(ProxyRole::Issuer), CUDAction::Create)].to_vec()
        )));

        assert_ok!(FundAdmin::projects_create_project(
            Origin::signed(1),
            return_field_name("Project 1"),
            return_field_description("Project 1 description"),
            return_field_name("QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg"),
            return_field_name("New York City"),
            1662867646,
            1694403646,
            generate_expenditures([
                (ExpenditureType::HardCost, CUDAction::Create, None),
                (ExpenditureType::SoftCost, CUDAction::Create, None),
                (ExpenditureType::Operational, CUDAction::Create, None),
                (ExpenditureType::Others, CUDAction::Create, None)].to_vec()),
            Some(generate_users_assignment([
                (2, ProxyRole::RegionalCenter, AssignAction::Assign),
                (3, ProxyRole::Investor, AssignAction::Assign),
                (4, ProxyRole::Builder, AssignAction::Assign),
                (5, ProxyRole::Issuer, AssignAction::Assign)].to_vec())),
        ));

        let project_id = return_field_name("Project 1").using_encoded(blake2_256);

        assert!(FundAdmin::projects_info(project_id).is_some());
        assert_eq!(FundAdmin::projects_info(project_id).unwrap().regional_center.unwrap().into_inner()[0], 2);
        assert_eq!(FundAdmin::projects_info(project_id).unwrap().investor.unwrap().into_inner()[0], 3);
        assert_eq!(FundAdmin::projects_info(project_id).unwrap().builder.unwrap().into_inner()[0], 4);
        assert_eq!(FundAdmin::projects_info(project_id).unwrap().issuer.unwrap().into_inner()[0], 5);
    });           
}

#[test]
fn projects_no_administrator_account_tries_to_create_project_shouldnt_work() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            FundAdmin::projects_create_project(
                Origin::signed(1),
                return_field_name("Project 1"),
                return_field_description("Project 1 description"),
                return_field_name("QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg"),
                return_field_name("New York City"),
                1662867646,
                1694403646,
                generate_expenditures([
                    (ExpenditureType::HardCost, CUDAction::Create, None),
                    (ExpenditureType::SoftCost, CUDAction::Create, None),
                    (ExpenditureType::Operational, CUDAction::Create, None),
                    (ExpenditureType::Others, CUDAction::Create, None)].to_vec()),
                None,
            ), RbacErr::NotAuthorized
        );
    });
}

#[test]
fn projects_admnistrator_creates_project_with_creation_date_bigger_than_completion_date_shouldnt_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_noop!(
            FundAdmin::projects_create_project(
                Origin::signed(1),
                return_field_name("Project 1"),
                return_field_description("Project 1 description"),
                return_field_name("QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg"),
                return_field_name("New York City"),
                1694403646,
                1662867646,
                generate_expenditures([
                    (ExpenditureType::HardCost, CUDAction::Create, None),
                    (ExpenditureType::SoftCost, CUDAction::Create, None),
                    (ExpenditureType::Operational, CUDAction::Create, None),
                    (ExpenditureType::Others, CUDAction::Create, None)].to_vec()),
                None,
            ), Error::<Test>::CompletionDateMustBeLater
        );
    });
}

#[test]
fn projects_create_project_allows_multiple_investors_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            generate_users_registration([
                (2, Some("Bob Investor"), Some(ProxyRole::Investor), CUDAction::Create),
                (3, Some("Alice Investor"), Some(ProxyRole::Investor), CUDAction::Create),
                (4, Some("Charlie Investor"), Some(ProxyRole::Investor), CUDAction::Create),
                (5, Some("Eve Investor"), Some(ProxyRole::Investor), CUDAction::Create)].to_vec()
        )));

        assert_ok!(FundAdmin::projects_create_project(
            Origin::signed(1),
            return_field_name("Project 1"),
            return_field_description("Project 1 description"),
            return_field_name("QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg"),
            return_field_name("New York City"),
            1662867646,
            1694403646,
            generate_expenditures([
                (ExpenditureType::HardCost, CUDAction::Create, None),
                (ExpenditureType::SoftCost, CUDAction::Create, None),
                (ExpenditureType::Operational, CUDAction::Create, None),
                (ExpenditureType::Others, CUDAction::Create, None)].to_vec()),
            Some(generate_users_assignment([
                (2, ProxyRole::Investor, AssignAction::Assign),
                (3, ProxyRole::Investor, AssignAction::Assign),
                (4, ProxyRole::Investor, AssignAction::Assign),
                (5, ProxyRole::Investor, AssignAction::Assign)].to_vec())),
        ));

        let project_id = return_field_name("Project 1").using_encoded(blake2_256);

        assert!(FundAdmin::projects_info(project_id).is_some());
        assert_eq!(FundAdmin::projects_info(project_id).unwrap().investor.unwrap().into_inner()[0], 2);
        assert_eq!(FundAdmin::projects_info(project_id).unwrap().investor.unwrap().into_inner()[1], 3);
        assert_eq!(FundAdmin::projects_info(project_id).unwrap().investor.unwrap().into_inner()[2], 4);
        assert_eq!(FundAdmin::projects_info(project_id).unwrap().investor.unwrap().into_inner()[3], 5);

    });
}

#[test]
fn projects_create_project_only_allows_one_builder_shouldnt_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            generate_users_registration([
                (2, Some("Bob Builder"), Some(ProxyRole::Builder), CUDAction::Create),
                (3, Some("Alice Builder"), Some(ProxyRole::Builder), CUDAction::Create)].to_vec()
        )));

        assert_err!(
            FundAdmin::projects_create_project(
                Origin::signed(1),
                return_field_name("Project 1"),
                return_field_description("Project 1 description"),
                return_field_name("QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg"),
                return_field_name("New York City"),
                1662867646,
                1694403646,
                generate_expenditures([
                    (ExpenditureType::HardCost, CUDAction::Create, None),
                    (ExpenditureType::SoftCost, CUDAction::Create, None),
                    (ExpenditureType::Operational, CUDAction::Create, None),
                    (ExpenditureType::Others, CUDAction::Create, None)].to_vec()),
                Some(generate_users_assignment([
                    (2, ProxyRole::Builder, AssignAction::Assign),
                    (3, ProxyRole::Builder, AssignAction::Assign),].to_vec())),
            ), Error::<Test>::MaxBuildersPerProjectReached
        );
    });
}

#[test]
fn projects_create_project_only_allows_one_issuer_shouldnt_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            generate_users_registration([
                (2, Some("Bob Issuer"), Some(ProxyRole::Issuer), CUDAction::Create),
                (3, Some("Alice Issuer"), Some(ProxyRole::Issuer), CUDAction::Create)].to_vec()
        )));

        assert_err!(
            FundAdmin::projects_create_project(
                Origin::signed(1),
                return_field_name("Project 1"),
                return_field_description("Project 1 description"),
                return_field_name("QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg"),
                return_field_name("New York City"),
                1662867646,
                1694403646,
                generate_expenditures([
                    (ExpenditureType::HardCost, CUDAction::Create, None),
                    (ExpenditureType::SoftCost, CUDAction::Create, None),
                    (ExpenditureType::Operational, CUDAction::Create, None),
                    (ExpenditureType::Others, CUDAction::Create, None)].to_vec()),
                Some(generate_users_assignment([
                    (2, ProxyRole::Issuer, AssignAction::Assign),
                    (3, ProxyRole::Issuer, AssignAction::Assign),].to_vec())),
            ), Error::<Test>::MaxIssuersPerProjectReached
        );
    });
}

#[test]
fn projects_create_project_only_allows_one_regional_center_shouldnt_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            Origin::signed(1),
            generate_users_registration([
                (2, Some("Bob Regional Center"), Some(ProxyRole::RegionalCenter), CUDAction::Create),
                (3, Some("Alice Regional Center"), Some(ProxyRole::RegionalCenter), CUDAction::Create)].to_vec()
        )));

        assert_err!(
            FundAdmin::projects_create_project(
                Origin::signed(1),
                return_field_name("Project 1"),
                return_field_description("Project 1 description"),
                return_field_name("QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg"),
                return_field_name("New York City"),
                1662867646,
                1694403646,
                generate_expenditures([
                    (ExpenditureType::HardCost, CUDAction::Create, None),
                    (ExpenditureType::SoftCost, CUDAction::Create, None),
                    (ExpenditureType::Operational, CUDAction::Create, None),
                    (ExpenditureType::Others, CUDAction::Create, None)].to_vec()),
                Some(generate_users_assignment([
                    (2, ProxyRole::RegionalCenter, AssignAction::Assign),
                    (3, ProxyRole::RegionalCenter, AssignAction::Assign),].to_vec())),
            ), Error::<Test>::MaxRegionalCenterPerProjectReached
        );
    });
}

#[test]
fn projects_create_project_initiliaze_drawdowns_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::projects_create_project(
            Origin::signed(1),
            return_field_name("Project 1"),
            return_field_description("Project 1 description"),
            return_field_name("QmYTWeZrWH1nZ2pn3VxuJ4t3UqKDZoLoXob2hcq4vemxbK.jpeg"),
            return_field_name("New York City"),
            1662867646,
            1694403646,
            generate_expenditures([
                (ExpenditureType::HardCost, CUDAction::Create, None),
                (ExpenditureType::SoftCost, CUDAction::Create, None),
                (ExpenditureType::Operational, CUDAction::Create, None),
                (ExpenditureType::Others, CUDAction::Create, None)].to_vec()),
            None,
        ));

        let project_id = return_field_name("Project 1").using_encoded(blake2_256);

        assert!(!FundAdmin::drawdowns_by_project(project_id).is_empty());
    });
}

