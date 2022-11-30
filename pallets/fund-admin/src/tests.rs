use crate::{mock::*, types::*, Config, Error};
use frame_support::{
	assert_noop, assert_ok, bounded_vec, error::BadOrigin, traits::ConstU32, BoundedVec,
};
use pallet_rbac::types::RoleBasedAccessControl;
use sp_io::hashing::blake2_256;
use sp_runtime::DispatchResult;
use std::vec;

type RbacErr = pallet_rbac::Error<Test>;

fn pallet_id() -> [u8; 32] {
	FundAdmin::pallet_id().to_id()
}

fn pallet_name() -> pallet_rbac::types::IdOrVec {
	pallet_rbac::types::IdOrVec::Vec("pallet_test".as_bytes().to_vec())
}

fn make_field_name(name: &str) -> FieldName {
	let name: BoundedVec<u8, ConstU32<100>> =
		name.as_bytes().to_vec().try_into().unwrap_or_default();
	name
}

fn make_field_description(description: &str) -> FieldDescription {
	let description: BoundedVec<u8, ConstU32<400>> =
		description.as_bytes().to_vec().try_into().unwrap_or_default();
	description
}

fn register_administrator() -> DispatchResult {
	FundAdmin::sudo_add_administrator(Origin::root(), 1, make_field_name("Administrator"))
		.map_err(|_| Error::<Test>::UserAlreadyRegistered)?;
	Ok(())
}

fn make_user(
	user_account: u64,
	user_name: Option<&str>,
	user_role: Option<ProxyRole>,
	action: CUDAction,
) -> BoundedVec<(u64, Option<FieldName>, Option<ProxyRole>, CUDAction), MaxRegistrationsAtTime> {
	let mut users: BoundedVec<
		(u64, Option<FieldName>, Option<ProxyRole>, CUDAction),
		MaxRegistrationsAtTime,
	> = bounded_vec![];
	let field_name = make_field_name(user_name.unwrap_or_default());
	users
		.try_push((user_account, Some(field_name), user_role, action))
		.unwrap_or_default();
	users
}

fn make_users() -> Option<BoundedVec<(u64, ProxyRole, AssignAction), MaxRegistrationsAtTime>> {
	let mut users: BoundedVec<(u64, ProxyRole, AssignAction), MaxRegistrationsAtTime> =
		bounded_vec![];
	users
		.try_push((2, ProxyRole::Builder, AssignAction::Assign))
		.unwrap_or_default();
	users
		.try_push((3, ProxyRole::RegionalCenter, AssignAction::Assign))
		.unwrap_or_default();

	let mut users_to_be_registered: BoundedVec<
		(u64, Option<FieldName>, Option<ProxyRole>, CUDAction),
		MaxRegistrationsAtTime,
	> = bounded_vec![];

	for user in users.iter() {
		let user_account = user.0;
		let user_role = user.1;

		users_to_be_registered
			.try_push((user_account, Some(make_field_name(&user_account.to_string())), Some(user_role), CUDAction::Create))
			.unwrap_or_default();
	}

	assert_ok!(FundAdmin::users(Origin::signed(1), users_to_be_registered));
	Some(users)
}

fn make_expenditures() -> BoundedVec<
	(
		Option<FieldName>,
		Option<ExpenditureType>,
		Option<ExpenditureAmount>,
		Option<NAICSCode>,
		Option<JobsMultiplier>,
		CUDAction,
		Option<BudgetExpenditureId>,
	),
	MaxRegistrationsAtTime,
> {
	let mut expenditures: BoundedVec<
		(
			Option<FieldName>,
			Option<ExpenditureType>,
			Option<ExpenditureAmount>,
			Option<NAICSCode>,
			Option<JobsMultiplier>,
			CUDAction,
			Option<BudgetExpenditureId>,
		),
		MaxRegistrationsAtTime,
	> = bounded_vec![];
	let field_name = make_field_name("Expenditure");
	let expenditure_type = ExpenditureType::HardCost;
	let expenditure_amount = 100;
	let naics_code = make_field_description("1293, 1231");
	let jobs_multiplier = 100;
	let budget_expenditure_id = None;
	expenditures
		.try_push((
			Some(field_name),
			Some(expenditure_type),
			Some(expenditure_amount),
			Some(naics_code),
			Some(jobs_multiplier),
			CUDAction::Create,
			budget_expenditure_id,
		))
		.unwrap_or_default();
	expenditures
}

fn field_name_to_string(boundedvec: &BoundedVec<u8, ConstU32<100>>) -> String {
	let mut s = String::new();
	for b in boundedvec.iter() {
		s.push(*b as char);
	}
	s
}

fn field_description_to_string(boundedvec: &BoundedVec<u8, ConstU32<400>>) -> String {
	let mut s = String::new();
	for b in boundedvec.iter() {
		s.push(*b as char);
	}
	s
}

// I N I T I A L
// -----------------------------------------------------------------------------------------
#[test]
fn cannon_initialize_pallet_twice_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_noop!(FundAdmin::initial_setup(Origin::root()), RbacErr::ScopeAlreadyExists);
	});
}

#[test]
fn sudo_register_administrator_account_works() {
	new_test_ext().execute_with(|| {
		let alice_name = make_field_name("Alice Keys");
		assert_ok!(FundAdmin::sudo_add_administrator(Origin::root(), 2, alice_name.clone()));
		assert!(FundAdmin::users_info(2).is_some());
	});
}

#[test]
fn sudo_a_non_sudo_user_cannot_register_administrator_account_should_not_work() {
	new_test_ext().execute_with(|| {
		let alice_name = make_field_name("Alice Keys");
		assert_noop!(
			FundAdmin::sudo_add_administrator(Origin::signed(1), 2, alice_name.clone()),
			BadOrigin
		);
	});
}

#[test]
fn sudo_register_an_administrator_account_twice_should_not_work() {
	new_test_ext().execute_with(|| {
		let alice_name = make_field_name("Alice Keys");
		assert_ok!(FundAdmin::sudo_add_administrator(Origin::root(), 2, alice_name.clone()));
		assert_noop!(
			FundAdmin::sudo_add_administrator(Origin::root(), 2, alice_name.clone()),
			Error::<Test>::UserAlreadyRegistered
		);
	});
}

#[test]
fn sudo_delete_administrator_account_works() {
	new_test_ext().execute_with(|| {
		let alice_name = make_field_name("Alice Keys");
		assert_ok!(FundAdmin::sudo_add_administrator(Origin::root(), 2, alice_name.clone()));
		assert!(FundAdmin::users_info(2).is_some());

		assert_ok!(FundAdmin::sudo_remove_administrator(Origin::root(), 2,));
		assert!(FundAdmin::users_info(2).is_none());
	});
}

#[test]
fn sudo_cannot_delete_an_administrator_account_that_does_not_exist_should_not_work() {
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
			make_field_name("Alice Keys")
		));
		assert!(FundAdmin::users_info(2).is_some());

		assert_ok!(FundAdmin::sudo_add_administrator(
			Origin::root(),
			3,
			make_field_name("Bob Keys")
		));
		assert!(FundAdmin::users_info(3).is_some());

		assert_ok!(FundAdmin::sudo_remove_administrator(Origin::root(), 2,));
		assert!(FundAdmin::users_info(2).is_none());
	});
}

#[test]
fn sudo_administrator_can_remove_themselves_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(FundAdmin::sudo_add_administrator(
			Origin::root(),
			2,
			make_field_name("Alice Keys")
		));
		assert!(FundAdmin::users_info(2).is_some());

		assert_ok!(FundAdmin::sudo_remove_administrator(Origin::root(), 2,));
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
			make_user(
				2,
				Some("Alice Administrator"),
				Some(ProxyRole::Administrator),
				CUDAction::Create
			)
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
			make_user(2, Some("Alice Builder"), Some(ProxyRole::Builder), CUDAction::Create)
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
			make_user(2, Some("Alice Investor"), Some(ProxyRole::Investor), CUDAction::Create)
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
			make_user(2, Some("Alice Issuer"), Some(ProxyRole::Issuer), CUDAction::Create)
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
			make_user(
				2,
				Some("Alice Regional Center"),
				Some(ProxyRole::RegionalCenter),
				CUDAction::Create
			)
		));

		assert!(FundAdmin::users_info(2).is_some());
	});
}

#[test]
fn users_a_non_registered_admin_tries_to_register_an_account_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			FundAdmin::users(
				Origin::signed(1),
				make_user(
					2,
					Some("Alice Regional Center"),
					Some(ProxyRole::RegionalCenter),
					CUDAction::Create
				)
			),
			RbacErr::NotAuthorized
		);
	});
}

#[test]
fn users_a_registered_admin_tries_to_register_an_account_twice_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(register_administrator());

		assert_ok!(FundAdmin::users(
			Origin::signed(1),
			make_user(
				2,
				Some("Alice Regional Center"),
				Some(ProxyRole::RegionalCenter),
				CUDAction::Create
			)
		));

		assert_noop!(
			FundAdmin::users(
				Origin::signed(1),
				make_user(
					2,
					Some("Alice Regional Center"),
					Some(ProxyRole::RegionalCenter),
					CUDAction::Create
				)
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
			make_user(
				2,
				Some("Alice Regional Center"),
				Some(ProxyRole::RegionalCenter),
				CUDAction::Create
			)
		));

		assert_ok!(FundAdmin::users(
			Origin::signed(1),
			make_user(2, Some("Alice Regional Center Updated"), None, CUDAction::Update)
		));

		assert_eq!(
			field_name_to_string(&FundAdmin::users_info(2).unwrap().name),
			String::from("Alice Regional Center Updated")
		);
	});
}

#[test]
fn users_update_role_of_a_registered_account_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(register_administrator());

		assert_ok!(FundAdmin::users(
			Origin::signed(1),
			make_user(
				2,
				Some("Alice Regional Center"),
				Some(ProxyRole::RegionalCenter),
				CUDAction::Create
			)
		));

		assert_ok!(FundAdmin::users(
			Origin::signed(1),
			make_user(2, Some("Alice Investor"), Some(ProxyRole::Investor), CUDAction::Update)
		));

		assert_eq!(FundAdmin::users_info(2).unwrap().role, ProxyRole::Investor);
		assert_eq!(FundAdmin::users_info(2).unwrap().name, make_field_name("Alice Investor"));
		00
	});
}

#[test]
fn users_update_a_non_registered_account_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(register_administrator());

		assert_noop!(
			FundAdmin::users(
				Origin::signed(1),
				make_user(
					2,
					Some("Alice Regional Center"),
					Some(ProxyRole::RegionalCenter),
					CUDAction::Update
				)
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
			make_user(
				2,
				Some("Alice Regional Center"),
				Some(ProxyRole::RegionalCenter),
				CUDAction::Create
			)
		));

		assert_ok!(FundAdmin::users(
			Origin::signed(1),
			make_user(2, None, None, CUDAction::Delete)
		));

		assert!(FundAdmin::users_info(2).is_none());
	});
}

#[test]
fn users_delete_a_non_registered_account_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(register_administrator());

		assert_noop!(
			FundAdmin::users(Origin::signed(1), make_user(2, None, None, CUDAction::Delete)),
			Error::<Test>::UserNotRegistered
		);
	});
}

//TODO: cannot delete a registered users if the user has assigned projects

// PROJECTS

#[test]
fn projects_register_a_project_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(register_administrator());

		assert_ok!(FundAdmin::projects_create_project(
			Origin::signed(1),
			make_field_name("Project 1"),
			make_field_description("Project 1 description"),
			None,
			make_field_name("Mexico"),
			1000,
			1200,
			make_expenditures(),
			None
		));

		assert_ok!(FundAdmin::projects_create_project(
			Origin::signed(1),
			make_field_name("Project 2"),
			make_field_description("Project 2 description"),
			None,
			make_field_name("Mexico"),
			1000,
			1200,
			make_expenditures(),
			make_users()
		));
	});
}
