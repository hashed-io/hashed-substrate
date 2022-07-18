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

  pub fn do_set_document(owner: T::AccountId, cid: CID, doc_name: DocName<T>, doc_desc: DocDesc<T>) -> DispatchResult {
    Self::validate_cid(&cid)?;
    Self::validate_doc_name(&doc_name)?;
    Self::validate_doc_desc(&doc_desc)?;
    let doc = Document{
      name: doc_name,
      description: doc_desc
    };
    <Documents<T>>::insert(owner.clone(), cid.clone(), doc.clone());
    Self::deposit_event(Event::DocStored(owner, cid, doc));
    Ok(())
  }

  pub fn do_share_document(owner: T::AccountId, cid: CID, mut shared_doc: SharedDocument<T>) -> DispatchResult {
    shared_doc.from = owner;
    Self::validate_cid(&cid)?;
    Self::validate_shared_doc(&shared_doc)?;
    ensure!(<SharedDocumentsByTo<T>>::contains_key(owner, cid), <Error<T>>::DocumentAlreadySharedWithUser);
    <Documents<T>>::insert(owner.clone(), cid.clone(), doc.clone());
    Self::deposit_event(Event::DocStored(owner, cid, doc));
    Ok(())
  }

  fn validate_shared_doc(shared_doc: &SharedDocument<T>)->DispatchResult{
    let SharedDocument {
      name,
      description,
      from,
      to,
      original_cid,
    } = shared_doc;
    Self::validate_cid(original_cid)?;
    Self::validate_doc_name(name)?;
    Self::validate_doc_desc(description)?;
    ensure!(from != to, <Error<T>>::DocumentSharedWithSelf);
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