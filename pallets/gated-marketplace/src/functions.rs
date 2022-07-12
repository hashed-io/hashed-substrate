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


    pub fn do_authorise(authority: T::AccountId, author: T::AccountId, authority_type: MarketplaceAuthority, marketplace_id: [u8;32], ) -> DispatchResult {
        //ensure the origin is owner or admin
        Self::can_enroll(authority, marketplace_id)?;
        //ensure used in case we only accept one role per user per marketplace
        //ensure!(!<MarketplacesByAuthority<T>>::contains_key(author.clone(), marketplace_id,), Error::<T>::AlreadyApplied);
        //outer match prevents users to try to add an owner
        match authority_type{
            MarketplaceAuthority::Owner => {
                Self::owner_exist(marketplace_id)?;
                Self::insert_in_auth_market_lists(author.clone(), authority_type.clone(), marketplace_id)?;
            },
            _ =>{
                //Inner match checks if the user has been added
                match Self::get_author(author.clone(), marketplace_id, authority_type.clone()){
                    Ok(_) => Err(Error::<T>::AlreadyApplied)?,
                    Err(_) => {
                        Self::insert_in_auth_market_lists(author.clone(), authority_type.clone(), marketplace_id)?;
                    }
                }
            }
        }
        Self::deposit_event(Event::AuthorityAdded(author, authority_type));
        Ok(())
    }


    pub fn remove_authorise(authority: T::AccountId, author: T::AccountId, authority_type: MarketplaceAuthority, marketplace_id: [u8;32], ) -> DispatchResult {
        //ensure the origin is owner or admin
        Self::can_enroll(authority.clone(), marketplace_id)?;
        match authority_type{
            MarketplaceAuthority::Owner => {
                Err(Error::<T>::CantRemoveOwner)?
            },
            MarketplaceAuthority::Admin => {
                match Self::is_admin(authority, marketplace_id){
                    Ok(_) => {
                        Err(Error::<T>::NegateRemoveAdminItself)?
                    }
                    Err(_) => {
                        match Self::get_author(author.clone(), marketplace_id, authority_type.clone()){
                            Ok(_) => {
                                Self::remove_rol(author.clone(), authority_type.clone(), marketplace_id)?;
                                Self::deposit_event(Event::AuthorityRemoved(author, authority_type));
                            } 
                            Err(_) => Err(Error::<T>::UserNotFound)?,
                        }
                    }
                }
            },
            _ =>{
                //Inner match checks if the user has been added
                match Self::get_author(author.clone(), marketplace_id, authority_type.clone()){
                    Ok(_) => {
                        Self::remove_rol(author.clone(), authority_type.clone(), marketplace_id)?;
                        Self::deposit_event(Event::AuthorityRemoved(author, authority_type));
                    }
                    Err(_) => Err(Error::<T>::UserNotFound)?,
                }
            }
        }
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
            account_auths.try_push(role.clone())
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


    fn remove_rol(author: T::AccountId, author_type: MarketplaceAuthority , marketplace_id : [u8;32])->DispatchResult{
        <MarketplacesByAuthority<T>>::try_mutate(author.clone(), marketplace_id, |account_auths|{
            let author_index = account_auths.iter().position(|a| *a==author_type.clone())
            .ok_or(Error::<T>::RolNotFoundForUser)?;
            account_auths.remove(author_index);
            Ok(())
        }).map_err(|_:Error::<T>| Error::<T>::UserNotFound)?;

        <AuthoritiesByMarketplace<T>>::try_mutate( marketplace_id, author_type, |account_auths|{
            let author_index = account_auths.iter().position(|a| *a==author.clone())
            .ok_or(Error::<T>::UserNotFound)?;
            account_auths.remove(author_index);
            Ok(())
        }).map_err(|_:Error::<T>| Error::<T>::RolNotFoundForUser)?;

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

    /// Lets us know if the user exists in the selected marketplace
    fn _try_get_author(author: T::AccountId, marketplace_id: [u8;32], author_type: MarketplaceAuthority) -> DispatchResult{
        <MarketplacesByAuthority<T>>::try_mutate::<_,_,_,DispatchError,_>(author, marketplace_id, |authorities_types|{
            let _derp = authorities_types.iter().position(|a| *a==author_type)
                .ok_or(Error::<T>::RolNotFoundForUser)?;
            Ok(())
        })
    }


    ///Lets us know if the selected user is an admin 
    fn is_admin(author: T::AccountId, marketplace_id: [u8;32]) -> DispatchResult{
        let roles = <MarketplacesByAuthority<T>>::try_get(author, marketplace_id)
            .map_err(|_| Error::<T>::UserNotFound)?;

        roles.iter().find(|&role|{
             role.eq(&MarketplaceAuthority::Admin)
        }).ok_or(Error::<T>::UserIsNotAdmin)?;
        Ok(())
    }


    fn get_author(author: T::AccountId, marketplace_id: [u8;32], author_type: MarketplaceAuthority) -> DispatchResult{
        let roles = <MarketplacesByAuthority<T>>::try_get(author, marketplace_id)
            .map_err(|_| Error::<T>::UserNotFoundForThisQuery)?;

        roles.iter().find(|&vector| vector ==&author_type).ok_or(Error::<T>::RolNotFoundForUser)?;

        Ok(())
    }

    fn owner_exist(marketplace_id: [u8;32]) -> DispatchResult{
        let roles = <AuthoritiesByMarketplace<T>>::try_get( marketplace_id, MarketplaceAuthority::Owner)
            .map_err(|_| Error::<T>::UserNotFoundForThisQuery)?;

        if roles.len() > 0{
            Err(Error::<T>::OnlyOneOwnerIsAllowed)? 
        }else{
            Ok(())   
        }
    }

}