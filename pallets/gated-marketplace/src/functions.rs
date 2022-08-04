use core::default;

use super::*;
use frame_support::pallet_prelude::*;
//use frame_system::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec;
use crate::types::*;
use pallet_rbac::types::*;


impl<T: Config> Pallet<T> {

    pub fn do_initial_setup()->DispatchResult{
        let pallet_id = Self::pallet_id();
        let mut super_roles = Vec::<Vec<u8>>::new();
        super_roles.push(MarketplaceRole::Owner.to_vec());
        super_roles.push(MarketplaceRole::Admin.to_vec());
        let super_role_ids = T::Rbac::create_and_set_roles(pallet_id, super_roles)?;
        for super_role in super_role_ids{
            T::Rbac::create_and_set_permissions(pallet_id, super_role, Permission::admin_permissions())?;
        }
        // participant role and permissions
        let participant_role_id = T::Rbac::create_and_set_roles(pallet_id, [MarketplaceRole::Participant.to_vec()].to_vec())?;
        T::Rbac::create_and_set_permissions(pallet_id, participant_role_id[0],["buy".as_bytes().to_vec(),"sell".as_bytes().to_vec()].to_vec() )?;

        Ok(())
    }

    pub fn do_create_marketplace(owner: T::AccountId, admin: T::AccountId ,marketplace: Marketplace<T>)->DispatchResult{
        // Gen market id
        let marketplace_id = marketplace.using_encoded(blake2_256);
        // ensure the generated id is unique
        ensure!(!<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceAlreadyExists);
        //Insert on marketplaces and marketplaces by auth
        T::Rbac::create_scope(Self::pallet_id(),marketplace_id.clone())?;
        Self::insert_in_auth_market_lists(owner.clone(), MarketplaceRole::Owner, marketplace_id)?;
        Self::insert_in_auth_market_lists(admin.clone(), MarketplaceRole::Admin, marketplace_id)?;
        <Marketplaces<T>>::insert(marketplace_id, marketplace);
        

        Self::deposit_event(Event::MarketplaceStored(owner, admin, marketplace_id));
        Ok(())
    }

    pub fn do_apply(applicant: T::AccountId, custodian: Option<T::AccountId>, marketplace_id: [u8;32], application : Application<T>)->DispatchResult{
        // marketplace exists?
        ensure!(<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceNotFound);
        // The user only can apply once by marketplace
        ensure!(!<ApplicationsByAccount<T>>::contains_key(applicant.clone(), marketplace_id), Error::<T>::AlreadyApplied);
        // Generate application Id
        let app_id = (marketplace_id, applicant.clone(), application.clone()).using_encoded(blake2_256);
        // Ensure another identical application doesnt exists
        ensure!(!<Applications<T>>::contains_key(app_id), Error::<T>::AlreadyApplied);

        if let Some(c) = custodian{
            // Ensure applicant and custodian arent the same
            ensure!(applicant.ne(&c),Error::<T>::ApplicantCannotBeCustodian);
            Self::insert_custodian(c, marketplace_id, applicant.clone())?;
        }

        Self::insert_in_applicants_lists(applicant.clone(),ApplicationStatus::default(), marketplace_id)?;
        <ApplicationsByAccount<T>>::insert(applicant, marketplace_id, app_id);
        <Applications<T>>::insert(app_id, application);

        Self::deposit_event(Event::ApplicationStored(app_id, marketplace_id));
        Ok(())
    }

    pub fn do_enroll(authority: T::AccountId, marketplace_id: [u8;32], account_or_application: AccountOrApplication<T>, approved: bool, feedback: BoundedVec<u8, T::MaxFeedbackLen>,)->DispatchResult{
        // ensure the origin is owner or admin
        //Self::can_enroll(authority, marketplace_id)?;
        Self::is_authorized(authority.clone(), &marketplace_id,Permission::Enroll)?;
        let next_status = match approved{
            true => ApplicationStatus::Approved,
            false => ApplicationStatus::Rejected,
        };
        let applicant = match account_or_application.clone() {
            AccountOrApplication::Account(acc)=> acc,
            AccountOrApplication::Application(application_id) => {
                <ApplicationsByAccount<T>>::iter().find_map(|(acc,m_id,app_id)|{
                    if  m_id == marketplace_id && app_id == application_id{
                        return Some(acc)
                    }
                    None
                }).ok_or(Error::<T>::ApplicationNotFound)?
            },
        };
        Self::change_applicant_status(applicant, marketplace_id, next_status, feedback)?;

        Self::deposit_event(Event::ApplicationProcessed(account_or_application, marketplace_id, next_status));
        Ok(())
    }


    pub fn do_authority(authority: T::AccountId, account: T::AccountId, authority_type: MarketplaceRole, marketplace_id: [u8;32], ) -> DispatchResult {
        //ensure the origin is owner or admin
        //TODO: implement copy trait for MarketplaceAuthority & T::AccountId
        //Self::can_enroll(authority, marketplace_id)?;
        Self::is_authorized(authority.clone(), &marketplace_id,Permission::AddAuth)?;
        //ensure the account is not already an authority
        // handled by T::Rbac::assign_role_to_user
        //ensure!(!Self::does_exist_authority(account.clone(), marketplace_id, authority_type), Error::<T>::AlreadyApplied);
        match authority_type{
            MarketplaceRole::Owner => {
                ensure!(!Self::owner_exist(marketplace_id), Error::<T>::OnlyOneOwnerIsAllowed);
                Self::insert_in_auth_market_lists(account.clone(), authority_type, marketplace_id)?;

            },
            _ =>{
                Self::insert_in_auth_market_lists(account.clone(), authority_type, marketplace_id)?;
            }
        }

        Self::deposit_event(Event::AuthorityAdded(account, authority_type));
        Ok(())
    }


    pub fn do_remove_authority(authority: T::AccountId, account: T::AccountId, authority_type: MarketplaceRole, marketplace_id: [u8;32], ) -> DispatchResult {
        //ensure the origin is owner or admin
        //Self::can_enroll(authority.clone(), marketplace_id)?;
        Self::is_authorized(authority.clone(), &marketplace_id,Permission::RemoveAuth)?;
        //ensure the account has the selected authority before to try to remove
        // T::Rbac handles the if role doesnt hasnt been asigned to the user
        //ensure!(Self::does_exist_authority(account.clone(), marketplace_id, authority_type), Error::<T>::AuthorityNotFoundForUser);

        match authority_type{
            MarketplaceRole::Owner => {
                ensure!(Self::owner_exist(marketplace_id), Error::<T>::OwnerNotFound);
                Err(Error::<T>::CantRemoveOwner)?;
            },
            MarketplaceRole::Admin => {
                // Admins can not delete themselves
                ensure!(authority != account, Error::<T>::AdminCannotRemoveItself);

                // Admis cannot be deleted between them, only the owner can
                ensure!(!Self::is_admin(authority, marketplace_id), Error::<T>::CannotDeleteAdmin);
        
                Self::remove_from_market_lists(account.clone(), authority_type, marketplace_id)?;
            },
            _ =>{

                Self::remove_from_market_lists(account.clone(), authority_type, marketplace_id)?;

            }
        }
        
        Self::deposit_event(Event::AuthorityRemoved(account, authority_type));
        Ok(())
    }


    pub fn do_update_label_marketplace(authority: T::AccountId, marketplace_id: [u8;32], new_label: BoundedVec<u8,T::LabelMaxLen>) -> DispatchResult {
        //ensure the marketplace exists
        ensure!(<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceNotFound);
        //ensure the origin is owner or admin
        //Self::can_enroll(authority, marketplace_id)?;
        Self::is_authorized(authority, &marketplace_id, Permission::UpdateLabel)?;
        //update marketplace
        Self::update_label(marketplace_id, new_label)?;
        Self::deposit_event(Event::MarketplaceLabelUpdated(marketplace_id));
        Ok(())
    }


    pub fn do_remove_marketplace(authority: T::AccountId, marketplace_id: [u8;32]) -> DispatchResult {
        //ensure the marketplace exists
        ensure!(<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceNotFound);
        //ensure the origin is owner or admin
        //Self::can_enroll(authority, marketplace_id)?;
        Self::is_authorized(authority, &marketplace_id, Permission::RemoveMarketplace)?;
        //remove marketplace
        Self::remove_selected_marketplace(marketplace_id)?;
        Self::deposit_event(Event::MarketplaceRemoved(marketplace_id));
        Ok(())
    }




    /*---- Helper functions ----*/

    pub fn set_up_application(
        fields : BoundedVec<(BoundedVec<u8,ConstU32<100> >,BoundedVec<u8,ConstU32<100>> ), T::MaxFiles>,
        custodian_fields: Option<(T::AccountId, BoundedVec<BoundedVec<u8,ConstU32<100>>, T::MaxFiles> )> 
    )-> (Option<T::AccountId>, BoundedVec<ApplicationField, T::MaxFiles> ){
        let mut f: Vec<ApplicationField>= fields.iter().map(|tuple|{
            ApplicationField{
                display_name: tuple.0.clone(), cid: tuple.1.clone(), custodian_cid: None,
            }
        }).collect();
        let custodian = match custodian_fields{
            Some(c_fields)=>{
                for i in 0..f.len(){
                    f[i].custodian_cid = Some(c_fields.1[i].clone());
                }

                Some(c_fields.0)
            },
            _ => None,
        };
        (custodian, BoundedVec::<ApplicationField, T::MaxFiles>::try_from(f).unwrap_or_default() )
    }

    fn insert_in_auth_market_lists(authority: T::AccountId, role: MarketplaceRole, marketplace_id: [u8;32])->DispatchResult{

        T::Rbac::assign_role_to_user(authority.clone(), Self::pallet_id(),
             &marketplace_id, role.id())?;
        
        Ok(())
    }

    fn insert_in_applicants_lists(applicant: T::AccountId, status: ApplicationStatus , marketplace_id : [u8;32])->DispatchResult{
        <ApplicantsByMarketplace<T>>::try_mutate(marketplace_id, status,|applicants|{
            applicants.try_push(applicant)
        }).map_err(|_| Error::<T>::ExceedMaxApplicants)?;

        Ok(())
    }

    fn insert_custodian(custodian: T::AccountId, marketplace_id : [u8;32], applicant: T::AccountId)-> DispatchResult{
        <Custodians<T>>::try_mutate(custodian, marketplace_id, | applications |{
            applications.try_push(applicant)
        }).map_err(|_| Error::<T>::ExceedMaxApplicationsPerCustodian)?;

        Ok(())
    }

    fn remove_from_applicants_lists(applicant: T::AccountId, status: ApplicationStatus , marketplace_id : [u8;32])->DispatchResult{
        <ApplicantsByMarketplace<T>>::try_mutate::<_,_,_,DispatchError,_>(marketplace_id, status, |applicants|{
            let applicant_index = applicants.iter().position(|a| *a==applicant.clone())
                .ok_or(Error::<T>::ApplicantNotFound)?;
            applicants.remove(applicant_index);

            Ok(())
        })
    }


    fn remove_from_market_lists(account: T::AccountId, author_type: MarketplaceRole , marketplace_id : [u8;32])->DispatchResult{

        T::Rbac::remove_role_from_user(account, Self::pallet_id(), 
            &marketplace_id, author_type.id())?;
        Ok(())

    }


    fn change_applicant_status(applicant: T::AccountId , marketplace_id : [u8;32], next_status: ApplicationStatus, feedback: BoundedVec::<u8, T::MaxFeedbackLen>, )->DispatchResult{
        let mut prev_status = ApplicationStatus::default();
        let app_id = <ApplicationsByAccount<T>>::get(applicant.clone(), marketplace_id)
            .ok_or(Error::<T>::ApplicationNotFound)?;
        <Applications<T>>::try_mutate::<_,_,DispatchError,_>(app_id, | application|{
            application.as_ref().ok_or( Error::<T>::ApplicationNotFound)?;
            if let Some(a) = application{
                prev_status.clone_from(&a.status);
                a.feedback = feedback;
                a.status.clone_from(&next_status)
            }
            Ok(())
        })?;
        ensure!(prev_status != next_status, Error::<T>::AlreadyEnrolled);
        //remove from previous state list
        Self::remove_from_applicants_lists(applicant.clone(),prev_status, marketplace_id)?;

        //insert in current state list
        Self::insert_in_applicants_lists(applicant.clone(), next_status,marketplace_id)?;

        if prev_status == ApplicationStatus::Approved{
            T::Rbac::remove_role_from_user(applicant.clone(), Self::pallet_id(), &marketplace_id, MarketplaceRole::Participant.id())?;
        }
        if next_status == ApplicationStatus::Approved{
            T::Rbac::assign_role_to_user(applicant, Self::pallet_id(), &marketplace_id, MarketplaceRole::Participant.id())?
        }

        Ok(())
    }

    fn is_authorized( authority: T::AccountId, marketplace_id: &[u8;32], permission: Permission ) -> DispatchResult{
        T::Rbac::is_authorized(
            authority,
            Self::pallet_id(), 
            &marketplace_id,
            &permission.id(),
        )
    }


    ///Lets us know if the selected user is an admin.
    /// It returns true if the user is an admin, false otherwise.
    fn is_admin(account: T::AccountId, marketplace_id: [u8;32]) -> bool{
        T::Rbac::has_role(account, Self::pallet_id(), 
            &marketplace_id, [MarketplaceRole::Admin.id()].to_vec()).is_ok()
    }


    /// Let us know if the selected account has the selected authority type. 
    /// It returns true if the account has the authority type, false otherwise
    // fn  does_exist_authority(account: T::AccountId, marketplace_id: [u8;32], authority_type: MarketplaceRole) -> bool{
    //     let roles = match <MarketplacesByAuthority<T>>::try_get(account, marketplace_id){
    //         Ok(roles) => roles,
    //         Err(_) => return false,
    //     };

    //     roles.iter().any(|authority| authority == &authority_type)
    // }

    /// Let us know if there's an owner for the selected marketplace. 
    /// It returns true if there's an owner, false otherwise
    fn owner_exist(marketplace_id: [u8;32]) -> bool {
        // let owners =  match <AuthoritiesByMarketplace<T>>::try_get( marketplace_id, MarketplaceAuthority::Owner){
        //     Ok(owners) => owners,
        //     Err(_) => return false,
        // };
        
        //owners.len() == 1 
        T::Rbac::get_role_users_len(Self::pallet_id(), 
            &marketplace_id, &MarketplaceRole::Owner.id()) == 1
    }

    /// Let us update the marketplace's label.
    /// It returns ok if the update was successful, error otherwise.
    fn  update_label(marketplace_id : [u8;32], new_label: BoundedVec<u8,T::LabelMaxLen>) -> DispatchResult {     
        <Marketplaces<T>>::try_mutate(marketplace_id, |marketplace|{
        let market = marketplace.as_mut().ok_or(Error::<T>::MarketplaceNotFound)?;
        market.label = new_label;
        Ok(())
        })
    }

    /// Let us delete the selected marketplace 
    /// and remove all of its associated authorities from all the storage sources.
    /// If returns ok if the deletion was successful, error otherwise.
    /// Errors only could happen if the storage sources are corrupted.
    fn remove_selected_marketplace(marketplace_id: [u8;32]) -> DispatchResult {
        //Before to remove the marketplace, we need to remove all its associated authorities 
        // as well as the applicants/applications.

        //First we need to get the list of all the authorities for the marketplace.
        let mut applications =  Vec::new();
        
        // remove from Applications lists
        for ele in <ApplicationsByAccount<T>>::iter() {
            if ele.1 == marketplace_id {
                applications.push(ele.2);
            }
        };

        for application in applications {
            <Applications<T>>::remove(application);
        }

        // remove from ApplicationsByAccount list
        <ApplicationsByAccount<T>>::iter().for_each(|(_k1, _k2, _k3)|{
            <ApplicationsByAccount<T>>::remove(_k1, marketplace_id);
        });  

        // remove from ApplicantsByMarketplace list
        <ApplicantsByMarketplace<T>>::remove_prefix(marketplace_id, None);

        // remove from Custodians list
        <Custodians<T>>::iter().for_each(|(_k1, _k2, _k3)|{
                <Custodians<T>>::remove(_k1, marketplace_id);
        });

        // remove from Marketplaces list
        <Marketplaces<T>>::remove(marketplace_id);

        T::Rbac::remove_scope(Self::pallet_id(), marketplace_id)?;

        Ok(())
    }


    /// Let us check the curent status of the selected application.
    /// If the status is rejected, we can safely remove its data from the storage sources
    /// so the user can apply again. 
    /// It doesn't affect any other storage source/workflow.
    pub fn is_application_in_rejected_status(account: T::AccountId, marketplace_id: [u8;32]) -> DispatchResult{
        let application_id = <ApplicationsByAccount<T>>::try_get(account.clone(), marketplace_id)
            .map_err(|_| Error::<T>::ApplicationIdNotFound)?;
        
        let application = <Applications<T>>::try_get(application_id)
            .map_err(|_| Error::<T>::ApplicationNotFound)?;

        match application.status {
            ApplicationStatus::Pending => Err(Error::<T>::ApplicationStatusStillPending)?, 
            ApplicationStatus::Approved => Err(Error::<T>::ApplicationHasAlreadyBeenApproved)?,
            ApplicationStatus::Rejected => {
                //If status is Rejected, we need to delete the previous application from all the storage sources.
                <Applications<T>>::remove(application_id);
                <ApplicationsByAccount<T>>::remove(account, marketplace_id);
                <ApplicantsByMarketplace<T>>::remove(marketplace_id, ApplicationStatus::Rejected);
            }
        }
        Ok(())
    }

    pub fn pallet_id()->u64{
        Self::index().try_into().unwrap()
    }

}