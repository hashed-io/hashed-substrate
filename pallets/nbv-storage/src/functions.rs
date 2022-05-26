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

use crate::types::{BDK_SERVICES_URL,};

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
        let vault =  <Vaults<T>>::take(vault_id).ok_or(Error::<T>::VaultNotFound)?;
        let vault_members = [
            vault.cosigners.as_slice(),
            &[vault.owner.clone()],
        ].concat();
        // Removes the vault from user->vault vector
        vault_members.iter().for_each(|signer|{
            <VaultsBySigner<T>>::mutate(signer, | vault_list |{
                let vault_index = vault_list.iter().position(|v| *v==vault_id);
                match vault_index{
                    Some(index) => {vault_list.remove(index);},
                    _ => log::warn!("Vault not found in members"),
                }
            });
        });
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

    pub fn gen_vaults_payload_by_bulk(pending_vaults : Vec<[u8;32]>) -> Vec<SingleVaultPayload>{
        let mut generated_vaults = Vec::<SingleVaultPayload>::new();
        pending_vaults.iter().for_each(|vault_to_complete| {
            // Contact bdk services and get descriptors
            let vault_result = Self::bdk_gen_vault(vault_to_complete.clone())
                .map_err(|e| log::error!("Error while generating http vault:{:?}",e) ).unwrap();
                //.expect("Error while generating the vault's output descriptors");
            // Build offchain vaults struct and push it to a Vec
            generated_vaults.push(SingleVaultPayload{
                vault_id: vault_to_complete.clone(),
                output_descriptor: vault_result.0.clone(),
                change_descriptor: vault_result.1.clone(),
            });
        });
        generated_vaults
    }

    pub fn gen_proposals_payload_by_bulk(pending_proposals : Vec<[u8;32]>) ->  Vec<SingleProposalPayload>{
        let mut generated_proposals = Vec::<SingleProposalPayload>::new();
        pending_proposals.iter().for_each(|proposal_to_complete|{
            let psbt = Self::bdk_gen_proposal(proposal_to_complete.clone()).expect("Error while generating proposal");
            generated_proposals.push(SingleProposalPayload{
                proposal_id:proposal_to_complete.clone(),
                psbt,
            })
        });
        generated_proposals
    }

    pub fn bdk_gen_vault(vault_id: [u8; 32]) -> Result<(Vec<u8>, Vec<u8>), http::Error> {
        // We will create a bunch of elements that we will put into a JSON Object.
        let raw_json = Self::generate_vault_json_body(vault_id);
        let request_body =
            str::from_utf8(raw_json.as_slice()).expect("Error converting Json to string");

        let url = [BDK_SERVICES_URL.clone(), b"/gen_output_descriptor"].concat();

        let response_body = Self::http_post(
            str::from_utf8(url.as_slice()).expect("Error converting Json to string"),
            request_body
        )?;
        // Create a str slice from the body.
        let body_str = str::from_utf8(&response_body).map_err(|_| {
            log::warn!("No UTF8 body");
            http::Error::Unknown
        })?;

        Self::parse_vault_descriptors(body_str).ok_or(http::Error::Unknown)
    }

    fn http_post(url: &str, request_body: &str)->Result<Vec<u8>, http::Error>{
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
            .map_err(|_| http::Error::IoError)?;
            let response =
            pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
        // Let's check the status code before we proceed to reading the response.
        if response.code != 200 {
            log::warn!("Unexpected status code: {}", response.code);
            return Err(http::Error::Unknown);
        }

        // Next we want to fully read the response body and collect it to a vector of bytes.
        Ok(response.body().collect::<Vec<u8>>())
    }

    fn generate_vault_json_body(vault_id: [u8; 32]) -> Vec<u8> {
        let mut body = Vec::new();

        //Get vault properties
        let vault = <Vaults<T>>::get(&vault_id).expect("Vault not found with id");
        let threshold = NumberValue {
            integer: vault.threshold.clone().into(),
            fraction: 0,
            fraction_length: 0,
            exponent: 0,
        };
        body.push(("threshold".chars().collect::<Vec<char>>(), JsonValue::Number(threshold)));
        let vault_signers =
        [vault.cosigners.clone().as_slice(), &[vault.owner.clone()]].concat();
        
        //get the xpub for each cosigner
        let mapped_xpubs: Vec<JsonValue> = Self::get_accounts_xpubs(vault_signers)
            .iter()
            .map(|xpub| {
                let xpub_field =
                    JsonValue::String(str::from_utf8(xpub).unwrap().chars().collect());
                JsonValue::Object([("xpub".chars().collect(), xpub_field)].to_vec())
                // JsonValue::String(  str::from_utf8(xpub).unwrap().chars().collect()  )
            })
            .collect();
        body.push(("cosigners".chars().collect::<Vec<char>>(), JsonValue::Array(mapped_xpubs)));
        let json_object = JsonValue::Object(body);

        // // Parse the JSON and print the resulting lite-json structure.
        jsonSerialize::format(&json_object, 4)
    }

    /// Parse the descriptors from the given JSON string using `lite-json`.
    ///
    /// Returns `None` when parsing failed or `Some((descriptor, change_descriptor))` when parsing is successful.
    fn parse_vault_descriptors(body_str: &str) -> Option<(Vec<u8>, Vec<u8>)> {
        let val = parse_json(body_str);
        match val.ok()? {
            JsonValue::Object(obj) => {
                let descriptor = Self::extract_json_str_by_name(obj.clone(), "descriptor")
                    .expect("Descriptor str not found");
                let change_descriptor =
                    Self::extract_json_str_by_name(obj.clone(), "change_descriptor")
                        .expect("Change descriptor str not found");
                Some((descriptor, change_descriptor))
            },
            _ => return None,
        }
    }

    pub fn bdk_gen_proposal(proposal_id: [u8;32])->Result<Vec<u8>, http::Error>{

        let raw_json = Self::gen_proposal_json_body(proposal_id);
        let request_body =
            str::from_utf8(raw_json.as_slice()).expect("Error converting Json to string");

        let url = [BDK_SERVICES_URL.clone(), b"/gen_psbt"].concat();

        let response_body = Self::http_post(
            str::from_utf8(url.as_slice()).expect("Error converting Json to string"),
            request_body
        )?;
        // The psbt is not a json object, its a byte blob
        Ok(response_body)
    }

    pub fn gen_proposal_json_body(proposal_id: [u8;32])-> Vec<u8>{
        let mut body = Vec::new();
        let proposal = <Proposals<T>>::get(proposal_id).expect("Proposal not found");
        let vault = <Vaults<T>>::get(proposal.vault_id.clone()).expect("Vault not found with id");
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
            .expect("Error converting descriptor").chars().collect();
        let change_descriptor: Vec<char> = str::from_utf8(
            vault.descriptors.change_descriptor.expect("Change descriptor not found").as_slice())
            .expect("Error converting descriptor").chars().collect();
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
        jsonSerialize::format(&json_object, 4)
    }

    pub fn do_insert_vault(vault: Vault<T>) -> DispatchResult {
        // generate vault id
        let vault_id = vault.using_encoded(blake2_256);
        // build a vector containing owner + signers
        let vault_members = vault.cosigners.clone().to_vec();
        //log::info!("Total vault members count: {:?}", vault_members.len());
        // iterate over that vector and add the vault id to the list of each user (signer)
        ensure!(Self::members_are_unique(vault_members.clone()), Error::<T>::DuplicateVaultMembers);
        vault_members.into_iter().try_for_each(|acc| {
            // check if all users have an xpub
            if !<XpubsByOwner<T>>::contains_key(acc.clone()) {
                return Err(Error::<T>::XPubNotFound);
            }
            <VaultsBySigner<T>>::try_mutate(acc, |vault_vec| {
                vault_vec.try_push(vault_id.clone())
            })
            .map_err(|_| Error::<T>::SignerVaultLimit)
        })?;
        <Vaults<T>>::insert(vault_id.clone(), vault.clone());

        Self::deposit_event(Event::VaultStored(vault_id, vault.owner));
        Ok(())
    }

    pub fn do_insert_descriptors(vault_id: [u8;32], descriptors: Descriptors<T::OutputDescriptorMaxLen>) -> DispatchResult {
        <Vaults<T>>::try_mutate(vault_id, | v |{
            match v {
                Some(vault) =>{
                    vault.descriptors.clone_from(&descriptors);
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
        Ok(())
    }

    pub fn do_insert_psbt(proposal_id: [u8;32], psbt: BoundedVec<u8, T::PSBTMaxLen>) ->DispatchResult{
        <Proposals<T>>::try_mutate(proposal_id,|p|{
            match p {
                Some(proposal) =>{
                    proposal.psbt.clone_from(&psbt);
                    Ok(())
                },
                None=> Err(Error::<T>::ProposalNotFound),
            }
        })?;
        Ok(())
    }

    pub fn get_pending_proposals() -> Vec<[u8; 32]>{
        <Proposals<T>>::iter()
            .filter_map(|(id, proposal)|{
                if proposal.psbt.is_empty() {
                    Some(id)
                } else {
                    None
                }
            })
        .collect()
    }

    pub fn get_pending_vaults() -> Vec<[u8; 32]> {
        <Vaults<T>>::iter()
            .filter_map(|(entry, vault)| {
                if vault.descriptors.output_descriptor.is_empty() {
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

    pub fn get_vault_members(vault_id : [u8;32])-> Vec<T::AccountId> {
        let vault =  <Vaults<T>>::get(vault_id).expect("Vault not found");
        [vault.cosigners.as_slice(),&[vault.owner.clone()],].concat()
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