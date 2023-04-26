use super::*;
use frame_support::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec;

pub type FieldName = BoundedVec<u8, ConstU32<100>>;
pub type FieldDescription = BoundedVec<u8, ConstU32<400>>;
pub type CID = BoundedVec<u8, ConstU32<100>>;
pub type Documents<T> = BoundedVec<(FieldName,CID), <T as Config>::MaxDocuments>;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

// Projects
pub type ProjectId = [u8; 32];
pub type CreationDate = u64;
pub type CompletionDate = u64;
pub type UpdatedDate = u64;
pub type RegistrationDate = u64;
pub type BankName = BoundedVec<u8, ConstU32<100>>;
pub type BankAddress = BoundedVec<u8, ConstU32<100>>;
pub type UsersAssignation<T> = BoundedVec<(
    AccountIdOf<T>,
    ProxyRole,
    AssignAction,
), <T as Config>::MaxRegistrationsAtTime>;
pub type Banks<T> = BoundedVec<(
    BankName,
    BankAddress,
), <T as Config>::MaxBanksPerProject>;
pub type PrivateGroupId = BoundedVec<u8, ConstU32<400>>;
pub type InflationRate = u32;
pub type ProjectsInflation<T> = BoundedVec<(
    ProjectId,
    Option<InflationRate>,
    CUDAction,
), <T as Config>::MaxRegistrationsAtTime>;

// Users
pub type DateRegistered = u64;
pub type Users<T> = BoundedVec<(
    AccountIdOf<T>,
    Option<FieldName>,
    Option<ProxyRole>,
    CUDAction,
), <T as Config>::MaxRegistrationsAtTime>;

// Transactions
pub type TransactionId = [u8; 32];
pub type Amount = u64;
pub type Transactions<T> = BoundedVec<(
    Option<ExpenditureId>,
    Option<ExpenditureAmount>,
    Option<Documents<T>>,
    CUDAction,
    Option<TransactionId>,
), <T as Config>::MaxRegistrationsAtTime>;
pub type TransactionsFeedback<T> = BoundedVec<(
    TransactionId,
    FieldDescription
), <T as Config>::MaxRegistrationsAtTime>;

// Drawdowns
pub type DrawdownId = [u8; 32];
pub type DrawdownNumber = u32;
pub type DrawdownStatusChanges<T> = BoundedVec<(DrawdownStatus, UpdatedDate),  <T as Config>::MaxStatusChangesPerDrawdown>;
pub type RecoveryRecord<T> = BoundedVec<(AccountIdOf<T>, UpdatedDate), <T as Config>::MaxRecoveryChanges>;

// Budget expenditures
pub type ExpenditureId = [u8; 32];
pub type ExpenditureAmount = Amount;
pub type NAICSCode = BoundedVec<u8, ConstU32<400>>;
pub type JobsMultiplier = u32;
pub type Expenditures<T> = BoundedVec<(
    Option<FieldName>,
    Option<ExpenditureType>,
    Option<ExpenditureAmount>,
    Option<NAICSCode>,
    Option<JobsMultiplier>,
    CUDAction,
    Option<ExpenditureId>
), <T as Config>::MaxRegistrationsAtTime>;

// Miscellaneous
pub type CreatedDate = u64;
pub type CloseDate = u64;
pub type TotalAmount = Amount;

// Job Elgibles
pub type JobEligibleId = [u8; 32];
pub type JobEligibleAmount = Amount;
pub type JobEligibles<T> = BoundedVec<(
    Option<FieldName>,
    Option<JobEligibleAmount>,
    Option<NAICSCode>,
    Option<JobsMultiplier>,
    CUDAction,
    Option<JobEligibleId>,
), <T as Config>::MaxRegistrationsAtTime>;

// Revenues
pub type RevenueAmount = Amount;
pub type RevenueId = [u8; 32];
pub type RevenueNumber = u32;
pub type RevenueStatusChanges<T> = BoundedVec<(RevenueStatus, UpdatedDate),  <T as Config>::MaxStatusChangesPerRevenue>;

// Revenue Transactions
pub type RevenueTransactionId = [u8; 32];
pub type RevenueTransactionAmount = Amount;
pub type RevenueTransactions<T> = BoundedVec<(
    Option<JobEligibleId>,
    Option<RevenueAmount>,
    Option<Documents<T>>,
    CUDAction,
    Option<RevenueTransactionId>,
), <T as Config>::MaxRegistrationsAtTime>;

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct ProjectData<T: Config> {
    pub builder: Option<BoundedVec<T::AccountId, T::MaxBuildersPerProject>>,
    pub investor: Option<BoundedVec<T::AccountId, T::MaxInvestorsPerProject>>,
    pub issuer: Option<BoundedVec<T::AccountId, T::MaxIssuersPerProject>>,
    pub regional_center: Option<BoundedVec<T::AccountId, T::MaxRegionalCenterPerProject>>,
    pub title: FieldName,
    pub description: FieldDescription,
    pub image: Option<CID>,
    pub address: FieldName,
    pub status: ProjectStatus,
    pub inflation_rate: Option<InflationRate>,
    pub banks: Option<Banks<T>>,
    pub creation_date: CreationDate,
    pub completion_date: CompletionDate,
    pub registration_date: RegistrationDate,
    pub updated_date: UpdatedDate,
	pub eb5_drawdown_status: Option<DrawdownStatus>,
	pub construction_loan_drawdown_status: Option<DrawdownStatus>,
	pub developer_equity_drawdown_status: Option<DrawdownStatus>,
    pub revenue_status: Option<RevenueStatus>,
    pub private_group_id: PrivateGroupId,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum ProjectStatus {
    Started,
    Completed,
}
impl Default for ProjectStatus {
    fn default() -> Self {
        ProjectStatus::Started
    }
}


#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct UserData<T: Config> {
    pub name: FieldName,
    pub role: ProxyRole,
    pub image: CID,
    pub date_registered: DateRegistered,
    pub email: FieldName,
    pub documents: Option<Documents<T>>,
}

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
pub struct ExpenditureData {
    pub project_id: ProjectId,
    pub name: FieldName,
    pub expenditure_type: ExpenditureType,
    pub expenditure_amount: ExpenditureAmount,
    pub naics_code: Option<FieldDescription>,
    pub jobs_multiplier: Option<JobsMultiplier>,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum ExpenditureType {
    HardCost,
    SoftCost,
    Operational,
    Others,
}

impl Default for ExpenditureType {
    fn default() -> Self {
        ExpenditureType::HardCost
    }
}

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct DrawdownData<T: Config> {
    pub project_id: ProjectId,
    pub drawdown_number: DrawdownNumber,
    pub drawdown_type: DrawdownType,
    pub total_amount: TotalAmount,
    pub status: DrawdownStatus,
    pub bulkupload_documents: Option<Documents<T>>,
    pub bank_documents: Option<Documents<T>>,
    pub description: Option<FieldDescription>,
    pub feedback: Option<FieldDescription>,
    pub status_changes: DrawdownStatusChanges<T>,
    pub recovery_record: RecoveryRecord<T>,
    pub created_date: CreatedDate,
    pub closed_date: CloseDate,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum DrawdownType {
    EB5,
    ConstructionLoan,
    DeveloperEquity,
}

impl Default for DrawdownType {
    fn default() -> Self {
        DrawdownType::EB5
    }
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum DrawdownStatus {
    Draft,
    Submitted,
    Approved,
    Rejected,
    Confirmed,
}

impl Default for DrawdownStatus {
    fn default() -> Self {
        DrawdownStatus::Draft
    }
}

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct TransactionData<T: Config> {
    pub project_id: ProjectId,
    pub drawdown_id: DrawdownId,
    pub expenditure_id: ExpenditureId,
    pub created_date: CreatedDate,
    pub updated_date: UpdatedDate,
    pub closed_date: CloseDate,
    pub feedback: Option<FieldDescription>,
    pub amount: ExpenditureAmount,
    pub status: TransactionStatus,
    pub documents: Option<Documents<T>>,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum TransactionStatus {
    Draft,
    Submitted,
    Approved,
    Rejected,
    Confirmed
}

impl Default for TransactionStatus {
    fn default() -> Self {
        TransactionStatus::Draft
    }
}

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
pub struct JobEligibleData {
    pub project_id: ProjectId,
    pub name: FieldName,
    pub job_eligible_amount: JobEligibleAmount,
    pub naics_code: Option<FieldDescription>,
    pub jobs_multiplier: Option<JobsMultiplier>,
}

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct RevenueData<T: Config> {
    pub project_id: ProjectId,
    pub revenue_number: RevenueNumber,
    pub total_amount: RevenueAmount,
    pub status: RevenueStatus,
    pub status_changes: RevenueStatusChanges<T>,
    pub recovery_record: RecoveryRecord<T>,
    pub created_date: CreatedDate,
    pub closed_date: CloseDate,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum RevenueStatus {
    Draft,
    Submitted,
    Approved,
    Rejected,
}

impl Default for RevenueStatus {
    fn default() -> Self {
        RevenueStatus::Draft
    }
}

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct RevenueTransactionData<T: Config> {
    pub project_id: ProjectId,
    pub revenue_id: RevenueId,
    pub job_eligible_id: JobEligibleId,
    pub created_date: CreatedDate,
    pub updated_date: UpdatedDate,
    pub closed_date: CloseDate,
    pub feedback: Option<FieldDescription>,
    pub amount: RevenueTransactionAmount,
    pub status: RevenueTransactionStatus,
    pub documents: Option<Documents<T>>,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum RevenueTransactionStatus {
    Draft,
    Submitted,
    Approved,
    Rejected,
}

impl Default for RevenueTransactionStatus {
    fn default() -> Self {
        RevenueTransactionStatus::Draft
    }
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum CUDAction {
    Create,
    Update,
    Delete,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum AssignAction {
    Assign,
    Unassign,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum ProxyRole {
    Administrator,
    Builder,
    Investor,
    Issuer,
    RegionalCenter,
}

impl ProxyRole {
    pub fn to_vec(self) -> Vec<u8>{
        match self{
            Self::Administrator => "Administrator".as_bytes().to_vec(),
            Self::Builder => "Builder".as_bytes().to_vec(),
            Self::Investor => "Investor".as_bytes().to_vec(),
            Self::Issuer => "Issuer".as_bytes().to_vec(),
            Self::RegionalCenter => "RegionalCenter".as_bytes().to_vec(),
        }
    }

    pub fn id(&self) -> [u8;32]{
        self.to_vec().using_encoded(blake2_256)
    }

    pub fn enum_to_vec() -> Vec<Vec<u8>>{
        use crate::types::ProxyRole::*;
        [Administrator.to_vec(), Builder.to_vec(), Investor.to_vec(), Issuer.to_vec(), RegionalCenter.to_vec()].to_vec()
    }

}



#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum ProxyPermission {
    CreateProject, // projects_create_project: admin
    EditProject, // projects_edit_project: admin
    DeleteProject, // projects_delete_project: admin
    AssignUsers, // projects_assign_user: admin
    ExecuteUsers, // users: admin
    EditUser, // users_edit_user: all
    Expenditures, // expenditures: admin
    SubmitDrawdown, // submit_drawdown: admin, builder
    ApproveDrawdown, // approve_drawdown: admin
    RejectDrawdown, // reject_drawdown: admin
    ExecuteTransactions, // transactions: admin, builder
    UpBulkupload, // up_bulkupload: builder
    InflationRate, // inflation: admin
    JobEligible, // job_eligible: admin
    RevenueTransaction, // revenue_transaction: builder
    SubmitRevenue, // submit_revenue: builder
    ApproveRevenue, // approve_revenue: admin
    RejectRevenue, // reject_revenue: admin
    BankConfirming, // bank_confirming: admin
    CancelDrawdownSubmission, // cancel_drawdown_submission: builder
    RecoveryDrawdown, // recovery_drawdown: admin
    RecoveryRevenue, // recovery_revenue: admin
    RecoveryTransaction, // recovery_drawdown_transaction: admin
    RecoveryRevenueTransaction, // recovery_revenue_transaction: admin
    BulkUploadTransaction, // bulk_upload_transaction: admin
}

impl ProxyPermission {
    pub fn to_vec(self) -> Vec<u8>{
        match self{
            Self::CreateProject => "CreateProject".as_bytes().to_vec(),
            Self::EditProject => "EditProject".as_bytes().to_vec(),
            Self::DeleteProject => "DeleteProject".as_bytes().to_vec(),
            Self::AssignUsers => "AssignUsers".as_bytes().to_vec(),
            Self::ExecuteUsers => "ExecuteUsers".as_bytes().to_vec(),
            Self::EditUser => "Edituser".as_bytes().to_vec(),
            Self::Expenditures => "Expenditures".as_bytes().to_vec(),
            Self::SubmitDrawdown => "SubmitDrawdown".as_bytes().to_vec(),
            Self::ApproveDrawdown => "ApproveDrawdown".as_bytes().to_vec(),
            Self::RejectDrawdown => "RejectDrawdown".as_bytes().to_vec(),
            Self::ExecuteTransactions => "ExecuteTransactions".as_bytes().to_vec(),
            Self::UpBulkupload => "UpBulkupload".as_bytes().to_vec(),
            Self::InflationRate => "InflationRate".as_bytes().to_vec(),
            Self::JobEligible => "JobEligible".as_bytes().to_vec(),
            Self::RevenueTransaction => "RevenueTransaction".as_bytes().to_vec(),
            Self::SubmitRevenue => "SubmitRevenue".as_bytes().to_vec(),
            Self::ApproveRevenue => "ApproveRevenue".as_bytes().to_vec(),
            Self::RejectRevenue => "RejectRevenue".as_bytes().to_vec(),
            Self::BankConfirming => "BankConfirming".as_bytes().to_vec(),
            Self::CancelDrawdownSubmission => "CancelDrawdownSubmission".as_bytes().to_vec(),
            Self::RecoveryDrawdown => "RecoveryDrawdown".as_bytes().to_vec(),
            Self::RecoveryRevenue => "RecoveryRevenue".as_bytes().to_vec(),
            Self::RecoveryTransaction => "RecoveryTransaction".as_bytes().to_vec(),
            Self::RecoveryRevenueTransaction => "RecoveryRevenueTransaction".as_bytes().to_vec(),
            Self::BulkUploadTransaction => "BulkUploadTransaction".as_bytes().to_vec(),
        }
    }

    pub fn id(&self) -> [u8;32]{
        self.to_vec().using_encoded(blake2_256)
    }

    pub fn administrator_permissions() -> Vec<Vec<u8>>{
        use crate::types::ProxyPermission::*;
        [
            CreateProject.to_vec(),
            EditProject.to_vec(),
            DeleteProject.to_vec(),
            AssignUsers.to_vec(),
            ExecuteUsers.to_vec(),
            EditUser.to_vec(),
            Expenditures.to_vec(),
            SubmitDrawdown.to_vec(),
            ApproveDrawdown.to_vec(),
            RejectDrawdown.to_vec(),
            ExecuteTransactions.to_vec(),
            UpBulkupload.to_vec(),        
            InflationRate.to_vec(),
            JobEligible.to_vec(),
            RevenueTransaction.to_vec(),
            SubmitRevenue.to_vec(),
            ApproveRevenue.to_vec(),
            RejectRevenue.to_vec(),
            BankConfirming.to_vec(),
            CancelDrawdownSubmission.to_vec(),
            RecoveryDrawdown.to_vec(),
            RecoveryRevenue.to_vec(),
            RecoveryTransaction.to_vec(),
            RecoveryRevenueTransaction.to_vec(),
            BulkUploadTransaction.to_vec(),
        ].to_vec()
    }

    pub fn builder_permissions() -> Vec<Vec<u8>>{
        use crate::types::ProxyPermission::*;
        [
            EditUser.to_vec(),
            SubmitDrawdown.to_vec(),
            ExecuteTransactions.to_vec(),
            UpBulkupload.to_vec(),
            RevenueTransaction.to_vec(),
            SubmitRevenue.to_vec(),
            CancelDrawdownSubmission.to_vec(),
        ].to_vec()
    }

    pub fn investor_permissions() -> Vec<Vec<u8>>{
        use crate::types::ProxyPermission::*;
        [EditUser.to_vec(),].to_vec()
    }

    pub fn issuer_permissions() -> Vec<Vec<u8>>{
        use crate::types::ProxyPermission::*;
        [EditUser.to_vec(),].to_vec()
    }

    pub fn regional_center_permissions() -> Vec<Vec<u8>>{
        use crate::types::ProxyPermission::*;
        [EditUser.to_vec(),].to_vec()
    }


}
