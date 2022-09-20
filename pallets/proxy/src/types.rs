use super::*;
use frame_support::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec;

//TODO: Fix types when using an Option, i.e: Option<CID>
pub type FieldName = BoundedVec<u8,ConstU32<100>>;
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
    pub status: ProjectStatus,
    pub creation_date: u64,
    pub completition_date: u64,
    pub updated_date: u64,
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

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Drawdown<T:Config>{
    pub project_id: [u8;32],
    pub drawdown_number: u32,
    pub drawdown_type: DrawdownType,
    pub total_amount: u32,
    pub status: DrawdownStatus,
    pub open_date: u64,
    pub close_date: u64,
    pub creator: T::AccountId,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Account<T: Config>{
    pub parent_id: T::AccountId,
    pub childrens: BoundedVec<T::AccountId, T::MaxChildrens>,
    pub name: BoundedVec<u8, ConstU32<100>>,
    pub account_type: AccountType,
    pub account_sub_type: AccountSubType,
    pub naics_code: u32,
    pub jobs_multiplier: u32,
    pub account_category: u32,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Transaction<T: Config>{
    pub drawdown_id: [u8;32],
    pub created_date: u64,
    pub balance: u32,
    pub documents: BoundedVec<u8, T::MaxDocuments>,
    pub accounting: BoundedVec<u8, T::MaxAccountsPerTransaction>,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Budgets<T: Config>{
    pub account_id: T::AccountId,
    pub balance: u32,
    pub created_date: u64,
    pub updated_date: u64,
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
pub struct AccountType{
    pub name: BoundedVec<u8, ConstU32<100>>,
    pub class: AccountClass,
    pub account_category: u32,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct TransactionSubtype{
    pub account_name: BoundedVec<u8, ConstU32<100>>,
    pub balance: u32,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, TypeInfo,)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Balance{
    pub amount: u32,
    pub symbol: Symbol,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum DrawdownType{
    EB5, 
    ConstructionLoan,
    DeveloperEquity,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum AccountClass{
    HardCost, 
    SoftCost,
}


#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum ProxyRole{
    Administrator,
    Developer,
    Investor,
    Issuer,
    RegionalCenter,
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


#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum AccountSubType{
    Assets,
    Equity,
    Expenses,
    Income,
    LiabiLiabilities,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum Symbol {
    USD,
}   

