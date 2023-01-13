use super::*;
use frame_support::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
//use frame_system::pallet_prelude::*;
use crate::types::*;

impl<T: Config> Pallet<T> {
	pub fn do_set_vault(
		owner: T::AccountId,
		user_id: UserId,
		public_key: PublicKey,
		cid: CID,
	) -> DispatchResult {
		Self::validate_cid(&cid)?;
		let hashed_account = owner.using_encoded(blake2_256);
		if let Some(uid) = <UserIds<T>>::get(&hashed_account) {
			ensure!(uid == user_id, Error::<T>::NotOwnerOfUserId);
		} else {
			ensure!(!<Vaults<T>>::contains_key(&user_id), Error::<T>::NotOwnerOfVault);
		}

		let vault = Vault { cid: cid.clone(), owner: owner.clone() };
		<Vaults<T>>::insert(user_id, vault.clone());
		<PublicKeys<T>>::insert(owner.clone(), public_key);
		<UserIds<T>>::insert(hashed_account.clone(), user_id);

		Self::deposit_event(Event::VaultStored(user_id, public_key, vault));
		Ok(())
	}

	pub fn do_set_owned_document(
		owner: T::AccountId,
		mut owned_doc: OwnedDoc<T>,
	) -> DispatchResult {
		owned_doc.owner = owner.clone();
		Self::validate_owned_doc(&owned_doc)?;
		let OwnedDoc { cid, .. } = owned_doc.clone();
		if let Some(doc) = <OwnedDocs<T>>::get(&cid) {
			ensure!(doc.owner == owner, Error::<T>::NotDocOwner);
		} else {
			<OwnedDocsByOwner<T>>::try_mutate(&owner, |owned_vec| owned_vec.try_push(cid.clone()))
				.map_err(|_| Error::<T>::ExceedMaxOwnedDocs)?;
		}
		<OwnedDocs<T>>::insert(cid.clone(), owned_doc.clone());
		Self::deposit_event(Event::OwnedDocStored(owned_doc));
		Ok(())
	}

	pub fn do_remove_owned_document(owner: T::AccountId, cid: CID) -> DispatchResult {
		let doc = <OwnedDocs<T>>::try_get(&cid).map_err(|_| Error::<T>::DocNotFound)?;
		ensure!(doc.owner == owner, Error::<T>::NotDocOwner);
		<OwnedDocsByOwner<T>>::try_mutate(&owner, |owned_vec| {
			let cid_index =
				owned_vec.iter().position(|c| *c == cid).ok_or(Error::<T>::CIDNotFound)?;
			owned_vec.remove(cid_index);
			Ok(())
		})
		.map_err(|_: Error<T>| Error::<T>::CIDNotFound)?;
		<OwnedDocs<T>>::remove(cid.clone());
		Self::deposit_event(Event::OwnedDocRemoved(doc));
		Ok(())
	}

	pub fn do_share_document(owner: T::AccountId, mut shared_doc: SharedDoc<T>) -> DispatchResult {
		shared_doc.from = owner;
		Self::validate_shared_doc(&shared_doc)?;
		let SharedDoc { cid, to, from, .. } = shared_doc.clone();

		<SharedDocsByFrom<T>>::try_mutate(&from, |shared_vec| shared_vec.try_push(cid.clone()))
			.map_err(|_| Error::<T>::ExceedMaxSharedFromDocs)?;

		<SharedDocsByTo<T>>::try_mutate(&to, |shared_vec| shared_vec.try_push(cid.clone()))
			.map_err(|_| Error::<T>::ExceedMaxSharedToDocs)?;

		<SharedDocs<T>>::insert(cid.clone(), shared_doc.clone());
		Self::deposit_event(Event::SharedDocStored(shared_doc));
		Ok(())
	}

	pub fn do_update_shared_document_metadata(
		to: T::AccountId,
		mut shared_doc: SharedDoc<T>,
	) -> DispatchResult {
		let doc = <SharedDocs<T>>::try_get(&shared_doc.cid).map_err(|_| Error::<T>::DocNotFound)?;
		ensure!(doc.to == to, Error::<T>::NotDocSharee);
		shared_doc.from = doc.from;
		shared_doc.to = to;
		<SharedDocs<T>>::insert(doc.cid.clone(), shared_doc.clone());
		Self::deposit_event(Event::SharedDocUpdated(shared_doc));
		Ok(())
	}

	pub fn do_remove_shared_document(to: T::AccountId, cid: CID) -> DispatchResult {
		let doc = <SharedDocs<T>>::try_get(&cid).map_err(|_| Error::<T>::DocNotFound)?;
		ensure!(doc.to == to, Error::<T>::NotDocSharee);
		<SharedDocsByTo<T>>::try_mutate(&to, |shared_vec| {
			let cid_index =
				shared_vec.iter().position(|c| *c == cid).ok_or(Error::<T>::CIDNotFound)?;
			shared_vec.remove(cid_index);
			Ok(())
		})
		.map_err(|_: Error<T>| Error::<T>::CIDNotFound)?;
		<SharedDocsByFrom<T>>::try_mutate(&doc.from, |shared_vec| {
			let cid_index =
				shared_vec.iter().position(|c| *c == cid).ok_or(Error::<T>::CIDNotFound)?;
			shared_vec.remove(cid_index);
			Ok(())
		})
		.map_err(|_: Error<T>| Error::<T>::CIDNotFound)?;
		<SharedDocs<T>>::remove(cid.clone());
		Self::deposit_event(Event::SharedDocRemoved(doc));
		Ok(())
	}

	pub fn do_create_group(
		creator: T::AccountId,
		group_id: T::AccountId,
		name: GroupName<T>,
		public_key: PublicKey,
		cid: CID,
	) -> DispatchResult {
		ensure!(!<Groups<T>>::contains_key(&group_id), Error::<T>::GroupAlreadyExists);
		Self::validate_cid(&cid)?;
		Self::validate_group_name(&name)?;
		Self::validate_has_public_key(&creator)?;

		let group_member = GroupMember {
			authorizer: creator.clone(),
			group: group_id.clone(),
			cid,
			member: creator.clone(),
			role: GroupRole::Owner,
		};
		Self::store_group_member(group_member.clone())?;
		PublicKeys::<T>::insert(group_id.clone(), public_key);
		let group = Group { creator: creator.clone(), group: group_id.clone(), name };
		Groups::<T>::insert(group_id.clone(), group.clone());
		Self::deposit_event(Event::GroupCreated(group));
		Ok(())
	}

	pub fn do_add_group_member(
		authorizer: T::AccountId,
		mut group_member: GroupMember<T>,
	) -> DispatchResult {
		group_member.authorizer = authorizer.clone();
		let GroupMember { ref group, ref member, .. } = group_member;
		ensure!(<Groups<T>>::contains_key(group), Error::<T>::GroupDoesNotExist);
		Self::validate_group_member(&group_member)?;
		let authorizer =
			GroupMembers::<T>::get(group, &authorizer).ok_or_else(|| Error::<T>::NoPermission)?;

		ensure!(authorizer.can_add_group_member(), Error::<T>::NoPermission);
		ensure!(
			!GroupMembers::<T>::contains_key(group, member),
			Error::<T>::UserAlreadyGroupMember
		);
		Self::store_group_member(group_member.clone())?;
		Self::deposit_event(Event::GroupMemberAdded(group_member));
		Ok(())
	}

	pub fn do_remove_group_member(
		authorizer: T::AccountId,
		group: T::AccountId,
		member: T::AccountId,
	) -> DispatchResult {
		ensure!(<Groups<T>>::contains_key(&group), Error::<T>::GroupDoesNotExist);
		let authorizer =
			GroupMembers::<T>::get(&group, &authorizer).ok_or_else(|| Error::<T>::NoPermission)?;

		let group_member = GroupMembers::<T>::get(&group, &member)
			.ok_or_else(|| Error::<T>::GroupMemberDoesNotExist)?;

		ensure!(authorizer.can_remove_group_member(&group_member), Error::<T>::NoPermission);
		MemberGroups::<T>::try_mutate(&member, |member_groups| {
			match member_groups.binary_search(&group) {
				Ok(idx) => member_groups.remove(idx),
				Err(_) => return Err(Error::<T>::MemberGroupDoesNotExist),
			};
			Ok(())
		})?;
		GroupMembers::<T>::remove(&group, &member);
		Self::store_group_member(group_member.clone())?;
		Self::deposit_event(Event::GroupMemberAdded(group_member));
		Ok(())
	}

	fn validate_owned_doc(owned_doc: &OwnedDoc<T>) -> DispatchResult {
		let OwnedDoc { cid, name, description, owner } = owned_doc;
		Self::validate_cid(cid)?;
		Self::validate_doc_name(name)?;
		Self::validate_doc_desc(description)?;
		Self::validate_has_public_key(owner)?;
		Ok(())
	}

	fn validate_group_member(group_member: &GroupMember<T>) -> DispatchResult {
		let GroupMember { authorizer, group, cid, member, .. } = group_member;
		ensure!(<Groups<T>>::contains_key(group), Error::<T>::GroupDoesNotExist);
		Self::validate_cid(cid)?;
		Self::validate_has_public_key(authorizer)?;
		Self::validate_has_public_key(member)?;
		Ok(())
	}

	fn validate_shared_doc(shared_doc: &SharedDoc<T>) -> DispatchResult {
		let SharedDoc { cid, name, description, from, to } = shared_doc;
		Self::validate_cid(cid)?;
		Self::validate_doc_name(name)?;
		Self::validate_doc_desc(description)?;
		ensure!(from != to, Error::<T>::DocSharedWithSelf);
		ensure!(!<SharedDocs<T>>::contains_key(cid), Error::<T>::DocAlreadyShared);
		Self::validate_has_public_key(from)?;
		Self::validate_has_public_key(to)?;
		Ok(())
	}

	fn validate_has_public_key(who: &T::AccountId) -> DispatchResult {
		ensure!(<PublicKeys<T>>::contains_key(who), Error::<T>::AccountHasNoPublicKey);
		Ok(())
	}
	fn validate_cid(cid: &CID) -> DispatchResult {
		ensure!(cid.len() > 0, Error::<T>::CIDNoneValue);
		Ok(())
	}

	fn validate_doc_name(doc_name: &DocName<T>) -> DispatchResult {
		ensure!(
			doc_name.len() >= T::DocNameMinLen::get().try_into().unwrap(),
			Error::<T>::DocNameTooShort
		);
		Ok(())
	}

	fn validate_group_name(group_name: &GroupName<T>) -> DispatchResult {
		ensure!(
			group_name.len() >= T::GroupNameMinLen::get().try_into().unwrap(),
			Error::<T>::GroupNameTooShort
		);
		Ok(())
	}

	fn validate_doc_desc(doc_desc: &DocDesc<T>) -> DispatchResult {
		ensure!(
			doc_desc.len() >= T::DocDescMinLen::get().try_into().unwrap(),
			Error::<T>::DocDescTooShort
		);
		Ok(())
	}

	fn store_group_member(group_member: GroupMember<T>) -> DispatchResult {
		let GroupMember { ref group, ref member, .. } = group_member;
		MemberGroups::<T>::try_mutate(member, |member_groups| {
			match member_groups.binary_search(&group) {
				Ok(_) => return Err(Error::<T>::MemberGroupAlreadyExists),
				Err(idx) => member_groups
					.try_insert(idx, group.clone())
					.map_err(|_| Error::<T>::ExceedMaxMemberGroups)?,
			};
			Ok(())
		})?;
		GroupMembers::<T>::insert(group.clone(), member.clone(), group_member);
		Ok(())
	}
}
