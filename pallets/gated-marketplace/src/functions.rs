use super::*;
use frame_support::pallet_prelude::*;
//use frame_system::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use crate::types::*;

impl<T: Config> Pallet<T> {

    pub fn do_create_marketplace(owner: T::AccountId, admin: T::AccountId ,marketplace: Marketplace<T>)->DispatchResult{
        // Gen market id
        let marketplace_id = marketplace.using_encoded(blake2_256);
        // ensure the generated id is unique
        ensure!(!<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceAlreadyExists );
        //Insert on marketplaces and marketplaces by auth
        Self::insert_in_auth_market_lists(owner.clone(), MarketplaceAuthority::Owner, marketplace_id.clone())?;
        Self::insert_in_auth_market_lists(admin.clone(), MarketplaceAuthority::Admin, marketplace_id.clone())?;
        <Marketplaces<T>>::insert(marketplace_id.clone(), marketplace.clone() );

        Self::deposit_event(Event::MarketplaceStored(owner, admin, marketplace_id));
        Ok(())
    }

    pub fn do_apply(applicant: T::AccountId, marketplace_id: [u8;32], application : Application<T>)->DispatchResult{
        // marketplace exists?
        ensure!(<Marketplaces<T>>::contains_key(marketplace_id.clone() ), Error::<T>::MarketplaceNotFound);
        // The user only can apply once by marketplace
        ensure!(!<ApplicationsByAccount<T>>::contains_key(applicant.clone(), marketplace_id.clone() ), Error::<T>::AlreadyApplied);

        let app_id = application.using_encoded(blake2_256);
        Self::insert_in_applicants_lists(applicant.clone(),ApplicationStatus::default(), marketplace_id)?;
        <ApplicationsByAccount<T>>::insert(applicant.clone(), marketplace_id.clone(), app_id);
        <Applications<T>>::insert(app_id.clone(), application.clone());

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
    /*---- Helper functions ----*/

    fn insert_in_auth_market_lists(authority: T::AccountId, role: MarketplaceAuthority, marketplace_id: [u8;32])->DispatchResult{
        <MarketplacesByAuthority<T>>::try_mutate(authority.clone(), marketplace_id.clone(), |account_auths|{
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

    fn remove_from_applicants_lists(applicant: T::AccountId, status: ApplicationStatus , marketplace_id : [u8;32])->DispatchResult{
        <ApplicantsByMarketplace<T>>::try_mutate::<_,_,_,DispatchError,_>(marketplace_id, status, |applicants|{
            let applicant_index = applicants.iter().position(|a| *a==applicant.clone())
                .ok_or(Error::<T>::ApplicantNotFound)?;
            applicants.remove(applicant_index);
            Ok(())
        })
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
        Self::remove_from_applicants_lists(applicant.clone(),prev_status, marketplace_id.clone())?;

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
}