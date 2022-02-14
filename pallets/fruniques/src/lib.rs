#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
use super::*;
	use frame_support::{pallet_prelude::*, BoundedVec};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::StaticLookup;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config + pallet_uniques::Config {

		type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;

	}


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T, I = ()>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		FruniqueCreated(T::AccountId, T::AccountId, T::ClassId, T::InstanceId),
		// A frunique/unique was succesfully divided!
		FruniqueDivided(T::AccountId, T::AccountId, T::ClassId, T::InstanceId),
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
		NoneValue,
		NoPermission,
		StorageOverflow,
		NotYetImplemented,
		FruniqueCntOverflow,
	}

	#[pallet::storage]
	#[pallet::getter(fn frunique_cnt)]
	/// Keeps track of the number of Kitties in existence.
	pub(super) type FruniqueCnt<T, I = ()> = StorageValue<_, u32, ValueQuery>;

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// Issue a new frunique from a public origin.
		///
		/// A new NFT (unique) is created and reserved,
		/// a fungible token (asset) is created and minted to the owner.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// `AssetDeposit` funds of sender are reserved.
		///
		/// Parameters:
		/// - `asset_id`: The identifier of the new asset. This must not be currently in use to identify
		/// an existing asset.
		/// - `class`: The identifier of the new asset class. This must not be currently in use.
		/// - `admin`: The admin of this class of assets. The admin is the initial address of each
		/// member of the asset class's admin team.
		///
		/// Emits `FruniqueCreated` event when successful.
		///
		/// Weight: `O(1)`
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create(
			origin: OriginFor<T>,
			#[pallet::compact] class_id: T::ClassId,
			instance_id: T::InstanceId,
			admin: <T::Lookup as sp_runtime::traits::StaticLookup>::Source,
		) -> DispatchResult {
			let owner = ensure_signed(origin.clone())?;

			let new_cnt = Self::frunique_cnt().checked_add(1)
				.ok_or(<Error<T,I>>::FruniqueCntOverflow)?;
			// create an NFT in the uniques pallet
			pallet_uniques::Pallet::<T>::create(origin.clone(), class_id.clone(), admin.clone())?;
			pallet_uniques::Pallet::<T>::mint(
				origin.clone(),
				class_id.clone(),
				instance_id.clone(),
				admin.clone(),
			)?;
			<FruniqueCnt<T,I>>::put(new_cnt);
			let admin = T::Lookup::lookup(admin)?;
			Self::deposit_event(Event::FruniqueCreated(owner, admin, class_id, instance_id));

			Ok(())
		}

		/// Create a new frunique that is a child or subset of the parent frunique
		///
		/// A new NFT (unique) is created and reserved,
		/// a fungible token (asset) is created and minted to the owner.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// `AssetDeposit` funds of sender are reserved.
		///
		/// Parameters:
		/// - `class_id`: the class for the item that is spawning
		/// - `instance_id`: the identifier of the asset that is spawning. This must not be currently in use to identify
		/// an existing asset.
		/// - `new_instance_id`: The identifier of the new asset being created. This must not be currently in use to identify
		/// an existing asset.
		/// - `admin`: The admin of this class of assets. The admin is the initial address of each
		/// member of the asset class's admin team.
		///
		/// Emits `FruniqueDivided` event when successful.
		///
		/// Weight: `O(1)`		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]

		#[pallet::weight(10_000 + T::DbWeight::get().writes(3))]
		pub fn divide(
			_origin: OriginFor<T>,
			#[pallet::compact] _class_id: T::ClassId,
			_instance_id: T::InstanceId,
			_new_instance_id: T::InstanceId,
			_amount: u64,
		) -> DispatchResult {
			ensure!(false, Error::<T, I>::NotYetImplemented);
			//let owner = ensure_signed(_origin.clone())?;
			//let instance = Asset::<T, I>::insert(&_class_id, &_instance_id, details);
			//let instance = pallet_uniques::Pallet::<T>::
			// Get the members from `special-pallet` pallet
			//let who = special_pallet::Pallet::<T>::get();
			// retrieve the instance being divided
			// let instance: T::InstanceId = Self.Asset::<T, _>::get(class_id.clone(), instance_id.clone())
			// 	.ok_or(Error::<T, _>::Unknown)?;
			// pallet_uniques::Pallet::<T>::mint(
			// 	origin.clone(),
			// 	class_id.clone(),
			// 	new_instance_id.clone(),
			// 	instance.admin.clone(),
			// )?;

			// set the parent instance_id to the metadata
			// // (probably will need to record this in the fruniques pallet storage)
			// pallet_uniques::Pallet::<T>::set_attribute(
			// 	origin.clone(),
			// 	class_id.clone(),
			// 	new_instance_id.clone(),
			// 	"parent".into(),            // key: BoundedVec<u8, T::KeyLimit>,
			// 	instance_id.clone().into(), // value: BoundedVec<u8, T::ValueLimit>,
			// )?;

			// pallet_uniques::Pallet::<T>::set_attribute(
			// 	origin.clone(),
			// 	class_id.clone(),
			// 	new_instance_id.clone(),
			// 	"amount".into(), // key: BoundedVec<u8, T::KeyLimit>,
			// 	amount,          // value: BoundedVec<u8, T::ValueLimit>,
			// )?;

			// let admin = T::Lookup::lookup(instance.admin)?;
			// Self::deposit_event(Event::FruniqueDivided(
			// 	owner,
			// 	admin,
			// 	class_id,
			// 	new_instance_id, // non-fungible token parameters
			// 	0,
			// 	0,
			// 	0,
			// )); // fungible token parameters
			Ok(())
		}

		/// ## NFT Division
		/// 
		/// PD: the Key/value length limits are ihnerited from the uniques pallet,
		/// so they're not explicitly declared on this pallet 
		/// 
		/// (for now I'll leave aside the division of the numerical value)
		/// 
		/// ### Boilerplate parameters:
		/// 
		/// - `admin`: The admin of this class of assets. The admin is the initial address of each
		/// member of the asset class's admin team.
		/// 
		/// ### Parameters needed in order to divide a unique:
		/// - `class_id`: The type of NFT that the function will create, categorized by numbers.
		/// - `instance_id`: The unique identifier of the instance to be fractioned/divided 
		/// - `_inherit_attrs`: Doesn't do anything fow now. Intended to enable the attribute inheritance
		/// 
		#[pallet::weight(10_000 + T::DbWeight::get().writes(4))]
		pub fn spawn(
			origin: OriginFor<T>, 
			class_id: T::ClassId, 
			instance_id: T::InstanceId,
			_inherit_attrs: bool,
			admin: <T::Lookup as sp_runtime::traits::StaticLookup>::Source,
		)->DispatchResult {
			// Boilerplate (setup, conversions, ensure_signed)
			let owner = ensure_signed( origin.clone())?;
			let enconded_id = instance_id.encode();
			let new_cnt = Self::frunique_cnt().checked_add(1)
				.ok_or(<Error<T,I>>::FruniqueCntOverflow)?;
			// TODO: Helper function to convert a enconded product to BoundedVec? (consider the BoundedVec Length)
			let parent_id_key = BoundedVec::<u8,T::KeyLimit>::try_from("parent_id".encode())
				.expect("Error on encoding the parent_id key to BoundedVec");
			let parent_id_val = BoundedVec::<u8,T::ValueLimit>::try_from(enconded_id)
				.expect("Error on converting the parent_id to BoundedVec");
			// Instance n number of nfts (with the respective parentId)
			let new_instance_id:u16 = Self::frunique_cnt().try_into().unwrap();
			// Mint a unique
			pallet_uniques::Pallet::<T>::mint(origin.clone(), class_id, 
			Self::u16_to_instance_id(new_instance_id ), admin.clone())?;
			// Set the respective attributtes 
			// (for encoding reasons the parentId is stored on hex format as a secondary side-effect, I hope it's not too much of a problem).
			pallet_uniques::Pallet::<T>::set_attribute(origin.clone(), class_id, Some(Self::u16_to_instance_id(new_instance_id)),
			parent_id_key ,parent_id_val)?;
			// TODO: Copy the attributes from the parent to the new frunique
			/* 
			Doesnt work, getting: type alias `Attribute` is private.
			Even when the trait pallet_uniques::Config<I> is specified 
			let attr = <pallet_uniques::Attribute<T,I>  >::get("placeholder-key");
			let attribute = <pallet_uniques::Attribute<T,_>  >::get((class_id, 
				Some(Self::u16_to_instance_id(new_instance_id) ) , 
				&parent_id_key));
				let attribute = pallet_uniques::Pallet<T,_>::Attribute::get((class_id, 
					Some(Self::u16_to_instance_id(new_instance_id) ) , 
					&parent_id_key));
			*/
			<FruniqueCnt<T,I>>::put(new_cnt);
			// TODO: set the divided value attribute. Numbers, divisions and floating points are giving a lot of problems
			// Emit event: fruniques created?
			let admin = T::Lookup::lookup(admin)?;
			Self::deposit_event(Event::FruniqueDivided(owner, admin, class_id, instance_id));
			// Freeze the nft to prevent trading it? Burn it? Not clear, so nothing at the moment
			Ok(())
		}

		
		
	}

	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		pub fn u16_to_instance_id(input: u16) -> T::InstanceId where <T as pallet_uniques::Config>::InstanceId: From<u16> {
			input.into()
		}

	}

}