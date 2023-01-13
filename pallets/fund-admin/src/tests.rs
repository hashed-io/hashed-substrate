use crate::{mock::*, types::*, Error, ProjectsInfo, GlobalScope, UsersInfo, UsersByProject,
ProjectsByUser, ExpendituresInfo, ExpendituresByProject, DrawdownsInfo, DrawdownsByProject, TransactionsInfo, 
TransactionsByDrawdown, JobEligiblesInfo, JobEligiblesByProject, RevenuesInfo,
RevenuesByProject, RevenueTransactionsInfo, TransactionsByRevenue};
use frame_support::{assert_noop, assert_ok, bounded_vec, error::BadOrigin, traits::ConstU32, BoundedVec};
use sp_runtime::DispatchResult;


type RbacErr = pallet_rbac::Error<Test>;

#[allow(dead_code)]
fn pallet_id () -> [u8;32] {
	FundAdmin::pallet_id().to_id()
}

#[allow(dead_code)]
fn pallet_name () -> pallet_rbac::types::IdOrVec {
	pallet_rbac::types::IdOrVec::Vec(
		"pallet_test".as_bytes().to_vec()
	)
}

fn make_field_name(name: &str) -> FieldName {
    let name: BoundedVec<u8, ConstU32<100>> = name.as_bytes().to_vec().try_into().unwrap_or_default();
    name
}

fn make_field_description(description: &str) -> FieldDescription {
    let description: BoundedVec<u8, ConstU32<400>> = description.as_bytes().to_vec().try_into().unwrap_or_default();
    description
}

fn make_files( n_files: u32) -> Documents<Test>{
	let mut files: Documents<Test> = bounded_vec![];
	for i in 0..n_files{
		let file_name: &str = &format!("file_{}", i);
        let file_description: &str = &format!("file_{}_description", i);
        files.try_push((
            make_field_name(file_name),
            make_field_name(file_description),
        )).unwrap_or_default();
	}
	files
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

fn register_administrator() -> DispatchResult {
    FundAdmin::sudo_add_administrator(
        RuntimeOrigin::root(),
        1,
        make_field_name("Administrator Test"),
        ).map_err(|_| Error::<Test>::UserAlreadyRegistered
    )?;
    Ok(())
}

fn make_user(
    user_account: u64,
    user_name: Option<FieldName>,
    user_role: Option<ProxyRole>,
    action: CUDAction
) -> Users<Test> {
    let mut users: Users<Test> = bounded_vec![];
    users.try_push((
        user_account, user_name, user_role, action
    )).unwrap_or_default();
    users
}

fn make_default_users() -> Users<Test> {
    let mut users: Users<Test> = bounded_vec![];
    let users_account = [2, 3, 4, 5];
    let users_name: Vec<FieldName> = ["Builder Test", "Investor Test", "Issuer Test", "Regional Center Test"].iter().map(|s| make_field_name(s)).collect();
    let users_role: Vec<ProxyRole> = [ProxyRole::Builder, ProxyRole::Investor, ProxyRole::Issuer, ProxyRole::RegionalCenter].iter().map(|s| *s).collect();
    let cud_action = CUDAction::Create;

    for i in 0..users_account.len() {
        users
            .try_push((
                users_account[i],
                Some(users_name[i].clone()),
                Some(users_role[i]),
                cud_action,
            ))
            .unwrap_or_default();
    };
	users
}

#[allow(dead_code)]
fn make_expenditure(
    expenditure_name: Option<FieldName>,
    expenditure_type: Option<ExpenditureType>,
    expenditure_amount: Option<ExpenditureAmount>,
    naics_code: Option<NAICSCode>,
    jobs_multiplier: Option<JobsMultiplier>,
    action: CUDAction,
    budget_expenditure_id: Option<ExpenditureId>
) -> Expenditures<Test> {
    let mut expenditures: Expenditures<Test> = bounded_vec![];
    expenditures.try_push((
        expenditure_name, expenditure_type, expenditure_amount, naics_code, jobs_multiplier, action, budget_expenditure_id
    )).unwrap_or_default();
    expenditures
}

fn make_default_expenditures() -> Expenditures<Test> {
	let mut expenditures: Expenditures<Test> = bounded_vec![];
    let expenditure_name: Vec<FieldName> = ["Expenditure Test 1", "Expenditure Test 2", "Expenditure Test 3", "Expenditure Test 4"].iter().map(|s| make_field_name(s)).collect();
	let expenditure_type: Vec<ExpenditureType> = [ExpenditureType::HardCost, ExpenditureType::SoftCost, ExpenditureType::Operational, ExpenditureType::Others].iter().map(|s| *s).collect();
	let expenditure_amount: Vec<u64> = [100, 200, 300, 400].iter().map(|s| *s).collect();
	let naics_code: Vec<FieldDescription> = [1231, 1232, 1233, 1234].iter().map(|s| make_field_description(&s.to_string())).collect();
	let jobs_multiplier: Vec<u32> = [20, 30, 40, 50].iter().map(|s| *s).collect();
    let cud_action = CUDAction::Create;
	let budget_expenditure_id = None;

    for i in 0..expenditure_name.len() {
        expenditures
            .try_push((
                Some(expenditure_name[i].clone()),
                Some(expenditure_type[i]),
                Some(expenditure_amount[i]),
                Some(naics_code[i].clone()),
                Some(jobs_multiplier[i]),
                cud_action,
                budget_expenditure_id,
            ))
            .unwrap_or_default();
    };
	expenditures
}

#[allow(dead_code)]
fn make_job_eligible(
    field_name: Option<FieldName>,
    job_eligible_amount: Option<JobEligibleAmount>,
    naics_code: Option<NAICSCode>,
    jobs_multiplier: Option<JobsMultiplier>,
    action: CUDAction,
    budget_job_eligible_id: Option<JobEligibleId>
) -> JobEligibles<Test> {
    let mut job_eligibles: JobEligibles<Test> = bounded_vec![];
    job_eligibles.try_push((
        field_name, job_eligible_amount, naics_code, jobs_multiplier, action, budget_job_eligible_id
    )).unwrap_or_default();
    job_eligibles
}

fn make_default_job_eligibles() -> JobEligibles<Test> {
    let mut job_eligibles: JobEligibles<Test> = bounded_vec![];
    let field_name = make_field_name("Job Eligible");
    let job_eligible_amount = 100;
    let naics_code = make_field_description("1293, 1231");
    let jobs_multiplier = 100;
    let budget_job_eligible_id = None;
    job_eligibles
        .try_push((
            Some(field_name),
            Some(job_eligible_amount),
            Some(naics_code),
            Some(jobs_multiplier),
            CUDAction::Create,
            budget_job_eligible_id,
        ))
        .unwrap_or_default();
    job_eligibles
}

#[allow(dead_code)]
fn make_user_assignation(
    user_account: u64,
    user_role: ProxyRole,
    action: AssignAction,
) -> UsersAssignation<Test> {
    let mut users_assignation: UsersAssignation<Test> = bounded_vec![];
    users_assignation.try_push((
        user_account, user_role, action
    )).unwrap_or_default();
    users_assignation
}

fn make_default_user_assignation() ->  UsersAssignation<Test> {
    let mut users_assignation: UsersAssignation<Test> = bounded_vec![];
    let user_account: Vec<u64> = [2, 3, 4, 5].iter().map(|s| *s).collect();
    let user_role: Vec<ProxyRole> = [ProxyRole::Builder, ProxyRole::Investor, ProxyRole::Issuer, ProxyRole::RegionalCenter].iter().map(|s| *s).collect();
    let action = AssignAction::Assign;

    for i in 0..user_account.len() {
        users_assignation
            .try_push((
                user_account[i],
                user_role[i],
                action,
            ))
            .unwrap_or_default();
    };
    users_assignation
}

#[allow(dead_code)]
fn make_allowed_bank(
    bank_name: BankName,
    bank_address: BankAddress,
) -> Banks<Test> {
    let mut banks: Banks<Test> = bounded_vec![];
    banks.try_push((
        bank_name, bank_address
    )).unwrap_or_default();
    banks
}

fn make_default_allowed_banks() -> Banks<Test> {
    let mut banks: Banks<Test> = bounded_vec![];
    let bank_name = make_field_name("Luxury Bank");
    let bank_address = make_field_name("San Francisco");
    banks
        .try_push((
            bank_name,
            bank_address,
        ))
        .unwrap_or_default();
    banks
}

fn make_default_simple_project() -> DispatchResult {

    register_administrator()?;

    FundAdmin::projects_create_project(
        RuntimeOrigin::signed(1),
        make_field_name("Project 1"),
        make_field_description("Project 1 description"),
        Some(make_field_name("project_image.jpeg")),
        make_field_name("New York"),
        None,
        1000,
        2000,
        make_default_expenditures(),
        None,
        None,
        make_field_description("P9f5wbr13BK74p1"),
    )?;
    Ok(())
}

fn make_default_full_project() -> DispatchResult {

    register_administrator()?;

    FundAdmin::users(
        RuntimeOrigin::signed(1),
        make_default_users(),
    )?;

    FundAdmin::projects_create_project(
        RuntimeOrigin::signed(1),
        make_field_name("Project 1"),
        make_field_description("Project 1 description"),
        Some(make_field_name("project_image.jpeg")),
        make_field_name("New York"),
        Some(make_default_allowed_banks()),
        1000,
        2000,
        make_default_expenditures(),
        Some(make_default_job_eligibles()),
        Some(make_default_user_assignation()),
        make_field_description("P9f5wbr13BK74p1"),
    )?;
    Ok(())
}


// I N I T I A L
// -----------------------------------------------------------------------------------------
#[test]
fn cannon_initialize_pallet_twice_shouldnt_work() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            FundAdmin::initial_setup(
                RuntimeOrigin::root()),
                RbacErr::ScopeAlreadyExists
            );
    });
}

#[test]
fn sudo_register_administrator_account_works() {
    new_test_ext().execute_with(|| {
        let alice_name = make_field_name("Alice Keys");
        assert_ok!(FundAdmin::sudo_add_administrator(
            RuntimeOrigin::root(),
            2,
            alice_name.clone()
        ));
        assert!(FundAdmin::users_info(2).is_some());
    });
}

#[test]
fn sudo_a_non_sudo_user_cannot_register_administrator_account_shouldnt_work() {
    new_test_ext().execute_with(|| {
        let alice_name = make_field_name("Alice Keys");
        assert_noop!(
            FundAdmin::sudo_add_administrator(RuntimeOrigin::signed(1), 2, alice_name.clone()),
            BadOrigin
        );
    });
}

#[test]
fn sudo_cannot_register_an_administrator_account_twice_shouldnt_work() {
    new_test_ext().execute_with(|| {
        let alice_name = make_field_name("Alice Keys");
        assert_ok!(FundAdmin::sudo_add_administrator(
            RuntimeOrigin::root(),
            2,
            alice_name.clone()
        ));
        assert_noop!(
            FundAdmin::sudo_add_administrator(RuntimeOrigin::root(), 2, alice_name.clone()),
            Error::<Test>::UserAlreadyRegistered
        );
    });
}

#[test]
fn sudo_delete_administrator_account_works() {
    new_test_ext().execute_with(|| {
        let alice_name = make_field_name("Alice Keys");
        assert_ok!(FundAdmin::sudo_add_administrator(
            RuntimeOrigin::root(),
            2,
            alice_name.clone()
        ));
        assert!(FundAdmin::users_info(2).is_some());

        assert_ok!(FundAdmin::sudo_remove_administrator(
            RuntimeOrigin::root(),
            2,
        ));
        assert!(FundAdmin::users_info(2).is_none());
    });
}

#[test]
fn sudo_cannot_delete_an_administrator_account_that_doesnt_exist_shouldnt_work() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            FundAdmin::sudo_remove_administrator(RuntimeOrigin::root(), 2),
            Error::<Test>::UserNotRegistered
        );
    });
}

#[test]
fn sudo_administrator_can_remove_another_administrator_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(FundAdmin::sudo_add_administrator(
            RuntimeOrigin::root(),
            2,
            make_field_name("Alice Keys")
        ));
        assert!(FundAdmin::users_info(2).is_some());

        assert_ok!(FundAdmin::sudo_add_administrator(
            RuntimeOrigin::root(),
            3,
            make_field_name("Bob Keys")
        ));
        assert!(FundAdmin::users_info(3).is_some());

        assert_ok!(FundAdmin::sudo_remove_administrator(
            RuntimeOrigin::root(),
            2,
        ));
        assert!(FundAdmin::users_info(2).is_none());
    });
}

#[test]
fn sudo_administrator_can_remove_themselves_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(FundAdmin::sudo_add_administrator(
            RuntimeOrigin::root(),
            2,
            make_field_name("Alice Keys")
        ));
        assert!(FundAdmin::users_info(2).is_some());

        assert_ok!(FundAdmin::sudo_remove_administrator(
            RuntimeOrigin::root(),
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
            RuntimeOrigin::signed(1),
            make_user(2, Some(make_field_name("Alice Administrator")), Some(ProxyRole::Administrator), CUDAction::Create)
        ));

        assert!(FundAdmin::users_info(2).is_some());
    });
}

#[test]
fn users_register_builder_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            make_user(2, Some(make_field_name("Alice Builder")), Some(ProxyRole::Builder), CUDAction::Create)
        ));

        assert!(FundAdmin::users_info(2).is_some());
    });
}

#[test]
fn users_register_investor_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            make_user(2, Some(make_field_name("Alice Investor")), Some(ProxyRole::Investor), CUDAction::Create)
        ));

        assert!(FundAdmin::users_info(2).is_some());
    });
}

#[test]
fn users_register_issuer_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            make_user(2, Some(make_field_name("Alice Issuer")), Some(ProxyRole::Issuer), CUDAction::Create)
        ));

        assert!(FundAdmin::users_info(2).is_some());
    });
}

#[test]
fn users_register_regional_center_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            make_user(2, Some(make_field_name("Alice Regional Center")), Some(ProxyRole::RegionalCenter), CUDAction::Create)
        ));

        assert!(FundAdmin::users_info(2).is_some());
    });
}

#[test]
fn users_register_multiple_accounts_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            make_default_users()
        ));

        assert!(FundAdmin::users_info(2).is_some());
        assert!(FundAdmin::users_info(3).is_some());
        assert!(FundAdmin::users_info(4).is_some());
        assert!(FundAdmin::users_info(5).is_some());

    });
}

#[test]
fn users_a_non_registered_admin_tries_to_register_an_account_shouldnt_work() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            FundAdmin::users(
                RuntimeOrigin::signed(1),
                make_user(2, Some(make_field_name("Alice Regional Center")), Some(ProxyRole::RegionalCenter), CUDAction::Create)
            ),
            Error::<Test>::UserNotRegistered
        );
    });
}

#[test]
fn users_a_registered_admin_tries_to_register_an_account_twice_shouldnt_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            make_user(2, Some(make_field_name("Alice Regional Center")), Some(ProxyRole::RegionalCenter), CUDAction::Create)
        ));

        assert_noop!(
            FundAdmin::users(
                RuntimeOrigin::signed(1),
                make_user(2, Some(make_field_name("Alice Regional Center")), Some(ProxyRole::RegionalCenter), CUDAction::Create)
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
            RuntimeOrigin::signed(1),
            make_user(2, Some(make_field_name("Alice Regional Center")), Some(ProxyRole::RegionalCenter), CUDAction::Create)
        ));

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            make_user(2, Some(make_field_name("Alice Regional Center Updated")), None, CUDAction::Update)
        ));

        assert_eq!(field_name_to_string(&FundAdmin::users_info(2).unwrap().name), String::from("Alice Regional Center Updated"));
    });
}

#[test]
fn users_admnistrator_updates_role_of_a_registered_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            make_user(2, Some(make_field_name("Alice Regional Center")), Some(ProxyRole::RegionalCenter), CUDAction::Create)
        ));

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            make_user(2, Some(make_field_name("Alice Investor")), Some(ProxyRole::Investor), CUDAction::Update)
        ));

        assert_eq!(FundAdmin::users_info(2).unwrap().role, ProxyRole::Investor);
        assert_eq!(FundAdmin::users_info(2).unwrap().name, make_field_name("Alice Investor"));00
    });
}


#[test]
fn users_update_a_non_registered_account_shouldnt_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_noop!(
            FundAdmin::users(
                RuntimeOrigin::signed(1),
                make_user(2, Some(make_field_name("Alice Regional Center")), Some(ProxyRole::RegionalCenter), CUDAction::Update)
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
            RuntimeOrigin::signed(1),
            make_user(2, Some(make_field_name("Alice Regional Center")), Some(ProxyRole::RegionalCenter), CUDAction::Create)
        ));

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            make_user(2, None, None, CUDAction::Delete)
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
                RuntimeOrigin::signed(1),
                make_user(2, None, None, CUDAction::Delete)
            ),
            Error::<Test>::UserNotRegistered
        );
    });
}

#[test]
fn users_user_updates_their_own_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            make_user(2, Some(make_field_name("Bob Regional Center")), Some(ProxyRole::RegionalCenter), CUDAction::Create)
        ));

        assert_ok!(FundAdmin::users_edit_user(
            RuntimeOrigin::signed(2),
            Some(make_field_name("Bob Regiona Center New York")),
            Some(make_field_name("image.png")),
            Some(make_field_name("bob.regionalcenter@fundadmin.com")),
            None,
        ));

        assert_eq!(FundAdmin::users_info(2).unwrap().role, ProxyRole::RegionalCenter);
        assert_eq!(FundAdmin::users_info(2).unwrap().name, make_field_name("Bob Regiona Center New York"));
        assert_eq!(FundAdmin::users_info(2).unwrap().image, make_field_name("image.png"));
        assert_eq!(FundAdmin::users_info(2).unwrap().email, make_field_name("bob.regionalcenter@fundadmin.com"));
    });
}

#[test]
fn users_only_investors_can_upload_documentation_to_their_account_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            make_user(2, Some(make_field_name("Bob Investor")), Some(ProxyRole::Investor), CUDAction::Create)
        ));

        assert_ok!(
            FundAdmin::users_edit_user(
                RuntimeOrigin::signed(2),
                None,
                None,
                None,
                Some(make_files(1)),
            )
        );
        assert_eq!(FundAdmin::users_info(2).unwrap().name, make_field_name("Bob Investor"));
        assert_eq!(FundAdmin::users_info(2).unwrap().documents, Some(make_files(1)));
    });
}

// P R O J E C T S
// -----------------------------------------------------------------------------------------

#[test]
fn projects_register_a_project_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::projects_create_project(
            RuntimeOrigin::signed(1),
            make_field_name("Project 1"),
            make_field_description("Project 1 description"),
            Some(make_field_name("project_image.jpeg")),
            make_field_name("New York"),
            None,
            1000,
            2000,
            make_default_expenditures(),
            None,
            None,
            make_field_description("P9f5wbr13BK74p1"),
        ));

        assert_eq!(ProjectsInfo::<Test>::iter().count(), 1);
        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
        assert_eq!(ProjectsInfo::<Test>::get(get_project_id).unwrap().title, make_field_name("Project 1"));

        assert_eq!(ExpendituresInfo::<Test>::iter().count(), ExpendituresByProject::<Test>::get(get_project_id).len());
        let get_expenditure_ids: Vec<[u8; 32]> = ExpendituresByProject::<Test>::get(get_project_id).iter().cloned().collect();
        for i in get_expenditure_ids {
            assert_eq!(ExpendituresInfo::<Test>::get(i).unwrap().project_id, get_project_id);
        }

        assert_eq!(DrawdownsInfo::<Test>::iter().count(), DrawdownsByProject::<Test>::get(get_project_id).len());
        let get_drawdown_ids: Vec<[u8; 32]> = DrawdownsByProject::<Test>::get(get_project_id).iter().cloned().collect();
        for i in get_drawdown_ids {
            assert_eq!(DrawdownsInfo::<Test>::get(i).unwrap().project_id, get_project_id);
        }

        assert_eq!(RevenuesInfo::<Test>::iter().count(), RevenuesByProject::<Test>::get(get_project_id).len());
        let get_revenue_ids: Vec<[u8; 32]> = RevenuesByProject::<Test>::get(get_project_id).iter().cloned().collect();
        for i in get_revenue_ids {
            assert_eq!(RevenuesInfo::<Test>::get(i).unwrap().project_id, get_project_id);
        }
    });
}

#[test]
fn projects_register_a_project_with_job_eligibles_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::projects_create_project(
            RuntimeOrigin::signed(1),
            make_field_name("Project 1"),
            make_field_description("Project 1 description"),
            Some(make_field_name("project_image.jpeg")),
            make_field_name("New York"),
            None,
            1000,
            2000,
            make_default_expenditures(),
            Some(make_default_job_eligibles()),
            None,
            make_field_description("P9f5wbr13BK74p1"),
        ));

        assert_eq!(ProjectsInfo::<Test>::iter_values().count(), 1);
        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
        assert_eq!(JobEligiblesInfo::<Test>::iter().count(), JobEligiblesByProject::<Test>::get(get_project_id).len());

        let get_job_eligible_ids: Vec<[u8; 32]> = JobEligiblesByProject::<Test>::get(get_project_id).iter().cloned().collect();
        for i in get_job_eligible_ids {
            assert_eq!(JobEligiblesInfo::<Test>::get(i).unwrap().project_id, get_project_id);
        }

    });
}

#[test]
fn projects_register_a_project_with_assigned_users_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            make_default_users(),
        ));

        assert_ok!(FundAdmin::projects_create_project(
            RuntimeOrigin::signed(1),
            make_field_name("Project 1"),
            make_field_description("Project 1 description"),
            Some(make_field_name("project_image.jpeg")),
            make_field_name("New York"),
            None,
            1000,
            2000,
            make_default_expenditures(),
            None,
            Some(make_default_user_assignation()),
            make_field_description("P9f5wbr13BK74p1"),
        ));

        assert_eq!(ProjectsInfo::<Test>::iter_values().count(), 1);
        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
        assert_eq!(UsersByProject::<Test>::get(get_project_id).len(), 4);

        let get_assigned_user_ids: Vec<u64> = UsersByProject::<Test>::get(get_project_id).iter().cloned().collect();
        for i in get_assigned_user_ids {
            assert_eq!(ProjectsByUser::<Test>::get(i).len(), 1);
            assert_eq!(ProjectsByUser::<Test>::get(i).iter().next().unwrap(), &get_project_id);
        }

    });
}

#[test]
fn projects_register_a_project_with_allowed_banks_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::projects_create_project(
            RuntimeOrigin::signed(1),
            make_field_name("Project 1"),
            make_field_description("Project 1 description"),
            Some(make_field_name("project_image.jpeg")),
            make_field_name("New York"),
            Some(make_default_allowed_banks()),
            1000,
            2000,
            make_default_expenditures(),
            None,
            None,
            make_field_description("P9f5wbr13BK74p1"),
        ));

        assert_eq!(ProjectsInfo::<Test>::iter_values().count(), 1);
        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

        assert_eq!(ProjectsInfo::<Test>::get(get_project_id).unwrap().banks, Some(make_default_allowed_banks()));

    });
}

#[test]
fn projects_register_a_project_without_a_group_id_should_fail() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_noop!(
            FundAdmin::projects_create_project(
                RuntimeOrigin::signed(1),
                make_field_name("Project 1"),
                make_field_description("Project 1 description"),
                Some(make_field_name("project_image.jpeg")),
                make_field_name("New York"),
                None,
                1000,
                2000,
                make_default_expenditures(),
                None,
                None,
                make_field_description(""),
            ),
            Error::<Test>::PrivateGroupIdIsEmpty
        );
    });
}

#[test]
fn projects_a_non_authorized_user_registers_a_project_should_fail() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            make_default_users(),
        ));

        let unauthorized_users: Vec<u64> = vec![2, 3, 4, 5];

        for i in unauthorized_users {
            assert_noop!(
                FundAdmin::projects_create_project(
                    RuntimeOrigin::signed(i),
                    make_field_name("Project 1"),
                    make_field_description("Project 1 description"),
                    Some(make_field_name("project_image.jpeg")),
                    make_field_name("New York"),
                    None,
                    1000,
                    2000,
                    make_default_expenditures(),
                    None,
                    None,
                    make_field_description("P9f5wbr13BK74p1"),
                ),
                RbacErr::NotAuthorized
            );
        }
    });
}


#[test]
fn projects_investors_can_be_only_assigned_to_one_project_should_fail() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            make_default_users(),
        ));

        let investor_data = make_user_assignation(3, ProxyRole::Investor, AssignAction::Assign);

        assert_ok!(FundAdmin::projects_create_project(
            RuntimeOrigin::signed(1),
            make_field_name("Project 1"),
            make_field_description("Project 1 description"),
            Some(make_field_name("project_image.jpeg")),
            make_field_name("New York"),
            None,
            1000,
            2000,
            make_default_expenditures(),
            None,
            Some(investor_data.clone()),
            make_field_description("P9f5wbr13BK74p1"),
        ));

        assert_noop!(
            FundAdmin::projects_create_project(
                RuntimeOrigin::signed(1),
                make_field_name("Project 2"),
                make_field_description("Project 2 description"),
                Some(make_field_name("project_image.jpeg")),
                make_field_name("New York"),
                None,
                1000,
                2000,
                make_default_expenditures(),
                None,
                Some(investor_data),
                make_field_description("P9f5wbr13BK74p1"),
            ),
            Error::<Test>::MaxProjectsPerInvestorReached
        );
    });
}

#[test]
fn projects_edit_a_project_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(register_administrator());

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            make_default_users(),
        ));

        assert_ok!(FundAdmin::projects_create_project(
            RuntimeOrigin::signed(1),
            make_field_name("Project 1"),
            make_field_description("Project 1 description"),
            Some(make_field_name("project_image.jpeg")),
            make_field_name("New York"),
            None,
            1000,
            2000,
            make_default_expenditures(),
            None,
            None,
            make_field_description("P9f5wbr13BK74p1"),
        ));

        assert_eq!(ProjectsInfo::<Test>::iter_values().count(), 1);
        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

        assert_ok!(FundAdmin::projects_edit_project(
            RuntimeOrigin::signed(1),
            get_project_id,
            Some(make_field_name("Project 1 edited")),
            Some(make_field_description("Project 1 description edited")),
            Some(make_field_name("project_image.jpeg")),
            Some(make_field_name("California")),
            None,
            Some(5000u64),
            Some(10000u64),
        ));

        assert_eq!(ProjectsInfo::<Test>::get(get_project_id).unwrap().title, make_field_name("Project 1 edited"));
    });
}

#[test]
fn projects_delete_project_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(make_default_full_project());

        assert_eq!(ProjectsInfo::<Test>::iter_values().count(), 1);
        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
        let get_expenditure_ids: Vec<[u8; 32]> = ExpendituresByProject::<Test>::get(get_project_id).iter().cloned().collect();
        let get_drawdown_ids: Vec<[u8; 32]> = DrawdownsByProject::<Test>::get(get_project_id).iter().cloned().collect();
        let get_revenue_ids: Vec<[u8; 32]> = RevenuesByProject::<Test>::get(get_project_id).iter().cloned().collect();
        let get_job_eligible_ids: Vec<[u8; 32]> = JobEligiblesByProject::<Test>::get(get_project_id).iter().cloned().collect();
        let get_assigned_user_ids: Vec<u64> = UsersByProject::<Test>::get(get_project_id).iter().cloned().collect();

        assert_ok!(FundAdmin::projects_delete_project(
            RuntimeOrigin::signed(1),
            get_project_id,
        ));

        // Ensure project data was deleted
        assert_eq!(ProjectsInfo::<Test>::contains_key(get_project_id), false);
        assert_eq!(ExpendituresInfo::<Test>::contains_key(get_project_id), false);
        for expenditure_id in get_expenditure_ids {
            assert_eq!(ExpendituresInfo::<Test>::contains_key(expenditure_id), false);
        }
        for drawdown_id in get_drawdown_ids {
            assert_eq!(DrawdownsInfo::<Test>::contains_key(drawdown_id), false);
        }
        for revenue_id in get_revenue_ids {
            assert_eq!(RevenuesInfo::<Test>::contains_key(revenue_id), false);
        }
        for job_eligible_id in get_job_eligible_ids {
            assert_eq!(JobEligiblesInfo::<Test>::contains_key(job_eligible_id), false);
        }
        for assigned_user_id in get_assigned_user_ids {
            assert_eq!(UsersByProject::<Test>::get(get_project_id).contains(&assigned_user_id), false);
            assert_eq!(ProjectsByUser::<Test>::get(assigned_user_id).contains(&get_project_id), false);
        }
    });
}

#[test]
fn projects_assign_a_builder_to_a_project_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(make_default_simple_project());

        let builder_data = make_user(
            2,
            Some(make_field_name("Builder Test")),
            Some(ProxyRole::Builder),
            CUDAction::Create,
        );

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            builder_data,
        ));

        let builder_assignment = make_user_assignation(
            2,
            ProxyRole::Builder,
            AssignAction::Assign,
        );

        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

        assert_ok!(FundAdmin::projects_assign_user(
            RuntimeOrigin::signed(1),
            get_project_id,
            builder_assignment,
        ));

        assert_eq!(UsersByProject::<Test>::get(get_project_id).contains(&2), true);
        assert_eq!(ProjectsByUser::<Test>::get(2).contains(&get_project_id), true);
    });
}

#[test]
fn projects_assign_an_investor_to_a_project_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(make_default_simple_project());
        
        let investor_data = make_user(
            3,
            Some(make_field_name("Investor Test")),
            Some(ProxyRole::Investor),
            CUDAction::Create,
        );

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            investor_data,
        ));

        let investor_assignment = make_user_assignation(
            3,
            ProxyRole::Investor,
            AssignAction::Assign,
        );

        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

        assert_ok!(FundAdmin::projects_assign_user(
            RuntimeOrigin::signed(1),
            get_project_id,
            investor_assignment,
        ));

        assert_eq!(UsersByProject::<Test>::get(get_project_id).contains(&3), true);
        assert_eq!(ProjectsByUser::<Test>::get(3).contains(&get_project_id), true);
    });
}

#[test]
fn projects_assign_an_issuer_to_a_project_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(make_default_simple_project());

        let issuer_data = make_user(
            4,
            Some(make_field_name("Issuer Test")),
            Some(ProxyRole::Issuer),
            CUDAction::Create,
        );

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            issuer_data,
        ));

        let issuer_assignment = make_user_assignation(
            4,
            ProxyRole::Issuer,
            AssignAction::Assign,
        );

        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

        assert_ok!(FundAdmin::projects_assign_user(
            RuntimeOrigin::signed(1),
            get_project_id,
            issuer_assignment,
        ));

        assert_eq!(UsersByProject::<Test>::get(get_project_id).contains(&4), true);
        assert_eq!(ProjectsByUser::<Test>::get(4).contains(&get_project_id), true);
    });
}

#[test]
fn projects_assign_a_regional_center_to_a_project_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(make_default_simple_project());

        let regional_center_data = make_user(
            5,
            Some(make_field_name("Regional Center Test")),
            Some(ProxyRole::RegionalCenter),
            CUDAction::Create,
        );

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            regional_center_data,
        ));

        let regional_center_assignment = make_user_assignation(
            5,
            ProxyRole::RegionalCenter,
            AssignAction::Assign,
        );

        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

        assert_ok!(FundAdmin::projects_assign_user(
            RuntimeOrigin::signed(1),
            get_project_id,
            regional_center_assignment,
        ));

        assert_eq!(UsersByProject::<Test>::get(get_project_id).contains(&5), true);
        assert_eq!(ProjectsByUser::<Test>::get(5).contains(&get_project_id), true);
    });
}

#[test]
fn projects_unassign_a_builder_from_a_project_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(make_default_simple_project());

        let builder_data = make_user(
            2,
            Some(make_field_name("Builder Test")),
            Some(ProxyRole::Builder),
            CUDAction::Create,
        );

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            builder_data,
        ));

        let builder_assignment = make_user_assignation(
            2,
            ProxyRole::Builder,
            AssignAction::Assign,
        );

        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

        assert_ok!(FundAdmin::projects_assign_user(
            RuntimeOrigin::signed(1),
            get_project_id,
            builder_assignment,
        ));

        assert_eq!(UsersByProject::<Test>::get(get_project_id).contains(&2), true);
        assert_eq!(ProjectsByUser::<Test>::get(2).contains(&get_project_id), true);

        let builder_unassignment = make_user_assignation(
            2,
            ProxyRole::Builder,
            AssignAction::Unassign,
        );

        assert_ok!(FundAdmin::projects_assign_user(
            RuntimeOrigin::signed(1),
            get_project_id,
            builder_unassignment,
        ));

        assert_eq!(UsersByProject::<Test>::get(get_project_id).contains(&2), false);
        assert_eq!(ProjectsByUser::<Test>::get(2).contains(&get_project_id), false);
    });
}

#[test]
fn projects_unassign_an_investor_from_a_project_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(make_default_simple_project());

        let investor_data = make_user(
            3,
            Some(make_field_name("Investor Test")),
            Some(ProxyRole::Investor),
            CUDAction::Create,
        );

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            investor_data,
        ));

        let investor_assignment = make_user_assignation(
            3,
            ProxyRole::Investor,
            AssignAction::Assign,
        );

        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

        assert_ok!(FundAdmin::projects_assign_user(
            RuntimeOrigin::signed(1),
            get_project_id,
            investor_assignment,
        ));

        assert_eq!(UsersByProject::<Test>::get(get_project_id).contains(&3), true);
        assert_eq!(ProjectsByUser::<Test>::get(3).contains(&get_project_id), true);

        let investor_unassignment = make_user_assignation(
            3,
            ProxyRole::Investor,
            AssignAction::Unassign,
        );

        assert_ok!(FundAdmin::projects_assign_user(
            RuntimeOrigin::signed(1),
            get_project_id,
            investor_unassignment,
        ));

        assert_eq!(UsersByProject::<Test>::get(get_project_id).contains(&3), false);
        assert_eq!(ProjectsByUser::<Test>::get(3).contains(&get_project_id), false);
    });
}

#[test]
fn projects_unassign_an_issuer_from_a_project_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(make_default_simple_project());

        let issuer_data = make_user(
            4,
            Some(make_field_name("Issuer Test")),
            Some(ProxyRole::Issuer),
            CUDAction::Create,
        );

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            issuer_data,
        ));

        let issuer_assignment = make_user_assignation(
            4,
            ProxyRole::Issuer,
            AssignAction::Assign,
        );

        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

        assert_ok!(FundAdmin::projects_assign_user(
            RuntimeOrigin::signed(1),
            get_project_id,
            issuer_assignment,
        ));

        assert_eq!(UsersByProject::<Test>::get(get_project_id).contains(&4), true);
        assert_eq!(ProjectsByUser::<Test>::get(4).contains(&get_project_id), true);

        let issuer_unassignment = make_user_assignation(
            4,
            ProxyRole::Issuer,
            AssignAction::Unassign,
        );

        assert_ok!(FundAdmin::projects_assign_user(
            RuntimeOrigin::signed(1),
            get_project_id,
            issuer_unassignment,
        ));

        assert_eq!(UsersByProject::<Test>::get(get_project_id).contains(&4), false);
        assert_eq!(ProjectsByUser::<Test>::get(4).contains(&get_project_id), false);
    });
}

#[test]
fn projects_unassign_a_regional_center_from_a_project_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(make_default_simple_project());

        let regional_center_data = make_user(
            5,
            Some(make_field_name("Regional Center Test")),
            Some(ProxyRole::RegionalCenter),
            CUDAction::Create,
        );

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            regional_center_data,
        ));

        let regional_center_assignment = make_user_assignation(
            5,
            ProxyRole::RegionalCenter,
            AssignAction::Assign,
        );

        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

        assert_ok!(FundAdmin::projects_assign_user(
            RuntimeOrigin::signed(1),
            get_project_id,
            regional_center_assignment,
        ));

        assert_eq!(UsersByProject::<Test>::get(get_project_id).contains(&5), true);
        assert_eq!(ProjectsByUser::<Test>::get(5).contains(&get_project_id), true);

        let regional_center_unassignment = make_user_assignation(
            5,
            ProxyRole::RegionalCenter,
            AssignAction::Unassign,
        );

        assert_ok!(FundAdmin::projects_assign_user(
            RuntimeOrigin::signed(1),
            get_project_id,
            regional_center_unassignment,
        ));

        assert_eq!(UsersByProject::<Test>::get(get_project_id).contains(&5), false);
        assert_eq!(ProjectsByUser::<Test>::get(5).contains(&get_project_id), false);
    });
}

#[test]
fn projects_cannot_assign_a_user_to_a_project_twice_should_fail() {
    new_test_ext().execute_with(|| {
        assert_ok!(make_default_simple_project());

        let builder_data = make_user(
            2,
            Some(make_field_name("Builder Test")),
            Some(ProxyRole::Builder),
            CUDAction::Create,
        );

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            builder_data,
        ));

        let builder_assignment = make_user_assignation(
            2,
            ProxyRole::Builder,
            AssignAction::Assign,
        );

        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

        assert_ok!(FundAdmin::projects_assign_user(
            RuntimeOrigin::signed(1),
            get_project_id,
            builder_assignment.clone(),
        ));

        assert_eq!(UsersByProject::<Test>::get(get_project_id).contains(&2), true);
        assert_eq!(ProjectsByUser::<Test>::get(2).contains(&get_project_id), true);

        assert_noop!(
            FundAdmin::projects_assign_user(
                RuntimeOrigin::signed(1),
                get_project_id,
                builder_assignment,
            ),
            Error::<Test>::UserAlreadyAssignedToProject
        );
    });
}

#[test]
fn user_cannot_be_assigned_to_a_project_with_a_different_role_should_fail() {
    new_test_ext().execute_with(|| {
        assert_ok!(make_default_simple_project());

        let builder_data = make_user(
            2,
            Some(make_field_name("Builder Test")),
            Some(ProxyRole::Builder),
            CUDAction::Create,
        );

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            builder_data,
        ));

        let investor_assignment = make_user_assignation(
            2,
            ProxyRole::Investor,
            AssignAction::Assign,
        );
    
        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

        assert_noop!(
            FundAdmin::projects_assign_user(
                RuntimeOrigin::signed(1),
                get_project_id,
                investor_assignment,
            ),
            Error::<Test>::UserCannotHaveMoreThanOneRole
        );
    });
}

#[test]
fn projects_a_user_cannot_have_more_than_one_role_in_a_project_should_fail() {
    new_test_ext().execute_with(|| {
        assert_ok!(make_default_simple_project());

        let builder_data = make_user(
            2,
            Some(make_field_name("Builder Test")),
            Some(ProxyRole::Builder),
            CUDAction::Create,
        );

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            builder_data,
        ));
    
        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

        let builder_assignment = make_user_assignation(
            2,
            ProxyRole::Builder,
            AssignAction::Assign,
        );

        assert_ok!(FundAdmin::projects_assign_user(
            RuntimeOrigin::signed(1),
            get_project_id,
            builder_assignment,
        ));

        let investor_assignment = make_user_assignation(
            2,
            ProxyRole::Investor,
            AssignAction::Assign,
        );

        assert_noop!(
            FundAdmin::projects_assign_user(
                RuntimeOrigin::signed(1),
                get_project_id,
                investor_assignment,
            ),
            Error::<Test>::UserCannotHaveMoreThanOneRole
        );
    });
}

#[test]
fn projects_cannot_delete_a_user_who_has_assigned_projects_should_fail() {
    new_test_ext().execute_with(|| {
        assert_ok!(make_default_simple_project());

        let builder_data = make_user(
            2,
            Some(make_field_name("Builder Test")),
            Some(ProxyRole::Builder),
            CUDAction::Create,
        );

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            builder_data,
        ));

        let builder_assignment = make_user_assignation(
            2,
            ProxyRole::Builder,
            AssignAction::Assign,
        );

        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

        assert_ok!(FundAdmin::projects_assign_user(
            RuntimeOrigin::signed(1),
            get_project_id,
            builder_assignment,
        ));

        assert_noop!(FundAdmin::users(
                RuntimeOrigin::signed(1),
                make_user(
                    2,
                    None,
                    None,
                    CUDAction::Delete,
                ),
            ),
            Error::<Test>::UserHasAssignedProjectsCannotDelete
        );
    });
}

#[test]
fn users_cannot_update_user_role_from_an_account_with_assigned_projects_should_fail() {
    new_test_ext().execute_with(|| {
        assert_ok!(make_default_simple_project());

        let builder_data = make_user(
            2,
            Some(make_field_name("Builder Test")),
            Some(ProxyRole::Builder),
            CUDAction::Create,
        );

        assert_ok!(FundAdmin::users(
            RuntimeOrigin::signed(1),
            builder_data,
        ));

        let builder_assignment = make_user_assignation(
            2,
            ProxyRole::Builder,
            AssignAction::Assign,
        );

        let get_project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

        assert_ok!(FundAdmin::projects_assign_user(
            RuntimeOrigin::signed(1),
            get_project_id,
            builder_assignment,
        ));

        assert_noop!(FundAdmin::users(
                RuntimeOrigin::signed(1),
                make_user(
                    2,
                    Some(make_field_name("Builder Test")),
                    Some(ProxyRole::Investor),
                    CUDAction::Update,
                ),
            ),
            Error::<Test>::UserHasAssignedProjectsCannotUpdateRole
        );
    });
}
