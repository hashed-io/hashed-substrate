#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod functions;
pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use crate::types::*;
	use frame_support::pallet_prelude::{ValueQuery, *};
	use frame_system::pallet_prelude::*;
	use sp_runtime::sp_std::vec::Vec;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		// ideally sudo or council
		type RemoveOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		#[pallet::constant]
		type MaxScopesPerPallet: Get<u32>;
		#[pallet::constant]
		type MaxRolesPerPallet: Get<u32>;
		#[pallet::constant]
		type RoleMaxLen: Get<u32>;
		#[pallet::constant]
		type PermissionMaxLen: Get<u32>;
		#[pallet::constant]
		type MaxPermissionsPerRole: Get<u32>;
		#[pallet::constant]
		type MaxRolesPerUser: Get<u32>;
		#[pallet::constant]
		type MaxUsersPerRole: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/* --- Onchain storage section --- */

	#[pallet::storage]
	#[pallet::getter(fn scopes)]
	pub(super) type Scopes<T: Config> = StorageMap<
		_,
		Identity,
		PalletId,                                   // pallet_id
		BoundedVec<ScopeId, T::MaxScopesPerPallet>, // scopes_id
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn roles)]
	pub(super) type Roles<T: Config> = StorageMap<
		_,
		Identity,
		RoleId,                        // role_id
		BoundedVec<u8, T::RoleMaxLen>, // role
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn pallet_roles)]
	pub(super) type PalletRoles<T: Config> = StorageMap<
		_,
		Identity,
		PalletId,                                 // pallet_id
		BoundedVec<RoleId, T::MaxRolesPerPallet>, // role_id
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn permissions)]
	pub(super) type Permissions<T: Config> = StorageDoubleMap<
		_,
		Identity,
		PalletId, // pallet_id
		Identity,
		PermissionId,                        // permission_id
		BoundedVec<u8, T::PermissionMaxLen>, // permission str
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn permissions_by_role)]
	pub(super) type PermissionsByRole<T: Config> = StorageDoubleMap<
		_,
		Identity,
		PalletId, // pallet_id
		Identity,
		RoleId,                                             // role_id
		BoundedVec<PermissionId, T::MaxPermissionsPerRole>, // permission_ids
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn roles_by_user)]
	pub(super) type RolesByUser<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::AccountId>, // user
			NMapKey<Identity, PalletId>,             // pallet_id
			NMapKey<Identity, ScopeId>,              // scope_id
		),
		BoundedVec<RoleId, T::MaxRolesPerUser>, // roles (ids)
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn users_by_scope)]
	pub(super) type UsersByScope<T: Config> = StorageNMap<
		_,
		(
			// getting "the trait bound `usize: scale_info::TypeInfo` is not satisfied" errors
			//  on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.
			NMapKey<Identity, PalletId>, // pallet_id
			NMapKey<Identity, ScopeId>,  // scope_id
			NMapKey<Identity, RoleId>,   // role_id
		),
		BoundedVec<T::AccountId, T::MaxUsersPerRole>, // users
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// An initial roles config was stored [pallet_id, Vec<role_id>]
		RolesStored(PalletId, BoundedVec<RoleId, T::MaxRolesPerPallet>),
		/// The permissions were created and set to the role [pallet_id, role_id, Vec<permission_id>]
		PermissionsCreatedAndSet(
			PalletId,
			RoleId,
			BoundedVec<PermissionId, T::MaxPermissionsPerRole>,
		),
		/// The user no longer has that role [pallet_id, scope_id, role_id, account_id]
		RoleRemovedFromUser(PalletId, ScopeId, RoleId, T::AccountId),
		/// The user now has that role [pallet_id, scope_id, role_id, account_id]
		RoleAssignedToUser(PalletId, ScopeId, RoleId, T::AccountId),
		/// The role no longer has the permission in the pallet context [pallet_id, role_id, permission_id]
		PermissionRevokedFromRole(PalletId, RoleId, PermissionId),
		/// The permission was removed from the pallet and all the roles that had it [pallet_id, permission_id, affected_roles]
		PermissionRemovedFromPallet(
			PalletId,
			PermissionId,
			BoundedVec<RoleId, T::MaxRolesPerPallet>,
		),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// The specified scope doesn't exists
		ScopeNotFound,
		/// The scope is already linked with the pallet
		ScopeAlreadyExists,
		/// The specified role doesn't exist or it hasn't been set to the user
		RoleNotFound,
		/// The permission doesn't exist in the pallet
		PermissionNotFound,
		/// The specified user hasn't been asigned to this scope
		UserNotFound,
		/// The provided role list must have unique elements
		DuplicateRole,
		/// The provided permission list must have unique elements
		DuplicatePermission,
		/// The user has that role asigned in that scope
		UserAlreadyHasRole,
		/// The role is already linked in the pallet
		RoleAlreadyLinkedToPallet,
		/// The role exists but it hasn't been linked to the pallet
		RoleNotLinkedToPallet,
		/// The permission is already linked to that role in that scope
		PermissionAlreadyLinkedToRole,
		/// The permission wasn't found in the roles capabilities
		PermissionNotLinkedToRole,
		/// The user doesn't have any roles in this pallet
		UserHasNoRoles,
		/// The role doesn't have any users assigned to it
		RoleHasNoUsers,
		/// The pallet name is too long
		ExceedPalletNameMaxLen,
		/// The pallet has too many scopes
		ExceedMaxScopesPerPallet,
		/// The pallet cannot have more roles
		ExceedMaxRolesPerPallet,
		/// The specified role cannot have more permission in this scope
		ExceedMaxPermissionsPerRole,
		/// The user cannot have more roles in this scope
		ExceedMaxRolesPerUser,
		/// This role cannot have assigned to more users in this scope
		ExceedMaxUsersPerRole,
		/// The role string is too long
		ExceedRoleMaxLen,
		/// The permission string is too long
		ExceedPermissionMaxLen,
		/// The user does not have the specified role
		NotAuthorized,
	}

	// SBP-M2 review: Missing document
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn tx_create_and_set_roles(
			origin: OriginFor<T>,
			pallet: IdOrVec,
			roles: Vec<Vec<u8>>,
		) -> DispatchResult {
			ensure!(
				T::RemoveOrigin::ensure_origin(origin.clone()).is_ok(),
				Error::<T>::NotAuthorized
			);
			Self::create_and_set_roles(pallet, roles)?;
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn tx_remove_role_from_user(
			origin: OriginFor<T>,
			user: T::AccountId,
			pallet: IdOrVec,
			scope_id: ScopeId,
			role_id: RoleId,
		) -> DispatchResult {
			ensure!(
				T::RemoveOrigin::ensure_origin(origin.clone()).is_ok(),
				Error::<T>::NotAuthorized
			);
			Self::remove_role_from_user(user, pallet, &scope_id, role_id)?;
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn tx_create_and_set_permissions(
			origin: OriginFor<T>,
			pallet: IdOrVec,
			role_id: RoleId,
			permissions: Vec<Vec<u8>>,
		) -> DispatchResult {
			ensure!(
				T::RemoveOrigin::ensure_origin(origin.clone()).is_ok(),
				Error::<T>::NotAuthorized
			);
			Self::create_and_set_permissions(pallet, role_id, permissions)?;
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn tx_assign_role_to_user(
			origin: OriginFor<T>,
			user: T::AccountId,
			pallet: IdOrVec,
			scope_id: ScopeId,
			role_id: RoleId,
		) -> DispatchResult {
			ensure!(
				T::RemoveOrigin::ensure_origin(origin.clone()).is_ok(),
				Error::<T>::NotAuthorized
			);
			Self::assign_role_to_user(user, pallet, &scope_id, role_id)?;
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn revoke_permission_from_role(
			origin: OriginFor<T>,
			pallet: IdOrVec,
			role_id: RoleId,
			permission_id: PermissionId,
		) -> DispatchResult {
			ensure!(
				T::RemoveOrigin::ensure_origin(origin.clone()).is_ok(),
				Error::<T>::NotAuthorized
			);
			Self::do_revoke_permission_from_role(pallet, role_id, permission_id)?;
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn remove_permission_from_pallet(
			origin: OriginFor<T>,
			pallet: IdOrVec,
			permission_id: PermissionId,
		) -> DispatchResult {
			ensure!(
				T::RemoveOrigin::ensure_origin(origin.clone()).is_ok(),
				Error::<T>::NotAuthorized
			);
			Self::do_remove_permission_from_pallet(pallet, permission_id)?;
			Ok(())
		}
	}
}
