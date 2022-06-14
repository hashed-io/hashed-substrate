use super::*;
use frame_support::pallet_prelude::*;
use frame_support::{sp_io::hashing::blake2_256};
use sp_runtime::sp_std::str;
use sp_runtime::sp_std::vec::Vec;
use sp_runtime::traits::BlockNumberProvider;
use sp_runtime::{
    offchain::{
        http,
        Duration,
    },
};
use lite_json::json::{JsonValue, NumberValue};
use lite_json::parse_json;
use lite_json::Serialize as jsonSerialize;
use crate::types::*;

impl<T: Config> Pallet<T> {
    /// Use with caution
    pub fn do_remove_xpub(who: T::AccountId) -> DispatchResult {
        let old_hash = <XpubsByOwner<T>>::take(who.clone()).ok_or(Error::<T>::XPubNotFound)?;
        <Xpubs<T>>::remove(old_hash);
        Self::deposit_event(Event::XPubRemoved(who));
        Ok(())
    }

    pub fn do_remove_vault(vault_id: [u8;32]) -> DispatchResult{
        // This removes the vault while retrieving its values
        let vault_members = Self::get_vault_members(vault_id)?;
        let vault =  <Vaults<T>>::take(vault_id).ok_or(Error::<T>::VaultNotFound)?;
        // Removes the vault from user->vault vector
        vault_members.iter().try_for_each(|signer|{
            <VaultsBySigner<T>>::try_mutate::<_,(),DispatchError,_>(signer, |vault_list|{
                let vault_index = vault_list.iter().position(|v| *v==vault_id).ok_or(Error::<T>::VaultNotFound)?;
                vault_list.remove(vault_index);
                Ok(())
            })
        })?;
        // Removes all vault proposals
        let vault_proposals = <ProposalsByVault<T>>::get(vault_id);
        vault_proposals.iter().try_for_each(|proposal_id|{
            Self::do_remove_proposal(*proposal_id)
        })?;
        Self::deposit_event(Event::VaultRemoved(vault_id, vault.owner));
        Ok(())
    }

    pub fn do_remove_proposal(proposal_id: [u8;32]) -> DispatchResult{
        let proposal = <Proposals<T>>::take(proposal_id).ok_or(Error::<T>::ProposalNotFound)?;
        <ProposalsByVault<T>>::try_mutate::<_,_,DispatchError,_>(proposal.vault_id, |proposal_list|{
            let proposal_index= proposal_list.iter().position(|p| p==&proposal_id).ok_or(Error::<T>::ProposalNotFound)?;
            proposal_list.remove(proposal_index);
            Ok(())
        })?;
        Self::deposit_event(Event::ProposalRemoved(proposal_id, proposal.proposer));
        Ok(())
    }

    pub fn do_save_psbt(signer: T::AccountId, proposal_id: [u8;32], signature_payload: BoundedVec<u8, T::PSBTMaxLen>) -> DispatchResult{
        // validations: proposal exists, signer is member of vault, proposal is pending, 
        let vault_id = <Proposals<T>>::get(proposal_id).ok_or(Error::<T>::ProposalNotFound)?.vault_id;
        ensure!(Self::is_vault_member(&signer, vault_id)?, Error::<T>::SignerPermissionsNeeded);
        let signature = ProposalSignatures{
            signer: signer.clone(),
            signature: signature_payload,
        };
        <Proposals<T>>::try_mutate::<_,(),DispatchError,_>(proposal_id, |proposal| {
            proposal.as_ref().ok_or(Error::<T>::ProposalNotFound)?;
            if let Some(p) = proposal {
                let signed_already = p.signed_psbts.iter().find(|&signature|{ signature.signer ==signer }).is_some();
                ensure!(!signed_already, Error::<T>::AlreadySigned);
                p.signed_psbts.try_push(signature).map_err(|_| Error::<T>::ExceedMaxCosignersPerVault)?;
            }
            Ok(())
        })?;
        Ok(())
    }
    // Check for xpubs duplicates (requires owner to be on the vault_signers Vec)
    pub fn members_are_unique( vault_signers: Vec<T::AccountId>) -> bool {
        let mut filtered_signers = vault_signers.clone();
        filtered_signers.sort();
        filtered_signers.dedup();
        // Signers length should be equal 
        vault_signers.len() == filtered_signers.len()
    }

    // check if the xpub is free to take/update or if its owned by the account
    pub fn get_xpub_status(who: T::AccountId, xpub_hash: [u8; 32]) -> XpubStatus {
        if <Xpubs<T>>::contains_key(xpub_hash) {
            if let Some(owned_hash) = <XpubsByOwner<T>>::get(who.clone()) {
                match xpub_hash == owned_hash {
                    true => return XpubStatus::Owned,
                    false => return XpubStatus::Taken,
                }
            } else {
                // xpub registered and the account doesnt own it: unavailable
                return XpubStatus::Taken;
            }
            // Does the user owns the registered xpub? if yes, available
        }
        // new xpub registry: available
        XpubStatus::Free
    }

    pub fn gen_vaults_payload_by_bulk(pending_vaults : Vec<[u8;32]>) -> Vec<SingleVaultPayload >{
        let mut generated_vaults = Vec::<SingleVaultPayload >::new();
        pending_vaults.iter().for_each(|vault_to_complete| {
            // Contact bdk services and get descriptors
            let vault_result = Self::bdk_gen_vault(vault_to_complete.clone());
            let mut vault_payload = SingleVaultPayload{
                vault_id: vault_to_complete.clone(),
                output_descriptor: Vec::default(),
                change_descriptor: Vec::default(),
                status: OffchainStatus::Valid,
            };
            match vault_result{
                Ok(descriptors) => {
                    vault_payload.output_descriptor.clone_from(&descriptors.0);
                    vault_payload.change_descriptor.clone_from(&descriptors.1);
                },
                Err(status) => {vault_payload.status.clone_from(&status)},
            };     
            // Build offchain vaults struct and push it to a Vec
            generated_vaults.push(vault_payload);
        });
        generated_vaults
    }

    pub fn gen_proposals_payload_by_bulk(pending_proposals : Vec<[u8;32]>) ->  Vec<SingleProposalPayload>{
        let mut generated_proposals = Vec::<SingleProposalPayload>::new();
        pending_proposals.iter().for_each(|proposal_to_complete|{
            let mut proposal_payload = SingleProposalPayload{
                proposal_id:proposal_to_complete.clone(),
                psbt : Vec::default(),
                status: OffchainStatus::Valid,
            };
            let psbt_result = Self::bdk_gen_proposal(proposal_to_complete.clone());
            match psbt_result{
                Ok(psbt) => {proposal_payload.psbt.clone_from(&psbt)},
                Err(status) => {proposal_payload.status.clone_from(&status)},
            };
            generated_proposals.push(proposal_payload);
        });
        generated_proposals
    }

    pub fn bdk_gen_vault(vault_id: [u8; 32]) -> Result<(Vec<u8>, Vec<u8>), OffchainStatus > {
        // We will create a bunch of elements that we will put into a JSON Object.
        let raw_json = Self::generate_vault_json_body(vault_id)?;
        let request_body =
        str::from_utf8(raw_json.as_slice()).map_err(|_| Self::build_offchain_err(false, "Vault json is not utf-8") )?;

        let url = [<BDKServicesURL<T>>::get().to_vec(), b"/gen_output_descriptor".encode()].concat();
        let response_body = Self::http_post(
            str::from_utf8(url.as_slice()).map_err(|_| Self::build_offchain_err(false, "URL is not utf-8") )?,
            request_body
        )?;
        // Create a str slice from the body.
        let body_str = str::from_utf8(&response_body).map_err(|_| {
            log::warn!("No UTF8 body");
            Self::build_offchain_err(false, "No UTF8 body")
        })?;

        Self::parse_vault_descriptors(body_str)
    }

    fn http_post(url: &str, request_body: &str)->Result<Vec<u8>, OffchainStatus >{
        let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(6_000));

        let request = http::Request::post(
            &url,
            [request_body.clone()].to_vec(),
        )
        .add_header("Content-Type", "application/json")
        .add_header("Accept", "application/json");

        let pending = request
            .body([request_body.clone()].to_vec())
            .deadline(deadline)
            .send()
            .map_err(|_|
                Self::build_offchain_err(true, "I/O error")
            )?;
        let response =
            pending.try_wait(deadline).map_err(|_| Self::build_offchain_err(true, "Request to bdk timed out"))?.
            map_err(|_|Self::build_offchain_err(false, "Unknown error on server's side"))?;
        match response.code{
            200..=299 => return Ok(response.body().collect::<Vec<u8>>()),
            400..=599 => {
                let code_encoded = response.code.to_ne_bytes();
                let code_str = str::from_utf8(&code_encoded).unwrap_or_default();
                log::warn!("Codigo? {} vs {:?}",response.code,code_encoded);
                return Err(Self::build_offchain_err(response.code>=500, code_str ))
            },
            _ =>return Err(Self::build_offchain_err(false, "Unknown error"))
        }
    }

    fn generate_vault_json_body(vault_id: [u8; 32]) -> Result<Vec<u8>, OffchainStatus >{
        let mut body = Vec::new();

        //Get vault properties
        let vault = <Vaults<T>>::get(&vault_id).ok_or(
            Self::build_offchain_err(false,"Vault not found"))?;
        let threshold = NumberValue {
            integer: vault.threshold.clone().into(),
            fraction: 0,
            fraction_length: 0,
            exponent: 0,
        };
        body.push(("threshold".chars().collect::<Vec<char>>(), JsonValue::Number(threshold)));
        let vault_signers = vault.cosigners.clone().to_vec();
        
        //get the xpub for each cosigner
        let mapped_xpubs: Vec<JsonValue> = Self::get_accounts_xpubs(vault_signers)
            .iter()
            .map(|xpub| {
                let xpub_field =
                    JsonValue::String(str::from_utf8(xpub).unwrap().chars().collect());
                JsonValue::Object([("xpub".chars().collect(), xpub_field)].to_vec())
            })
            .collect();
        body.push(("cosigners".chars().collect::<Vec<char>>(), JsonValue::Array(mapped_xpubs)));
        let json_object = JsonValue::Object(body);

        // // Parse the JSON and print the resulting lite-json structure.
        Ok(jsonSerialize::format(&json_object, 4))
    }

    /// Parse the descriptors from the given JSON string using `lite-json`.
    ///
    /// Returns `None` when parsing failed or `Some((descriptor, change_descriptor))` when parsing is successful.
    fn parse_vault_descriptors(body_str: &str) -> Result<(Vec<u8>, Vec<u8>), OffchainStatus > {
        let val = parse_json(body_str);
        match val.ok() {
            Some(JsonValue::Object(obj) )=> {
                let descriptor = Self::extract_json_str_by_name(obj.clone(), "descriptor")
                    .ok_or(Self::build_offchain_err(false,"Descriptor not found in bdk response"))?;
                let change_descriptor =
                    Self::extract_json_str_by_name(obj.clone(), "change_descriptor")
                        .ok_or(Self::build_offchain_err(false,"Change descriptor not found in bdk response"))?;
                Ok((descriptor, change_descriptor))
            },
            _ => {
                return Err(Self::build_offchain_err(false,"Error parsing response json"))
            },
        }
    }

    pub fn bdk_gen_proposal(proposal_id: [u8;32])->Result<Vec<u8>, OffchainStatus >{

        let raw_json = Self::gen_proposal_json_body(proposal_id)?;
        let request_body =
            str::from_utf8(raw_json.as_slice()).map_err(|_| Self::build_offchain_err(false, "Request body is not UTF-8") )?;

        let url = [<BDKServicesURL<T>>::get().to_vec(), b"/gen_psbt".encode()].concat();

        let response_body = Self::http_post(
            str::from_utf8(url.as_slice()).map_err(|_| Self::build_offchain_err(false, "URL is not UTF-8") )?,
            request_body
        )?;
        // The psbt is not a json object, its a byte blob
        Ok(response_body)
    }

    pub fn gen_proposal_json_body(proposal_id: [u8;32])-> Result<Vec<u8>,OffchainStatus>{
        let mut body = Vec::new();
        let proposal = <Proposals<T>>::get(proposal_id).ok_or(
            Self::build_offchain_err(false,"Proposal not found"))?;
        let vault = <Vaults<T>>::get(proposal.vault_id.clone()).ok_or(
            Self::build_offchain_err(false,"Vault not found"))?;
        let amount = NumberValue {
            integer: proposal.amount.clone() as i64,
            fraction: 0,
            fraction_length: 0,
            exponent: 0,
        };
        let fee = NumberValue {
            integer: proposal.fee_sat_per_vb.clone().into(),
            fraction: 0,
            fraction_length: 0,
            exponent: 0,
        };
        let to_address = str::from_utf8(proposal.to_address.as_slice()).expect("Error converting recipient address").chars().collect();
        let output_descriptor: Vec<char> = str::from_utf8(
            vault.descriptors.output_descriptor.as_slice())
            .map_err(|_| Self::build_offchain_err(false,"Output descriptor is not utf-8"))?.chars().collect();
        let change_descriptor: Vec<char> = str::from_utf8(
            vault.descriptors.change_descriptor.unwrap_or_default().as_slice() )
            .map_err(|_| Self::build_offchain_err(false,"Change descriptor is not utf-8"))?.chars().collect();
        let descriptors_body = [
            ("descriptor".chars().collect::<Vec<char>>(), JsonValue::String(output_descriptor)),
            ("change_descriptor".chars().collect::<Vec<char>>(), JsonValue::String(change_descriptor)),
        ].to_vec();
        body.push(("amount".chars().collect::<Vec<char>>(), JsonValue::Number(amount) ));
        body.push(("fee_sat_per_vb".chars().collect::<Vec<char>>(), JsonValue::Number(fee) ));
        body.push(("to_address".chars().collect::<Vec<char>>(), JsonValue::String(to_address) ));
        body.push(("descriptors".chars().collect::<Vec<char>>(), JsonValue::Object(descriptors_body) ));
        let json_object = JsonValue::Object(body);

        // // Parse the JSON and print the resulting lite-json structure.
        Ok(jsonSerialize::format(&json_object, 4) )
    }

    pub fn do_insert_vault(vault: Vault<T>) -> DispatchResult {
        // generate vault id
        ensure!(Self::members_are_unique(vault.cosigners.clone().to_vec()), Error::<T>::DuplicateVaultMembers);
        let vault_id = vault.using_encoded(blake2_256);
        // build a vector containing owner + signers
        let vault_members = vault.cosigners.to_vec();
        // iterate over that vector and add the vault id to the list of each user (signer)
        vault_members.clone().into_iter().try_for_each(|acc| {
            // check if all users have an xpub
            if !<XpubsByOwner<T>>::contains_key(acc.clone()) {
                return Err(Error::<T>::XPubNotFound);
            }
            <VaultsBySigner<T>>::try_mutate(acc, |vault_vec| {
                vault_vec.try_push(vault_id.clone())
            })
            .map_err(|_| Error::<T>::SignerVaultLimit)
        })?;

        // insert owner in case it isn't on the cosigners list
        if !vault_members.contains(&vault.owner) {
            <VaultsBySigner<T>>::try_mutate(&vault.owner, |vault_vec| {
                vault_vec.try_push(vault_id.clone())
            })
            .map_err(|_| Error::<T>::SignerVaultLimit)?;
        }
        <Vaults<T>>::insert(vault_id.clone(), vault.clone());

        Self::deposit_event(Event::VaultStored(vault_id, vault.owner));
        Ok(())
    }

    pub fn do_insert_descriptors(vault_id: [u8;32], descriptors: Descriptors<T::OutputDescriptorMaxLen>, status: BDKStatus<T::VaultDescriptionMaxLen>) -> DispatchResult {
        <Vaults<T>>::try_mutate(vault_id, | v |{
            match v {
                Some(vault) =>{
                    vault.descriptors.clone_from(&descriptors);
                    vault.offchain_status.clone_from(&status);
                    Ok(())
                },
                None=> Err(Error::<T>::VaultNotFound),
            }
        })?;
        Self::deposit_event(Event::DescriptorsStored(vault_id));
        Ok(())
    }

    pub fn do_propose(proposal: Proposal<T>)->DispatchResult{
        let proposal_id = proposal.using_encoded(blake2_256);
        <Proposals<T>>::insert(proposal_id, proposal.clone());
        <ProposalsByVault<T>>::try_mutate(proposal.vault_id,|proposals|{
            proposals.try_push(proposal_id)
        }).map_err(|_| Error::<T>::ExceedMaxProposalsPerVault)?;

        Self::deposit_event(Event::ProposalStored(proposal_id, proposal.proposer));
        Ok(())
    }

    pub fn do_insert_psbt(proposal_id: [u8;32], psbt: BoundedVec<u8, T::PSBTMaxLen>, status: BDKStatus<T::VaultDescriptionMaxLen>) ->DispatchResult{
        <Proposals<T>>::try_mutate(proposal_id,|p|{
            match p {
                Some(proposal) =>{
                    proposal.psbt.clone_from(&psbt);
                    proposal.offchain_status.clone_from(&status);
                    Ok(())
                },
                None=> Err(Error::<T>::ProposalNotFound),
            }
        })?;
        Self::deposit_event(Event::PSBTStored(proposal_id));
        Ok(())
    }

    pub fn get_pending_proposals() -> Vec<[u8; 32]>{
        <Proposals<T>>::iter()
            .filter_map(|(id, proposal)|{
                if proposal.psbt.is_empty() && 
                (proposal.offchain_status.eq(&BDKStatus::<T::VaultDescriptionMaxLen>::Pending) || 
                proposal.offchain_status.eq(&BDKStatus::<T::VaultDescriptionMaxLen>::RecoverableError(
                    BoundedVec::<u8,T::VaultDescriptionMaxLen>::default() )) ){
                    Some(id)
                } else {
                    None
                }
            })
        .collect()
    }

    pub fn is_vault_member(account: &T::AccountId, vault_id : [u8;32]) -> Result<bool, DispatchError>{
       Ok(Self::get_vault_members(vault_id)?.contains(account))
    }

    pub fn get_pending_vaults() -> Vec<[u8; 32]> {
        <Vaults<T>>::iter()
            .filter_map(|(entry, vault)| {
                if vault.descriptors.output_descriptor.is_empty() && 
                (vault.offchain_status.eq(&BDKStatus::<T::VaultDescriptionMaxLen>::Pending) || 
                 vault.offchain_status.eq(&BDKStatus::<T::VaultDescriptionMaxLen>::RecoverableError(
                    BoundedVec::<u8,T::VaultDescriptionMaxLen>::default() )) )  {
                    Some(entry)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_accounts_xpubs(accounts: Vec<T::AccountId>) -> Vec<Vec<u8>> {
        // rely on pallet storage (just in case the identity gets reseted by user error)
        let mut xpub_vec = Vec::<Vec<u8>>::default();
        accounts.iter().for_each(|account| {
            let xpub_id =
                <XpubsByOwner<T>>::get(account).expect("The account doesn't have an xpub");
            let xpub =
                <Xpubs<T>>::get(xpub_id).expect("Error trying to retrieve xpub from its ID");
            xpub_vec.push(
                // format the xpub to string
                xpub.to_vec(),
            );
        });
        xpub_vec
    }

    pub fn get_vault_members(vault_id : [u8;32])-> Result<Vec<T::AccountId>, DispatchError> {
        let vault =  <Vaults<T>>::get(vault_id).ok_or(Error::<T>::VaultNotFound)?;
        let mut members = [vault.cosigners.as_slice(),&[vault.owner.clone()],].concat();
        members.sort();
        members.dedup();
        Ok(members)
    }
    
    fn build_offchain_err(recoverable: bool, msj: &str )-> OffchainStatus{
        let bounded_msj = msj.encode();
        match recoverable{
            true => OffchainStatus::RecoverableError(bounded_msj),
            false => OffchainStatus::IrrecoverableError(bounded_msj),
        }
    }

    pub fn chars_to_bytes(v: Vec<char>) -> Vec<u8> {
        v.iter().map(|c| *c as u8).collect::<Vec<u8>>()
    }

    pub fn extract_json_str_by_name(
        tuple: Vec<(Vec<char>, JsonValue)>,
        s: &str,
    ) -> Option<Vec<u8>> {
        let filtered = tuple.into_iter().find(|(key, _)| key.iter().copied().eq(s.chars()));
        match filtered.expect("Error retrieving json field").1 {
            JsonValue::String(chars) => return Some(Self::chars_to_bytes(chars)),
            _ => return None,
        }
    }
}

/*--- Block Number provider section. Needed to implement locks on offchain storage*/
impl<T: Config> BlockNumberProvider for Pallet<T> {
    type BlockNumber = T::BlockNumber;

    fn current_block_number() -> Self::BlockNumber {
        <frame_system::Pallet<T>>::block_number()
    }
}