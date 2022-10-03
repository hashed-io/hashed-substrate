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
pub struct ProjectData<T: Config>{
    pub developer: Option<BoundedVec<T::AccountId, T::MaxDevelopersPerProject>>,
    pub investor: Option<BoundedVec<T::AccountId, T::MaxInvestorsPerProject>>,
    pub issuer: Option<BoundedVec<T::AccountId, T::MaxIssuersPerProject>>,
    pub regional_center: Option<BoundedVec<T::AccountId, T::MaxRegionalCenterPerProject>>,
    pub tittle: FieldName,
    pub description: FieldDescription,
    pub image: CID,
    pub adress: FieldName, 
    pub status: ProjectStatus,
    pub project_type: ProjectType,
    pub creation_date: u64,
    pub completition_date: u64,
    pub updated_date: u64,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum ProjectStatus{
    Started,
    Completed,
}
impl Default for ProjectStatus{
    fn default() -> Self {
        ProjectStatus::Started
    }
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum ProjectType{
    Construction, 
    ConstructionOperation,
    ConstructionBridge, 
    Operation,
}


#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct UserData<T: Config>{
    pub name: FieldName,
    pub role: Option<ProxyRole>,
    pub image: CID,
    pub date_registered: u64,
    pub email: FieldName,
    pub documents: Option<Documents<T>>,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum ProxyRole{
    Administrator,
    Developer,
    Investor,
    Issuer,
    RegionalCenter,
}


#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
pub struct ExpenditureData {
    pub project_id: [u8;32],
    pub name: FieldName,
    pub expenditure_type: ExpenditureType,
    pub balance: u64,
    pub naics_code: Option<u32>,
    pub jobs_multiplier: Option<u32>,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum ExpenditureType{
    HardCost, 
    SoftCost,
    Operational, 
    Others,
}

impl Default for ExpenditureType{
    fn default() -> Self {
        ExpenditureType::HardCost
    }
}


#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
pub struct BudgetData{
    pub expenditure_id: [u8;32],
    pub balance: u64,
    pub created_date: u64,
    pub updated_date: u64,
}


#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct DrawdownData<T:Config>{
    pub project_id: [u8;32],
    pub drawdown_number: u32,
    pub drawdown_type: DrawdownType,
    pub total_amount: u64,
    pub status: DrawdownStatus,
    //TODO: add Option<Files> -> Bulk Upload
    pub created_date: u64,
    pub close_date: u64,
    pub creator: Option<T::AccountId>,
}


#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct TransactionData<T: Config>{
    pub drawdown_id: [u8;32],
    pub created_date: u64,
    pub balance: u32,
    pub documents: BoundedVec<u8, T::MaxDocuments>,
    pub accounting: BoundedVec<u8, T::MaxAccountsPerTransaction>,
}


#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum DrawdownStatus{
    Draft, 
    Submitted,
    Approved,
    Reviewed,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct TransactionSubtype{
    pub account_name: BoundedVec<u8, ConstU32<100>>,
    pub balance: u32,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum DrawdownType{
    EB5, 
    ConstructionLoan,
    DeveloperEquity,
}




impl ProxyRole{
    pub fn to_vec(self) -> Vec<u8>{
        match self{
            //TOREVIEW: optimization (?)
            //Self::Administrator => b"Administrator".to_vec(),
            Self::Administrator => "Administrator".as_bytes().to_vec(),
            Self::Developer => "Developer".as_bytes().to_vec(),
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
        [Administrator.to_vec(), Developer.to_vec(), Investor.to_vec(), Issuer.to_vec(), RegionalCenter.to_vec()].to_vec()
    }

}



/// Extrinsics which require previous authorization to call them
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum ProxyPermission{
    RegisterUser, // users_register_user
    UpdateUser, // users_update_user
    DeleteUser, // users_delete_user
    CreateProject, // projects_create_project
    EditProject, // projects_edit_project
    DeleteProject, // projects_delete_project
    AssignUser, // projects_assign_user
    UnassignUser, // projects_unassign_user
}

impl ProxyPermission{ 
    pub fn to_vec(self) -> Vec<u8>{
        match self{
            Self::RegisterUser => "RegisterUser".as_bytes().to_vec(),
            Self::UpdateUser => "UpdateUser".as_bytes().to_vec(),
            Self::DeleteUser => "DeleteUser".as_bytes().to_vec(),
            Self::CreateProject => "CreateProject".as_bytes().to_vec(),
            Self::EditProject => "EditProject".as_bytes().to_vec(),
            Self::DeleteProject => "DeleteProject".as_bytes().to_vec(),
            Self::AssignUser => "AssignUser".as_bytes().to_vec(),
            Self::UnassignUser => "UnassignUser".as_bytes().to_vec(),

        }
    }

    pub fn id(&self) -> [u8;32]{
        self.to_vec().using_encoded(blake2_256)
    }

    pub fn administrator_permissions() -> Vec<Vec<u8>>{
        use crate::types::ProxyPermission::*;
        //TODO: change it to mut when add new roles
        let administrator_permissions = [
            RegisterUser.to_vec(), 
            UpdateUser.to_vec(),
            DeleteUser.to_vec(),
            CreateProject.to_vec(),
            EditProject.to_vec(),
            DeleteProject.to_vec(),
            AssignUser.to_vec(),
            UnassignUser.to_vec(),
        ].to_vec();
        administrator_permissions
    }

    // pub fn developer_permissions() -> Vec<Vec<u8>>{
    //     //use crate::types::ProxyPermission::*;
    //     let developer_permissions = [
    //     ].to_vec();
    //     developer_permissions
    // }


}