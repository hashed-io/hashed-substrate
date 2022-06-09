use super::*;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use crate::types::*;
impl<T: Config> Pallet<T> {

    pub fn do_create_marketplace(marketplace: Marketplace<T>)->DispatchResult{
        // Gen market id
        let marketplace_id = marketplace.using_encoded(blake2_256);
        //Insert on marketplaces and marketplaces by auth
        <Marketplaces<T>>::insert(marketplace_id.clone(), marketplace.clone() );
        Self::insert_in_auth_market_lists(marketplace.owner, MarketplaceAuthority::Owner, marketplace_id.clone())?;
        Self::insert_in_auth_market_lists(marketplace.admin, MarketplaceAuthority::Admin, marketplace_id.clone())?;
        Ok(())
    }

    pub fn do_apply(application : Application<T>)->DispatchResult{
        let app_id = application.using_encoded(blake2_256);
        <Applications<T>>::insert(app_id.clone(), application.clone());
        <ApplicationsByAccount<T>>::insert(application.applicant.clone(), application.marketplace_id.clone(), app_id);
        Self::insert_in_applicants_lists(application.applicant,ApplicationStatus::default(), application.marketplace_id)?;
        Ok(())
    }

    pub fn do_enroll(marketplace_id: [u8;32], account_or_application: AccountOrApplication<T>, approved: bool)->DispatchResult{
        let next_status = match approved{
            true => ApplicationStatus::Approved,
            false => ApplicationStatus::Rejected,
        };
        let applicant = match account_or_application{
            AccountOrApplication::Account(acc)=> acc,
            AccountOrApplication::Application(application_id) => {
                <Applications<T>>::get(application_id).ok_or(Error::<T>::ApplicationNotFound)?.applicant
            },
        };
        
        Self::change_applicant_status(applicant, next_status, marketplace_id)?;
        // TODO: if rejected remove application and files? 
        Ok(())
    }
    /*---- Helper functions ----*/

    fn insert_in_auth_market_lists(authority: T::AccountId, role: MarketplaceAuthority, marketplace_id: [u8;32])->DispatchResult{
        <MarketplacesByAuthority<T>>::try_mutate(authority.clone(), role.clone(), |auth_markets|{
            auth_markets.try_push(marketplace_id.clone())
        }).map_err(|_| Error::<T>::ExceedMaxMarketsPerAuth)?;

        <AuthoritiesByMarketplace<T>>::try_mutate(marketplace_id, role, | accounts|{
            accounts.try_push(authority)
        }).map_err(|_| Error::<T>::ExceedMaxMarketsPerAuth)?;
        Ok(())
    }

    fn insert_in_applicants_lists(applicant: T::AccountId, status: ApplicationStatus , marketplace_id : [u8;32])->DispatchResult{
        //TODO: remove previous entry on pending/rejected? another function?
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

    fn change_applicant_status(applicant: T::AccountId, next_status: ApplicationStatus , marketplace_id : [u8;32])->DispatchResult{
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
}