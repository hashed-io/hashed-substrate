use super::*;
use frame_support::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec;

pub type FieldName = BoundedVec<u8, ConstU32<100>>;
pub type FieldDescription = BoundedVec<u8, ConstU32<400>>;
pub type CID = BoundedVec<u8, ConstU32<100>>;
pub type Documents<T> = BoundedVec<(FieldName,CID), <T as Config>::MaxDocuments>;

// Projects
pub type ProjectId = [u8; 32];
pub type CreationDate = u64;
pub type CompletionDate = u64;
pub type UpdatedDate = u64;
pub type RegistrationDate = u64;
pub type BankName = BoundedVec<u8, ConstU32<100>>;
pub type BankAddress = BoundedVec<u8, ConstU32<100>>;

// Users
pub type DateRegistered = u64;

// Transactions
pub type TransactionId = [u8; 32];
pub type Amount = u64;

// Drawdowns
pub type DrawdownId = [u8; 32];
pub type DrawdownNumber = u32;

// Budget expenditures
pub type ExpenditureId = [u8; 32];
pub type ExpenditureAmount = u64;
pub type NAICSCode = BoundedVec<u8, ConstU32<400>>;
pub type JobsMultiplier = u32;
pub type InflationRate = u32;

// Miscellaneous
pub type CreatedDate = u64;
pub type CloseDate = u64;
pub type TotalAmount = u64;

// Revenues
pub type RevenueAmount = u128;
pub type JobEligibleId = [u8; 32];
pub type JobEligibleAmount = u128;
pub type RevenueId = [u8; 32];
pub type RevenueNumber = u32;
pub type RevenueTransactionId = [u8; 32];

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
    pub banks: Option<BoundedVec<(BankName, BankAddress), T::MaxBanksPerProject>>,
    pub creation_date: CreationDate,
    pub completion_date: CompletionDate,
    pub registration_date: RegistrationDate,
    pub updated_date: UpdatedDate,
	pub eb5_drawdown_status: Option<DrawdownStatus>,
	pub construction_loan_drawdown_status: Option<DrawdownStatus>,
	pub developer_equity_drawdown_status: Option<DrawdownStatus>,
    pub revenue_status: Option<RevenueStatus>,
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

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum ProxyRole {
    Administrator,
    Builder,
    Investor,
    Issuer,
    RegionalCenter,
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
    pub documents: Option<Documents<T>>,
    pub description: Option<FieldDescription>,
    pub feedback: Option<FieldDescription>,
    pub created_date: CreatedDate,
    pub close_date: CloseDate,
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
}

impl Default for TransactionStatus {
    fn default() -> Self {
        TransactionStatus::Draft
    }
}

// Possibles names: JobEligibleRenevueData, BudgetRevenueData, JobEligibleData
#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
pub struct JobEligibleData {
    pub project_id: ProjectId,
    pub name: FieldName,
    pub job_eligible_amount: JobEligibleAmount,
    pub naics_code: Option<FieldDescription>,
    pub jobs_multiplier: Option<JobsMultiplier>,
}

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
pub struct RevenueData {
    pub project_id: ProjectId,
    pub revenue_number: RevenueNumber,
    pub total_amount: RevenueAmount,
    pub status: RevenueStatus,
    pub created_date: CreatedDate,
    pub close_date: CloseDate,
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
    pub amount: RevenueAmount,
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



/// Extrinsics which require previous authorization to call them
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum ProxyPermission {
    RegisterUser, // users
    EditUser, // users_edit_user
    CreateProject, // projects_create_project
    EditProject, // projects_edit_project
    DeleteProject, // projects_delete_project
    AssignUser, // projects_assign_user
    Expenditures, // expenditures
    SubmitDrawdown, // submit_drawdown
    ApproveDrawdown, // approve_drawdown
    RejectDrawdown, // reject_drawdown
    UpBulkupload, // up_bulkupload
    Inflation, // inflation
    JobEligible, // job_eligible
}

impl ProxyPermission {
    pub fn to_vec(self) -> Vec<u8>{
        match self{
            Self::RegisterUser => "RegisterUser".as_bytes().to_vec(),
            Self::EditUser => "EditUser".as_bytes().to_vec(),
            Self::CreateProject => "CreateProject".as_bytes().to_vec(),
            Self::EditProject => "EditProject".as_bytes().to_vec(),
            Self::DeleteProject => "DeleteProject".as_bytes().to_vec(),
            Self::AssignUser => "AssignUser".as_bytes().to_vec(),
            Self::Expenditures => "Expenditures".as_bytes().to_vec(),
            Self::SubmitDrawdown => "SubmitDrawdown".as_bytes().to_vec(),
            Self::ApproveDrawdown => "ApproveDrawdown".as_bytes().to_vec(),
            Self::RejectDrawdown => "RejectDrawdown".as_bytes().to_vec(),
            Self::UpBulkupload => "UpBulkupload".as_bytes().to_vec(),
            Self::Inflation => "Inflation".as_bytes().to_vec(),
            Self::JobEligible => "JobEligible".as_bytes().to_vec(),
        }
    }

    pub fn id(&self) -> [u8;32]{
        self.to_vec().using_encoded(blake2_256)
    }

    pub fn administrator_permissions() -> Vec<Vec<u8>>{
        use crate::types::ProxyPermission::*;
        let administrator_permissions = [
            RegisterUser.to_vec(),
            EditUser.to_vec(),
            CreateProject.to_vec(),
            EditProject.to_vec(),
            DeleteProject.to_vec(),
            AssignUser.to_vec(),
            Expenditures.to_vec(),
            SubmitDrawdown.to_vec(),
            ApproveDrawdown.to_vec(),
            RejectDrawdown.to_vec(),
            UpBulkupload.to_vec(),
            Inflation.to_vec(),
            JobEligible.to_vec(),
        ].to_vec();
        administrator_permissions
    }

    pub fn builder_permissions() -> Vec<Vec<u8>>{
        use crate::types::ProxyPermission::*;
        [
            EditUser.to_vec(),
            SubmitDrawdown.to_vec(),
            UpBulkupload.to_vec(),
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
