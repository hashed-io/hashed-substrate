use super::*;
use frame_support::pallet_prelude::*;
//use frame_system::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec;
use crate::types::*;

impl<T: Config> Pallet<T> {

    pub fn do_create_marketplace(owner: T::AccountId, admin: T::AccountId ,marketplace: Marketplace<T>)->DispatchResult{
        // Gen market id
        let marketplace_id = marketplace.using_encoded(blake2_256);
        // ensure the generated id is unique
        ensure!(!<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceAlreadyExists );
        //Insert on marketplaces and marketplaces by auth
        Self::insert_in_auth_market_lists(owner.clone(), MarketplaceAuthority::Owner, marketplace_id)?;
        Self::insert_in_auth_market_lists(admin.clone(), MarketplaceAuthority::Admin, marketplace_id)?;
        <Marketplaces<T>>::insert(marketplace_id, marketplace);

        Self::deposit_event(Event::MarketplaceStored(owner, admin, marketplace_id));
        Ok(())
    }

    pub fn do_apply(applicant: T::AccountId, custodian: Option<T::AccountId>, marketplace_id: [u8;32], application : Application<T>)->DispatchResult{
        // marketplace exists?
        ensure!(<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceNotFound);
        // The user only can apply once by marketplace
        ensure!(!<ApplicationsByAccount<T>>::contains_key(applicant.clone(), marketplace_id), Error::<T>::AlreadyApplied);

        let app_id = application.using_encoded(blake2_256);
        Self::insert_in_applicants_lists(applicant.clone(),ApplicationStatus::default(), marketplace_id)?;
        <ApplicationsByAccount<T>>::insert(applicant, marketplace_id, app_id);
        <Applications<T>>::insert(app_id, application);

        if let Some(c) = custodian{
            Self::insert_custodian(c, marketplace_id, app_id)?;
        }
        Self::deposit_event(Event::ApplicationStored(app_id, marketplace_id));
        Ok(())
    }

    pub fn do_enroll(authority: T::AccountId,marketplace_id: [u8;32], account_or_application: AccountOrApplication<T>, approved: bool)->DispatchResult{
        // ensure the origin is owner or admin
        Self::can_enroll(authority, marketplace_id)?;
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
        Self::change_applicant_status(applicant, marketplace_id, next_status.clone())?;
        // TODO: if rejected remove application and files? 
        Self::deposit_event(Event::ApplicationProcessed(account_or_application, marketplace_id, next_status));
        Ok(())
    }


    pub fn do_authority(authority: T::AccountId, account: T::AccountId, authority_type: MarketplaceAuthority, marketplace_id: [u8;32], ) -> DispatchResult {
        //ensure the origin is owner or admin
        //TODO: implement copy trait for MarketplaceAuthority & T::AccountId
        Self::can_enroll(authority, marketplace_id)?;

        //ensure the account is not already an authority
        ensure!(!Self::does_exist_authority(account.clone(), marketplace_id, authority_type), Error::<T>::AlreadyApplied);

        match authority_type{
            MarketplaceAuthority::Owner => {
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


    pub fn do_remove_authority(authority: T::AccountId, account: T::AccountId, authority_type: MarketplaceAuthority, marketplace_id: [u8;32], ) -> DispatchResult {
        //ensure the origin is owner or admin
        Self::can_enroll(authority.clone(), marketplace_id)?;

        //ensure the account has the selected authority before to try to remove
        ensure!(Self::does_exist_authority(account.clone(), marketplace_id, authority_type), Error::<T>::AuthorityNotFoundForUser);

        match authority_type{
            MarketplaceAuthority::Owner => {
                ensure!(Self::owner_exist(marketplace_id), Error::<T>::OwnerNotFound);
                Err(Error::<T>::CantRemoveOwner)?;
            },
            MarketplaceAuthority::Admin => {
                // Admins can not delete themselves
                ensure!(authority != account, Error::<T>::NegateRemoveAdminItself);

                // Admis cannot be deleted between them, only the owner can
                ensure!(!Self::is_admin(authority.clone(), marketplace_id), Error::<T>::CannotDeleteAdmin);
        
                Self::remove_from_market_lists(account.clone(), authority_type, marketplace_id)?;
            },
            _ =>{

                Self::remove_from_market_lists(account.clone(), authority_type, marketplace_id)?;

            }
        }
        
        Self::deposit_event(Event::AuthorityRemoved(account, authority_type));
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

    fn insert_in_auth_market_lists(authority: T::AccountId, role: MarketplaceAuthority, marketplace_id: [u8;32])->DispatchResult{

        <MarketplacesByAuthority<T>>::try_mutate(authority.clone(), marketplace_id, |account_auths|{
            account_auths.try_push(role)
        }).map_err(|_| Error::<T>::ExceedMaxRolesPerAuth)?;

        <AuthoritiesByMarketplace<T>>::try_mutate(marketplace_id, role, | accounts|{
            accounts.try_push(authority)
        }).map_err(|_| Error::<T>::ExceedMaxMarketsPerAuth)?;
        Ok(())
    }

    fn insert_in_applicants_lists(applicant: T::AccountId, status: ApplicationStatus , marketplace_id : [u8;32])->DispatchResult{
        <ApplicantsByMarketplace<T>>::try_mutate(marketplace_id, status,|applicants|{
            applicants.try_push(applicant)
        }).map_err(|_| Error::<T>::ExceedMaxApplicants)?;
        Ok(())
    }

    fn insert_custodian(custodian: T::AccountId, marketplace_id : [u8;32], application_id: [u8;32])-> DispatchResult{
        <Custodians<T>>::try_mutate(custodian, marketplace_id, | applications |{
            applications.try_push(application_id)
        }).map_err(|_| Error::<T>::ExceedMaxApplicants)?;
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


    fn remove_from_market_lists(account: T::AccountId, author_type: MarketplaceAuthority , marketplace_id : [u8;32])->DispatchResult{
        <MarketplacesByAuthority<T>>::try_mutate(account.clone(), marketplace_id, |account_auths|{
            let author_index = account_auths.iter().position(|a| *a==author_type)
            .ok_or(Error::<T>::UserNotFound)?;
            account_auths.remove(author_index);
            Ok(())
        }).map_err(|_:Error::<T>| Error::<T>::UserNotFound)?;

        <AuthoritiesByMarketplace<T>>::try_mutate( marketplace_id, author_type, |accounts|{
            let author_index = accounts.iter().position(|a| *a==account.clone())
            .ok_or(Error::<T>::UserNotFound)?;
            accounts.remove(author_index);
            Ok(())
        }).map_err(|_:Error::<T>| Error::<T>::UserNotFound)?;

        Ok(())

    }


    fn change_applicant_status(applicant: T::AccountId , marketplace_id : [u8;32], next_status: ApplicationStatus)->DispatchResult{
        let mut prev_status = ApplicationStatus::default();
        let app_id = <ApplicationsByAccount<T>>::get(applicant.clone(), marketplace_id)
            .ok_or(Error::<T>::ApplicationNotFound)?;
        <Applications<T>>::try_mutate::<_,_,DispatchError,_>(app_id, | application|{
            application.as_ref().ok_or( Error::<T>::ApplicationNotFound)?;
            if let Some(a) = application{
                prev_status.clone_from(&a.status);
                a.status.clone_from(&next_status)
            }
            Ok(())
        })?;
        //remove from previous state list
        Self::remove_from_applicants_lists(applicant.clone(),prev_status, marketplace_id)?;

        //insert in current state list
        Self::insert_in_applicants_lists(applicant, next_status,marketplace_id )?;
        Ok(())
    }

    fn can_enroll( authority: T::AccountId, marketplace_id: [u8;32] ) -> DispatchResult{
        // to enroll, the account needs to be an owner or an admin
        let roles = <MarketplacesByAuthority<T>>::try_get(authority, marketplace_id)
            .map_err(|_| Error::<T>::CannotEnroll)?;
        // iter().any could be called too but this maps directly to desired error
        roles.iter().find(|&role|{
            role.eq(&MarketplaceAuthority::Owner) || role.eq(&MarketplaceAuthority::Admin)
        }).ok_or(Error::<T>::CannotEnroll)?;
        Ok(())
    }


    ///Lets us know if the selected user is an admin.
    /// It returns true if the user is an admin, false otherwise.
    fn is_admin(account: T::AccountId, marketplace_id: [u8;32]) -> bool{
        let roles = match <MarketplacesByAuthority<T>>::try_get(account, marketplace_id){
            Ok(roles) => roles,
            Err(_) => return false,
        };

        roles.iter().any(|&authority_type| authority_type == MarketplaceAuthority::Admin)
    }


    /// Let us know if the selected account has the selected authority type. 
    /// It returns true if the account has the authority type, false otherwise
    fn  does_exist_authority(account: T::AccountId, marketplace_id: [u8;32], authority_type: MarketplaceAuthority) -> bool{
        let roles = match <MarketplacesByAuthority<T>>::try_get(account, marketplace_id){
            Ok(roles) => roles,
            Err(_) => return false,
        };

        roles.iter().any(|authority| authority == &authority_type)
    }

    /// Let us know if there's an owner for the selected marketplace. 
    /// It returns true if there's an owner, false otherwise
    fn owner_exist(marketplace_id: [u8;32]) -> bool {
        let owners =  match <AuthoritiesByMarketplace<T>>::try_get( marketplace_id, MarketplaceAuthority::Owner){
            Ok(owners) => owners,
            Err(_) => return false,
        };

        //TODO: try to change to -> owners.len() == 1 because we only have one owner
        owners.len() == 1
    }

}