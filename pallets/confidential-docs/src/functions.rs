use super::*;
use frame_support::pallet_prelude::*;
//use frame_system::pallet_prelude::*;
use crate::types::*;

impl<T: Config> Pallet<T> {
	
  pub fn do_set_vault(owner: T::AccountId, user_id: UserId, public_key: PublicKey, cid: CID) -> DispatchResult {
    Self::validate_cid(&cid)?;
    ensure!(!<Vaults<T>>::contains_key(&user_id), <Error<T>>::UserAlreadyHasVault);
    ensure!(!<PublicKeys<T>>::contains_key(&owner), <Error<T>>::AccountAlreadyHasPublicKey);
    let vault = Vault{
      cid: cid.clone(),
      owner: owner.clone()
    };
    <Vaults<T>>::insert(user_id, vault.clone());
    <PublicKeys<T>>::insert(owner.clone(), public_key.clone());

    Self::deposit_event(Event::VaultStored(user_id, public_key, vault));
    Ok(())
  }

  pub fn do_set_owned_document(owner: T::AccountId, mut owned_doc: OwnedDoc<T>) -> DispatchResult {
    owned_doc.owner = owner.clone();
    Self::validate_owned_doc(&owned_doc)?;
    let OwnedDoc {
      cid,
      ..
    } = owned_doc.clone();
    if let Some(doc) = <OwnedDocs<T>>::get(&cid) {
      ensure!(doc.owner == owner, <Error<T>>::NotDocOwner); 
    } else {
      <OwnedDocsByOwner<T>>::try_mutate(&owner, |owned_vec| {
        owned_vec.try_push(cid.clone())
      }).map_err(|_| <Error<T>>::ExceedMaxOwnedDocs)?;
    }
    <OwnedDocs<T>>::insert(cid.clone(), owned_doc.clone());
    Self::deposit_event(Event::OwnedDocStored(owned_doc));
    Ok(())
  }

  pub fn do_share_document(owner: T::AccountId, mut shared_doc: SharedDoc<T>) -> DispatchResult {
    shared_doc.from = owner;
    Self::validate_shared_doc(&shared_doc)?;
    let SharedDoc {
      cid,
      to,
      ..
    } = shared_doc.clone();
    
    <SharedDocsByTo<T>>::try_mutate(&to, |shared_vec| {
      shared_vec.try_push(cid.clone())
    }).map_err(|_| <Error<T>>::ExceedMaxSharedToDocs)?;

    <SharedDocs<T>>::insert(cid.clone(), shared_doc.clone());
    Self::deposit_event(Event::SharedDocStored(shared_doc));
    Ok(())
  }

  fn validate_owned_doc(owned_doc: &OwnedDoc<T>)->DispatchResult{
    let OwnedDoc {
      cid,
      name,
      description,
      owner
    } = owned_doc;
    Self::validate_cid(cid)?;
    Self::validate_doc_name(name)?;
    Self::validate_doc_desc(description)?;
    Self::validate_has_public_key(owner)?;
    Ok(())
  }

  fn validate_shared_doc(shared_doc: &SharedDoc<T>)->DispatchResult{
    let SharedDoc {
      cid,
      name,
      description,
      from,
      to,
    } = shared_doc;
    Self::validate_cid(cid)?;
    Self::validate_doc_name(name)?;
    Self::validate_doc_desc(description)?;
    ensure!(from != to, <Error<T>>::DocSharedWithSelf);
    ensure!(!<SharedDocs<T>>::contains_key(cid), <Error<T>>::DocAlreadyShared);
    Self::validate_has_public_key(from)?;
    Self::validate_has_public_key(to)?;
    Ok(())
  }

  fn validate_has_public_key(who: &T::AccountId)->DispatchResult{
    ensure!(<PublicKeys<T>>::contains_key(who), <Error<T>>::AccountHasNoPublicKey);
    Ok(())
  }
  fn validate_cid(cid: &CID)->DispatchResult{
    ensure!(cid.len() > 0, <Error<T>>::CIDNoneValue);
    Ok(())
  }

  fn validate_doc_name(doc_name: &DocName<T>)->DispatchResult{
    ensure!(doc_name.len() >= T::DocNameMinLen::get().try_into().unwrap(), <Error<T>>::DocNameTooShort);
    Ok(())
  }

  fn validate_doc_desc(doc_desc: &DocDesc<T>)->DispatchResult{
    ensure!(doc_desc.len() >= T::DocDescMinLen::get().try_into().unwrap(), <Error<T>>::DocDescTooShort);
    Ok(())
  }
}