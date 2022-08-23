
use crate::{mock::*, Error, types::ProposalStatus};
use codec::{Encode};
use frame_support::{
	assert_noop, assert_ok,
	//traits::{ConstU32,},
	BoundedVec,
};
use sp_io::hashing::blake2_256;
fn dummy_xpub() -> BoundedVec<u8,XPubLen >{
	BoundedVec::<u8,XPubLen >::try_from(
		b"[adc450e3/84'/1'/0'/0]tpubDEMkzn5sBo8Nct35y2BEFhJTqhsa72yeUf5S6ymb85G6LW2okSh1fDkrMhgCtYsrsCAuspm4yVjC63VUA6qrcQ54tVm5TKwhWFBLyyCjabX/*"
		.encode())
	.expect("Error on encoding the xpub key to BoundedVec")
}

fn dummy_xpub_2() -> BoundedVec<u8,XPubLen >{
	BoundedVec::<u8,XPubLen >::try_from(
		b"[621c051d/123456789'/123456789'/123456789'/123456789]tpubDF3cwMypW7CJnZ4WwzwgYkd1bJzJsPTnLbFN3zdeGKfEx38jDjBzRntupghKC6A5szrjELasjrhBRXStKKUmS8wHZQxkVPN7P88iXxbC3s1/*"
		.encode())
	.expect("Error on encoding the xpub key to BoundedVec")
}

fn dummy_xpub_3() -> BoundedVec<u8,XPubLen> {
	BoundedVec::<u8,XPubLen >::try_from(b"Zpub74kbYv5LXvBaJRcbSiihEEwuDiBSDztjtpSVmt6C6nB3ntbcEy4pLP3cJGVWsKbYKaAynfCwXnkuVncPGQ9Y4XwWJDWrDMUwTztdxBe7GcM"
		.encode())
	.expect("Error on encoding the xpub key to BoundedVec")
}

fn dummy_xpub_4() -> BoundedVec<u8,XPubLen> {
	BoundedVec::<u8,XPubLen >::try_from(b"Zpub75bKLk9fCjgfELzLr2XS5TEcCXXGrci4EDwAcppFNBDwpNy53JhJS8cbRjdv39noPDKSfzK7EPC1Ciyfb7jRwY7DmiuYJ6WDr2nEL6yTkHi"
		.encode())
	.expect("Error on encoding the xpub key to BoundedVec")
}

fn dummy_description() ->  BoundedVec<u8, VaultDescriptionMaxLen>{
	BoundedVec::<u8,VaultDescriptionMaxLen>::try_from(b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".encode())
				.expect("Error on encoding the description to BoundedVec")
}

fn dummy_testnet_recipient_address() ->BoundedVec<u8,XPubLen> {
	BoundedVec::<u8,XPubLen >::try_from(b"tb1qhfku74zsrhvre7053xqsnh36gsey3ur7slwwnfn04g5506rmdchqrf7w30"
		.encode())
	.expect("Error on encoding the xpub key to BoundedVec")
}

fn dummy_psbt() -> BoundedVec<u8, PSBTMaxLen>{
	BoundedVec::<u8, PSBTMaxLen>::try_from(b"cHNidP8BAK4BAAAAAa5F8SDWH2Hlqgky89rGlhG/4DnKqcbRlL+jQ6F0FBP5AQAAAAD9////AhAnAAAAAAAAR1IhApLkOyyLZWwh3QTadFlmp7x3xt+dPgyL/tQ47r+exVRiIQO7Ut/DRj54BKrR0Kf7c42enyfrbV4TDSpsMiqhfrnQm1KuokkAAAAAAAAiACD0hQx+A3+kUAR7iBY5VjkG2DViANmiP0xOBPixU1x36AAAAAAAAQDqAgAAAAABAecL0e2g6vO11ZpVRcHuBDFZNdXUqcDOmYsg7lK86S3cAAAAAAD+////AlpmwwIAAAAAFgAU370BMJPnoYItIaum9dnKt8LCLI8wdQAAAAAAACIAIILP1EkLWcvTQ15pBdk3paMwDIvglbUG6FQBBon3sRAMAkcwRAIgOYjunqLCM9LhnLS9lVoPSVR6z5Phk9BxodHar/ncgGgCIALhH3N/Q1yD7FxE7SSA9sogkcW3WXH1kxy3BLuMcU1zASECoJ99bEErPxgEAT+Nt7GhfwlgQ24fC//v/3LCUQnpzzBkgSEAAQErMHUAAAAAAAAiACCCz9RJC1nL00NeaQXZN6WjMAyL4JW1BuhUAQaJ97EQDAEFR1IhAip4P8CC/dZji38IFOD6ZjW50Pv3RazsvZExGHoy+MupIQPjlUrnEv00n6ytsa4sIMXdSjKHlXn94P4PBuOifenW51KuIgYCKng/wIL91mOLfwgU4PpmNbnQ+/dFrOy9kTEYejL4y6kMsCkMNwAAAAAAAAAAIgYD45VK5xL9NJ+srbGuLCDF3Uoyh5V5/eD+Dwbjon3p1ucMyqFhVgAAAAAAAAAAAAAiAgMacPy3H41FU/Xw+P81xScxWS/jO5Ny1gGnON1fo+4zbQzKoWFWAQAAAAAAAAAiAgOZ2MtgB/5WFgVoNU56XwjdHdTDuO2TYeQNe8TSV2tq7QywKQw3AQAAAAAAAAAA"
		.encode()).unwrap_or_default()
}

#[test]
fn set_xpub_identity_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub() ));
		assert_eq!(NBVStorage::xpubs_by_owner(test_pub(1)), Some( dummy_xpub().using_encoded(blake2_256)) );
	});
}

#[test]
fn inserting_same_xpub_should_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_noop!(NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub()),Error::<Test>::XPubAlreadyTaken);
		
	});
}

#[test]
fn inserting_without_removing_xpub_should_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_noop!(NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub_2()),Error::<Test>::UserAlreadyHasXpub);
		
	});
}

#[test]
fn removing_xpub_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!(NBVStorage::remove_xpub(Origin::signed(test_pub(1))));
	});
}

#[test]
fn replacing_xpub_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!(NBVStorage::remove_xpub(Origin::signed(test_pub(1))));
		assert_ok!(NBVStorage::set_xpub(Origin::signed(test_pub(1)),dummy_xpub_2() )  );
	});
}

#[test]
fn removing_twice_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!(NBVStorage::remove_xpub(Origin::signed(test_pub(1))));
		assert_noop!(NBVStorage::remove_xpub(Origin::signed(test_pub(1))), Error::<Test>::XPubNotFound);
		
	});
}

#[test]
fn creating_vault_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),].to_vec()).unwrap();
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 2, dummy_description(),true, cosigners) );
		assert!(!NBVStorage::vaults_by_signer(test_pub(1)).is_empty());


	});
}


#[test]
fn vault_without_cosigners_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		default();
		assert_noop!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 1, dummy_description(), true,cosigners), Error::<Test>::NotEnoughCosigners );
		assert!( NBVStorage::vaults_by_signer(test_pub(1)).is_empty());

	});
}

#[test]
fn vault_with_invalid_threshold_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
			try_from([ test_pub(2),].to_vec()).unwrap();
		assert_noop!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 0, dummy_description(), true, cosigners.clone()), Error::<Test>::InvalidVaultThreshold );
		assert_noop!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 3, dummy_description(), true, cosigners), Error::<Test>::InvalidVaultThreshold );
		assert!( NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
	});
}

#[test]
fn vault_with_duplicate_members_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
			try_from([ test_pub(2),test_pub(1),].to_vec()).unwrap();
		assert_noop!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 1, dummy_description(), true,
			cosigners.clone()), Error::<Test>::DuplicateVaultMembers );
		assert!( NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		assert!( NBVStorage::vaults_by_signer(test_pub(2)).is_empty());
	});
}

#[test]
fn vault_with_duplicate_incomplete_members() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
			try_from([ test_pub(2),test_pub(1),].to_vec()).unwrap();
		assert_noop!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 1, dummy_description(), true,
			cosigners.clone()), Error::<Test>::DuplicateVaultMembers );
		assert!( NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		assert!( NBVStorage::vaults_by_signer(test_pub(2)).is_empty());
	});
}

#[test]
fn exceeding_max_cosigners_per_vault_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(3)), dummy_xpub_3()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(4)), dummy_xpub_4()) );
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),test_pub(3), test_pub(4)].to_vec()).unwrap();
		assert_noop!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 2, dummy_description(),true, cosigners), Error::<Test>::ExceedMaxCosignersPerVault );
		assert!(NBVStorage::vaults_by_signer(test_pub(1)).is_empty());


	});
}


#[test]
fn vault_signer_without_xpub_shouldnt_exist() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
			try_from([ test_pub(2),].to_vec()).unwrap();
		let cosigners2 = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
			try_from([ test_pub(1),].to_vec()).unwrap();
		// Case 1: cosigner with no xpub
		assert_noop!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 1, dummy_description(), true, cosigners.clone()), Error::<Test>::XPubNotFound );
		assert!( NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		assert!( NBVStorage::vaults_by_signer(test_pub(2)).is_empty());
		// Case 2: owner with no xpub
		assert_noop!(NBVStorage::create_vault( Origin::signed(test_pub(2)) , 1, dummy_description(), true, cosigners2.clone()), Error::<Test>::XPubNotFound );
		assert!( NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		assert!( NBVStorage::vaults_by_signer(test_pub(2)).is_empty());
	});
}

#[test]
fn signer_reached_max_vaults() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(3)), dummy_xpub_3()) );
		let cosigners1 = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
			try_from([ test_pub(2),].to_vec()).unwrap();
		let cosigners2 = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
			try_from([ test_pub(2), test_pub(3)].to_vec()).unwrap();
		let cosigners3 = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
			try_from([ test_pub(3)].to_vec()).unwrap();

		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 2, dummy_description(), true, cosigners1) );
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 3, dummy_description(), true, cosigners2) );
		assert_noop!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 2, dummy_description(), true, cosigners3), Error::<Test>::SignerVaultLimit );

		assert_eq!( NBVStorage::vaults_by_signer(test_pub(1)).len(), 2);
	});
}

#[test]
fn removing_vault_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(3)), dummy_xpub_3()) );
		
		// Insert a normal vault
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),test_pub(3)].to_vec()).unwrap();
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 1, dummy_description(), false, cosigners) );
		assert!(!NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		// Try to remove xpub (vault depends on it)
		let vault_id = NBVStorage::vaults_by_signer(test_pub(1)).pop().unwrap();
		assert_ok!(NBVStorage::remove_vault(Origin::signed(test_pub(1)),vault_id));

	});
}

#[test]
fn removing_vault_which_isnt_yours_shoulnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		
		// Insert a normal vault
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),].to_vec()).unwrap();
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 2, dummy_description(), true, cosigners) );
		assert!(!NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		// Try to remove xpub (vault depends on it)
		let vault_id = NBVStorage::vaults_by_signer(test_pub(1)).pop().unwrap();
		assert_noop!(NBVStorage::remove_vault(Origin::signed(test_pub(2)),vault_id),  Error::<Test>::VaultOwnerPermissionsNeeded);

	});
}

#[test]
fn removing_vault_and_xpub_in_order_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		
		// Insert a normal vault
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),].to_vec()).unwrap();
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 2, dummy_description(), true, cosigners) );
		assert!(!NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		// TODO: Remove vault
		let vault_id = NBVStorage::vaults_by_signer(test_pub(1)).pop().unwrap();
		assert_ok!(NBVStorage::remove_vault(Origin::signed(test_pub(1)),vault_id));
		// Try to remove xpub (vault depends on it)
		assert_ok!(NBVStorage::remove_xpub(Origin::signed(test_pub(1))));

	});
}

#[test]
fn removing_xpub_before_vault_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		
		// Insert a normal vault
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),].to_vec()).unwrap();
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 2, dummy_description(), true, cosigners) );
		assert!(!NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		// Try to remove xpub (vault depends on it)
		assert_noop!(NBVStorage::remove_xpub(Origin::signed(test_pub(1))), Error::<Test>::XpubLinkedToVault);

	});
}

#[test]
fn proposing_should_work(){
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		// Insert a normal vault
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),].to_vec()).unwrap();
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 2, dummy_description(), true, cosigners) );
		assert!(!NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = NBVStorage::vaults_by_signer(test_pub(1)).pop().unwrap();
		assert_ok!(NBVStorage::propose(Origin::signed(test_pub(1)),vault_id,dummy_testnet_recipient_address(),1000,dummy_description()));
	});
}

#[test]
fn proposing_from_external_user_should_work(){
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		// Insert a normal vault
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),].to_vec()).unwrap();
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 2, dummy_description(), true, cosigners) );
		assert!(!NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = NBVStorage::vaults_by_signer(test_pub(1)).pop().unwrap();
		// user 3 is not on the vault so it should expect an error
		assert_noop!(NBVStorage::propose(Origin::signed(test_pub(3)),vault_id,dummy_testnet_recipient_address(),1000,dummy_description()),
			Error::<Test>::SignerPermissionsNeeded);
	});
}

#[test]
fn proposing_twice_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		// Insert a normal vault
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),].to_vec()).unwrap();
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 2, dummy_description(), true, cosigners) );
		assert!(!NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = NBVStorage::vaults_by_signer(test_pub(1)).pop().unwrap();
		assert_ok!(NBVStorage::propose(Origin::signed(test_pub(1)),vault_id,dummy_testnet_recipient_address(),1000,dummy_description()));
		assert_noop!(NBVStorage::propose(Origin::signed(test_pub(1)),vault_id,dummy_testnet_recipient_address(),1000,dummy_description()),
			Error::<Test>::AlreadyProposed);
	});
}

#[test]
fn exceeding_max_proposals_per_vault_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		// Insert a normal vault
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),].to_vec()).unwrap();
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 2, dummy_description(), true, cosigners) );
		assert!(!NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = NBVStorage::vaults_by_signer(test_pub(1)).pop().unwrap();
		assert_ok!(NBVStorage::propose(Origin::signed(test_pub(1)),vault_id,dummy_testnet_recipient_address(),1000,dummy_description()));
		assert_ok!(NBVStorage::propose(Origin::signed(test_pub(1)),vault_id,dummy_testnet_recipient_address(),1001,dummy_description()));
		assert_noop!(NBVStorage::propose(Origin::signed(test_pub(1)),vault_id,dummy_testnet_recipient_address(),1002,dummy_description()),
			Error::<Test>::ExceedMaxProposalsPerVault);
	});
}

#[test]
fn saving_psbt_should_work(){
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		// Insert a normal vault
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),].to_vec()).unwrap();
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 2, dummy_description(), true, cosigners) );
		assert!(!NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = NBVStorage::vaults_by_signer(test_pub(1)).pop().unwrap();
		assert_ok!(NBVStorage::propose(Origin::signed(test_pub(1)),vault_id,dummy_testnet_recipient_address(),1000,dummy_description()));
		// obtaining proposal id and saving a psbt
		let proposal_id = NBVStorage::proposals_by_vault(vault_id).pop().unwrap();
		assert_ok!(NBVStorage::save_psbt(Origin::signed(test_pub(1)), proposal_id, dummy_psbt()));
	});
}

#[test]
fn saving_psbt_to_a_nonexistent_proposal_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		// Insert a normal vault
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),].to_vec()).unwrap();
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 2, dummy_description(), true, cosigners) );
		assert!(!NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		// user 3 is not on the vault so it should expect an error
		let proposal_id = [0;32];
		assert_noop!(NBVStorage::save_psbt(Origin::signed(test_pub(1)), proposal_id, dummy_psbt()), Error::<Test>::ProposalNotFound);
	});
}

#[test]
fn saving_psbt_form_external_user_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		// Insert a normal vault
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),].to_vec()).unwrap();
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 2, dummy_description(), true, cosigners) );
		assert!(!NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = NBVStorage::vaults_by_signer(test_pub(1)).pop().unwrap();
		assert_ok!(NBVStorage::propose(Origin::signed(test_pub(1)),vault_id,dummy_testnet_recipient_address(),1000,dummy_description()));
		// obtaining proposal id and saving a psbt with a user that is not in the vault
		let proposal_id = NBVStorage::proposals_by_vault(vault_id).pop().unwrap();
		// user 3 is not on 
		assert_noop!(NBVStorage::save_psbt(Origin::signed(test_pub(3)), proposal_id, dummy_psbt()), Error::<Test>::SignerPermissionsNeeded);
	});
}

#[test]
fn saving_twice_psbt_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		// Insert a normal vault
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),].to_vec()).unwrap();
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 2, dummy_description(), true, cosigners) );
		assert!(!NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = NBVStorage::vaults_by_signer(test_pub(1)).pop().unwrap();
		assert_ok!(NBVStorage::propose(Origin::signed(test_pub(1)),vault_id,dummy_testnet_recipient_address(),1000,dummy_description()));
		// obtaining proposal id and saving a psbt with a user that is not in the vault
		let proposal_id = NBVStorage::proposals_by_vault(vault_id).pop().unwrap();
		// user 3 is not on the vaults cosigners
		assert_ok!(NBVStorage::save_psbt(Origin::signed(test_pub(1)), proposal_id, dummy_psbt()) );
		assert_noop!(NBVStorage::save_psbt(Origin::signed(test_pub(1)), proposal_id, dummy_psbt()), Error::<Test>::AlreadySigned);
	});
}

// TODO: Set offchainStatus proposal from pending to Valid
#[test]
fn finalize_psbt_should_work(){
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		// Insert a normal vault
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),].to_vec()).unwrap();
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 1, dummy_description(), true, cosigners) );
		assert!(!NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = NBVStorage::vaults_by_signer(test_pub(1)).pop().unwrap();
		assert_ok!(NBVStorage::propose(Origin::signed(test_pub(1)),vault_id,dummy_testnet_recipient_address(),1000,dummy_description()));
		// obtaining proposal id and saving a psbt with a user that is not in the vault
		let proposal_id = NBVStorage::proposals_by_vault(vault_id).pop().unwrap();

		assert_ok!(NBVStorage::save_psbt(Origin::signed(test_pub(1)), proposal_id, dummy_psbt()) );
		assert_ok!(NBVStorage::finalize_psbt(Origin::signed(test_pub(1)), proposal_id,false));
		assert!(NBVStorage::proposals(proposal_id).unwrap().status.eq(&ProposalStatus::ReadyToFinalize(false)));
	});
}

#[test]
fn finalize_psbt_twice_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		// Insert a normal vault
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),].to_vec()).unwrap();
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 1, dummy_description(), true, cosigners) );
		assert!(!NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = NBVStorage::vaults_by_signer(test_pub(1)).pop().unwrap();
		assert_ok!(NBVStorage::propose(Origin::signed(test_pub(1)),vault_id,dummy_testnet_recipient_address(),1000,dummy_description()));
		// obtaining proposal id and saving a psbt with a user that is not in the vault
		let proposal_id = NBVStorage::proposals_by_vault(vault_id).pop().unwrap();

		assert_ok!(NBVStorage::save_psbt(Origin::signed(test_pub(1)), proposal_id, dummy_psbt()) );
		assert_ok!(NBVStorage::finalize_psbt(Origin::signed(test_pub(1)), proposal_id,false));
		assert!(NBVStorage::proposals(proposal_id).unwrap().status.eq(&ProposalStatus::ReadyToFinalize(false)));
		assert_noop!(NBVStorage::finalize_psbt(Origin::signed(test_pub(1)), proposal_id,false), Error::<Test>::PendingProposalRequired);
	});
}

#[test]
fn finalize_psbt_without_signatures_shouldnt_work(){
	new_test_ext().execute_with(|| {
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(1)), dummy_xpub()) );
		assert_ok!( NBVStorage::set_xpub(Origin::signed(test_pub(2)), dummy_xpub_2()) );
		// Insert a normal vault
		let cosigners = BoundedVec::<<Test as frame_system::Config>::AccountId, MaxCosignersPerVault>::
		try_from([ test_pub(2),].to_vec()).unwrap();
		assert_ok!(NBVStorage::create_vault( Origin::signed(test_pub(1)) , 1, dummy_description(), true, cosigners) );
		assert!(!NBVStorage::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = NBVStorage::vaults_by_signer(test_pub(1)).pop().unwrap();
		assert_ok!(NBVStorage::propose(Origin::signed(test_pub(1)),vault_id,dummy_testnet_recipient_address(),1000,dummy_description()));
		// obtaining proposal id and saving a psbt with a user that is not in the vault
		let proposal_id = NBVStorage::proposals_by_vault(vault_id).pop().unwrap();

		assert_noop!(NBVStorage::finalize_psbt(Origin::signed(test_pub(1)), proposal_id,false), Error::<Test>::NotEnoughSignatures);
	});
}