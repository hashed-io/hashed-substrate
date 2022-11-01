use super::*;
use frame_support::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec;

//TODO: Fix types when using an Option, i.e: Option<CID>
pub type FieldName = BoundedVec<u8, ConstU32<100>>;
pub type FieldDescription = BoundedVec<u8, ConstU32<400>>;
pub type CID = BoundedVec<u8,ConstU32<100>>;
pub type Documents<T> = BoundedVec<(FieldName,CID), <T as Config>::MaxDocuments>;


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
    pub image: CID,
    pub address: FieldName, 
    pub status: ProjectStatus,
    pub inflation_rate: Option<u32>,
    pub creation_date: u64,
    pub completion_date: u64,
    pub registration_date: u64,
    pub updated_date: u64,
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
    pub date_registered: u64,
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
    pub project_id: [u8;32],
    pub name: FieldName,
    pub expenditure_type: ExpenditureType,
    pub expenditure_amount: u64,
    pub naics_code: Option<u32>,
    pub jobs_multiplier: Option<u32>,
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


#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
pub struct BudgetData {
    pub expenditure_id: [u8;32],
    pub balance: u64,
    pub created_date: u64,
    pub updated_date: u64,
}


#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct DrawdownData<T: Config> {
    pub project_id: [u8;32],
    pub drawdown_number: u32,
    pub drawdown_type: DrawdownType,
    pub total_amount: u64,
    pub status: DrawdownStatus,
    pub documents: Option<Documents<T>>,
    pub description: Option<FieldDescription>,
    pub feedback: Option<FieldDescription>,
    pub created_date: u64,
    pub close_date: u64,
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
    pub project_id: [u8;32],
    pub drawdown_id: [u8;32],
    pub expenditure_id: [u8;32],
    pub created_date: u64,
    pub updated_date: u64,
    pub closed_date: u64,
    pub feedback: Option<FieldDescription>,
    pub amount: u64,
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
            //TOREVIEW: optimization (?)
            //Self::Administrator => b"Administrator".to_vec(),
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