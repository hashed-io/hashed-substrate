use super::*;
use frame_support::{pallet_prelude::*};
//use frame_system::pallet_prelude::*;
//use frame_support::sp_io::hashing::blake2_256;


use crate::types::*;

impl<T: Config> Pallet<T> {

    pub fn do_create_project(
        admin: T::AccountId, 
        tittle: BoundedVec<u8, T::ProjectNameMaxLen>,
        description: BoundedVec<u8, T::ProjectDescMaxLen>,
        image: BoundedVec<u8, T::CIDMaxLen>,
        developer: Option<T::AccountId>,
        builder: Option<T::AccountId>,
        issuer: Option<T::AccountId>,
        regional_center: Option<T::AccountId>,
     ) -> DispatchResult {
        //TODO: admin only 

        Ok(())
    }






}