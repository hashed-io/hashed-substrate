use super::*;
use frame_support::pallet_prelude::*;
use frame_support::{sp_io::hashing::blake2_256};
use sp_runtime::sp_std::str;
use sp_runtime::sp_std::vec::Vec;
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
    pub fn remove_xpub_from_pallet_storage(who: T::AccountId) -> Result<(), Error<T>> {
        // No error can be propagated from the remove functions
        if <XpubsByOwner<T>>::contains_key(who.clone()) {
            let old_hash = <XpubsByOwner<T>>::take(who).expect("Old hash not found");
            <Xpubs<T>>::remove(old_hash);
            return Ok(());
        }
        return Err(<Error<T>>::XPubNotFound);
    }

    // Ensure at that certain point, no xpub field exists on the identity
    pub fn xpub_field_available(
        fields: &BoundedVec<
            (pallet_identity::Data, pallet_identity::Data),
            T::MaxAdditionalFields,
        >,
    ) -> bool {
        let key = BoundedVec::<u8, ConstU32<32>>::try_from(b"xpub".encode())
            .expect("Error on encoding the xpub key to BoundedVec");
        let xpub_count =
            fields.iter().find(|(k, _)| k == &pallet_identity::Data::Raw(key.clone()));
        xpub_count.is_none()
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

    pub fn bdk_gen_vault(vault_id: [u8; 32]) -> Result<(Vec<u8>, Vec<u8>), http::Error> {
        // We want to keep the offchain worker execution time reasonable, so we set a hard-coded
        // deadline to 2s to complete the external call.
        // You can also wait idefinitely for the response, however you may still get a timeout
        // coming from the host machine.
        let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(6_000));
        // We will create a bunch of elements that we will put into a JSON Object.
        //let request_body =  Vec::new();
        // Initiate an external HTTP GET request.
        // This is using high-level wrappers from `sp_runtime`, for the low-level calls that
        // you can find in `sp_io`. The API is trying to be similar to `reqwest`, but
        // since we are running in a custom WASM execution environment we can't simply
        // import the library here.
        let raw_json = Self::generate_vault_json_body(vault_id);
        let request_body =
            str::from_utf8(raw_json.as_slice()).expect("Error converting Json to string");
        log::info!("Objecto JSON construido: {:?}", request_body.clone());

        let url = [BDK_SERVICES_URL.clone(), b"/gen_output_descriptor"].concat();

        let request = http::Request::post(
            str::from_utf8(&url).expect("Error converting the BDK URL"),
            [request_body.clone()].to_vec(),
        )
        .add_header("Content-Type", "application/json")
        .add_header("Accept", "application/json");
        // We set the deadline for sending of the request, note that awaiting response can
        // have a separate deadline. Next we send the request, before that it's also possible
        // to alter request headers or stream body content in case of non-GET requests.
        let pending = request
            .body([request_body.clone()].to_vec())
            .deadline(deadline)
            .send()
            .map_err(|_| http::Error::IoError)?;
        // The request is already being processed by the host, we are free to do anything
        // else in the worker (we can send multiple concurrent requests too).
        // At some point however we probably want to check the response though,
        // so we can block current thread and wait for it to finish.
        // Note that since the request is being driven by the host, we don't have to wait
        // for the request to have it complete, we will just not read the response.
        let response =
            pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
        // Let's check the status code before we proceed to reading the response.
        if response.code != 200 {
            log::warn!("Unexpected status code: {}", response.code);
            return Err(http::Error::Unknown);
        }

        // Next we want to fully read the response body and collect it to a vector of bytes.
        // Note that the return object allows you to read the body in chunks as well
        // with a way to control the deadline.
        let body = response.body().collect::<Vec<u8>>();

        // Create a str slice from the body.
        let body_str = str::from_utf8(&body).map_err(|_| {
            log::warn!("No UTF8 body");
            http::Error::Unknown
        })?;

        Self::parse_vault_descriptors(body_str).ok_or(http::Error::Unknown)
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
        //get the xpub for each cosigner
        //let cosigners = vault.cosigners.
        //let s = String::from("edgerg");
        //let o = JsonValue::String(s.chars().collect()) ;
        let vault_signers =
            [vault.cosigners.clone().as_slice(), &[vault.owner.clone()]].concat();

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

    pub fn do_insert_vault(vault: Vault<T>) -> DispatchResult {
        // generate vault id
        let vault_id = vault.using_encoded(blake2_256);
        // build a vector containing owner + signers
        let vault_members =
            [vault.cosigners.clone().as_slice(), &[vault.owner.clone()]].concat();
        log::info!("Total vault members count: {:?}", vault_members.len());
        // iterate over that vector and add the vault id to the list of each user (signer)
        //let vaults_by_signer_insertion_result =
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
        //ensure!(vaults_by_signer_insertion_result.is_ok(), Error::<T>::SignerVaultLimit);
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