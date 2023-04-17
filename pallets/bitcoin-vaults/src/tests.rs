use crate::{
	mock::*,
	types::{BDKStatus, Descriptors, ProposalStatus},
	Error, ProofOfReserves, Proposals, Vaults,
};
use codec::Encode;
use core::convert::TryFrom;
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_core::sr25519::Public;
use sp_io::hashing::blake2_256;
use sp_runtime::DispatchResult;

static XPUBS: [&str;4] = ["[adc450e3/84'/1'/0'/0]tpubDEMkzn5sBo8Nct35y2BEFhJTqhsa72yeUf5S6ymb85G6LW2okSh1fDkrMhgCtYsrsCAuspm4yVjC63VUA6qrcQ54tVm5TKwhWFBLyyCjabX/*",
"[621c051d/123456789'/123456789'/123456789'/123456789]tpubDF3cwMypW7CJnZ4WwzwgYkd1bJzJsPTnLbFN3zdeGKfEx38jDjBzRntupghKC6A5szrjELasjrhBRXStKKUmS8wHZQxkVPN7P88iXxbC3s1/*",
"Zpub74kbYv5LXvBaJRcbSiihEEwuDiBSDztjtpSVmt6C6nB3ntbcEy4pLP3cJGVWsKbYKaAynfCwXnkuVncPGQ9Y4XwWJDWrDMUwTztdxBe7GcM",
"Zpub75bKLk9fCjgfELzLr2XS5TEcCXXGrci4EDwAcppFNBDwpNy53JhJS8cbRjdv39noPDKSfzK7EPC1Ciyfb7jRwY7DmiuYJ6WDr2nEL6yTkHi"
];

fn gen_xpub(i: usize) -> BoundedVec<u8, XPubLen> {
	BoundedVec::<u8, XPubLen>::try_from(XPUBS.get(i).unwrap().as_bytes().to_vec()).unwrap()
}

fn dummy_description() -> BoundedVec<u8, VaultDescriptionMaxLen> {
	BoundedVec::<u8,VaultDescriptionMaxLen>::try_from(b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".encode())
				.expect("Error on encoding the description to BoundedVec")
}

fn dummy_testnet_recipient_address() -> BoundedVec<u8, XPubLen> {
	BoundedVec::<u8, XPubLen>::try_from(
		b"tb1qhfku74zsrhvre7053xqsnh36gsey3ur7slwwnfn04g5506rmdchqrf7w30".encode(),
	)
	.expect("Error on encoding the xpub key to BoundedVec")
}

fn dummy_psbt() -> BoundedVec<u8, PSBTMaxLen> {
	BoundedVec::<u8, PSBTMaxLen>::try_from(b"cHNidP8BAK4BAAAAAa5F8SDWH2Hlqgky89rGlhG/4DnKqcbRlL+jQ6F0FBP5AQAAAAD9////AhAnAAAAAAAAR1IhApLkOyyLZWwh3QTadFlmp7x3xt+dPgyL/tQ47r+exVRiIQO7Ut/DRj54BKrR0Kf7c42enyfrbV4TDSpsMiqhfrnQm1KuokkAAAAAAAAiACD0hQx+A3+kUAR7iBY5VjkG2DViANmiP0xOBPixU1x36AAAAAAAAQDqAgAAAAABAecL0e2g6vO11ZpVRcHuBDFZNdXUqcDOmYsg7lK86S3cAAAAAAD+////AlpmwwIAAAAAFgAU370BMJPnoYItIaum9dnKt8LCLI8wdQAAAAAAACIAIILP1EkLWcvTQ15pBdk3paMwDIvglbUG6FQBBon3sRAMAkcwRAIgOYjunqLCM9LhnLS9lVoPSVR6z5Phk9BxodHar/ncgGgCIALhH3N/Q1yD7FxE7SSA9sogkcW3WXH1kxy3BLuMcU1zASECoJ99bEErPxgEAT+Nt7GhfwlgQ24fC//v/3LCUQnpzzBkgSEAAQErMHUAAAAAAAAiACCCz9RJC1nL00NeaQXZN6WjMAyL4JW1BuhUAQaJ97EQDAEFR1IhAip4P8CC/dZji38IFOD6ZjW50Pv3RazsvZExGHoy+MupIQPjlUrnEv00n6ytsa4sIMXdSjKHlXn94P4PBuOifenW51KuIgYCKng/wIL91mOLfwgU4PpmNbnQ+/dFrOy9kTEYejL4y6kMsCkMNwAAAAAAAAAAIgYD45VK5xL9NJ+srbGuLCDF3Uoyh5V5/eD+Dwbjon3p1ucMyqFhVgAAAAAAAAAAAAAiAgMacPy3H41FU/Xw+P81xScxWS/jO5Ny1gGnON1fo+4zbQzKoWFWAQAAAAAAAAAiAgOZ2MtgB/5WFgVoNU56XwjdHdTDuO2TYeQNe8TSV2tq7QywKQw3AQAAAAAAAAAA"
		.encode()).unwrap_or_default()
}

fn dummy_descriptor() -> BoundedVec<u8, OutputDescriptorMaxLen> {
	let d_size: usize = OutputDescriptorMaxLen::get().try_into().unwrap();
	BoundedVec::<u8, OutputDescriptorMaxLen>::try_from(vec![0; d_size]).unwrap()
}

fn gen_cosigners(cosigners_acc: &[u8]) -> BoundedVec<Public, MaxCosignersPerVault> {
	let o = cosigners_acc.into_iter().map(|&acc| test_pub(acc)).collect::<Vec<Public>>();
	BoundedVec::<Public, MaxCosignersPerVault>::try_from(o).unwrap()
}

fn set_xpub(acc_to_set: u8, xpub_index: usize) -> DispatchResult {
	BitcoinVaults::set_xpub(RuntimeOrigin::signed(test_pub(acc_to_set)), gen_xpub(xpub_index))
}

fn create_vault(
	owner_acc_index: u8,
	threshold: u32,
	include_owner: bool,
	cosigners: &[u8],
) -> DispatchResult {
	BitcoinVaults::create_vault(
		RuntimeOrigin::signed(test_pub(owner_acc_index)),
		threshold,
		dummy_description(),
		include_owner,
		gen_cosigners(cosigners),
	)
}

fn make_vault_valid(vault_id: [u8; 32]) {
	Vaults::<Test>::mutate(vault_id, |v_option| {
		let v = v_option.as_mut().unwrap();
		v.offchain_status.clone_from(&BDKStatus::Valid);
		v.descriptors.clone_from(&Descriptors {
			output_descriptor: dummy_descriptor(),
			change_descriptor: Some(dummy_descriptor()),
		});
	});
}

fn propose(proposer: u8, vault_id: [u8; 32], amount: u64) -> DispatchResult {
	BitcoinVaults::propose(
		RuntimeOrigin::signed(test_pub(proposer)),
		vault_id,
		dummy_testnet_recipient_address(),
		amount,
		dummy_description(),
	)
}

fn make_proposal_valid(proposal_id: [u8; 32]) {
	Proposals::<Test>::mutate(proposal_id, |p_option| {
		let p = p_option.as_mut().unwrap();
		p.offchain_status.clone_from(&BDKStatus::Valid);
	});
}

fn save_psbt(author: u8, proposal_id: [u8; 32]) -> DispatchResult {
	BitcoinVaults::save_psbt(RuntimeOrigin::signed(test_pub(author)), proposal_id, dummy_psbt())
}

fn create_proof(author: u8, vault_id: [u8; 32]) -> DispatchResult {
	BitcoinVaults::create_proof(
		RuntimeOrigin::signed(test_pub(author)),
		vault_id,
		dummy_description(),
		dummy_psbt(),
	)
}

fn save_proof_psbt(author: u8, vault_id: [u8; 32]) -> DispatchResult {
	BitcoinVaults::save_proof_psbt(RuntimeOrigin::signed(test_pub(author)), vault_id, dummy_psbt())
}

fn finalize_proof(author: u8, vault_id: [u8; 32]) -> DispatchResult {
	BitcoinVaults::finalize_proof(RuntimeOrigin::signed(test_pub(author)), vault_id, dummy_psbt())
}

#[test]
fn set_xpub_identity_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(set_xpub(1, 0));
		assert_eq!(
			BitcoinVaults::xpubs_by_owner(test_pub(1)),
			Some(gen_xpub(0).using_encoded(blake2_256))
		);
		print!("{:?}", gen_xpub(1));
	});
}

#[test]
fn inserting_same_xpub_should_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_noop!(set_xpub(2, 0), Error::<Test>::XPubAlreadyTaken);
	});
}

#[test]
fn inserting_without_removing_xpub_should_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_noop!(set_xpub(1, 1), Error::<Test>::UserAlreadyHasXpub);
	});
}

#[test]
fn removing_xpub_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(BitcoinVaults::remove_xpub(RuntimeOrigin::signed(test_pub(1))));
	});
}

#[test]
fn replacing_xpub_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(BitcoinVaults::remove_xpub(RuntimeOrigin::signed(test_pub(1))));
		assert_ok!(set_xpub(1, 1));
	});
}

#[test]
fn removing_twice_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(BitcoinVaults::remove_xpub(RuntimeOrigin::signed(test_pub(1))));
		assert_noop!(
			BitcoinVaults::remove_xpub(RuntimeOrigin::signed(test_pub(1))),
			Error::<Test>::XPubNotFound
		);
	});
}

#[test]
fn creating_vault_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		assert_ok!(create_vault(1, 2, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
	});
}

#[test]
fn vault_without_cosigners_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_noop!(create_vault(1, 1, true, &[]), Error::<Test>::NotEnoughCosigners);
		assert!(BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
	});
}

#[test]
fn vault_with_invalid_threshold_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		assert_noop!(create_vault(1, 0, true, &[2]), Error::<Test>::InvalidVaultThreshold);
		assert_noop!(create_vault(1, 3, true, &[2]), Error::<Test>::InvalidVaultThreshold);
		assert!(BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
	});
}

#[test]
fn vault_with_duplicate_members_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		assert_noop!(create_vault(1, 1, true, &[1, 2]), Error::<Test>::DuplicateVaultMembers);
		assert!(BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		assert!(BitcoinVaults::vaults_by_signer(test_pub(2)).is_empty());
	});
}

#[test]
fn vault_with_duplicate_incomplete_members() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_noop!(create_vault(1, 1, true, &[1, 2]), Error::<Test>::DuplicateVaultMembers);
		assert!(BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		assert!(BitcoinVaults::vaults_by_signer(test_pub(2)).is_empty());
	});
}

#[test]
fn exceeding_max_cosigners_per_vault_should_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		assert_ok!(set_xpub(3, 2));
		assert_ok!(set_xpub(4, 3));
		assert_noop!(
			create_vault(1, 2, true, &[2, 3, 4]),
			Error::<Test>::ExceedMaxCosignersPerVault
		);
		assert!(BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
	});
}

#[test]
fn vault_signer_without_xpub_shouldnt_exist() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		// Case 1: cosigner with no xpub
		assert_noop!(create_vault(1, 1, true, &[2]), Error::<Test>::XPubNotFound);
		assert!(BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		assert!(BitcoinVaults::vaults_by_signer(test_pub(2)).is_empty());
		// Case 2: owner with no xpub
		assert_noop!(create_vault(2, 1, true, &[1]), Error::<Test>::XPubNotFound);
		assert!(BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		assert!(BitcoinVaults::vaults_by_signer(test_pub(2)).is_empty());
	});
}

#[test]
fn signer_reached_max_vaults() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		assert_ok!(set_xpub(3, 2));

		assert_ok!(create_vault(1, 2, true, &[2]));
		assert_ok!(create_vault(1, 3, true, &[2, 3]));
		assert_noop!(create_vault(1, 2, true, &[3]), Error::<Test>::SignerVaultLimit);

		assert_eq!(BitcoinVaults::vaults_by_signer(test_pub(1)).len(), 2);
	});
}

#[test]
fn removing_vault_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		assert_ok!(set_xpub(3, 2));

		// Insert a normal vault
		assert_ok!(create_vault(1, 1, false, &[2, 3]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		// Try to remove xpub (vault depends on it)
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		assert_ok!(BitcoinVaults::remove_vault(RuntimeOrigin::signed(test_pub(1)), vault_id));
	});
}

#[test]
fn removing_vault_which_isnt_yours_shoulnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));

		// Insert a normal vault
		assert_ok!(create_vault(1, 2, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		// Try to remove xpub (vault depends on it)
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		assert_noop!(
			BitcoinVaults::remove_vault(RuntimeOrigin::signed(test_pub(2)), vault_id),
			Error::<Test>::VaultOwnerPermissionsNeeded
		);
	});
}

#[test]
fn removing_vault_and_xpub_in_order_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));

		// Insert a normal vault
		assert_ok!(create_vault(1, 2, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		// TODO: Remove vault
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		assert_ok!(BitcoinVaults::remove_vault(RuntimeOrigin::signed(test_pub(1)), vault_id));
		// Try to remove xpub (vault depends on it)
		assert_ok!(BitcoinVaults::remove_xpub(RuntimeOrigin::signed(test_pub(1))));
	});
}

#[test]
fn removing_xpub_before_vault_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));

		// Insert a normal vault
		assert_ok!(create_vault(1, 2, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		// Try to remove xpub (vault depends on it)
		assert_noop!(
			BitcoinVaults::remove_xpub(RuntimeOrigin::signed(test_pub(1))),
			Error::<Test>::XpubLinkedToVault
		);
	});
}

#[test]
fn proposing_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 2, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);
		assert_ok!(propose(1, vault_id, 1000));
	});
}

#[test]
fn proposing_from_external_user_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 2, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		// user 3 is not on the vault so it should expect an error
		assert_noop!(propose(3, vault_id, 1000), Error::<Test>::SignerPermissionsNeeded);
	});
}

#[test]
fn proposing_twice_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 2, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);
		assert_ok!(propose(1, vault_id, 1000));
		assert_noop!(propose(1, vault_id, 1000), Error::<Test>::AlreadyProposed);
	});
}

#[test]
fn exceeding_max_proposals_per_vault_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 2, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);
		assert_ok!(propose(1, vault_id, 1000));
		assert_ok!(propose(1, vault_id, 1001));
		assert_noop!(propose(1, vault_id, 1002), Error::<Test>::ExceedMaxProposalsPerVault);
	});
}

#[test]
fn saving_psbt_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 2, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);
		assert_ok!(propose(1, vault_id, 1000));
		// obtaining proposal id and saving a psbt
		let proposal_id = BitcoinVaults::proposals_by_vault(vault_id).pop().unwrap();
		assert_ok!(save_psbt(1, proposal_id,));
	});
}

#[test]
fn saving_psbt_to_a_nonexistent_proposal_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 2, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		// user 3 is not on the vault so it should expect an error
		let proposal_id = [0; 32];
		assert_noop!(save_psbt(1, proposal_id), Error::<Test>::ProposalNotFound);
	});
}

#[test]
fn saving_psbt_form_external_user_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 2, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);
		assert_ok!(propose(1, vault_id, 1000));
		// obtaining proposal id and saving a psbt with a user that is not in the vault
		let proposal_id = BitcoinVaults::proposals_by_vault(vault_id).pop().unwrap();
		// user 3 is not on
		assert_noop!(save_psbt(3, proposal_id), Error::<Test>::SignerPermissionsNeeded);
	});
}

#[test]
fn saving_twice_psbt_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 2, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);
		assert_ok!(propose(1, vault_id, 1000,));
		// obtaining proposal id and saving a psbt with a user that is not in the vault
		let proposal_id = BitcoinVaults::proposals_by_vault(vault_id).pop().unwrap();
		// user 3 is not on the vaults cosigners
		assert_ok!(save_psbt(1, proposal_id));
		assert_noop!(save_psbt(1, proposal_id), Error::<Test>::AlreadySigned);
	});
}

// TODO: Set offchainStatus proposal from pending to Valid
#[test]
fn finalize_psbt_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 1, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);
		assert_ok!(propose(1, vault_id, 1000,));
		// obtaining proposal id and saving a psbt with a user that is not in the vault
		let proposal_id = BitcoinVaults::proposals_by_vault(vault_id).pop().unwrap();
		make_proposal_valid(proposal_id);

		assert_ok!(save_psbt(1, proposal_id));
		// When a proposal meets the threshold changes it status to ReadyToFinalize false
		assert!(BitcoinVaults::proposals(proposal_id)
			.unwrap()
			.status
			.eq(&ProposalStatus::ReadyToFinalize(false)));
	});
}

#[test]
fn finalize_psbt_twice_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 1, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);
		assert_ok!(propose(1, vault_id, 1000,));
		// obtaining proposal id and saving a psbt with a user that is not in the vault
		let proposal_id = BitcoinVaults::proposals_by_vault(vault_id).pop().unwrap();
		make_proposal_valid(proposal_id);

		assert_ok!(save_psbt(1, proposal_id));
		// When a proposal meets the threshold changes it status to ReadyToFinalize false
		assert!(BitcoinVaults::proposals(proposal_id)
			.unwrap()
			.status
			.eq(&ProposalStatus::ReadyToFinalize(false)));
		assert_noop!(
			BitcoinVaults::finalize_psbt(RuntimeOrigin::signed(test_pub(1)), proposal_id, false),
			Error::<Test>::PendingProposalRequired
		);
	});
}

#[test]
fn finalize_psbt_without_signatures_shouldnt_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 1, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);
		assert_ok!(propose(1, vault_id, 1000,));
		// obtaining proposal id and saving a psbt with a user that is not in the vault
		let proposal_id = BitcoinVaults::proposals_by_vault(vault_id).pop().unwrap();
		make_proposal_valid(proposal_id);

		assert_noop!(
			BitcoinVaults::finalize_psbt(RuntimeOrigin::signed(test_pub(1)), proposal_id, false),
			Error::<Test>::NotEnoughSignatures
		);
	});
}

#[test]
fn proof_of_reserve_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 1, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);
		// user 3 is not on the vault so it should expect an error
		assert_ok!(create_proof(1, vault_id));
	});
}

#[test]
fn proof_of_reserve_from_external_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 1, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);
		// user 3 is not on the vault so it should expect an error
		assert_noop!(create_proof(3, vault_id), Error::<Test>::SignerPermissionsNeeded);
	});
}
#[test]
fn proof_of_reserve_from_nonexistent_vault_should_not_work() {
	new_test_ext().execute_with(|| {
		let vault_id = [0; 32];
		// user 3 is not on the vault so it should expect an error
		assert_noop!(create_proof(1, vault_id), Error::<Test>::VaultNotFound);
	});
}

#[test]
fn proof_of_reserve_from_invalid_vault_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 1, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		// user 3 is not on the vault so it should expect an error
		assert_noop!(create_proof(1, vault_id), Error::<Test>::InvalidVault);
	});
}

#[test]
fn save_proof_psbt_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 1, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);
		// user 3 is not on the vault so it should expect an error
		assert_ok!(create_proof(1, vault_id));

		assert_ok!(save_proof_psbt(1, vault_id));
	});
}

#[test]
fn save_nonexistent_proof_psbt_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 1, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);

		assert_noop!(save_proof_psbt(1, vault_id), Error::<Test>::ProofNotFound);
	});
}

#[test]
fn save_proof_psbt_invalid_vault_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 1, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		// Invalid vault error should appear first than ProofNotFound
		assert_noop!(save_proof_psbt(1, vault_id), Error::<Test>::InvalidVault);
	});
}

#[test]
fn save_proof_psbt_nonexistent_vault_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));

		let vault_id = [0; 32];
		// Invalid vault error should appear first than ProofNotFound
		assert_noop!(save_proof_psbt(1, vault_id), Error::<Test>::VaultNotFound);
	});
}

#[test]
fn save_twice_proof_psbt_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 1, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);
		assert_ok!(create_proof(1, vault_id));
		assert_ok!(save_proof_psbt(1, vault_id));

		assert_noop!(save_proof_psbt(1, vault_id,), Error::<Test>::AlreadySigned);
	});
}

#[test]
fn ready_to_finalize_proof_psbt_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 1, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);

		assert_ok!(create_proof(1, vault_id));

		assert_ok!(save_proof_psbt(1, vault_id));

		assert_ok!(save_proof_psbt(2, vault_id));
		assert!(ProofOfReserves::<Test>::get(vault_id).unwrap().status.is_ready_to_finalize())
	});
}

#[test]
fn finalize_proof_psbt_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 1, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);

		assert_ok!(create_proof(1, vault_id));

		assert_ok!(save_proof_psbt(1, vault_id));

		assert_ok!(save_proof_psbt(2, vault_id));

		assert_ok!(finalize_proof(2, vault_id));
		assert_eq!(
			ProofOfReserves::<Test>::get(vault_id).unwrap().status,
			ProposalStatus::Broadcasted
		)
	});
}

#[test]
fn finalize_proof_twice_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 1, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);

		assert_ok!(create_proof(1, vault_id));

		assert_ok!(save_proof_psbt(1, vault_id));
		assert_ok!(save_proof_psbt(2, vault_id));

		assert_ok!(finalize_proof(2, vault_id));
		assert_noop!(finalize_proof(2, vault_id), Error::<Test>::AlreadyBroadcasted);
	});
}

#[test]
fn finalize_incomplete_proof_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 2, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);

		assert_ok!(create_proof(1, vault_id));

		assert_ok!(save_proof_psbt(1, vault_id));
		assert_noop!(finalize_proof(2, vault_id), Error::<Test>::NotEnoughSignatures);
	});
}

#[test]
fn finalize_nonexistent_proof_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(set_xpub(1, 0));
		assert_ok!(set_xpub(2, 1));
		// Insert a normal vault
		assert_ok!(create_vault(1, 1, true, &[2]));
		assert!(!BitcoinVaults::vaults_by_signer(test_pub(1)).is_empty());
		let vault_id = BitcoinVaults::vaults_by_signer(test_pub(1)).pop().unwrap();
		make_vault_valid(vault_id);

		assert_noop!(finalize_proof(2, vault_id), Error::<Test>::ProofNotFound);
	});
}
