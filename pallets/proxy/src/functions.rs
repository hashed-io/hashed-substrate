use super::*;
use frame_support::{pallet_prelude::*};
use frame_support::traits::Time;
use frame_support::sp_io::hashing::blake2_256;

//use frame_system::pallet_prelude::*;



use crate::types::*;

impl<T: Config> Pallet<T> {

    pub fn do_create_project(
        admin: T::AccountId, 
        tittle: BoundedVec<u8, T::ProjectNameMaxLen>,
        description: BoundedVec<u8, T::ProjectDescMaxLen>,
        image: BoundedVec<u8, T::CIDMaxLen>,
        completition_date: u64,
        developer: Option<T::AccountId>,
        builder: Option<T::AccountId>,
        issuer: Option<T::AccountId>,
        regional_center: Option<T::AccountId>,
     ) -> DispatchResult {
        //TODO: admin only 

        //Add timestamp 
        let timestamp = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

        //Create project_id
        //TOREVIEW: We could use only name as project_id or use a method/storagemap to check if the name is already in use
        let project_id = (tittle.clone()).using_encoded(blake2_256);

        //ensure completition date is in the future
        ensure!(completition_date > timestamp, Error::<T>::CompletitionDateMustBeLater);
        
        //Create project data
        let project_data = Project{
            tittle,
            description,
            image,
            developer,
            builder,
            issuer,
            regional_center,
            creation_date: timestamp,
            completition_date,
            updated_date: timestamp,
        };

        //Insert project data
        // ensure that the project_id is not already in use
        ensure!(!Projects::<T>::contains_key(project_id), Error::<T>::ProjectIdAlreadyInUse);
        Projects::<T>::insert(project_id, project_data);

        // Emit event
        Self::deposit_event(Event::ProjectCreated(admin, project_id));

        Ok(())
    }

    // H E L P E R S
    // --------------------------------------------------------------------------------------------
    fn get_timestamp_in_milliseconds() -> Option<(u64)> {
        let timestamp:u64 = T::Timestamp::now().into();

        Some(timestamp)
    }



}