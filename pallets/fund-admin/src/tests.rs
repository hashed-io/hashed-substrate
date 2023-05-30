use crate::{
  mock::*, types::*, DrawdownsByProject, DrawdownsInfo, Error, ExpendituresByProject,
  ExpendituresInfo, GlobalScope, JobEligiblesByProject, JobEligiblesInfo, ProjectsByUser,
  ProjectsInfo, RevenueTransactionsInfo, RevenuesByProject, RevenuesInfo, TransactionsByDrawdown,
  TransactionsByRevenue, TransactionsInfo, UsersByProject, UsersInfo,
};
use frame_support::{
  assert_noop, assert_ok, bounded_vec, error::BadOrigin, traits::ConstU32, BoundedVec,
};
use sp_runtime::DispatchResult;

type RbacErr = pallet_rbac::Error<Test>;

#[allow(dead_code)]
fn pallet_id() -> [u8; 32] {
  FundAdmin::pallet_id().to_id()
}

#[allow(dead_code)]
fn pallet_name() -> pallet_rbac::types::IdOrVec {
  pallet_rbac::types::IdOrVec::Vec("pallet_test".as_bytes().to_vec())
}

fn make_field_name(name: &str) -> FieldName {
  let name: BoundedVec<u8, ConstU32<100>> = name.as_bytes().to_vec().try_into().unwrap_or_default();
  name
}

fn make_field_description(description: &str) -> FieldDescription {
  let description: BoundedVec<u8, ConstU32<400>> =
    description.as_bytes().to_vec().try_into().unwrap_or_default();
  description
}

fn make_documents(n_files: u32) -> Documents<Test> {
  let mut documents: Documents<Test> = bounded_vec![];
  for i in 0..n_files {
    let file_name: &str = &format!("file_{}", i);
    let file_description: &str = &format!("file_{}_description", i);
    documents
      .try_push((make_field_name(file_name), make_field_name(file_description)))
      .unwrap_or_default();
  }
  documents
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
  )
  .map_err(|_| Error::<Test>::UserAlreadyRegistered)?;
  Ok(())
}

fn make_user(
  user_account: u64,
  user_name: Option<FieldName>,
  user_role: Option<ProxyRole>,
  action: CUDAction,
) -> Users<Test> {
  let mut users: Users<Test> = bounded_vec![];
  users.try_push((user_account, user_name, user_role, action)).unwrap_or_default();
  users
}

fn make_default_users() -> Users<Test> {
  let mut users: Users<Test> = bounded_vec![];
  let users_account = [2, 3, 4, 5];
  let users_name: Vec<FieldName> =
    ["Builder Test", "Investor Test", "Issuer Test", "Regional Center Test"]
      .iter()
      .map(|s| make_field_name(s))
      .collect();
  let users_role: Vec<ProxyRole> =
    [ProxyRole::Builder, ProxyRole::Investor, ProxyRole::Issuer, ProxyRole::RegionalCenter]
      .iter()
      .map(|s| *s)
      .collect();
  let cud_action = CUDAction::Create;

  for i in 0..users_account.len() {
    users
      .try_push((users_account[i], Some(users_name[i].clone()), Some(users_role[i]), cud_action))
      .unwrap_or_default();
  }
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
  budget_expenditure_id: Option<ExpenditureId>,
) -> Expenditures<Test> {
  let mut expenditures: Expenditures<Test> = bounded_vec![];
  expenditures
    .try_push((
      expenditure_name,
      expenditure_type,
      expenditure_amount,
      naics_code,
      jobs_multiplier,
      action,
      budget_expenditure_id,
    ))
    .unwrap_or_default();
  expenditures
}

fn make_default_expenditures() -> Expenditures<Test> {
  let mut expenditures: Expenditures<Test> = bounded_vec![];
  let expenditure_name: Vec<FieldName> =
    ["Expenditure Test 1", "Expenditure Test 2", "Expenditure Test 3", "Expenditure Test 4"]
      .iter()
      .map(|s| make_field_name(s))
      .collect();
  let expenditure_type: Vec<ExpenditureType> = [
    ExpenditureType::HardCost,
    ExpenditureType::SoftCost,
    ExpenditureType::Operational,
    ExpenditureType::Others,
  ]
  .iter()
  .map(|s| *s)
  .collect();
  let expenditure_amount: Vec<u64> = [100, 200, 300, 400].iter().map(|s| *s).collect();
  let naics_code: Vec<FieldDescription> = [1231, 1232, 1233, 1234]
    .iter()
    .map(|s| make_field_description(&s.to_string()))
    .collect();
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
  }
  expenditures
}

#[allow(dead_code)]
fn make_job_eligible(
  field_name: Option<FieldName>,
  job_eligible_amount: Option<JobEligibleAmount>,
  naics_code: Option<NAICSCode>,
  jobs_multiplier: Option<JobsMultiplier>,
  action: CUDAction,
  budget_job_eligible_id: Option<JobEligibleId>,
) -> JobEligibles<Test> {
  let mut job_eligibles: JobEligibles<Test> = bounded_vec![];
  job_eligibles
    .try_push((
      field_name,
      job_eligible_amount,
      naics_code,
      jobs_multiplier,
      action,
      budget_job_eligible_id,
    ))
    .unwrap_or_default();
  job_eligibles
}

fn make_default_job_eligibles() -> JobEligibles<Test> {
  let mut job_eligibles: JobEligibles<Test> = bounded_vec![];
  let field_name = make_field_name("Job Eligible Test");
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
  users_assignation
    .try_push((user_account, user_role, action))
    .unwrap_or_default();
  users_assignation
}

fn make_default_user_assignation() -> UsersAssignation<Test> {
  let mut users_assignation: UsersAssignation<Test> = bounded_vec![];
  let user_account: Vec<u64> = [2, 3, 4, 5].iter().map(|s| *s).collect();
  let user_role: Vec<ProxyRole> =
    [ProxyRole::Builder, ProxyRole::Investor, ProxyRole::Issuer, ProxyRole::RegionalCenter]
      .iter()
      .map(|s| *s)
      .collect();
  let action = AssignAction::Assign;

  for i in 0..user_account.len() {
    users_assignation
      .try_push((user_account[i], user_role[i], action))
      .unwrap_or_default();
  }
  users_assignation
}

#[allow(dead_code)]
fn make_allowed_bank(bank_name: BankName, bank_address: BankAddress) -> Banks<Test> {
  let mut banks: Banks<Test> = bounded_vec![];
  banks.try_push((bank_name, bank_address)).unwrap_or_default();
  banks
}

fn make_default_allowed_banks() -> Banks<Test> {
  let mut banks: Banks<Test> = bounded_vec![];
  let bank_name = make_field_name("Luxury Bank");
  let bank_address = make_field_name("San Francisco");
  banks.try_push((bank_name, bank_address)).unwrap_or_default();
  banks
}

fn make_transaction_feedback(
  transaction_id: TransactionId,
  feedback: FieldDescription,
) -> TransactionsFeedback<Test> {
  let mut transaction_feedback: TransactionsFeedback<Test> = bounded_vec![];
  transaction_feedback.try_push((transaction_id, feedback)).unwrap_or_default();
  transaction_feedback
}

fn make_transaction(
  expenditure_id: Option<ExpenditureId>,
  expenditure_amount: Option<ExpenditureAmount>,
  action: CUDAction,
  transaction_id: Option<TransactionId>,
) -> Transactions<Test> {
  let mut transactions: Transactions<Test> = bounded_vec![];
  let documents = Some(make_documents(1));
  transactions
    .try_push((expenditure_id, expenditure_amount, documents, action, transaction_id))
    .unwrap_or_default();
  transactions
}

fn make_revenue_transaction(
  job_eligible_id: Option<JobEligibleId>,
  job_eligible_amount: Option<JobEligibleAmount>,
  action: CUDAction,
  revenue_transaction_id: Option<RevenueTransactionId>,
) -> RevenueTransactions<Test> {
  let mut revenue_transactions: RevenueTransactions<Test> = bounded_vec![];
  let documents = Some(make_documents(1));
  revenue_transactions
    .try_push((job_eligible_id, job_eligible_amount, documents, action, revenue_transaction_id))
    .unwrap_or_default();
  revenue_transactions
}

fn make_project_inflation(
  project_id: ProjectId,
  inflation: Option<InflationRate>,
  action: CUDAction,
) -> ProjectsInflation<Test> {
  let mut projects_inflation: ProjectsInflation<Test> = bounded_vec![];
  projects_inflation.try_push((project_id, inflation, action)).unwrap_or_default();
  projects_inflation
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

/// Create a project with all the fields
/// This project will be used to test all the functions
/// of the project pallet
/// This project will be created by the admin
///
/// ### Default users:
/// - user_account: 1 -> admin
/// - user_account: 2 -> builder
/// - user_account: 3 -> investor
/// - user_account: 4 -> issuer
/// - user_account: 5 -> regional center
fn make_default_full_project() -> DispatchResult {
  register_administrator()?;

  FundAdmin::users(RuntimeOrigin::signed(1), make_default_users())?;

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

fn get_drawdown_id(
  project_id: ProjectId,
  drawdown_type: DrawdownType,
  drawdown_number: DrawdownNumber,
) -> DrawdownId {
  let mut drawdown_id: DrawdownId = [0; 32];
  let drawdonws_by_project = DrawdownsByProject::<Test>::get(project_id);

  for i in 0..drawdonws_by_project.len() {
    let drawdown_data = DrawdownsInfo::<Test>::get(drawdonws_by_project[i]).unwrap();
    if drawdown_data.drawdown_type == drawdown_type
      && drawdown_data.drawdown_number == drawdown_number
    {
      drawdown_id = drawdonws_by_project[i];
    }
  }
  drawdown_id
}

fn get_budget_expenditure_id(
  project_id: ProjectId,
  name: FieldName,
  expenditure_type: ExpenditureType,
) -> ExpenditureId {
  let mut expenditure_id: [u8; 32] = [0; 32];
  let expenditures_by_project = ExpendituresByProject::<Test>::get(project_id);

  for i in 0..expenditures_by_project.len() {
    let expenditure_data = ExpendituresInfo::<Test>::get(expenditures_by_project[i]).unwrap();
    if expenditure_data.name == name && expenditure_data.expenditure_type == expenditure_type {
      expenditure_id = expenditures_by_project[i];
    }
  }
  expenditure_id
}

fn get_transaction_id(
  project_id: ProjectId,
  drawdown_id: DrawdownId,
  expenditure_id: ExpenditureId,
) -> TransactionId {
  let mut transaction_id: [u8; 32] = [0; 32];
  let transactions_by_drawdown = TransactionsByDrawdown::<Test>::get(project_id, drawdown_id);

  for i in 0..transactions_by_drawdown.len() {
    let transaction_data = TransactionsInfo::<Test>::get(transactions_by_drawdown[i]).unwrap();
    if transaction_data.project_id == project_id
      && transaction_data.drawdown_id == drawdown_id
      && transaction_data.expenditure_id == expenditure_id
    {
      transaction_id = transactions_by_drawdown[i];
    }
  }
  transaction_id
}

fn get_revenue_id(project_id: ProjectId, revenue_number: RevenueNumber) -> RevenueId {
  let mut revenue_id: RevenueId = [0; 32];
  let revenues_by_project = RevenuesByProject::<Test>::get(project_id);

  for i in 0..revenues_by_project.len() {
    let revenue_data = RevenuesInfo::<Test>::get(revenues_by_project[i]).unwrap();
    if revenue_data.revenue_number == revenue_number {
      revenue_id = revenues_by_project[i];
    }
  }
  revenue_id
}

fn get_job_eligible_id(project_id: ProjectId, name: FieldName) -> JobEligibleId {
  let mut job_eligible_id: [u8; 32] = [0; 32];
  let job_eligibles_by_project = JobEligiblesByProject::<Test>::get(project_id);

  for i in 0..job_eligibles_by_project.len() {
    let job_eligible_data = JobEligiblesInfo::<Test>::get(job_eligibles_by_project[i]).unwrap();
    if job_eligible_data.name == name {
      job_eligible_id = job_eligibles_by_project[i];
    }
  }
  job_eligible_id
}

fn get_revenue_transaction_id(
  project_id: ProjectId,
  revenue_id: RevenueId,
  job_eligible_id: JobEligibleId,
) -> RevenueTransactionId {
  let mut revenue_transaction_id: RevenueTransactionId = [0; 32];
  let transactions_by_revenue = TransactionsByRevenue::<Test>::get(project_id, revenue_id);

  for i in 0..transactions_by_revenue.len() {
    let revenue_transaction_data =
      RevenueTransactionsInfo::<Test>::get(transactions_by_revenue[i]).unwrap();
    if revenue_transaction_data.project_id == project_id
      && revenue_transaction_data.revenue_id == revenue_id
      && revenue_transaction_data.job_eligible_id == job_eligible_id
    {
      revenue_transaction_id = transactions_by_revenue[i];
    }
  }
  revenue_transaction_id
}

// I N I T I A L
// -----------------------------------------------------------------------------------------
#[test]
fn global_scope_is_created_after_pallet_initialization() {
  new_test_ext().execute_with(|| {
    assert!(GlobalScope::<Test>::exists());
  });
}

#[test]
fn cannon_initialize_pallet_twice_shouldnt_work() {
  new_test_ext().execute_with(|| {
    assert_noop!(FundAdmin::initial_setup(RuntimeOrigin::root()), RbacErr::ScopeAlreadyExists);
  });
}

#[test]
fn sudo_register_administrator_account_works() {
  new_test_ext().execute_with(|| {
    let alice_name = make_field_name("Alice Keys");
    assert_ok!(FundAdmin::sudo_add_administrator(RuntimeOrigin::root(), 2, alice_name.clone()));
    assert!(UsersInfo::<Test>::contains_key(2));
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
    assert_ok!(FundAdmin::sudo_add_administrator(RuntimeOrigin::root(), 2, alice_name.clone()));
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
    assert_ok!(FundAdmin::sudo_add_administrator(RuntimeOrigin::root(), 2, alice_name.clone()));
    assert!(FundAdmin::users_info(2).is_some());

    assert_ok!(FundAdmin::sudo_remove_administrator(RuntimeOrigin::root(), 2,));
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

    assert_ok!(FundAdmin::sudo_remove_administrator(RuntimeOrigin::root(), 2,));
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

    assert_ok!(FundAdmin::sudo_remove_administrator(RuntimeOrigin::root(), 2,));
    assert!(FundAdmin::users_info(2).is_none());
  });
}

// B A L A N C E S
// =================================================================================================
#[test]
fn balances_main_account_has_an_initial_balance_works() {
  new_test_ext().execute_with(|| {
    // Get administrator free balance
    let free_balance = Balances::free_balance(1);
    assert_eq!(free_balance, InitialAdminBalance::get());
  });
}

#[test]
fn balances_any_other_account_should_have_a_zero_balance_works() {
  new_test_ext().execute_with(|| {
    // Get non-registered user free balance
    let free_balance = Balances::free_balance(1);
    let free_balance_2 = Balances::free_balance(2);
    let free_balance_3 = Balances::free_balance(3);

    assert_eq!(free_balance, InitialAdminBalance::get());
    assert_eq!(free_balance_2, 0);
    assert_eq!(free_balance_3, 0);
  });
}

#[test]
fn balances_a_new_registered_user_should_have_a_initial_balance_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(register_administrator());

    assert_ok!(FundAdmin::users(
      RuntimeOrigin::signed(1),
      make_user(
        2,
        Some(make_field_name("Alice Builder")),
        Some(ProxyRole::Builder),
        CUDAction::Create
      )
    ));

    assert!(FundAdmin::users_info(2).is_some());

    // Get non-registered user free balance
    let admin_free_balance = Balances::free_balance(1);
    let user_free_balance = Balances::free_balance(2);
    assert_eq!(admin_free_balance, InitialAdminBalance::get() - TransferAmount::get());
    assert_eq!(user_free_balance, TransferAmount::get());
  });
}

#[test]
fn balances_an_administrator_goes_out_of_balance_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(register_administrator());

    assert_ok!(FundAdmin::users(
      RuntimeOrigin::signed(1),
      make_user(
        2,
        Some(make_field_name("Alice Builder")),
        Some(ProxyRole::Builder),
        CUDAction::Create
      )
    ));

    let admin_free_balance = Balances::free_balance(1);
    assert_eq!(admin_free_balance, InitialAdminBalance::get() - TransferAmount::get());

    Balances::transfer(RuntimeOrigin::signed(1), 2, admin_free_balance - TransferAmount::get() / 2)
      .unwrap();

    assert_noop!(
      FundAdmin::users(
        RuntimeOrigin::signed(1),
        make_user(
          3,
          Some(make_field_name("Bob Investor")),
          Some(ProxyRole::Investor),
          CUDAction::Create
        ),
      ),
      Error::<Test>::InsufficientFundsToTransfer
    );
  });
}

#[test]
fn balances_an_administrator_does_not_have_anough_free_balance_to_perform_a_user_registration() {
  new_test_ext().execute_with(|| {
    assert_ok!(register_administrator());

    assert_ok!(FundAdmin::users(
      RuntimeOrigin::signed(1),
      make_user(
        2,
        Some(make_field_name("Alice Builder")),
        Some(ProxyRole::Builder),
        CUDAction::Create
      )
    ));

    let admin_free_balance = Balances::free_balance(1);
    assert_eq!(admin_free_balance, InitialAdminBalance::get() - TransferAmount::get());

    Balances::transfer(RuntimeOrigin::signed(1), 2, admin_free_balance).unwrap();

    assert_noop!(
      FundAdmin::users(
        RuntimeOrigin::signed(1),
        make_user(
          3,
          Some(make_field_name("Bob Investor")),
          Some(ProxyRole::Investor),
          CUDAction::Create
        ),
      ),
      Error::<Test>::AdminHasNoFreeBalance
    );
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
      make_user(
        2,
        Some(make_field_name("Alice Administrator")),
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
      RuntimeOrigin::signed(1),
      make_user(
        2,
        Some(make_field_name("Alice Builder")),
        Some(ProxyRole::Builder),
        CUDAction::Create
      )
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
      make_user(
        2,
        Some(make_field_name("Alice Investor")),
        Some(ProxyRole::Investor),
        CUDAction::Create
      )
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
      make_user(
        2,
        Some(make_field_name("Alice Issuer")),
        Some(ProxyRole::Issuer),
        CUDAction::Create
      )
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
      make_user(
        2,
        Some(make_field_name("Alice Regional Center")),
        Some(ProxyRole::RegionalCenter),
        CUDAction::Create
      )
    ));

    assert!(FundAdmin::users_info(2).is_some());
  });
}

#[test]
fn users_register_multiple_accounts_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(register_administrator());

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), make_default_users()));

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
        make_user(
          2,
          Some(make_field_name("Alice Regional Center")),
          Some(ProxyRole::RegionalCenter),
          CUDAction::Create
        )
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
      make_user(
        2,
        Some(make_field_name("Alice Regional Center")),
        Some(ProxyRole::RegionalCenter),
        CUDAction::Create
      )
    ));

    assert_noop!(
      FundAdmin::users(
        RuntimeOrigin::signed(1),
        make_user(
          2,
          Some(make_field_name("Alice Regional Center")),
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
      RuntimeOrigin::signed(1),
      make_user(
        2,
        Some(make_field_name("Alice Regional Center")),
        Some(ProxyRole::RegionalCenter),
        CUDAction::Create
      )
    ));

    assert_ok!(FundAdmin::users(
      RuntimeOrigin::signed(1),
      make_user(2, Some(make_field_name("Alice Regional Center Updated")), None, CUDAction::Update)
    ));

    assert_eq!(
      field_name_to_string(&FundAdmin::users_info(2).unwrap().name),
      String::from("Alice Regional Center Updated")
    );
  });
}

#[test]
fn users_admnistrator_updates_role_of_a_registered_account_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(register_administrator());

    assert_ok!(FundAdmin::users(
      RuntimeOrigin::signed(1),
      make_user(
        2,
        Some(make_field_name("Alice Regional Center")),
        Some(ProxyRole::RegionalCenter),
        CUDAction::Create
      )
    ));

    assert_ok!(FundAdmin::users(
      RuntimeOrigin::signed(1),
      make_user(
        2,
        Some(make_field_name("Alice Investor")),
        Some(ProxyRole::Investor),
        CUDAction::Update
      )
    ));

    assert_eq!(FundAdmin::users_info(2).unwrap().role, ProxyRole::Investor);
    assert_eq!(FundAdmin::users_info(2).unwrap().name, make_field_name("Alice Investor"));
    00
  });
}

#[test]
fn users_update_a_non_registered_account_shouldnt_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(register_administrator());

    assert_noop!(
      FundAdmin::users(
        RuntimeOrigin::signed(1),
        make_user(
          2,
          Some(make_field_name("Alice Regional Center")),
          Some(ProxyRole::RegionalCenter),
          CUDAction::Update
        )
      ),
      Error::<Test>::UserNotRegistered
    );
  });
}

#[test]
fn users_delete_a_registered_account_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(register_administrator());

    assert_ok!(FundAdmin::users(
      RuntimeOrigin::signed(1),
      make_user(
        2,
        Some(make_field_name("Alice Regional Center")),
        Some(ProxyRole::RegionalCenter),
        CUDAction::Create
      )
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
      FundAdmin::users(RuntimeOrigin::signed(1), make_user(2, None, None, CUDAction::Delete)),
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
      make_user(
        2,
        Some(make_field_name("Bob Regional Center")),
        Some(ProxyRole::RegionalCenter),
        CUDAction::Create
      )
    ));

    assert_ok!(FundAdmin::users_edit_user(
      RuntimeOrigin::signed(2),
      Some(make_field_name("Bob Regiona Center New York")),
      Some(make_field_name("image.png")),
      Some(make_field_name("bob.regionalcenter@fundadmin.com")),
      None,
    ));

    assert_eq!(FundAdmin::users_info(2).unwrap().role, ProxyRole::RegionalCenter);
    assert_eq!(
      FundAdmin::users_info(2).unwrap().name,
      make_field_name("Bob Regiona Center New York")
    );
    assert_eq!(FundAdmin::users_info(2).unwrap().image, make_field_name("image.png"));
    assert_eq!(
      FundAdmin::users_info(2).unwrap().email,
      make_field_name("bob.regionalcenter@fundadmin.com")
    );
  });
}

#[test]
fn users_only_investors_can_upload_documentation_to_their_account_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(register_administrator());

    assert_ok!(FundAdmin::users(
      RuntimeOrigin::signed(1),
      make_user(
        2,
        Some(make_field_name("Bob Investor")),
        Some(ProxyRole::Investor),
        CUDAction::Create
      )
    ));

    assert_ok!(FundAdmin::users_edit_user(
      RuntimeOrigin::signed(2),
      None,
      None,
      None,
      Some(make_documents(1)),
    ));
    assert_eq!(FundAdmin::users_info(2).unwrap().name, make_field_name("Bob Investor"));
    assert_eq!(FundAdmin::users_info(2).unwrap().documents, Some(make_documents(1)));
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
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    assert_eq!(ProjectsInfo::<Test>::get(project_id).unwrap().title, make_field_name("Project 1"));

    assert_eq!(
      ExpendituresInfo::<Test>::iter().count(),
      ExpendituresByProject::<Test>::get(project_id).len()
    );
    let get_expenditure_ids: Vec<[u8; 32]> =
      ExpendituresByProject::<Test>::get(project_id).iter().cloned().collect();
    for i in get_expenditure_ids {
      assert_eq!(ExpendituresInfo::<Test>::get(i).unwrap().project_id, project_id);
    }

    assert_eq!(
      DrawdownsInfo::<Test>::iter().count(),
      DrawdownsByProject::<Test>::get(project_id).len()
    );
    let get_drawdown_ids: Vec<[u8; 32]> =
      DrawdownsByProject::<Test>::get(project_id).iter().cloned().collect();
    for i in get_drawdown_ids {
      assert_eq!(DrawdownsInfo::<Test>::get(i).unwrap().project_id, project_id);
    }

    assert_eq!(
      RevenuesInfo::<Test>::iter().count(),
      RevenuesByProject::<Test>::get(project_id).len()
    );
    let get_revenue_ids: Vec<[u8; 32]> =
      RevenuesByProject::<Test>::get(project_id).iter().cloned().collect();
    for i in get_revenue_ids {
      assert_eq!(RevenuesInfo::<Test>::get(i).unwrap().project_id, project_id);
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
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    assert_eq!(
      JobEligiblesInfo::<Test>::iter().count(),
      JobEligiblesByProject::<Test>::get(project_id).len()
    );

    let get_job_eligible_ids: Vec<[u8; 32]> =
      JobEligiblesByProject::<Test>::get(project_id).iter().cloned().collect();
    for i in get_job_eligible_ids {
      assert_eq!(JobEligiblesInfo::<Test>::get(i).unwrap().project_id, project_id);
    }
  });
}

#[test]
fn projects_register_a_project_with_assigned_users_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(register_administrator());

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), make_default_users(),));

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
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    assert_eq!(UsersByProject::<Test>::get(project_id).len(), 4);

    let get_assigned_user_ids: Vec<u64> =
      UsersByProject::<Test>::get(project_id).iter().cloned().collect();
    for i in get_assigned_user_ids {
      assert_eq!(ProjectsByUser::<Test>::get(i).len(), 1);
      assert_eq!(ProjectsByUser::<Test>::get(i).iter().next().unwrap(), &project_id);
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
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().banks,
      Some(make_default_allowed_banks())
    );
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
      Error::<Test>::PrivateGroupIdEmpty
    );
  });
}

#[test]
fn projects_a_non_authorized_user_registers_a_project_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(register_administrator());

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), make_default_users(),));

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

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), make_default_users(),));

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

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), make_default_users(),));

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
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    assert_ok!(FundAdmin::projects_edit_project(
      RuntimeOrigin::signed(1),
      project_id,
      Some(make_field_name("Project 1 edited")),
      Some(make_field_description("Project 1 description edited")),
      Some(make_field_name("project_image.jpeg")),
      Some(make_field_name("California")),
      None,
      Some(5000u64),
      Some(10000u64),
    ));

    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().title,
      make_field_name("Project 1 edited")
    );
  });
}

#[test]
fn projects_delete_project_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());

    assert_eq!(ProjectsInfo::<Test>::iter_values().count(), 1);
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let get_expenditure_ids: Vec<[u8; 32]> =
      ExpendituresByProject::<Test>::get(project_id).iter().cloned().collect();
    let get_drawdown_ids: Vec<[u8; 32]> =
      DrawdownsByProject::<Test>::get(project_id).iter().cloned().collect();
    let get_revenue_ids: Vec<[u8; 32]> =
      RevenuesByProject::<Test>::get(project_id).iter().cloned().collect();
    let get_job_eligible_ids: Vec<[u8; 32]> =
      JobEligiblesByProject::<Test>::get(project_id).iter().cloned().collect();
    let get_assigned_user_ids: Vec<u64> =
      UsersByProject::<Test>::get(project_id).iter().cloned().collect();

    assert_ok!(FundAdmin::projects_delete_project(RuntimeOrigin::signed(1), project_id,));

    // Ensure project data was deleted
    assert_eq!(ProjectsInfo::<Test>::contains_key(project_id), false);
    assert_eq!(ExpendituresInfo::<Test>::contains_key(project_id), false);
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
      assert_eq!(UsersByProject::<Test>::get(project_id).contains(&assigned_user_id), false);
      assert_eq!(ProjectsByUser::<Test>::get(assigned_user_id).contains(&project_id), false);
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

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), builder_data,));

    let builder_assignment = make_user_assignation(2, ProxyRole::Builder, AssignAction::Assign);

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    assert_ok!(FundAdmin::projects_assign_user(
      RuntimeOrigin::signed(1),
      project_id,
      builder_assignment,
    ));

    assert_eq!(UsersByProject::<Test>::get(project_id).contains(&2), true);
    assert_eq!(ProjectsByUser::<Test>::get(2).contains(&project_id), true);
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

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), investor_data,));

    let investor_assignment = make_user_assignation(3, ProxyRole::Investor, AssignAction::Assign);

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    assert_ok!(FundAdmin::projects_assign_user(
      RuntimeOrigin::signed(1),
      project_id,
      investor_assignment,
    ));

    assert_eq!(UsersByProject::<Test>::get(project_id).contains(&3), true);
    assert_eq!(ProjectsByUser::<Test>::get(3).contains(&project_id), true);
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

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), issuer_data,));

    let issuer_assignment = make_user_assignation(4, ProxyRole::Issuer, AssignAction::Assign);

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    assert_ok!(FundAdmin::projects_assign_user(
      RuntimeOrigin::signed(1),
      project_id,
      issuer_assignment,
    ));

    assert_eq!(UsersByProject::<Test>::get(project_id).contains(&4), true);
    assert_eq!(ProjectsByUser::<Test>::get(4).contains(&project_id), true);
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

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), regional_center_data,));

    let regional_center_assignment =
      make_user_assignation(5, ProxyRole::RegionalCenter, AssignAction::Assign);

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    assert_ok!(FundAdmin::projects_assign_user(
      RuntimeOrigin::signed(1),
      project_id,
      regional_center_assignment,
    ));

    assert_eq!(UsersByProject::<Test>::get(project_id).contains(&5), true);
    assert_eq!(ProjectsByUser::<Test>::get(5).contains(&project_id), true);
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

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), builder_data,));

    let builder_assignment = make_user_assignation(2, ProxyRole::Builder, AssignAction::Assign);

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    assert_ok!(FundAdmin::projects_assign_user(
      RuntimeOrigin::signed(1),
      project_id,
      builder_assignment,
    ));

    assert_eq!(UsersByProject::<Test>::get(project_id).contains(&2), true);
    assert_eq!(ProjectsByUser::<Test>::get(2).contains(&project_id), true);

    let builder_unassignment = make_user_assignation(2, ProxyRole::Builder, AssignAction::Unassign);

    assert_ok!(FundAdmin::projects_assign_user(
      RuntimeOrigin::signed(1),
      project_id,
      builder_unassignment,
    ));

    assert_eq!(UsersByProject::<Test>::get(project_id).contains(&2), false);
    assert_eq!(ProjectsByUser::<Test>::get(2).contains(&project_id), false);
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

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), investor_data,));

    let investor_assignment = make_user_assignation(3, ProxyRole::Investor, AssignAction::Assign);

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    assert_ok!(FundAdmin::projects_assign_user(
      RuntimeOrigin::signed(1),
      project_id,
      investor_assignment,
    ));

    assert_eq!(UsersByProject::<Test>::get(project_id).contains(&3), true);
    assert_eq!(ProjectsByUser::<Test>::get(3).contains(&project_id), true);

    let investor_unassignment =
      make_user_assignation(3, ProxyRole::Investor, AssignAction::Unassign);

    assert_ok!(FundAdmin::projects_assign_user(
      RuntimeOrigin::signed(1),
      project_id,
      investor_unassignment,
    ));

    assert_eq!(UsersByProject::<Test>::get(project_id).contains(&3), false);
    assert_eq!(ProjectsByUser::<Test>::get(3).contains(&project_id), false);
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

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), issuer_data,));

    let issuer_assignment = make_user_assignation(4, ProxyRole::Issuer, AssignAction::Assign);

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    assert_ok!(FundAdmin::projects_assign_user(
      RuntimeOrigin::signed(1),
      project_id,
      issuer_assignment,
    ));

    assert_eq!(UsersByProject::<Test>::get(project_id).contains(&4), true);
    assert_eq!(ProjectsByUser::<Test>::get(4).contains(&project_id), true);

    let issuer_unassignment = make_user_assignation(4, ProxyRole::Issuer, AssignAction::Unassign);

    assert_ok!(FundAdmin::projects_assign_user(
      RuntimeOrigin::signed(1),
      project_id,
      issuer_unassignment,
    ));

    assert_eq!(UsersByProject::<Test>::get(project_id).contains(&4), false);
    assert_eq!(ProjectsByUser::<Test>::get(4).contains(&project_id), false);
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

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), regional_center_data,));

    let regional_center_assignment =
      make_user_assignation(5, ProxyRole::RegionalCenter, AssignAction::Assign);

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    assert_ok!(FundAdmin::projects_assign_user(
      RuntimeOrigin::signed(1),
      project_id,
      regional_center_assignment,
    ));

    assert_eq!(UsersByProject::<Test>::get(project_id).contains(&5), true);
    assert_eq!(ProjectsByUser::<Test>::get(5).contains(&project_id), true);

    let regional_center_unassignment =
      make_user_assignation(5, ProxyRole::RegionalCenter, AssignAction::Unassign);

    assert_ok!(FundAdmin::projects_assign_user(
      RuntimeOrigin::signed(1),
      project_id,
      regional_center_unassignment,
    ));

    assert_eq!(UsersByProject::<Test>::get(project_id).contains(&5), false);
    assert_eq!(ProjectsByUser::<Test>::get(5).contains(&project_id), false);
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

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), builder_data,));

    let builder_assignment = make_user_assignation(2, ProxyRole::Builder, AssignAction::Assign);

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    assert_ok!(FundAdmin::projects_assign_user(
      RuntimeOrigin::signed(1),
      project_id,
      builder_assignment.clone(),
    ));

    assert_eq!(UsersByProject::<Test>::get(project_id).contains(&2), true);
    assert_eq!(ProjectsByUser::<Test>::get(2).contains(&project_id), true);

    assert_noop!(
      FundAdmin::projects_assign_user(RuntimeOrigin::signed(1), project_id, builder_assignment,),
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

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), builder_data,));

    let investor_assignment = make_user_assignation(2, ProxyRole::Investor, AssignAction::Assign);

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    assert_noop!(
      FundAdmin::projects_assign_user(RuntimeOrigin::signed(1), project_id, investor_assignment,),
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

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), builder_data,));

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let builder_assignment = make_user_assignation(2, ProxyRole::Builder, AssignAction::Assign);

    assert_ok!(FundAdmin::projects_assign_user(
      RuntimeOrigin::signed(1),
      project_id,
      builder_assignment,
    ));

    let investor_assignment = make_user_assignation(2, ProxyRole::Investor, AssignAction::Assign);

    assert_noop!(
      FundAdmin::projects_assign_user(RuntimeOrigin::signed(1), project_id, investor_assignment,),
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

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), builder_data,));

    let builder_assignment = make_user_assignation(2, ProxyRole::Builder, AssignAction::Assign);

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    assert_ok!(FundAdmin::projects_assign_user(
      RuntimeOrigin::signed(1),
      project_id,
      builder_assignment,
    ));

    assert_noop!(
      FundAdmin::users(RuntimeOrigin::signed(1), make_user(2, None, None, CUDAction::Delete,),),
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

    assert_ok!(FundAdmin::users(RuntimeOrigin::signed(1), builder_data,));

    let builder_assignment = make_user_assignation(2, ProxyRole::Builder, AssignAction::Assign);

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    assert_ok!(FundAdmin::projects_assign_user(
      RuntimeOrigin::signed(1),
      project_id,
      builder_assignment,
    ));

    assert_noop!(
      FundAdmin::users(
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

// E X P E N D I T U R E S
// =================================================================================================
#[test]
fn expenditures_add_a_hard_cost_budget_expenditure_for_a_given_project_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: HardCost")),
      Some(ExpenditureType::HardCost),
      Some(100),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(expenditure_data),
      None,
    ));

    let get_expenditure_ids: Vec<[u8; 32]> =
      ExpendituresByProject::<Test>::get(project_id).iter().cloned().collect();
    let mut target_expenditure_id: [u8; 32] = [0; 32];

    for expenditure_id in get_expenditure_ids {
      let expenditure_data = ExpendituresInfo::<Test>::get(expenditure_id)
        .ok_or(Error::<Test>::ExpenditureNotFound)
        .unwrap();
      if expenditure_data.name == make_field_name("Expenditure Test: HardCost") {
        target_expenditure_id = expenditure_id;
        break;
      }
    }

    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().name,
      make_field_name("Expenditure Test: HardCost")
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().expenditure_type,
      ExpenditureType::HardCost
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().expenditure_amount,
      100
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().naics_code,
      Some(make_field_description("16344, 45862, 57143"))
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().jobs_multiplier,
      Some(200)
    );
  });
}

#[test]
fn expenditures_add_a_softcost_budget_expenditure_for_a_given_project_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: SoftCost")),
      Some(ExpenditureType::SoftCost),
      Some(100),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(expenditure_data),
      None,
    ));

    let get_expenditure_ids: Vec<[u8; 32]> =
      ExpendituresByProject::<Test>::get(project_id).iter().cloned().collect();
    let mut target_expenditure_id: [u8; 32] = [0; 32];

    for expenditure_id in get_expenditure_ids {
      let expenditure_data = ExpendituresInfo::<Test>::get(expenditure_id)
        .ok_or(Error::<Test>::ExpenditureNotFound)
        .unwrap();
      if expenditure_data.name == make_field_name("Expenditure Test: SoftCost") {
        target_expenditure_id = expenditure_id;
        break;
      }
    }

    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().name,
      make_field_name("Expenditure Test: SoftCost")
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().expenditure_type,
      ExpenditureType::SoftCost
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().expenditure_amount,
      100
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().naics_code,
      Some(make_field_description("16344, 45862, 57143"))
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().jobs_multiplier,
      Some(200)
    );
  });
}

#[test]
fn expenditures_add_an_operational_budget_expenditure_for_a_given_project_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: Operational")),
      Some(ExpenditureType::Operational),
      Some(100),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(expenditure_data),
      None,
    ));

    let get_expenditure_ids: Vec<[u8; 32]> =
      ExpendituresByProject::<Test>::get(project_id).iter().cloned().collect();
    let mut target_expenditure_id: [u8; 32] = [0; 32];

    for expenditure_id in get_expenditure_ids {
      let expenditure_data = ExpendituresInfo::<Test>::get(expenditure_id)
        .ok_or(Error::<Test>::ExpenditureNotFound)
        .unwrap();
      if expenditure_data.name == make_field_name("Expenditure Test: Operational") {
        target_expenditure_id = expenditure_id;
        break;
      }
    }

    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().name,
      make_field_name("Expenditure Test: Operational")
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().expenditure_type,
      ExpenditureType::Operational
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().expenditure_amount,
      100
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().naics_code,
      Some(make_field_description("16344, 45862, 57143"))
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().jobs_multiplier,
      Some(200)
    );
  });
}

#[test]
fn expenditures_add_an_others_budget_expenditure_for_a_given_project_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: Others")),
      Some(ExpenditureType::Others),
      Some(100),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(expenditure_data),
      None,
    ));

    let get_expenditure_ids: Vec<[u8; 32]> =
      ExpendituresByProject::<Test>::get(project_id).iter().cloned().collect();
    let mut target_expenditure_id: [u8; 32] = [0; 32];

    for expenditure_id in get_expenditure_ids {
      let expenditure_data = ExpendituresInfo::<Test>::get(expenditure_id)
        .ok_or(Error::<Test>::ExpenditureNotFound)
        .unwrap();
      if expenditure_data.name == make_field_name("Expenditure Test: Others") {
        target_expenditure_id = expenditure_id;
        break;
      }
    }

    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().name,
      make_field_name("Expenditure Test: Others")
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().expenditure_type,
      ExpenditureType::Others
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().expenditure_amount,
      100
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().naics_code,
      Some(make_field_description("16344, 45862, 57143"))
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().jobs_multiplier,
      Some(200)
    );
  });
}

#[test]
fn expenditures_cannot_send_an_empty_array_of_expenditures_for_a_given_project_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let expenditure_data: Expenditures<Test> = bounded_vec![];

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        Some(expenditure_data),
        None,
      ),
      Error::<Test>::EmptyExpenditures
    );
  });
}

#[test]
fn expenditures_cannot_create_a_budget_expenditure_without_a_name_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let expenditure_data = make_expenditure(
      None,
      Some(ExpenditureType::HardCost),
      Some(100),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        Some(expenditure_data),
        None,
      ),
      Error::<Test>::ExpenditureNameRequired
    );
  });
}

#[test]
fn expenditures_cannot_create_a_budget_without_expenditure_type_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: Hard Cost")),
      None,
      Some(100),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        Some(expenditure_data),
        None,
      ),
      Error::<Test>::ExpenditureTypeRequired
    );
  });
}

#[test]
fn expenditures_cannot_create_a_budget_expenditute_without_an_amount_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: Hard Cost")),
      Some(ExpenditureType::HardCost),
      None,
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        Some(expenditure_data),
        None,
      ),
      Error::<Test>::ExpenditureAmountRequired
    );
  });
}

#[test]
fn expenditures_cannot_create_a_budget_expenditure_with_an_empty_name_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let expenditure_data = make_expenditure(
      Some(make_field_name("")),
      Some(ExpenditureType::HardCost),
      Some(100),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        Some(expenditure_data),
        None,
      ),
      Error::<Test>::EmptyExpenditureName
    );
  });
}

#[test]
fn expenditures_edit_a_given_expenditure_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: Hard Cost")),
      Some(ExpenditureType::HardCost),
      Some(100),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(expenditure_data),
      None,
    ));

    let get_expenditure_ids = ExpendituresByProject::<Test>::get(project_id);

    let mut target_expenditure_id: [u8; 32] = [0; 32];

    for expenditure_id in get_expenditure_ids {
      let expenditure_data = ExpendituresInfo::<Test>::get(expenditure_id)
        .ok_or(Error::<Test>::ExpenditureNotFound)
        .unwrap();

      if expenditure_data.name == make_field_name("Expenditure Test: Hard Cost") {
        target_expenditure_id = expenditure_id;
        break;
      }
    }

    let mod_expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: Hard Cost Modified")),
      Some(ExpenditureType::HardCost),
      Some(1000000),
      Some(make_field_description("16344, 57143")),
      Some(200),
      CUDAction::Update,
      Some(target_expenditure_id),
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(mod_expenditure_data),
      None,
    ));

    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().name,
      make_field_name("Expenditure Test: Hard Cost Modified")
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().expenditure_type,
      ExpenditureType::HardCost
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().expenditure_amount,
      1000000
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().naics_code,
      Some(make_field_description("16344, 57143"))
    );
    assert_eq!(
      ExpendituresInfo::<Test>::get(target_expenditure_id).unwrap().jobs_multiplier,
      Some(200)
    );
  });
}

#[test]
fn expenditures_edit_a_given_expenditure_from_another_project_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    assert_ok!(FundAdmin::projects_create_project(
      RuntimeOrigin::signed(1),
      make_field_name("Project 2"),
      make_field_description("Project 2 description"),
      Some(make_field_name("project_image.jpeg")),
      make_field_name("Brooklyn"),
      None,
      1000,
      2000,
      make_default_expenditures(),
      None,
      None,
      make_field_description("P9f5wbr13BK74p1"),
    ));

    let mut project_ids: Vec<ProjectId> = ProjectsInfo::<Test>::iter_keys().collect();
    let first_project_id = project_ids.pop().unwrap();
    let second_project_id = project_ids.pop().unwrap();

    let second_expenditure_id =
      ExpendituresByProject::<Test>::get(second_project_id).pop().unwrap();

    let mod_expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: Hard Cost Modified")),
      Some(ExpenditureType::HardCost),
      Some(1000000),
      Some(make_field_description("16344, 57143")),
      Some(200),
      CUDAction::Update,
      Some(second_expenditure_id),
    );

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        first_project_id,
        Some(mod_expenditure_data),
        None,
      ),
      Error::<Test>::ExpenditureDoesNotBelongToProject
    );
  });
}

#[test]
fn expenditures_expenditure_id_is_required_while_editing_a_given_expenditure_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: Hard Cost")),
      Some(ExpenditureType::HardCost),
      Some(100),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(expenditure_data),
      None,
    ));

    let mod_expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: Hard Cost Modified")),
      Some(ExpenditureType::HardCost),
      Some(1000000),
      Some(make_field_description("16344, 57143")),
      Some(200),
      CUDAction::Update,
      None,
    );

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        Some(mod_expenditure_data),
        None,
      ),
      Error::<Test>::ExpenditureIdRequired
    );
  });
}

#[test]
fn expenditures_admnistrator_tries_to_update_a_non_existent_expenditure_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: Hard Cost")),
      Some(ExpenditureType::HardCost),
      Some(100),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(expenditure_data),
      None,
    ));

    let get_expenditure_ids = ExpendituresByProject::<Test>::get(project_id);

    let mut target_expenditure_id: [u8; 32] = [0; 32];

    for expenditure_id in get_expenditure_ids {
      let expenditure_data = ExpendituresInfo::<Test>::get(expenditure_id)
        .ok_or(Error::<Test>::ExpenditureNotFound)
        .unwrap();

      if expenditure_data.name == make_field_name("Expenditure Test: Hard Cost") {
        target_expenditure_id = expenditure_id;
        break;
      }
    }

    let del_expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: Hard Cost Modified")),
      Some(ExpenditureType::HardCost),
      Some(1000000),
      Some(make_field_description("16344, 57143")),
      Some(200),
      CUDAction::Delete,
      Some(target_expenditure_id),
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(del_expenditure_data),
      None,
    ));

    let mod_expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: Hard Cost Modified")),
      Some(ExpenditureType::HardCost),
      Some(1000000),
      Some(make_field_description("16344, 57143")),
      Some(200),
      CUDAction::Update,
      Some(target_expenditure_id),
    );

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        Some(mod_expenditure_data),
        None,
      ),
      Error::<Test>::ExpenditureNotFound
    );
  });
}

#[test]
fn expenditures_delete_a_selected_expenditure_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: Hard Cost")),
      Some(ExpenditureType::HardCost),
      Some(100),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(expenditure_data),
      None,
    ));

    let get_expenditure_ids = ExpendituresByProject::<Test>::get(project_id);

    let mut target_expenditure_id: [u8; 32] = [0; 32];

    for expenditure_id in get_expenditure_ids {
      let expenditure_data = ExpendituresInfo::<Test>::get(expenditure_id)
        .ok_or(Error::<Test>::ExpenditureNotFound)
        .unwrap();

      if expenditure_data.name == make_field_name("Expenditure Test: Hard Cost") {
        target_expenditure_id = expenditure_id;
        break;
      }
    }

    let del_expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: Hard Cost Modified")),
      Some(ExpenditureType::HardCost),
      Some(1000000),
      Some(make_field_description("16344, 57143")),
      Some(200),
      CUDAction::Delete,
      Some(target_expenditure_id),
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(del_expenditure_data),
      None,
    ));
  });
}

#[test]
fn expenditures_expenditure_id_es_required_to_delete_an_expenditure() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: Hard Cost")),
      Some(ExpenditureType::HardCost),
      Some(100),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(expenditure_data),
      None,
    ));

    let del_expenditure_data = make_expenditure(
      Some(make_field_name("Expenditure Test: Hard Cost Modified")),
      Some(ExpenditureType::HardCost),
      Some(1000000),
      Some(make_field_description("16344, 57143")),
      Some(200),
      CUDAction::Delete,
      None,
    );

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        Some(del_expenditure_data),
        None,
      ),
      Error::<Test>::ExpenditureIdRequired
    );
  });
}

#[test]
fn expenditures_an_admin_can_delete_an_expenditure_containing_transactions_with_zero_amount_works()
{
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data = make_transaction(Some(expenditure_id), Some(0), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      false,
    ));

    let del_expenditure_data =
      make_expenditure(None, None, None, None, None, CUDAction::Delete, Some(expenditure_id));

    assert_eq!(ExpendituresByProject::<Test>::get(project_id).len(), 4);

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(del_expenditure_data),
      None,
    ));

    assert_eq!(ExpendituresByProject::<Test>::contains_key(project_id), true);
    assert_eq!(ExpendituresByProject::<Test>::get(project_id).len(), 3);
    assert_eq!(ExpendituresInfo::<Test>::get(expenditure_id).is_some(), false);
  });
}

#[test]
fn expenditures_an_administrator_deletes_an_expenditure_given_a_drawdown_with_multiple_expenditures_work(
) {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data = make_transaction(Some(expenditure_id), Some(0), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      false,
    ));

    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 2"),
      ExpenditureType::SoftCost,
    );
    assert_eq!(ExpendituresInfo::<Test>::get(expenditure_id).is_some(), true);

    let transaction_data = make_transaction(Some(expenditure_id), Some(0), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      false,
    ));

    assert_eq!(ExpendituresByProject::<Test>::get(project_id).len(), 4);
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 2);

    let del_expenditure_data =
      make_expenditure(None, None, None, None, None, CUDAction::Delete, Some(expenditure_id));

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(del_expenditure_data),
      None,
    ));

    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 1);
    assert_eq!(ExpendituresByProject::<Test>::contains_key(project_id), true);
    assert_eq!(ExpendituresByProject::<Test>::get(project_id).len(), 3);
    assert_eq!(ExpendituresInfo::<Test>::get(expenditure_id).is_some(), false);
  });
}

#[test]
fn expenditures_an_admin_deletes_all_expenditures_for_a_given_project_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let del_expenditure_data =
      make_expenditure(None, None, None, None, None, CUDAction::Delete, Some(expenditure_id));

    assert_eq!(ExpendituresByProject::<Test>::iter_keys().count(), 1);
    assert_eq!(ExpendituresByProject::<Test>::get(project_id).len(), 4);

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(del_expenditure_data),
      None,
    ));

    assert_eq!(ExpendituresByProject::<Test>::iter_keys().count(), 1);
    assert_eq!(ExpendituresByProject::<Test>::get(project_id).len(), 3);
    assert_eq!(ExpendituresInfo::<Test>::get(expenditure_id).is_some(), false);

    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 2"),
      ExpenditureType::SoftCost,
    );

    let del_expenditure_data =
      make_expenditure(None, None, None, None, None, CUDAction::Delete, Some(expenditure_id));

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(del_expenditure_data),
      None,
    ));
    assert_eq!(ExpendituresByProject::<Test>::iter_keys().count(), 1);
    assert_eq!(ExpendituresByProject::<Test>::get(project_id).len(), 2);

    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 3"),
      ExpenditureType::Operational,
    );

    let del_expenditure_data =
      make_expenditure(None, None, None, None, None, CUDAction::Delete, Some(expenditure_id));

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(del_expenditure_data),
      None,
    ));
    assert_eq!(ExpendituresByProject::<Test>::iter_keys().count(), 1);
    assert_eq!(ExpendituresByProject::<Test>::get(project_id).len(), 1);

    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 4"),
      ExpenditureType::Others,
    );

    let del_expenditure_data =
      make_expenditure(None, None, None, None, None, CUDAction::Delete, Some(expenditure_id));

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      Some(del_expenditure_data),
      None,
    ));

    assert_eq!(ExpendituresByProject::<Test>::iter_keys().count(), 0);
    assert_eq!(ExpendituresByProject::<Test>::get(project_id).len(), 0);
  });
}

#[test]
fn expenditures_an_admin_cannot_delete_a_expenditure_that_is_being_used_draft_status_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      false,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Draft);

    let del_expenditure_data =
      make_expenditure(None, None, None, None, None, CUDAction::Delete, Some(expenditure_id));

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        Some(del_expenditure_data),
        None,
      ),
      Error::<Test>::ExpenditureHasNonZeroTransactions
    );
  });
}

#[test]
fn expenditures_an_admin_cannot_delete_a_expenditure_that_is_in_use_submitted_status_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Submitted);

    let del_expenditure_data =
      make_expenditure(None, None, None, None, None, CUDAction::Delete, Some(expenditure_id));

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        Some(del_expenditure_data),
        None,
      ),
      Error::<Test>::ExpenditureHasNonZeroTransactions
    );
  });
}

#[test]
fn expenditures_an_admin_cannot_delete_a_expenditure_that_is_in_use_approved_status_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Approved);

    let del_expenditure_data =
      make_expenditure(None, None, None, None, None, CUDAction::Delete, Some(expenditure_id));

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        Some(del_expenditure_data),
        None,
      ),
      Error::<Test>::ExpenditureHasNonZeroTransactions
    );
  });
}

#[test]
fn expenditures_an_admin_cannot_delete_a_expenditure_that_is_in_use_confirmed_status_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    let del_expenditure_data =
      make_expenditure(None, None, None, None, None, CUDAction::Delete, Some(expenditure_id));

    let bank_documents = make_documents(1);

    assert_ok!(FundAdmin::bank_confirming_documents(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      Some(bank_documents.clone()),
      CUDAction::Create,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Confirmed);

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        Some(del_expenditure_data),
        None,
      ),
      Error::<Test>::ExpenditureHasNonZeroTransactions
    );
  });
}

// J O B   E L I G I B L E S
// =================================================================================================
#[test]
fn job_eligibles_create_a_job_eligible_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let job_eligible_data = make_job_eligible(
      Some(make_field_name("Job Eligible Test: Construction")),
      Some(1000),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      None,
      Some(job_eligible_data),
    ));

    let get_job_eligible_id: [u8; 32] =
      JobEligiblesByProject::<Test>::get(project_id).pop().unwrap();

    assert!(JobEligiblesInfo::<Test>::contains_key(get_job_eligible_id));
    assert_eq!(
      JobEligiblesInfo::<Test>::get(get_job_eligible_id).unwrap().name,
      make_field_name("Job Eligible Test: Construction")
    );
    assert_eq!(
      JobEligiblesInfo::<Test>::get(get_job_eligible_id).unwrap().job_eligible_amount,
      1000
    );
    assert_eq!(
      JobEligiblesInfo::<Test>::get(get_job_eligible_id).unwrap().naics_code,
      Some(make_field_description("16344, 45862, 57143"))
    );
    assert_eq!(
      JobEligiblesInfo::<Test>::get(get_job_eligible_id).unwrap().jobs_multiplier,
      Some(200)
    );
  });
}

#[test]
fn job_eligibles_cannot_send_an_empty_array_of_job_eligibles_for_a_given_project() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let job_eligible_data: JobEligibles<Test> = bounded_vec![];

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        None,
        Some(job_eligible_data),
      ),
      Error::<Test>::JobEligiblesEmpty
    );
  });
}

#[test]
fn job_eligibles_cannot_create_a_job_eligible_without_a_name_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let job_eligible_data = make_job_eligible(
      None,
      Some(1000),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        None,
        Some(job_eligible_data),
      ),
      Error::<Test>::JobEligibleNameRequired
    );
  });
}

#[test]
fn job_eligibles_cannot_create_a_job_eligible_without_an_amount_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let job_eligible_data = make_job_eligible(
      Some(make_field_name("Job Eligible Test: Hard Cost")),
      None,
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        None,
        Some(job_eligible_data),
      ),
      Error::<Test>::JobEligibleAmountRequired
    );
  });
}

#[test]
fn job_eligibles_cannot_create_a_job_eligible_with_an_empty_name_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let job_eligible_data = make_job_eligible(
      Some(make_field_name("")),
      Some(1000),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        None,
        Some(job_eligible_data),
      ),
      Error::<Test>::JobEligiblesNameRequired
    );
  });
}

#[test]
fn job_eligibles_edit_a_job_eligible_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let job_eligible_data = make_job_eligible(
      Some(make_field_name("Job Eligible Test: Construction")),
      Some(1000),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      None,
      Some(job_eligible_data),
    ));

    let get_job_eligible_id: [u8; 32] =
      JobEligiblesByProject::<Test>::get(project_id).pop().unwrap();

    let mod_job_eligible_data = make_job_eligible(
      Some(make_field_name("Job Eligible Test: Construction Modified")),
      Some(5000),
      Some(make_field_description("16344, 57143")),
      Some(320),
      CUDAction::Update,
      Some(get_job_eligible_id),
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      None,
      Some(mod_job_eligible_data),
    ));

    assert!(JobEligiblesInfo::<Test>::contains_key(get_job_eligible_id));
    assert_eq!(
      JobEligiblesInfo::<Test>::get(get_job_eligible_id).unwrap().name,
      make_field_name("Job Eligible Test: Construction Modified")
    );
    assert_eq!(
      JobEligiblesInfo::<Test>::get(get_job_eligible_id).unwrap().job_eligible_amount,
      5000
    );
    assert_eq!(
      JobEligiblesInfo::<Test>::get(get_job_eligible_id).unwrap().naics_code,
      Some(make_field_description("16344, 57143"))
    );
    assert_eq!(
      JobEligiblesInfo::<Test>::get(get_job_eligible_id).unwrap().jobs_multiplier,
      Some(320)
    );
  });
}

#[test]
fn job_eligibles_edit_a_given_job_eligible_from_another_project_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    assert_ok!(FundAdmin::projects_create_project(
      RuntimeOrigin::signed(1),
      make_field_name("Project 2"),
      make_field_description("Project 2 description"),
      Some(make_field_name("project_image.jpeg")),
      make_field_name("Brooklyn"),
      None,
      1000,
      2000,
      make_default_expenditures(),
      None,
      None,
      make_field_description("P9f5wbr13BK74p1"),
    ));

    let mut project_ids: Vec<ProjectId> = ProjectsInfo::<Test>::iter_keys().collect();
    let first_project_id = project_ids.pop().unwrap();
    let second_project_id = project_ids.pop().unwrap();

    let first_job_eligible_data = make_job_eligible(
      Some(make_field_name("Job Eligible Test: Construction")),
      Some(1000),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    let second_job_eligible_data = make_job_eligible(
      Some(make_field_name("Job Eligible Test: Development")),
      Some(22000),
      Some(make_field_description("45612, 97856, 43284")),
      Some(540),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      first_project_id,
      None,
      Some(first_job_eligible_data),
    ));

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      second_project_id,
      None,
      Some(second_job_eligible_data),
    ));

    let second_job_eligible_id =
      JobEligiblesByProject::<Test>::get(second_project_id).pop().unwrap();

    let mod_first_job_eligible_data = make_job_eligible(
      Some(make_field_name("Job Eligible Test: Construction Modified")),
      Some(5000),
      Some(make_field_description("16344, 57143")),
      Some(320),
      CUDAction::Update,
      Some(second_job_eligible_id),
    );

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        first_project_id,
        None,
        Some(mod_first_job_eligible_data),
      ),
      Error::<Test>::JobEligibleDoesNotBelongToProject
    );
  });
}

#[test]
fn job_eligibles_edit_a_given_job_eligible_with_an_invalid_id_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let job_eligible_data = make_job_eligible(
      Some(make_field_name("Job Eligible Test: Construction")),
      Some(1000),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      None,
      Some(job_eligible_data),
    ));

    let mod_job_eligible_data = make_job_eligible(
      Some(make_field_name("Job Eligible Test: Construction Modified")),
      Some(5000),
      Some(make_field_description("16344, 57143")),
      Some(320),
      CUDAction::Update,
      Some([0; 32]),
    );

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        None,
        Some(mod_job_eligible_data),
      ),
      Error::<Test>::JobEligibleNotFound
    );
  });
}

#[test]
fn job_eligibles_job_eligible_id_is_required_to_update_a_given_job_eligible_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let job_eligible_data = make_job_eligible(
      Some(make_field_name("Job Eligible Test: Construction")),
      Some(1000),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      None,
      Some(job_eligible_data),
    ));

    let mod_job_eligible_data = make_job_eligible(
      Some(make_field_name("Job Eligible Test: Construction Modified")),
      Some(5000),
      Some(make_field_description("16344, 57143")),
      Some(320),
      CUDAction::Update,
      None,
    );

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        None,
        Some(mod_job_eligible_data),
      ),
      Error::<Test>::JobEligibleIdRequired
    );
  });
}

#[test]
fn job_eligibles_delete_a_given_job_eligible_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let job_eligible_data = make_job_eligible(
      Some(make_field_name("Job Eligible Test: Construction")),
      Some(1000),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      None,
      Some(job_eligible_data),
    ));

    let job_eligible_id = JobEligiblesByProject::<Test>::get(project_id).pop().unwrap();

    let del_job_eligible_data =
      make_job_eligible(None, None, None, None, CUDAction::Delete, Some(job_eligible_id));

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      None,
      Some(del_job_eligible_data),
    ));

    assert_eq!(JobEligiblesByProject::<Test>::get(project_id).len(), 0);
    assert_eq!(JobEligiblesInfo::<Test>::iter().count(), 0);
  });
}

#[test]
fn job_eligibles_delete_a_given_job_eligible_with_an_invalid_id_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let job_eligible_data = make_job_eligible(
      Some(make_field_name("Job Eligible Test: Construction")),
      Some(1000),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      None,
      Some(job_eligible_data),
    ));

    let del_job_eligible_data =
      make_job_eligible(None, None, None, None, CUDAction::Delete, Some([0; 32]));

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        None,
        Some(del_job_eligible_data),
      ),
      Error::<Test>::JobEligibleNotFound
    );
  });
}

#[test]
fn job_eligibles_deleting_a_job_eligible_requires_a_job_eligible_id_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let job_eligible_data = make_job_eligible(
      Some(make_field_name("Job Eligible Test: Construction")),
      Some(1000),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      None,
      Some(job_eligible_data),
    ));

    let del_job_eligible_data = make_job_eligible(None, None, None, None, CUDAction::Delete, None);

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        None,
        Some(del_job_eligible_data),
      ),
      Error::<Test>::JobEligibleIdRequired
    );
  });
}

#[test]
// fn job_eligibles_admin_cannot_delete_a_job_eligible_if_has_non_zero_transactions_should_fail()
fn job_eligibles_admin_can_delete_a_job_eligible_if_has_non_zero_transactions_draft_status_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(0), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      false,
    ));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Draft);
    assert_eq!(TransactionsByRevenue::<Test>::get(project_id, revenue_id).len(), 1);

    let job_eligible_data =
      make_job_eligible(None, None, None, None, CUDAction::Delete, Some(job_eligible_id));

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      None,
      Some(job_eligible_data),
    ));
  });
}

#[test]
fn job_eligibles_admin_cannnot_delete_a_job_eligible_if_has_non_zero_transactions_draft_status_should_fail(
) {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(1000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      false,
    ));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Draft);
    assert_eq!(TransactionsByRevenue::<Test>::get(project_id, revenue_id).len(), 1);

    let job_eligible_data =
      make_job_eligible(None, None, None, None, CUDAction::Delete, Some(job_eligible_id));

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        None,
        Some(job_eligible_data),
      ),
      Error::<Test>::JobEligibleHasNonZeroTransactions
    );
  });
}

#[test]
fn job_eligibles_an_administrator_deletes_a_job_eligible_given_a_revenue_with_multiple_job_eligibles_works(
) {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(0), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      false,
    ));

    let job_eligible_data = make_job_eligible(
      Some(make_field_name("Job Eligible Test: Construction")),
      Some(1000),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      None,
      Some(job_eligible_data),
    ));

    let job_eligible_id_2 =
      get_job_eligible_id(project_id, make_field_name("Job Eligible Test: Construction"));
    assert_eq!(JobEligiblesInfo::<Test>::get(job_eligible_id_2).is_some(), true);

    let revenue_transaction_data_2 =
      make_revenue_transaction(Some(job_eligible_id_2), Some(1000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data_2),
      false,
    ));

    assert_eq!(JobEligiblesByProject::<Test>::get(project_id).len(), 2);
    assert_eq!(TransactionsByRevenue::<Test>::get(project_id, revenue_id).len(), 2);

    let del_job_eligible_data =
      make_job_eligible(None, None, None, None, CUDAction::Delete, Some(job_eligible_id));

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      None,
      Some(del_job_eligible_data),
    ));

    assert_eq!(TransactionsByRevenue::<Test>::get(project_id, revenue_id).len(), 1);
    assert_eq!(JobEligiblesByProject::<Test>::get(project_id).len(), 1);
    assert_eq!(JobEligiblesInfo::<Test>::get(job_eligible_id).is_some(), false);
  });
}

#[test]
fn job_eligibles_an_admin_cannot_delete_a_job_eligible_that_is_being_used_draft_status_should_fail()
{
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      false,
    ));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Draft);
    assert_eq!(TransactionsByRevenue::<Test>::get(project_id, revenue_id).len(), 1);

    let del_job_eligible_data =
      make_job_eligible(None, None, None, None, CUDAction::Delete, Some(job_eligible_id));

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        None,
        Some(del_job_eligible_data),
      ),
      Error::<Test>::JobEligibleHasNonZeroTransactions
    );
  });
}

#[test]
fn job_eligibles_an_admin_cannot_delete_a_job_eligible_that_is_being_used_submitted_status_should_fail(
) {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      true,
    ));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Submitted);
    assert_eq!(TransactionsByRevenue::<Test>::get(project_id, revenue_id).len(), 1);

    let del_job_eligible_data =
      make_job_eligible(None, None, None, None, CUDAction::Delete, Some(job_eligible_id));

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        None,
        Some(del_job_eligible_data),
      ),
      Error::<Test>::JobEligibleHasNonZeroTransactions
    );
  });
}

#[test]

fn job_eligibles_an_admin_cannot_delete_a_job_eligible_that_is_being_used_approved_status_should_fail(
) {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_revenue(RuntimeOrigin::signed(1), project_id, revenue_id,));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Approved);
    assert_eq!(TransactionsByRevenue::<Test>::get(project_id, revenue_id).len(), 1);

    let del_job_eligible_data =
      make_job_eligible(None, None, None, None, CUDAction::Delete, Some(job_eligible_id));

    assert_noop!(
      FundAdmin::expenditures_and_job_eligibles(
        RuntimeOrigin::signed(1),
        project_id,
        None,
        Some(del_job_eligible_data),
      ),
      Error::<Test>::JobEligibleHasNonZeroTransactions
    );
  });
}

// D R A W D O W N S
// =================================================================================================
#[test]
fn drawdowns_drawdowns_are_initialized_correctly_after_a_project_is_created_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_simple_project());

    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    assert_eq!(DrawdownsByProject::<Test>::get(project_id).len(), 3);
    let drawdowns_ids = DrawdownsByProject::<Test>::get(project_id);

    for drawdown_id in drawdowns_ids {
      assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().project_id, project_id);
      assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().drawdown_number, 1);
      assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().total_amount, 0);
      assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Draft);
      assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().bulkupload_documents, None);
      assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().bank_documents, None);
      assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().description, None);
      assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().feedback, None);
    }
  });
}

#[test]
fn drawdowns_a_builder_saves_a_drawdown_as_a_draft_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      false,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Draft);

    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 1);
    let transaction_id = get_transaction_id(project_id, drawdown_id, expenditure_id);
    assert_eq!(TransactionsInfo::<Test>::get(transaction_id).unwrap().project_id, project_id);
    assert_eq!(TransactionsInfo::<Test>::get(transaction_id).unwrap().drawdown_id, drawdown_id);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().expenditure_id,
      expenditure_id
    );
    assert_eq!(TransactionsInfo::<Test>::get(transaction_id).unwrap().closed_date, 0);
    assert_eq!(TransactionsInfo::<Test>::get(transaction_id).unwrap().feedback, None);
    assert_eq!(TransactionsInfo::<Test>::get(transaction_id).unwrap().amount, 10000);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Draft
    );
  });
}

#[test]
fn drawdowns_a_user_modifies_a_transaction_in_draft_status_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      false,
    ));

    let transaction_id = get_transaction_id(project_id, drawdown_id, expenditure_id);
    let mod_transaction_data =
      make_transaction(Some(expenditure_id), Some(20000), CUDAction::Update, Some(transaction_id));

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(mod_transaction_data),
      false,
    ));

    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Draft
    );
    assert_eq!(TransactionsInfo::<Test>::get(transaction_id).unwrap().amount, 20000);
  });
}

#[test]
fn drawdowns_a_builder_cannot_submit_a_drawdown_twice_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      false,
    ));

    let transaction_id = get_transaction_id(project_id, drawdown_id, expenditure_id);
    let mod_transaction_data =
      make_transaction(Some(expenditure_id), Some(20000), CUDAction::Update, Some(transaction_id));

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(mod_transaction_data.clone()),
      true,
    ));

    assert_noop!(
      FundAdmin::submit_drawdown(
        RuntimeOrigin::signed(2),
        project_id,
        drawdown_id,
        Some(mod_transaction_data),
        true,
      ),
      Error::<Test>::CannotPerformActionOnSubmittedDrawdown
    );
  });
}

#[test]
fn drawdowns_a_user_deletes_a_transaction_in_draft_status_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      false,
    ));

    let transaction_id = get_transaction_id(project_id, drawdown_id, expenditure_id);
    let del_transaction_data =
      make_transaction(None, None, CUDAction::Delete, Some(transaction_id));

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(del_transaction_data),
      false,
    ));

    assert_eq!(TransactionsInfo::<Test>::contains_key(transaction_id), false);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().total_amount, 0);
  });
}

#[test]
fn drawdowns_a_user_cannot_save_transactions_as_draft_if_transactions_are_not_provided_should_fail()
{
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);

    assert_noop!(
      FundAdmin::submit_drawdown(RuntimeOrigin::signed(2), project_id, drawdown_id, None, false,),
      Error::<Test>::TransactionsRequired
    );
  });
}

#[test]
fn drawdowns_a_user_cannot_send_an_empty_array_of_transactions_when_saving_as_a_draft_should_fail()
{
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);

    let empty_transaction_data: Transactions<Test> = bounded_vec![];

    assert_noop!(
      FundAdmin::submit_drawdown(
        RuntimeOrigin::signed(2),
        project_id,
        drawdown_id,
        Some(empty_transaction_data),
        false,
      ),
      Error::<Test>::EmptyTransactions
    );
  });
}

#[test]
fn drawdowns_a_user_cannot_send_a_transaction_without_the_expenditure_id_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);

    let transaction_data = make_transaction(None, Some(10000), CUDAction::Create, None);

    assert_noop!(
      FundAdmin::submit_drawdown(
        RuntimeOrigin::signed(2),
        project_id,
        drawdown_id,
        Some(transaction_data),
        false,
      ),
      Error::<Test>::ExpenditureIdRequired
    );
  });
}

#[test]
fn drawdowns_a_user_cannot_create_a_transaction_without_an_amount_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data = make_transaction(Some(expenditure_id), None, CUDAction::Create, None);

    assert_noop!(
      FundAdmin::submit_drawdown(
        RuntimeOrigin::signed(2),
        project_id,
        drawdown_id,
        Some(transaction_data),
        false,
      ),
      Error::<Test>::AmountRequired
    );
  });
}

#[test]
fn drawdowns_transaction_id_is_required_when_editing_a_transaction_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      false,
    ));

    let mod_transaction_data =
      make_transaction(Some(expenditure_id), Some(20000), CUDAction::Update, None);

    assert_noop!(
      FundAdmin::submit_drawdown(
        RuntimeOrigin::signed(2),
        project_id,
        drawdown_id,
        Some(mod_transaction_data),
        false,
      ),
      Error::<Test>::TransactionIdRequired
    );
  });
}

#[test]
fn drawdowns_transaction_id_is_required_when_deleting_a_transaction_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      false,
    ));

    let del_transaction_data = make_transaction(None, None, CUDAction::Delete, None);

    assert_noop!(
      FundAdmin::submit_drawdown(
        RuntimeOrigin::signed(2),
        project_id,
        drawdown_id,
        Some(del_transaction_data),
        false,
      ),
      Error::<Test>::TransactionIdRequired
    );
  });
}

#[test]
fn drawdowns_a_user_submits_a_drawdown_for_approval_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    let drawdown_data = DrawdownsInfo::<Test>::get(drawdown_id).unwrap();

    assert_eq!(drawdown_data.status, DrawdownStatus::Submitted);
    assert_eq!(drawdown_data.total_amount, 10000);
  });
}

#[test]
fn drawdowns_a_user_submits_a_draft_drawdown_for_approval_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      false,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Draft);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      None,
      true,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Submitted);
  });
}

#[test]
fn drawdowns_a_user_tries_to_add_transactions_using_an_empty_array_before_submitting_the_drawdown_should_fail(
) {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);

    let transaction_data: Transactions<Test> = bounded_vec![];

    assert_noop!(
      FundAdmin::submit_drawdown(
        RuntimeOrigin::signed(2),
        project_id,
        drawdown_id,
        Some(transaction_data),
        true,
      ),
      Error::<Test>::EmptyTransactions
    );
  });
}

#[test]
fn drawdowns_a_drawdown_cannot_be_submitted_if_has_no_transactions_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);

    assert_noop!(
      FundAdmin::submit_drawdown(RuntimeOrigin::signed(2), project_id, drawdown_id, None, true,),
      Error::<Test>::DrawdownHasNoTransactions
    );
  });
}

#[test]
fn drawdowns_a_builder_deletes_all_transactions_while_submitting_a_drawdown_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      false,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Draft);

    let transaction_id = get_transaction_id(project_id, drawdown_id, expenditure_id);

    let del_transaction_data =
      make_transaction(Some(expenditure_id), None, CUDAction::Delete, Some(transaction_id));

    assert_noop!(
      FundAdmin::submit_drawdown(
        RuntimeOrigin::signed(2),
        project_id,
        drawdown_id,
        Some(del_transaction_data),
        true,
      ),
      Error::<Test>::DrawdownHasNoTransactions
    );
  });
}

#[test]
fn drawdowns_after_a_drawdown_is_submitted_the_status_is_updated_in_project_data_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().eb5_drawdown_status,
      Some(DrawdownStatus::Submitted)
    );
  });
}

#[test]
fn drawdowns_an_administrators_approves_a_submitted_drawdown_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Approved);
  });
}

#[test]
fn drawdowns_an_administrator_cannot_aproves_a_drawdown_that_is_not_submitted_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      false,
    ));

    assert_noop!(
      FundAdmin::approve_drawdown(RuntimeOrigin::signed(1), project_id, drawdown_id, None, None,),
      Error::<Test>::DrawdownNotSubmitted
    );
  });
}

#[test]
fn drawdowns_after_a_drawdown_is_approved_the_next_one_is_generated_autoamtically_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    let next_drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 2);

    assert_eq!(DrawdownsInfo::<Test>::get(next_drawdown_id).unwrap().status, DrawdownStatus::Draft);
  });
}

#[test]
fn drawdowns_an_administrator_rejects_a_given_drawdown_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    let transaction_id = get_transaction_id(project_id, drawdown_id, expenditure_id);

    let feedback = make_field_description("Transaction was rejected bacause it was not valid");

    let transaction_feedback = make_transaction_feedback(transaction_id, feedback.clone());

    assert_ok!(FundAdmin::reject_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      Some(transaction_feedback),
      None,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Rejected);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Rejected
    );
    assert_eq!(TransactionsInfo::<Test>::get(transaction_id).unwrap().feedback, Some(feedback));
  });
}

#[test]
fn drawdowns_an_administrator_cannot_rejects_a_drawdown_that_is_not_submitted_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      false,
    ));

    let transaction_feedback = make_transaction_feedback(
      get_transaction_id(project_id, drawdown_id, expenditure_id),
      make_field_description("Transaction was rejected bacause it was not valid"),
    );

    assert_noop!(
      FundAdmin::reject_drawdown(
        RuntimeOrigin::signed(1),
        project_id,
        drawdown_id,
        Some(transaction_feedback),
        None,
      ),
      Error::<Test>::DrawdownNotSubmitted
    );
  });
}

#[test]
fn drawdowns_an_administrator_cannot_rejects_a_drawdown_without_a_feedback_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_noop!(
      FundAdmin::reject_drawdown(RuntimeOrigin::signed(1), project_id, drawdown_id, None, None,),
      Error::<Test>::EB5MissingFeedback
    );
  });
}

#[test]
fn drawdowns_an_administrator_rejects_a_eb5_drawdown_with_an_empty_feedback_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    let transaction_feedback: TransactionsFeedback<Test> = bounded_vec![];

    assert_noop!(
      FundAdmin::reject_drawdown(
        RuntimeOrigin::signed(1),
        project_id,
        drawdown_id,
        Some(transaction_feedback),
        None,
      ),
      Error::<Test>::EmptyEb5Feedback
    );
  });
}

// B U L K   D R A W D O W N S
// =================================================================================================
#[test]
fn bulkupload_a_builder_submits_a_construction_loan_drawdown_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::ConstructionLoan, 1);

    let drawdown_description = make_field_description("Construction Loan Drawdown 1");
    let total_amount = 100000u64;
    let documents = make_documents(1);

    assert_ok!(FundAdmin::up_bulkupload(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      drawdown_description.clone(),
      total_amount,
      documents.clone(),
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Submitted);
    assert_eq!(
      DrawdownsInfo::<Test>::get(drawdown_id).unwrap().description,
      Some(drawdown_description)
    );
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().total_amount, total_amount);
    assert_eq!(
      DrawdownsInfo::<Test>::get(drawdown_id).unwrap().bulkupload_documents,
      Some(documents)
    );
  });
}

#[test]
fn bulkupload_a_builder_submits_a_developer_equity_drawdown_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::DeveloperEquity, 1);

    let drawdown_description = make_field_description("Developer Equity Drawdown 1");
    let total_amount = 100000u64;
    let documents = make_documents(1);

    assert_ok!(FundAdmin::up_bulkupload(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      drawdown_description.clone(),
      total_amount,
      documents.clone(),
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Submitted);
    assert_eq!(
      DrawdownsInfo::<Test>::get(drawdown_id).unwrap().description,
      Some(drawdown_description)
    );
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().total_amount, total_amount);
    assert_eq!(
      DrawdownsInfo::<Test>::get(drawdown_id).unwrap().bulkupload_documents,
      Some(documents)
    );
  });
}

#[test]
fn bulkupload_a_builder_submits_a_eb5_drawdown_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);

    let drawdown_description = make_field_description("EB5 Drawdown 1");
    let total_amount = 100000u64;
    let documents = make_documents(1);

    assert_noop!(
      FundAdmin::up_bulkupload(
        RuntimeOrigin::signed(2),
        project_id,
        drawdown_id,
        drawdown_description.clone(),
        total_amount,
        documents.clone(),
      ),
      Error::<Test>::DrawdownTypeNotSupportedForBulkUpload
    );
  });
}

#[test]
fn bulkupload_a_builder_submits_an_empty_array_of_documents_for_a_construction_loan_drawdown_should_fail(
) {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::ConstructionLoan, 1);

    let drawdown_description = make_field_description("Construction Loan Drawdown 1");
    let total_amount = 100000u64;
    let documents: Documents<Test> = bounded_vec![];

    assert_noop!(
      FundAdmin::up_bulkupload(
        RuntimeOrigin::signed(2),
        project_id,
        drawdown_id,
        drawdown_description.clone(),
        total_amount,
        documents.clone(),
      ),
      Error::<Test>::BulkUploadDocumentsRequired
    );
  });
}

#[test]
fn bulkupload_a_builder_submits_an_empty_adescription_for_a_construction_loan_drawdown_should_fail()
{
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::ConstructionLoan, 1);

    let drawdown_description: FieldDescription = bounded_vec![];
    let total_amount = 100000u64;
    let documents = make_documents(1);

    assert_noop!(
      FundAdmin::up_bulkupload(
        RuntimeOrigin::signed(2),
        project_id,
        drawdown_id,
        drawdown_description.clone(),
        total_amount,
        documents.clone(),
      ),
      Error::<Test>::BulkUploadDescriptionRequired
    );
  });
}

#[test]
fn bulkupload_after_a_contruction_loan_is_submitted_their_status_is_updated_in_project_data_works()
{
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::ConstructionLoan, 1);

    let drawdown_description = make_field_description("Construction Loan Drawdown 1");
    let total_amount = 100000u64;
    let documents = make_documents(1);

    assert_ok!(FundAdmin::up_bulkupload(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      drawdown_description.clone(),
      total_amount,
      documents.clone(),
    ));

    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().construction_loan_drawdown_status,
      Some(DrawdownStatus::Submitted)
    );
  });
}

#[test]
fn bulkupload_after_a_developer_equity_is_submitted_their_status_is_updated_in_project_data_works()
{
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::DeveloperEquity, 1);

    let drawdown_description = make_field_description("Developer Equity Drawdown 1");
    let total_amount = 100000u64;
    let documents = make_documents(1);

    assert_ok!(FundAdmin::up_bulkupload(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      drawdown_description.clone(),
      total_amount,
      documents.clone(),
    ));

    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().developer_equity_drawdown_status,
      Some(DrawdownStatus::Submitted)
    );
  });
}

#[test]
fn bulkupload_an_administrator_saves_transactions_without_approving_the_drawdown_pseudo_draft_works(
) {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::ConstructionLoan, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    assert_ok!(FundAdmin::up_bulkupload(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      make_field_description("Construction Loan Drawdown 1"),
      100000u64,
      make_documents(1),
    ));

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      Some(false),
      Some(transaction_data),
    ));

    let transaction_id = get_transaction_id(project_id, drawdown_id, expenditure_id);

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Submitted);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Submitted
    );
  });
}

#[test]
fn bulkupload_an_administrator_saves_transactions_and_approves_the_drawdown_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::ConstructionLoan, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    assert_ok!(FundAdmin::up_bulkupload(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      make_field_description("Construction Loan Drawdown 1"),
      100000u64,
      make_documents(1),
    ));

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      Some(true),
      Some(transaction_data),
    ));

    let transaction_id = get_transaction_id(project_id, drawdown_id, expenditure_id);

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Approved);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Approved
    );
  });
}

#[test]
fn bulkupload_an_array_of_transactions_is_required_to_save_transactions_as_a_pseudo_draft_should_fail(
) {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::ConstructionLoan, 1);

    assert_ok!(FundAdmin::up_bulkupload(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      make_field_description("Construction Loan Drawdown 1"),
      100000u64,
      make_documents(1),
    ));

    assert_noop!(
      FundAdmin::approve_drawdown(
        RuntimeOrigin::signed(1),
        project_id,
        drawdown_id,
        Some(false),
        None,
      ),
      Error::<Test>::TransactionsRequired
    );
  });
}

#[test]
fn bulkupload_an_administrator_sends_an_empty_array_of_transactions_as_a_pseudo_draft_should_fail()
{
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::ConstructionLoan, 1);

    assert_ok!(FundAdmin::up_bulkupload(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      make_field_description("Construction Loan Drawdown 1"),
      100000u64,
      make_documents(1),
    ));

    let transaction_data: Transactions<Test> = bounded_vec![];

    assert_noop!(
      FundAdmin::approve_drawdown(
        RuntimeOrigin::signed(1),
        project_id,
        drawdown_id,
        Some(false),
        Some(transaction_data),
      ),
      Error::<Test>::EmptyTransactions
    );
  });
}

#[test]
fn bulkupload_an_administrator_sends_an_empty_array_while_approving_a_drawdown_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::ConstructionLoan, 1);

    assert_ok!(FundAdmin::up_bulkupload(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      make_field_description("Construction Loan Drawdown 1"),
      100000u64,
      make_documents(1),
    ));

    let transaction_data: Transactions<Test> = bounded_vec![];

    assert_noop!(
      FundAdmin::approve_drawdown(
        RuntimeOrigin::signed(1),
        project_id,
        drawdown_id,
        Some(true),
        Some(transaction_data),
      ),
      Error::<Test>::EmptyTransactions
    );
  });
}

#[test]
fn bulkupload_an_administrator_rejects_a_contruction_loan_drawdown_with_a_feedback_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::ConstructionLoan, 1);

    assert_ok!(FundAdmin::up_bulkupload(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      make_field_description("Construction Loan Drawdown 1"),
      100000u64,
      make_documents(1),
    ));

    let bulkupload_feedback = make_field_description("Bulkupload Feedback");

    assert_ok!(FundAdmin::reject_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      Some(bulkupload_feedback.clone()),
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Rejected);
    assert_eq!(
      DrawdownsInfo::<Test>::get(drawdown_id).unwrap().feedback,
      Some(bulkupload_feedback)
    );
  });
}

#[test]
fn bulkupload_an_administrator_rejects_a_developer_equity_drawdown_with_a_feedback_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::DeveloperEquity, 1);

    assert_ok!(FundAdmin::up_bulkupload(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      make_field_description("Developer Equity Drawdown 1"),
      100000u64,
      make_documents(1),
    ));

    let bulkupload_feedback = make_field_description("Bulkupload Feedback");

    assert_ok!(FundAdmin::reject_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      Some(bulkupload_feedback.clone()),
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Rejected);
    assert_eq!(
      DrawdownsInfo::<Test>::get(drawdown_id).unwrap().feedback,
      Some(bulkupload_feedback)
    );
  });
}

#[test]
fn bulkupload_an_administrator_rejects_a_bulkupload_drawdown_without_a_feedback_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::ConstructionLoan, 1);

    assert_ok!(FundAdmin::up_bulkupload(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      make_field_description("Construction Loan Drawdown 1"),
      100000u64,
      make_documents(1),
    ));

    assert_noop!(
      FundAdmin::reject_drawdown(RuntimeOrigin::signed(1), project_id, drawdown_id, None, None,),
      Error::<Test>::NoFeedbackProvidedForBulkUpload
    );
  });
}

#[test]
fn bulkupload_an_administrator_rejects_a_bulkupload_drawdown_with_an_empty_feedback_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::ConstructionLoan, 1);

    assert_ok!(FundAdmin::up_bulkupload(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      make_field_description("Construction Loan Drawdown 1"),
      100000u64,
      make_documents(1),
    ));

    assert_noop!(
      FundAdmin::reject_drawdown(
        RuntimeOrigin::signed(1),
        project_id,
        drawdown_id,
        None,
        Some(make_field_description("")),
      ),
      Error::<Test>::EmptyBulkUploadFeedback
    );
  });
}

//TODO: A rejected drawdown changes its status from rejected to submitted after a builder submits again the drawdown

// R E V E N U E S
// =================================================================================================
#[test]
fn revenues_are_initialized_correctly_after_a_project_is_created_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    assert_eq!(RevenuesInfo::<Test>::iter().count(), 1);

    let revenue_id = RevenuesInfo::<Test>::iter_keys().next().unwrap();

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().project_id, project_id);
    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().revenue_number, 1);
    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().total_amount, 0);
    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Draft);
  });
}

#[test]
fn revenues_a_builder_saves_a_revenue_as_draft_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      false,
    ));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Draft);

    let revenue_transaction_id =
      get_revenue_transaction_id(project_id, revenue_id, job_eligible_id);
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(revenue_transaction_id).unwrap().project_id,
      project_id
    );
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(revenue_transaction_id).unwrap().revenue_id,
      revenue_id
    );
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(revenue_transaction_id)
        .unwrap()
        .job_eligible_id,
      job_eligible_id
    );
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(revenue_transaction_id)
        .unwrap()
        .closed_date,
      0
    );
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(revenue_transaction_id).unwrap().feedback,
      None
    );
    assert_eq!(RevenueTransactionsInfo::<Test>::get(revenue_transaction_id).unwrap().amount, 10000);
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(revenue_transaction_id).unwrap().status,
      RevenueTransactionStatus::Draft
    );
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(revenue_transaction_id).unwrap().documents,
      Some(make_documents(1))
    );
  });
}

#[test]
fn revenues_a_builder_modifies_a_transaction_in_draft_status_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      false,
    ));

    let revenue_transaction_id =
      get_revenue_transaction_id(project_id, revenue_id, job_eligible_id);
    let mod_revenue_transaction_data = make_revenue_transaction(
      Some(job_eligible_id),
      Some(20000),
      CUDAction::Update,
      Some(revenue_transaction_id),
    );

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(mod_revenue_transaction_data),
      false,
    ));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Draft);
    assert_eq!(RevenueTransactionsInfo::<Test>::get(revenue_transaction_id).unwrap().amount, 20000);
  });
}

#[test]
fn revenues_a_user_deletes_a_transaction_in_draft_status_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      false,
    ));

    let revenue_transaction_id =
      get_revenue_transaction_id(project_id, revenue_id, job_eligible_id);
    let del_revenue_transaction_data =
      make_revenue_transaction(None, None, CUDAction::Delete, Some(revenue_transaction_id));

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(del_revenue_transaction_data),
      false,
    ));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Draft);
    assert_eq!(RevenueTransactionsInfo::<Test>::contains_key(revenue_transaction_id), false);
  });
}

#[test]
fn revenues_a_builder_cannot_submit_a_revenue_if_there_is_no_revenue_transaction_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);

    assert_noop!(
      FundAdmin::submit_revenue(RuntimeOrigin::signed(2), project_id, revenue_id, None, false,),
      Error::<Test>::RevenueTransactionsRequired
    );
  });
}

#[test]
fn revenues_a_user_cannot_submit_a_revenue_as_draft_with_an_empty_array_of_transactions_should_fail(
) {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);

    let empty_transaction_data: RevenueTransactions<Test> = bounded_vec![];

    assert_noop!(
      FundAdmin::submit_revenue(
        RuntimeOrigin::signed(2),
        project_id,
        revenue_id,
        Some(empty_transaction_data),
        false,
      ),
      Error::<Test>::RevenueTransactionsEmpty
    );
  });
}

#[test]
fn revenues_a_user_cannot_create_a_revenue_transaction_with_no_job_eligible_id_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);

    let revenue_transaction_data =
      make_revenue_transaction(None, Some(10000), CUDAction::Create, None);

    assert_noop!(
      FundAdmin::submit_revenue(
        RuntimeOrigin::signed(2),
        project_id,
        revenue_id,
        Some(revenue_transaction_data),
        false,
      ),
      Error::<Test>::JobEligibleIdRequired
    );
  });
}

#[test]
fn revenues_a_user_cannot_create_a_revenue_transaction_with_no_amount_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), None, CUDAction::Create, None);

    assert_noop!(
      FundAdmin::submit_revenue(
        RuntimeOrigin::signed(2),
        project_id,
        revenue_id,
        Some(revenue_transaction_data),
        false,
      ),
      Error::<Test>::RevenueAmountRequired
    );
  });
}

#[test]
fn revenues_transaction_id_is_required_for_updating_a_transaction_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      false,
    ));

    let mod_revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(20000), CUDAction::Update, None);

    assert_noop!(
      FundAdmin::submit_revenue(
        RuntimeOrigin::signed(2),
        project_id,
        revenue_id,
        Some(mod_revenue_transaction_data),
        false,
      ),
      Error::<Test>::RevenueTransactionIdRequired
    );
  });
}

#[test]
fn revenues_transaction_id_is_required_for_deleting_a_transaction_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      false,
    ));

    let del_revenue_transaction_data =
      make_revenue_transaction(None, None, CUDAction::Delete, None);

    assert_noop!(
      FundAdmin::submit_revenue(
        RuntimeOrigin::signed(2),
        project_id,
        revenue_id,
        Some(del_revenue_transaction_data),
        false,
      ),
      Error::<Test>::RevenueTransactionIdRequired
    );
  });
}

#[test]
fn revenues_a_builder_submits_a_revenue_for_approval_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      true,
    ));

    let revenue_data = RevenuesInfo::<Test>::get(revenue_id).unwrap();

    assert_eq!(revenue_data.status, RevenueStatus::Submitted);
    assert_eq!(revenue_data.total_amount, 10000);
  });
}

#[test]
fn revenues_a_builder_submits_a_draft_revenue_for_approval_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      false,
    ));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Draft);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      None,
      true,
    ));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Submitted);
  });
}

#[test]
fn revenues_a_user_tries_to_submit_a_revenue_for_approval_without_being_a_builder_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_noop!(
      FundAdmin::submit_revenue(
        RuntimeOrigin::signed(3),
        project_id,
        revenue_id,
        Some(revenue_transaction_data),
        true,
      ),
      RbacErr::NotAuthorized
    );
  });
}

#[test]
fn revenues_a_revenue_cannot_be_submitted_for_approval_if_it_is_already_submitted_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data.clone()),
      true,
    ));

    assert_noop!(
      FundAdmin::submit_revenue(
        RuntimeOrigin::signed(2),
        project_id,
        revenue_id,
        Some(revenue_transaction_data),
        true,
      ),
      Error::<Test>::CannotPerformActionOnSubmittedRevenue
    );
  });
}

#[test]
fn revenues_a_revenue_cannot_be_submitted_if_has_no_transactions_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);

    assert_noop!(
      FundAdmin::submit_revenue(RuntimeOrigin::signed(2), project_id, revenue_id, None, true,),
      Error::<Test>::RevenueHasNoTransactions
    );
  });
}

#[test]
fn revenues_a_builder_deletes_all_transactions_while_submitting_a_revenue_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      false,
    ));

    let revenue_transaction_id =
      get_revenue_transaction_id(project_id, revenue_id, job_eligible_id);
    let del_revenue_transaction_data = make_revenue_transaction(
      Some(job_eligible_id),
      Some(10000),
      CUDAction::Delete,
      Some(revenue_transaction_id),
    );

    assert_noop!(
      FundAdmin::submit_revenue(
        RuntimeOrigin::signed(2),
        project_id,
        revenue_id,
        Some(del_revenue_transaction_data),
        true,
      ),
      Error::<Test>::RevenueHasNoTransactions
    );
  });
}

#[test]
fn revenues_a_builder_tries_to_submit_a_revenue_with_an_empty_array_of_transactions_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let empty_revenue_transaction_data: RevenueTransactions<Test> = bounded_vec![];

    assert_noop!(
      FundAdmin::submit_revenue(
        RuntimeOrigin::signed(2),
        project_id,
        revenue_id,
        Some(empty_revenue_transaction_data),
        true,
      ),
      Error::<Test>::RevenueTransactionsEmpty
    );
  });
}

#[test]
fn revenues_after_a_revenue_is_submitted_the_status_is_updated_in_project_data_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      true,
    ));

    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().revenue_status,
      Some(RevenueStatus::Submitted)
    );
  });
}

#[test]
fn revenues_an_administrator_approves_a_submitted_revenue_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_revenue(RuntimeOrigin::signed(1), project_id, revenue_id,));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Approved);
  });
}

#[test]
fn revenues_an_administrator_cannot_approve_a_revenue_if_it_is_not_submitted_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);

    assert_noop!(
      FundAdmin::approve_revenue(RuntimeOrigin::signed(1), project_id, revenue_id,),
      Error::<Test>::RevenueNotSubmitted
    );
  });
}

#[test]
fn revenues_after_a_revenue_is_submitted_the_next_one_is_generated_automaticaly_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_revenue(RuntimeOrigin::signed(1), project_id, revenue_id,));

    let next_revenue_id = get_revenue_id(project_id, 2);

    assert_eq!(RevenuesInfo::<Test>::get(next_revenue_id).unwrap().status, RevenueStatus::Draft);
  });
}

#[test]
fn revenues_an_administrator_rejects_a_given_revenue_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      true,
    ));

    let revenue_transaction_id =
      get_revenue_transaction_id(project_id, revenue_id, job_eligible_id);

    let feedback = make_field_description("Transaction was rejected because it was not valid");

    let transaction_feedback = make_transaction_feedback(revenue_transaction_id, feedback.clone());

    assert_ok!(FundAdmin::reject_revenue(
      RuntimeOrigin::signed(1),
      project_id,
      revenue_id,
      transaction_feedback,
    ));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Rejected);
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(revenue_transaction_id).unwrap().status,
      RevenueTransactionStatus::Rejected
    );
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(revenue_transaction_id).unwrap().feedback,
      Some(feedback)
    );
  });
}

#[test]
fn revenues_an_administrator_cannot_reject_a_revenue_if_it_is_not_submitted_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(transaction_data),
      false,
    ));

    let revenue_transaction_id =
      get_revenue_transaction_id(project_id, revenue_id, job_eligible_id);
    let feedback = make_field_description("Transaction was rejected because it was not valid");
    let transaction_feedback = make_transaction_feedback(revenue_transaction_id, feedback.clone());

    assert_noop!(
      FundAdmin::reject_revenue(
        RuntimeOrigin::signed(1),
        project_id,
        revenue_id,
        transaction_feedback,
      ),
      Error::<Test>::RevenueNotSubmitted
    );
  });
}

#[test]
fn revenues_an_administrator_cannot_reject_a_revenue_with_an_empty_array_of_feedback_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(transaction_data),
      true,
    ));

    let transaction_feedback: TransactionsFeedback<Test> = bounded_vec![];

    assert_noop!(
      FundAdmin::reject_revenue(
        RuntimeOrigin::signed(1),
        project_id,
        revenue_id,
        transaction_feedback,
      ),
      Error::<Test>::RevenueTransactionsFeedbackEmpty
    );
  });
}

#[test]
fn revenues_after_a_revenue_is_rejected_the_status_is_updated_in_project_data_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(transaction_data),
      true,
    ));

    let revenue_transaction_id =
      get_revenue_transaction_id(project_id, revenue_id, job_eligible_id);
    let feedback = make_field_description("Transaction was rejected because it was not valid");
    let transaction_feedback = make_transaction_feedback(revenue_transaction_id, feedback.clone());

    assert_ok!(FundAdmin::reject_revenue(
      RuntimeOrigin::signed(1),
      project_id,
      revenue_id,
      transaction_feedback,
    ));

    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().revenue_status,
      Some(RevenueStatus::Rejected)
    );
  });
}

// I N F L A T I O N  R A T E
// =================================================================================================
#[test]
fn inflation_rate_an_administrator_can_set_the_inflation_rate_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let inflation_rate = 70;
    let inflation_rate_data =
      make_project_inflation(project_id, Some(inflation_rate), CUDAction::Create);

    assert_ok!(FundAdmin::inflation_rate(RuntimeOrigin::signed(1), inflation_rate_data,));

    assert_eq!(ProjectsInfo::<Test>::get(project_id).unwrap().inflation_rate, Some(inflation_rate));
  });
}

#[test]
fn inflation_rate_an_administrator_cannot_submit_an_empty_array_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());

    let inflation_rate_data: ProjectsInflation<Test> = bounded_vec![];

    assert_noop!(
      FundAdmin::inflation_rate(RuntimeOrigin::signed(1), inflation_rate_data,),
      Error::<Test>::ProjectsInflationRateEmpty
    );
  });
}

#[test]
fn inflation_rate_an_administrator_cannot_set_the_inflation_rate_without_a_value_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let inflation_rate_data = make_project_inflation(project_id, None, CUDAction::Create);

    assert_noop!(
      FundAdmin::inflation_rate(RuntimeOrigin::signed(1), inflation_rate_data,),
      Error::<Test>::InflationRateRequired
    );
  });
}

#[test]
fn inflation_rate_an_administrator_cannot_set_the_inflation_rate_if_it_is_already_set_should_fail()
{
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let inflation_rate = 70;
    let inflation_rate_data =
      make_project_inflation(project_id, Some(inflation_rate), CUDAction::Create);

    assert_ok!(FundAdmin::inflation_rate(RuntimeOrigin::signed(1), inflation_rate_data.clone(),));

    assert_noop!(
      FundAdmin::inflation_rate(RuntimeOrigin::signed(1), inflation_rate_data,),
      Error::<Test>::InflationRateAlreadySet
    );
  });
}

#[test]
fn inflation_rate_an_administrator_updates_the_inflation_rate_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let inflation_rate = 70;
    let inflation_rate_data =
      make_project_inflation(project_id, Some(inflation_rate), CUDAction::Create);

    assert_ok!(FundAdmin::inflation_rate(RuntimeOrigin::signed(1), inflation_rate_data.clone(),));

    let inflation_rate = 80;
    let inflation_rate_data =
      make_project_inflation(project_id, Some(inflation_rate), CUDAction::Update);

    assert_ok!(FundAdmin::inflation_rate(RuntimeOrigin::signed(1), inflation_rate_data,));

    assert_eq!(ProjectsInfo::<Test>::get(project_id).unwrap().inflation_rate, Some(inflation_rate));
  });
}

#[test]
fn inflation_rate_an_administrator_cannot_update_the_inflation_rate_if_it_is_not_set_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let inflation_rate = 80;
    let inflation_rate_data =
      make_project_inflation(project_id, Some(inflation_rate), CUDAction::Update);

    assert_noop!(
      FundAdmin::inflation_rate(RuntimeOrigin::signed(1), inflation_rate_data,),
      Error::<Test>::InflationRateNotSet
    );
  });
}

#[test]
fn inflation_rate_inflation_value_is_required_while_updating_the_inflation_rate_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let inflation_rate = 70;
    let inflation_rate_data =
      make_project_inflation(project_id, Some(inflation_rate), CUDAction::Create);

    assert_ok!(FundAdmin::inflation_rate(RuntimeOrigin::signed(1), inflation_rate_data.clone(),));

    let inflation_rate_data = make_project_inflation(project_id, None, CUDAction::Update);

    assert_noop!(
      FundAdmin::inflation_rate(RuntimeOrigin::signed(1), inflation_rate_data,),
      Error::<Test>::InflationRateRequired
    );
  });
}

#[test]
fn inflation_rate_an_administrator_deletes_the_inflation_rate_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let inflation_rate = 70;
    let inflation_rate_data =
      make_project_inflation(project_id, Some(inflation_rate), CUDAction::Create);

    assert_ok!(FundAdmin::inflation_rate(RuntimeOrigin::signed(1), inflation_rate_data.clone(),));

    let inflation_rate_data = make_project_inflation(project_id, None, CUDAction::Delete);

    assert_ok!(FundAdmin::inflation_rate(RuntimeOrigin::signed(1), inflation_rate_data,));

    assert_eq!(ProjectsInfo::<Test>::get(project_id).unwrap().inflation_rate, None);
  });
}

#[test]
fn inflation_rate_an_administrator_cannot_delete_the_inflation_rate_if_it_is_not_set_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let inflation_rate_data = make_project_inflation(project_id, None, CUDAction::Delete);

    assert_noop!(
      FundAdmin::inflation_rate(RuntimeOrigin::signed(1), inflation_rate_data,),
      Error::<Test>::InflationRateNotSet
    );
  });
}

// B A N K   D O C U M E N T S
// =================================================================================================
#[test]
fn bank_documents_an_administrator_uploads_bank_documents_for_a_given_eb5_drawdown_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    let bank_documents = make_documents(1);

    assert_ok!(FundAdmin::bank_confirming_documents(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      Some(bank_documents.clone()),
      CUDAction::Create,
    ));

    assert_eq!(
      DrawdownsInfo::<Test>::get(drawdown_id).unwrap().bank_documents,
      Some(bank_documents)
    );
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Confirmed);
  });
}

#[test]
fn bank_documents_cannot_upload_documents_for_a_contruction_loan_drawdown_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::ConstructionLoan, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    let bank_documents = make_documents(1);

    assert_noop!(
      FundAdmin::bank_confirming_documents(
        RuntimeOrigin::signed(1),
        project_id,
        drawdown_id,
        Some(bank_documents.clone()),
        CUDAction::Create,
      ),
      Error::<Test>::OnlyEB5DrawdownsCanUploadBankDocuments
    );
  });
}

#[test]
fn bank_documents_cannot_upload_documents_for_a_developer_equity_drawdown_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::DeveloperEquity, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    let bank_documents = make_documents(1);

    assert_noop!(
      FundAdmin::bank_confirming_documents(
        RuntimeOrigin::signed(1),
        project_id,
        drawdown_id,
        Some(bank_documents.clone()),
        CUDAction::Create,
      ),
      Error::<Test>::OnlyEB5DrawdownsCanUploadBankDocuments
    );
  });
}

#[test]
fn bank_documents_cannot_upload_documents_without_an_array_of_documents_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    assert_noop!(
      FundAdmin::bank_confirming_documents(
        RuntimeOrigin::signed(1),
        project_id,
        drawdown_id,
        None,
        CUDAction::Create,
      ),
      Error::<Test>::BankConfirmingDocumentsNotProvided
    );
  });
}

#[test]
fn bank_documents_cannot_upload_documents_with_an_empty_array_of_documents_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    let bank_documents: Documents<Test> = bounded_vec![];

    assert_noop!(
      FundAdmin::bank_confirming_documents(
        RuntimeOrigin::signed(1),
        project_id,
        drawdown_id,
        Some(bank_documents),
        CUDAction::Create,
      ),
      Error::<Test>::BankConfirmingDocumentsEmpty
    );
  });
}

#[test]
fn bank_documents_cannot_upload_documents_if_the_drawdown_is_not_approved_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    let bank_documents = make_documents(1);

    assert_noop!(
      FundAdmin::bank_confirming_documents(
        RuntimeOrigin::signed(1),
        project_id,
        drawdown_id,
        Some(bank_documents),
        CUDAction::Create,
      ),
      Error::<Test>::DrawdowMustBeInApprovedStatus
    );
  });
}

#[test]
fn bank_documents_cannot_upload_documents_twice_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    let bank_documents = make_documents(1);

    assert_ok!(FundAdmin::bank_confirming_documents(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      Some(bank_documents.clone()),
      CUDAction::Create,
    ));

    assert_noop!(
      FundAdmin::bank_confirming_documents(
        RuntimeOrigin::signed(1),
        project_id,
        drawdown_id,
        Some(bank_documents),
        CUDAction::Create,
      ),
      Error::<Test>::DrawdownHasAlreadyBankConfirmingDocuments
    );
  });
}

#[test]
fn bank_documents_an_administrator_updates_the_bank_documents_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    let bank_documents = make_documents(1);

    assert_ok!(FundAdmin::bank_confirming_documents(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      Some(bank_documents.clone()),
      CUDAction::Create,
    ));

    let bank_documents = make_documents(2);

    assert_ok!(FundAdmin::bank_confirming_documents(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      Some(bank_documents.clone()),
      CUDAction::Update,
    ));

    assert_eq!(
      DrawdownsInfo::<Test>::get(drawdown_id).unwrap().bank_documents,
      Some(bank_documents)
    );
  });
}

#[test]
fn bank_documents_cannot_update_documents_without_uploading_documents_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    let bank_documents = make_documents(1);

    assert_ok!(FundAdmin::bank_confirming_documents(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      Some(bank_documents.clone()),
      CUDAction::Create,
    ));

    assert_noop!(
      FundAdmin::bank_confirming_documents(
        RuntimeOrigin::signed(1),
        project_id,
        drawdown_id,
        None,
        CUDAction::Update,
      ),
      Error::<Test>::BankConfirmingDocumentsNotProvided
    );
  });
}

#[test]
fn bank_documents_cannot_update_documents_with_an_empty_array_of_documents_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    let bank_documents = make_documents(1);

    assert_ok!(FundAdmin::bank_confirming_documents(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      Some(bank_documents.clone()),
      CUDAction::Create,
    ));

    let bank_documents = make_documents(0);

    assert_noop!(
      FundAdmin::bank_confirming_documents(
        RuntimeOrigin::signed(1),
        project_id,
        drawdown_id,
        Some(bank_documents),
        CUDAction::Update,
      ),
      Error::<Test>::BankConfirmingDocumentsEmpty
    );
  });
}

#[test]
fn bank_documents_cannot_update_bank_documents_if_the_drawdown_is_not_confirmed_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    let bank_documents = make_documents(2);

    assert_noop!(
      FundAdmin::bank_confirming_documents(
        RuntimeOrigin::signed(1),
        project_id,
        drawdown_id,
        Some(bank_documents),
        CUDAction::Update,
      ),
      Error::<Test>::DrawdowMustBeInConfirmedStatus
    );
  });
}

#[test]
fn bank_documents_an_administrator_deletes_bank_documents_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    let bank_documents = make_documents(1);

    assert_ok!(FundAdmin::bank_confirming_documents(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      Some(bank_documents.clone()),
      CUDAction::Create,
    ));

    assert_ok!(FundAdmin::bank_confirming_documents(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      CUDAction::Delete,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().bank_documents, None);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Approved);
  });
}

#[test]
fn bank_documents_cannot_delete_documents_if_the_drawdown_is_not_confirmed_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_noop!(
      FundAdmin::bank_confirming_documents(
        RuntimeOrigin::signed(1),
        project_id,
        drawdown_id,
        None,
        CUDAction::Delete,
      ),
      Error::<Test>::DrawdowMustBeInConfirmedStatus
    );
  });
}

// R E S E T    D R A W D O W N
// =================================================================================================
#[test]
fn reset_drawdown_a_builder_resets_a_eb5_drawdown_should_work() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    let transactions_by_drawdown = TransactionsByDrawdown::<Test>::get(project_id, drawdown_id);

    assert_ok!(FundAdmin::reset_drawdown(RuntimeOrigin::signed(2), project_id, drawdown_id,));

    for transaction in transactions_by_drawdown {
      assert_eq!(TransactionsInfo::<Test>::contains_key(transaction), false);
    }

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Draft);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().total_amount, 0);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().bulkupload_documents, None);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().bank_documents, None);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().description, None);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().feedback, None);
    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().eb5_drawdown_status,
      Some(DrawdownStatus::Draft)
    );
  });
}

#[test]
fn reset_drawdown_a_builder_resets_a_construction_loan_drawdown_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::ConstructionLoan, 1);

    let drawdown_description = make_field_description("Construction Loan Drawdown 1");
    let total_amount = 100000u64;
    let documents = make_documents(1);

    assert_ok!(FundAdmin::up_bulkupload(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      drawdown_description.clone(),
      total_amount,
      documents.clone(),
    ));

    assert_ok!(FundAdmin::reset_drawdown(RuntimeOrigin::signed(2), project_id, drawdown_id,));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Draft);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().total_amount, 0);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().bulkupload_documents, None);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().bank_documents, None);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().description, None);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().feedback, None);
    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().construction_loan_drawdown_status,
      Some(DrawdownStatus::Draft)
    );
  });
}

#[test]
fn reset_drawdown_a_builder_resets_a_developer_equity_drawdown_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::DeveloperEquity, 1);

    let drawdown_description = make_field_description("Developer Equity Drawdown 1");
    let total_amount = 100000u64;
    let documents = make_documents(1);

    assert_ok!(FundAdmin::up_bulkupload(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      drawdown_description.clone(),
      total_amount,
      documents.clone(),
    ));

    assert_ok!(FundAdmin::reset_drawdown(RuntimeOrigin::signed(2), project_id, drawdown_id,));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Draft);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().total_amount, 0);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().bulkupload_documents, None);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().bank_documents, None);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().description, None);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().feedback, None);
    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().developer_equity_drawdown_status,
      Some(DrawdownStatus::Draft)
    );
  });
}

#[test]
fn reset_drawdown_a_builder_cannot_reset_a_drawdown_if_it_is_not_submitted_should_fail() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);

    assert_noop!(
      FundAdmin::reset_drawdown(RuntimeOrigin::signed(2), project_id, drawdown_id,),
      Error::<Test>::DrawdownNotSubmitted
    );
  });
}

#[test]
fn reset_drawdown_a_builder_cannot_reset_an_approved_drawdown() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);
    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    assert_noop!(
      FundAdmin::reset_drawdown(RuntimeOrigin::signed(2), project_id, drawdown_id,),
      Error::<Test>::DrawdownNotSubmitted
    );
  });
}

// E R R O R    R E C O V E R Y   D R A W D O W N S
// =================================================================================================
#[test]
fn an_administrators_updates_a_transaction_for_an_approved_drawdown_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);

    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );
    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);
    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().eb5_drawdown_status,
      Some(DrawdownStatus::Draft)
    );

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    let transaction_id = get_transaction_id(project_id, drawdown_id, expenditure_id);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Submitted);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().total_amount, 10000);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Submitted
    );
    assert_eq!(TransactionsInfo::<Test>::get(transaction_id).unwrap().amount, 10000);
    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().eb5_drawdown_status,
      Some(DrawdownStatus::Submitted)
    );

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().recovery_record.len(), 0);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Approved);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Approved
    );

    let new_total_amount = 200000u64;
    let transaction_data = make_transaction(
      Some(expenditure_id),
      Some(new_total_amount),
      CUDAction::Update,
      Some(transaction_id),
    );

    assert_ok!(FundAdmin::recovery_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      transaction_data,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().recovery_record.len(), 1);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().total_amount, new_total_amount);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Approved);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Approved
    );
  });
}

#[test]
fn an_administrators_adds_a_new_transaction_to_an_approved_drawdown_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);

    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );
    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);
    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().eb5_drawdown_status,
      Some(DrawdownStatus::Draft)
    );

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    let transaction_id = get_transaction_id(project_id, drawdown_id, expenditure_id);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Submitted);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().total_amount, 10000);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Submitted
    );
    assert_eq!(TransactionsInfo::<Test>::get(transaction_id).unwrap().amount, 10000);
    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().eb5_drawdown_status,
      Some(DrawdownStatus::Submitted)
    );
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 1);

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().recovery_record.len(), 0);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Approved);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Approved
    );
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 1);

    let expenditure_id_2 = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 2"),
      ExpenditureType::SoftCost,
    );

    let transaction_data =
      make_transaction(Some(expenditure_id_2), Some(2000), CUDAction::Create, None);

    assert_ok!(FundAdmin::recovery_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      transaction_data,
    ));

    let transaction_id_2 = get_transaction_id(project_id, drawdown_id, expenditure_id_2);

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().recovery_record.len(), 1);
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 2);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Approved);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Approved
    );
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id_2).unwrap().status,
      TransactionStatus::Approved
    );
  });
}

#[test]
fn an_administrators_deletes_a_transaction_for_an_approved_drawdown_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);

    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );
    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);
    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().eb5_drawdown_status,
      Some(DrawdownStatus::Draft)
    );

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    let transaction_id = get_transaction_id(project_id, drawdown_id, expenditure_id);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Submitted);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().total_amount, 10000);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Submitted
    );
    assert_eq!(TransactionsInfo::<Test>::get(transaction_id).unwrap().amount, 10000);
    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().eb5_drawdown_status,
      Some(DrawdownStatus::Submitted)
    );
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 1);

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().recovery_record.len(), 0);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Approved);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Approved
    );
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 1);

    let transaction_data =
      make_transaction(Some(expenditure_id), None, CUDAction::Delete, Some(transaction_id));

    assert_ok!(FundAdmin::recovery_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      transaction_data,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().recovery_record.len(), 1);
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 0);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Approved);
    assert_eq!(TransactionsInfo::<Test>::get(transaction_id).is_none(), true);
  });
}

#[test]
fn an_administrators_updates_a_transaction_for_a_confirmed_drawdown_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);

    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );
    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);
    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().eb5_drawdown_status,
      Some(DrawdownStatus::Draft)
    );

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    let transaction_id = get_transaction_id(project_id, drawdown_id, expenditure_id);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Submitted);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().total_amount, 10000);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Submitted
    );
    assert_eq!(TransactionsInfo::<Test>::get(transaction_id).unwrap().amount, 10000);
    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().eb5_drawdown_status,
      Some(DrawdownStatus::Submitted)
    );

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().recovery_record.len(), 0);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Approved);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Approved
    );
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 1);

    let bank_documents = make_documents(1);
    assert_ok!(FundAdmin::bank_confirming_documents(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      Some(bank_documents.clone()),
      CUDAction::Create,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Confirmed);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Confirmed
    );
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 1);

    let new_total_amount = 200000u64;
    let transaction_data = make_transaction(
      Some(expenditure_id),
      Some(new_total_amount),
      CUDAction::Update,
      Some(transaction_id),
    );

    assert_ok!(FundAdmin::recovery_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      transaction_data,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().total_amount, new_total_amount);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().recovery_record.len(), 1);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Confirmed);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Confirmed
    );
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 1);
  });
}

#[test]
fn an_administrators_adds_a_new_transaction_to_a_confirmed_drawdown_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);

    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );
    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);
    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().eb5_drawdown_status,
      Some(DrawdownStatus::Draft)
    );

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    let transaction_id = get_transaction_id(project_id, drawdown_id, expenditure_id);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Submitted);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().total_amount, 10000);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Submitted
    );
    assert_eq!(TransactionsInfo::<Test>::get(transaction_id).unwrap().amount, 10000);
    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().eb5_drawdown_status,
      Some(DrawdownStatus::Submitted)
    );
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 1);

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().recovery_record.len(), 0);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Approved);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Approved
    );
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 1);

    let bank_documents = make_documents(1);
    assert_ok!(FundAdmin::bank_confirming_documents(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      Some(bank_documents.clone()),
      CUDAction::Create,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Confirmed);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Confirmed
    );
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 1);

    let expenditure_id_2 = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 2"),
      ExpenditureType::SoftCost,
    );
    let transaction_data =
      make_transaction(Some(expenditure_id_2), Some(2000), CUDAction::Create, None);

    assert_ok!(FundAdmin::recovery_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      transaction_data,
    ));

    let transaction_id_2 = get_transaction_id(project_id, drawdown_id, expenditure_id_2);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().recovery_record.len(), 1);
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 2);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Confirmed);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Confirmed
    );
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id_2).unwrap().status,
      TransactionStatus::Confirmed
    );
  });
}

#[test]
fn an_administrators_deletes_a_transaction_for_a_confirmed_drawdown_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();
    let drawdown_id = get_drawdown_id(project_id, DrawdownType::EB5, 1);

    let expenditure_id = get_budget_expenditure_id(
      project_id,
      make_field_name("Expenditure Test 1"),
      ExpenditureType::HardCost,
    );
    let transaction_data =
      make_transaction(Some(expenditure_id), Some(10000), CUDAction::Create, None);
    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().eb5_drawdown_status,
      Some(DrawdownStatus::Draft)
    );

    assert_ok!(FundAdmin::submit_drawdown(
      RuntimeOrigin::signed(2),
      project_id,
      drawdown_id,
      Some(transaction_data),
      true,
    ));

    let transaction_id = get_transaction_id(project_id, drawdown_id, expenditure_id);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Submitted);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().total_amount, 10000);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Submitted
    );
    assert_eq!(TransactionsInfo::<Test>::get(transaction_id).unwrap().amount, 10000);
    assert_eq!(
      ProjectsInfo::<Test>::get(project_id).unwrap().eb5_drawdown_status,
      Some(DrawdownStatus::Submitted)
    );
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 1);

    assert_ok!(FundAdmin::approve_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      None,
      None,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().recovery_record.len(), 0);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Approved);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Approved
    );
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 1);

    let bank_documents = make_documents(1);
    assert_ok!(FundAdmin::bank_confirming_documents(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      Some(bank_documents.clone()),
      CUDAction::Create,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Confirmed);
    assert_eq!(
      TransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      TransactionStatus::Confirmed
    );
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 1);

    let transaction_data =
      make_transaction(Some(expenditure_id), None, CUDAction::Delete, Some(transaction_id));

    assert_ok!(FundAdmin::recovery_drawdown(
      RuntimeOrigin::signed(1),
      project_id,
      drawdown_id,
      transaction_data,
    ));

    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().recovery_record.len(), 1);
    assert_eq!(TransactionsByDrawdown::<Test>::get(project_id, drawdown_id).len(), 0);
    assert_eq!(DrawdownsInfo::<Test>::get(drawdown_id).unwrap().status, DrawdownStatus::Confirmed);
    assert_eq!(TransactionsInfo::<Test>::get(transaction_id).is_none(), true);
  });
}

// E R R O R    R E C O V E R Y   R E V E N U E S
// =================================================================================================
#[test]
fn an_administrators_updates_a_transaction_for_an_approved_revenue_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      true,
    ));

    let transaction_id = get_revenue_transaction_id(project_id, revenue_id, job_eligible_id);
    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Submitted);
    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().total_amount, 10000);
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      RevenueTransactionStatus::Submitted
    );
    assert_eq!(RevenueTransactionsInfo::<Test>::get(transaction_id).unwrap().amount, 10000);
    assert_eq!(TransactionsByRevenue::<Test>::get(project_id, revenue_id).len(), 1);

    assert_ok!(FundAdmin::approve_revenue(RuntimeOrigin::signed(1), project_id, revenue_id,));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().recovery_record.len(), 0);
    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Approved);
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      RevenueTransactionStatus::Approved
    );
    assert_eq!(TransactionsByRevenue::<Test>::get(project_id, revenue_id).len(), 1);

    let new_total_amount = 200000u64;
    let revenue_transaction_data = make_revenue_transaction(
      Some(job_eligible_id),
      Some(new_total_amount),
      CUDAction::Update,
      Some(transaction_id),
    );

    assert_ok!(FundAdmin::recovery_revenue(
      RuntimeOrigin::signed(1),
      project_id,
      revenue_id,
      revenue_transaction_data,
    ));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().recovery_record.len(), 1);
    assert_eq!(TransactionsByRevenue::<Test>::get(project_id, revenue_id).len(), 1);
    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Approved);
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      RevenueTransactionStatus::Approved
    );
  });
}

#[test]
fn an_administrators_adds_a_transaction_for_an_approved_revenue_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      true,
    ));

    let transaction_id = get_revenue_transaction_id(project_id, revenue_id, job_eligible_id);
    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Submitted);
    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().total_amount, 10000);
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      RevenueTransactionStatus::Submitted
    );
    assert_eq!(RevenueTransactionsInfo::<Test>::get(transaction_id).unwrap().amount, 10000);
    assert_eq!(TransactionsByRevenue::<Test>::get(project_id, revenue_id).len(), 1);

    assert_ok!(FundAdmin::approve_revenue(RuntimeOrigin::signed(1), project_id, revenue_id,));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().recovery_record.len(), 0);
    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Approved);
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      RevenueTransactionStatus::Approved
    );
    assert_eq!(TransactionsByRevenue::<Test>::get(project_id, revenue_id).len(), 1);

    let job_eligible_data = make_job_eligible(
      Some(make_field_name("Job Eligible Test: Construction")),
      Some(1000),
      Some(make_field_description("16344, 45862, 57143")),
      Some(200),
      CUDAction::Create,
      None,
    );

    assert_ok!(FundAdmin::expenditures_and_job_eligibles(
      RuntimeOrigin::signed(1),
      project_id,
      None,
      Some(job_eligible_data),
    ));

    let job_eligible_id_2 =
      get_job_eligible_id(project_id, make_field_name("Job Eligible Test: Construction"));
    assert_eq!(JobEligiblesInfo::<Test>::get(job_eligible_id_2).is_some(), true);

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id_2), Some(20000), CUDAction::Create, None);

    assert_ok!(FundAdmin::recovery_revenue(
      RuntimeOrigin::signed(1),
      project_id,
      revenue_id,
      revenue_transaction_data,
    ));

    let transaction_id_2 = get_revenue_transaction_id(project_id, revenue_id, job_eligible_id_2);

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().recovery_record.len(), 1);
    assert_eq!(TransactionsByRevenue::<Test>::get(project_id, revenue_id).len(), 2);
    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Approved);
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      RevenueTransactionStatus::Approved
    );
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(transaction_id_2).unwrap().status,
      RevenueTransactionStatus::Approved
    );
  });
}

#[test]
fn an_administrators_deletes_a_transaction_for_an_approved_revenue_works() {
  new_test_ext().execute_with(|| {
    assert_ok!(make_default_full_project());
    let project_id = ProjectsInfo::<Test>::iter_keys().next().unwrap();

    let revenue_id = get_revenue_id(project_id, 1);
    let job_eligible_id = get_job_eligible_id(project_id, make_field_name("Job Eligible Test"));

    let revenue_transaction_data =
      make_revenue_transaction(Some(job_eligible_id), Some(10000), CUDAction::Create, None);

    assert_ok!(FundAdmin::submit_revenue(
      RuntimeOrigin::signed(2),
      project_id,
      revenue_id,
      Some(revenue_transaction_data),
      true,
    ));

    let transaction_id = get_revenue_transaction_id(project_id, revenue_id, job_eligible_id);
    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Submitted);
    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().total_amount, 10000);
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      RevenueTransactionStatus::Submitted
    );
    assert_eq!(RevenueTransactionsInfo::<Test>::get(transaction_id).unwrap().amount, 10000);
    assert_eq!(TransactionsByRevenue::<Test>::get(project_id, revenue_id).len(), 1);

    assert_ok!(FundAdmin::approve_revenue(RuntimeOrigin::signed(1), project_id, revenue_id,));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().recovery_record.len(), 0);
    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Approved);
    assert_eq!(
      RevenueTransactionsInfo::<Test>::get(transaction_id).unwrap().status,
      RevenueTransactionStatus::Approved
    );
    assert_eq!(TransactionsByRevenue::<Test>::get(project_id, revenue_id).len(), 1);

    let revenue_transaction_data = make_revenue_transaction(
      Some(job_eligible_id),
      Some(20000),
      CUDAction::Delete,
      Some(transaction_id),
    );

    assert_ok!(FundAdmin::recovery_revenue(
      RuntimeOrigin::signed(1),
      project_id,
      revenue_id,
      revenue_transaction_data,
    ));

    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().recovery_record.len(), 1);
    assert_eq!(TransactionsByRevenue::<Test>::get(project_id, revenue_id).len(), 0);
    assert_eq!(RevenuesInfo::<Test>::get(revenue_id).unwrap().status, RevenueStatus::Approved);
    assert_eq!(RevenueTransactionsInfo::<Test>::get(transaction_id).is_none(), true);
  });
}
