use super::*;
use frame_support::{pallet_prelude::*};
//use frame_system::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec;
use crate::types::*;
use frame_support::traits::{Currency};
use frame_support::traits::ExistenceRequirement::KeepAlive;

impl<T: Config> Pallet<T> {

    pub fn do_create_marketplace(owner: T::AccountId, admin: T::AccountId ,marketplace: Marketplace<T>)->DispatchResult{
        // Gen market id
        let marketplace_id = marketplace.using_encoded(blake2_256);
        // ensure the generated id is unique
        ensure!(!<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceAlreadyExists);
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

    pub fn do_enroll(authority: T::AccountId,marketplace_id: [u8;32], account_or_application: AccountOrApplication<T>, approved: bool, feedback: BoundedVec<u8, T::MaxFeedbackLen>,)->DispatchResult{
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
        Self::change_applicant_status(applicant, marketplace_id, next_status, feedback)?;
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
        Self::can_enroll(authority, marketplace_id)?;
        //update marketplace
        Self::update_label(marketplace_id, new_label)?;
        Self::deposit_event(Event::MarketplaceLabelUpdated(marketplace_id));
        Ok(())
    }


    pub fn do_remove_marketplace(authority: T::AccountId, marketplace_id: [u8;32]) -> DispatchResult {
        //ensure the marketplace exists
        ensure!(<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceNotFound);
        //ensure the origin is owner or admin
        Self::can_enroll(authority, marketplace_id)?;
        //remove marketplace
        Self::remove_selected_marketplace(marketplace_id)?;
        Self::deposit_event(Event::MarketplaceRemoved(marketplace_id));
        Ok(())
    }

    pub fn do_enlist_sell_offer(authority: T::AccountId, marketplace_id: [u8;32], collection_id: T::CollectionId, item_id: T::ItemId, offer_type: OfferType, price: BalanceOf<T>,) -> DispatchResult {
        //TODO: ensure the user is a Marketparticipant

        //ensure the marketplace exists
        ensure!(<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceNotFound);

        //ensure the collection exists
        if let Some(a) = pallet_uniques::Pallet::<T>::owner(collection_id, item_id) {
            ensure!(a == authority, Error::<T>::NotOwner);
        } else {
            Err(Error::<T>::CollectionNotFound)?;
        }

        //Add timestamp to the offer
        let(timestamp, timestamp2) = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        //create an offer_sell_id 
        //TODO: create an offer id generator, used in cases where the offer_id generated is not unique
        let offer_id = (marketplace_id, authority.clone(), collection_id, timestamp, timestamp2).using_encoded(blake2_256);

        //create offer structure 
        let offer_data = OfferData::<T> {
            marketplace_id: marketplace_id,
            collection_id: collection_id,
            item_id: item_id,
            creator: authority.clone(),
            price: price,
            creation_date: timestamp,
            expiration_date: timestamp2,
            status: OfferStatus::Open,
            offer_type: offer_type,
            buyer: None,
        };

        //insert in OffersByItem
        Self::is_item_already_for_sale(collection_id, item_id, marketplace_id)?;

        <OffersByItem<T>>::try_mutate(collection_id, item_id, |offers| {
            offers.try_push(offer_id)
        }).map_err(|_| Error::<T>::OfferStorageError)?;
        //TODO: chnage error messagem, it isn't the right one

        //insert in OffersByAccount
        <OffersByAccount<T>>::try_mutate(authority.clone(), |offer| {
            offer.try_push(offer_id)
        }).map_err(|_| Error::<T>::OfferStorageError)?;

        //insert in OffersInfo
        // validate offer already exists
        ensure!(!<OffersInfo<T>>::contains_key(offer_id), Error::<T>::OfferAlreadyExists);
        <OffersInfo<T>>::insert(offer_id, offer_data);

        //Insert in OffersByMarketplace
        <OffersByMarketplace<T>>::try_mutate(marketplace_id, |offer| {
            offer.try_push(offer_id)
        }).map_err(|_| Error::<T>::OfferStorageError)?;

        Self::deposit_event(Event::OfferStored(collection_id, item_id));
        Ok(())
    }

    pub fn do_take_sell_offer(buyer: T::AccountId, offer_id: [u8;32], marketplace_id: [u8;32], collection_id: T::CollectionId, item_id: T::ItemId,) -> DispatchResult {
        //ensure the collection & owner exists
        let owner_item = pallet_uniques::Pallet::<T>::owner(collection_id, item_id).ok_or(Error::<T>::OwnerNotFound)?;

        //ensure owner is not the same as the buyer
        ensure!(owner_item != buyer.clone(), Error::<T>::CannotTakeOffer); 

        //ensure the selected item has a valid offer_id in OffersInfo
        ensure!(<OffersInfo<T>>::contains_key(offer_id), Error::<T>::OfferNotFound);

        //ensure the offer_id exists in OffersByItem
        Self::does_exist_offer_id_for_this_item(collection_id, item_id, offer_id)?;

        //ensure the offer is open and available
        ensure!(Self::ask_offer_status(offer_id, OfferStatus::Open), Error::<T>::OfferIsNotAvailable);
       
        //Transfer balance to the seller
        let price_item =  Self::get_offer_price(offer_id).map_err(|_| Error::<T>::OfferNotFound)?;
        let total_user_balance = T::LocalCurrency::total_balance(&buyer);
        ensure!(total_user_balance >= price_item, Error::<T>::NotEnoughBalance);
        //Transfer the balance
        T::LocalCurrency::transfer(&buyer, &owner_item, price_item, KeepAlive)?;

        //Use uniques transfer to transfer the item to the buyer
        pallet_uniques::Pallet::<T>::do_transfer(collection_id, item_id, buyer.clone(), |_, _|{
            Ok(())
        })?;

        //TODO: ensure the offer is not expired

        //update offer status from all marketplaces
        Self::update_offer_status(buyer.clone(), collection_id, item_id, marketplace_id)?;

        //remove all SellOrder offer types from OffersByItem so the item it's available to beign sold again
        Self::delete_all_sell_orders_for_this_item(collection_id, item_id )?;
        
        //TODO: add the offer_id from this offer to the buyer's history

        Self::deposit_event(Event::OfferTransferred(offer_id, buyer));
        Ok(())
    }

    pub fn do_duplicate_offer(authority: T::AccountId, offer_id: [u8;32], marketplace_id: [u8;32], collection_id: T::CollectionId, item_id: T::ItemId, modified_price: BalanceOf<T>) -> DispatchResult{
        //ensure new marketplace_id exits
        ensure!(<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceNotFound);

        //ensure that the offer_id exists
        ensure!(<OffersInfo<T>>::contains_key(offer_id), Error::<T>::OfferNotFound);

        //ensure the offer_id exists in OffersByItem
        Self::does_exist_offer_id_for_this_item(collection_id, item_id, offer_id)?;

        //get the offer data
        let mut copy_offer_data = <OffersInfo<T>>::get(offer_id).ok_or(Error::<T>::OfferNotFound)?;

        //modify the offer data
        //by know we only allow to modify its price by the user
        //we modify its marketplace_id because the offer is duplicated to another marketplace
        copy_offer_data.price = modified_price;
        copy_offer_data.marketplace_id = marketplace_id;

        //generate a new offer_id
        let new_offer_id = (marketplace_id, authority.clone(), collection_id, copy_offer_data.creation_date, copy_offer_data.expiration_date).using_encoded(blake2_256);

        //insert in OffersInfo
        // validate new offer_id does not exists
        ensure!(!<OffersInfo<T>>::contains_key(new_offer_id), Error::<T>::OfferAlreadyExists);
        <OffersInfo<T>>::insert(new_offer_id, copy_offer_data);

        //insert in OffersByMarketplace
        <OffersByMarketplace<T>>::try_mutate(marketplace_id, |offer| {
            offer.try_push(new_offer_id)
        }).map_err(|_| Error::<T>::OfferStorageError)?; 
        
        //insert in OffersByAccount
        <OffersByAccount<T>>::try_mutate(authority.clone(), |offer| {
            offer.try_push(new_offer_id)
        }).map_err(|_| Error::<T>::OfferStorageError)?;
        
        //add the new offer_id to OffersByItem
        <OffersByItem<T>>::try_mutate(collection_id, item_id, |offers| {
            offers.try_push(new_offer_id)
        }).map_err(|_| Error::<T>::OfferStorageError)?;

        Self::deposit_event(Event::OfferDuplicated(new_offer_id, marketplace_id));

        Ok(())
    }

    pub fn do_remove_offer(authority: T::AccountId, offer_id: [u8;32], marketplace_id: [u8;32], collection_id: T::CollectionId, item_id: T::ItemId, ) -> DispatchResult {
        //ensure marketplace_id exits
        ensure!(<Marketplaces<T>>::contains_key(marketplace_id), Error::<T>::MarketplaceNotFound);
        
        //ensure the offer_id exists & get the offer data 
        let copy_offer_data = <OffersInfo<T>>::get(offer_id).ok_or(Error::<T>::OfferNotFound)?;

        // ensure the owner is the same as the authority
        ensure!(copy_offer_data.creator == authority.clone(), Error::<T>::CannotRemoveOffer);

        //ensure the offer_id exists in OffersByItem
        Self::does_exist_offer_id_for_this_item(collection_id, item_id, offer_id)?;

        //ensure the offer status is Open
        ensure!(Self::ask_offer_status(offer_id, OfferStatus::Open), Error::<T>::CannotDeleteOffer);


        //remove the offer from OfferInfo
        <OffersInfo<T>>::remove(offer_id);

        //remove the offer from OffersByMarketplace
        <OffersByMarketplace<T>>::try_mutate(marketplace_id, |offers| {
            let offer_index = offers.iter().position(|x| *x == offer_id).ok_or(Error::<T>::OfferNotFound)?;
            offers.remove(offer_index);
            Ok(())
        }).map_err(|_:Error::<T>| Error::<T>::OfferNotFound)?;

        //remove the offer from OffersByAccount
        <OffersByAccount<T>>::try_mutate(authority.clone(), |offers| {
            let offer_index = offers.iter().position(|x| *x == offer_id).ok_or(Error::<T>::OfferNotFound)?;
            offers.remove(offer_index);
            Ok(())
        }).map_err(|_:Error::<T>| Error::<T>::OfferNotFound)?;

        //remove the offer from OffersByItem
        <OffersByItem<T>>::try_mutate(collection_id, item_id, |offers| {
            let offer_index = offers.iter().position(|x| *x == offer_id).ok_or(Error::<T>::OfferNotFound)?;
            offers.remove(offer_index);
            Ok(())
        }).map_err(|_:Error::<T>| Error::<T>::OfferNotFound)?;
    
    
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
        //remove from previous state list
        Self::remove_from_applicants_lists(applicant.clone(),prev_status, marketplace_id)?;

        //insert in current state list
        Self::insert_in_applicants_lists(applicant, next_status,marketplace_id)?;
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
        
        owners.len() == 1
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
        //TODO: evaluate use iter_key_prefix ->instead iter()
        //Before to remove the marketplace, we need to remove all its associated authorities 
        // as well as the applicants/applications.

        //First we need to get the list of all the authorities for the marketplace.
        let _users = <AuthoritiesByMarketplace<T>>::iter_prefix(marketplace_id)
        .map(|(_authority, users)| users).flatten().collect::<Vec<_>>();

        //1. remove from MarketplacesByAuthority
        _users.iter().for_each(|user|{
            <MarketplacesByAuthority<T>>::remove(user, marketplace_id);
        });

        //2. remove from authorities by marketplace list
        <AuthoritiesByMarketplace<T>>::remove_prefix(marketplace_id, None);

        //3. remove from Applications lists
        let mut applications =  Vec::new();

        for ele in <ApplicationsByAccount<T>>::iter() {
            if ele.1 == marketplace_id {
                applications.push(ele.2);
            }
        };

        for application in applications {
            <Applications<T>>::remove(application);
        }

        //4. remove from ApplicationsByAccount list
        <ApplicationsByAccount<T>>::iter().for_each(|(_k1, _k2, _k3)|{
            <ApplicationsByAccount<T>>::remove(_k1, marketplace_id);
        });  

        //5. remove from ApplicantsByMarketplace list
        <ApplicantsByMarketplace<T>>::remove_prefix(marketplace_id, None);

        //6. remove from Custodians list
        <Custodians<T>>::iter().for_each(|(_k1, _k2, _k3)|{
                <Custodians<T>>::remove(_k1, marketplace_id);
        });

        //7. remove from Marketplaces list
        <Marketplaces<T>>::remove(marketplace_id);

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

    fn convert_moment_to_u64_in_milliseconds(date: T::Moment) -> Result<u64, DispatchError> {
        let date_as_u64_millis;
        if let Some(_date_as_u64) = TryInto::<u64>::try_into(date).ok() {
            date_as_u64_millis = _date_as_u64;
        } else {
            return Err(Error::<T>::TimestampError)?;
        }
        return Ok(date_as_u64_millis);
    }


    fn ask_offer_status(offer_id: [u8;32], offer_status: OfferStatus,) -> bool{
        //we already know that the offer exists, so we don't need to check it here.
        //we have added a NotFound status in case the storage source is corrupted.
        if let Some(offer) = <OffersInfo<T>>::get(offer_id) {
            return offer.status == offer_status;
        } else {
            return false;
        }

    }


    fn update_offer_status(buyer: T::AccountId, collection_id: T::CollectionId, item_id: T::ItemId, marketplace_id: [u8;32]) -> DispatchResult{
        let offer_ids = <OffersByItem<T>>::try_get(collection_id, item_id).map_err(|_| Error::<T>::OfferNotFound)?;
        for offer_id in offer_ids {
            <OffersInfo<T>>::try_mutate::<_,_,DispatchError,_>(offer_id, |offer|{
                let offer = offer.as_mut().ok_or(Error::<T>::OfferNotFound)?;
                offer.status = OfferStatus::Closed;
                offer.buyer = Some((buyer.clone(), marketplace_id));
                Ok(())
            })?;

        }
        Ok(())
    }


    fn get_offer_price(offer_id: [u8;32],) -> Result<BalanceOf<T>, DispatchError> {
        //we already know that the offer exists, so we don't need to check it here.
        //we have added a NotFound status in case the storage source is corrupted.
        if let Some(offer) = <OffersInfo<T>>::get(offer_id) {
            return Ok(offer.price);
        } else {
            return Err(Error::<T>::OfferNotFound)?;
        }
    }

    fn get_timestamp_in_milliseconds() -> Option<(u64, u64)> {
        let timestamp: <T as pallet_timestamp::Config>::Moment = <pallet_timestamp::Pallet<T>>::get();
        let timestamp2 = Self::convert_moment_to_u64_in_milliseconds(timestamp).unwrap_or(0);
        let timestamp3 = timestamp2 + (7 * 24 * 60 * 60 * 1000);

        Some((timestamp2, timestamp3))
    }


    fn is_item_already_for_sale(collection_id: T::CollectionId, item_id: T::ItemId, marketplace_id: [u8;32]) -> DispatchResult {
        let offers =  <OffersByItem<T>>::get(collection_id, item_id);

        //if len is == 0, it means that there is no offers for this item, maybe it's the first entry
        if offers.len() == 0 {
            return Ok(())
        } else if offers.len() > 0 {
            for offer in offers {
                let offer_info = <OffersInfo<T>>::get(offer).ok_or(Error::<T>::OfferNotFound)?;
                //ensure the offer_type is SellOrder, because this vector also contains buy offers.
                if offer_info.marketplace_id == marketplace_id && offer_info.offer_type == OfferType::SellOrder {
                    return Err(Error::<T>::OfferAlreadyExists)?;
                }
            }
        } 

        Ok(())

    }

    fn does_exist_offer_id_for_this_item(collection_id: T::CollectionId, item_id: T::ItemId, offer_id: [u8;32]) -> DispatchResult {
        let offers =  <OffersByItem<T>>::try_get(collection_id, item_id).map_err(|_| Error::<T>::OfferNotFound)?;
        //find the offer_id in the vector of offers_ids
        offers.iter().find(|&x| *x == offer_id).ok_or(Error::<T>::OfferNotFound)?;
        Ok(())
    }

    fn delete_all_sell_orders_for_this_item(collection_id: T::CollectionId, item_id: T::ItemId) -> DispatchResult {
        //ensure the item has offers associated with it.
        ensure!(<OffersByItem<T>>::contains_key(collection_id, item_id), Error::<T>::OfferNotFound);
        let offers_ids = <OffersByItem<T>>::take(collection_id, item_id);
        //let mut remaining_offer_ids: Vec<[u8;32]> = Vec::new();
        let mut buy_offer_ids: BoundedVec<[u8;32], T::MaxOffersPerMarket> = BoundedVec::default();

        for offer_id in offers_ids {
            let offer_info = <OffersInfo<T>>::get(offer_id).ok_or(Error::<T>::OfferNotFound)?;
            //ensure the offer_type is SellOrder, because this vector also contains offers of BuyOrder OfferType.
            if offer_info.offer_type != OfferType::SellOrder {
                buy_offer_ids.try_push(offer_id).map_err(|_| Error::<T>::LimitExceeded)?;
            }
        }

        //ensure we already took the entry from the storage map, so we can insert it again.
        ensure!(!<OffersByItem<T>>::contains_key(collection_id, item_id), Error::<T>::OfferNotFound);
        <OffersByItem<T>>::insert(collection_id, item_id, buy_offer_ids);

        Ok(())
    }




}