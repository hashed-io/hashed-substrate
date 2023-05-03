use super::*;
use crate::{mock::*, types::*,Error};
use frame_system::RawOrigin;
use frame_support::{
	assert_noop, assert_ok,
	traits::Currency,
    BoundedVec,
};

fn new_account(account_id: u64) -> <Test as frame_system::Config>::AccountId {
	account_id
}

fn dummy_description() -> BoundedVec<u8, StringLimit> {
	BoundedVec::<u8, StringLimit>::try_from(b"dummy description".to_vec()).unwrap()
}

//owner_id = 1 
//admin_id = 2
//buy_fee = 2%
//sell_fee = 4%

#[test]
fn sign_up_works() {
    new_test_ext().execute_with(|| {
        let user = new_account(3);
        Balances::make_free_balance_be(&user, 100);
        let args = SignUpArgs::BuyerOrSeller {
            first_name: ShortString::try_from(b"Afloat".to_vec()).unwrap(),
            last_name: ShortString::try_from(b"User".to_vec()).unwrap(),
            email: LongString::try_from(b"Afloatuser@gmail.com".to_vec()).unwrap(),
            state: 1,
        };

        assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args));

        assert!(UserInfo::<Test>::contains_key(user));
    });
}

#[test]
fn update_user_info_edit_works() {
    new_test_ext().execute_with(|| {
        let user = new_account(3);
        Balances::make_free_balance_be(&user, 100); 

        let args = SignUpArgs::BuyerOrSeller {
            first_name: ShortString::try_from(b"Afloat".to_vec()).unwrap(),
            last_name: ShortString::try_from(b"User".to_vec()).unwrap(),
            email: LongString::try_from(b"Afloatuser@gmail.com".to_vec()).unwrap(),
            state: 1,
        };

        assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args));

        let update_args = UpdateUserArgs::Edit {
            first_name: Some(ShortString::try_from(b"New".to_vec()).unwrap()),
            last_name: Some(ShortString::try_from(b"User".to_vec()).unwrap()),
            email: Some(LongString::try_from(b"Info".to_vec()).unwrap()),
            lang_key: None,
            phone: None,
            credits_needed: None,
            cpa_id: None,
            state: None,
        };

        assert_ok!(Afloat::update_user_info(
            RawOrigin::Signed(user.clone()).into(),
            user.clone(),
            update_args
        ));

        let updated_user = UserInfo::<Test>::get(user).unwrap();
        assert_eq!(updated_user.first_name, ShortString::try_from(b"New".to_vec()).unwrap());
        assert_eq!(updated_user.last_name, ShortString::try_from(b"User".to_vec()).unwrap());
        assert_eq!(updated_user.email, LongString::try_from(b"Info".to_vec()).unwrap());
        
    });
}

#[test]
fn update_other_user_info_by_not_admin_fails() {
    new_test_ext().execute_with(|| {
        let user = new_account(3);
        let other_user = new_account(4);

        Balances::make_free_balance_be(&user, 100);
        Balances::make_free_balance_be(&other_user, 100);

        let args = SignUpArgs::BuyerOrSeller {
            first_name: ShortString::try_from(b"Afloat".to_vec()).unwrap(),
            last_name: ShortString::try_from(b"User".to_vec()).unwrap(),
            email: LongString::try_from(b"Afloatuser@gmail.com".to_vec()).unwrap(),
            state: 1,
        };

        assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args));

        let update_args = UpdateUserArgs::Edit {
            first_name: Some(ShortString::try_from(b"New".to_vec()).unwrap()),
            last_name: Some(ShortString::try_from(b"User".to_vec()).unwrap()),
            email: Some(LongString::try_from(b"Info".to_vec()).unwrap()),
            lang_key: None,
            phone: None,
            credits_needed: None,
            cpa_id: None,
            state: None,
        };

        assert_noop!(
            Afloat::update_user_info(
                RawOrigin::Signed(other_user.clone()).into(),
                user.clone(),
                update_args
            ),
            Error::<Test>::Unauthorized
        );
    });
}

#[test]
fn update_other_user_info_by_admin_works() {
    new_test_ext().execute_with(|| {
        let owner = new_account(1);
        let admin = new_account(2);
        let user = new_account(3);
        let other_user = new_account(4);

        Balances::make_free_balance_be(&owner, 100);
        Balances::make_free_balance_be(&admin, 100);
        Balances::make_free_balance_be(&user, 100);
        Balances::make_free_balance_be(&other_user, 100);

        let args = SignUpArgs::BuyerOrSeller {
            first_name: ShortString::try_from(b"Afloat".to_vec()).unwrap(),
            last_name: ShortString::try_from(b"User".to_vec()).unwrap(),
            email: LongString::try_from(b"Afloatuser@gmail.com".to_vec()).unwrap(),
            state: 1,
        };

        assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args));

        let update_args = UpdateUserArgs::Edit {
            first_name: Some(ShortString::try_from(b"New".to_vec()).unwrap()),
            last_name: Some(ShortString::try_from(b"User".to_vec()).unwrap()),
            email: Some(LongString::try_from(b"Info".to_vec()).unwrap()),
            lang_key: None,
            phone: None,
            credits_needed: None,
            cpa_id: None,
            state: None,
        };

        assert_ok!(Afloat::update_user_info(
            RawOrigin::Signed(admin.clone()).into(),
            user.clone(),
            update_args
        ));

        let updated_user = UserInfo::<Test>::get(user).unwrap();
        assert_eq!(updated_user.first_name, ShortString::try_from(b"New".to_vec()).unwrap());
        assert_eq!(updated_user.last_name, ShortString::try_from(b"User".to_vec()).unwrap());
        assert_eq!(updated_user.email, LongString::try_from(b"Info".to_vec()).unwrap());
    });
}

#[test]
fn update_user_info_delete_works() {
    new_test_ext().execute_with(|| {
        let user = new_account(3);
        Balances::make_free_balance_be(&user, 100);

        let args = SignUpArgs::BuyerOrSeller {
            first_name: ShortString::try_from(b"Afloat".to_vec()).unwrap(),
            last_name: ShortString::try_from(b"User".to_vec()).unwrap(),
            email: LongString::try_from(b"Afloatuser@gmail.com".to_vec()).unwrap(),
            state: 1,
        };

        assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args));

        assert_ok!(Afloat::update_user_info(
            RawOrigin::Signed(user.clone()).into(),
            user.clone(),
            UpdateUserArgs::Delete
        ));

        assert!(!UserInfo::<Test>::contains_key(user));
    });
}

#[test]
fn kill_storage_works() {
    new_test_ext().execute_with(|| {
        let owner = new_account(1);
        let admin = new_account(2);

        let user1 = new_account(3);
        let user2 = new_account(4);

        Balances::make_free_balance_be(&user1, 100);
        Balances::make_free_balance_be(&user2, 100);

        let args = SignUpArgs::BuyerOrSeller {
            first_name: ShortString::try_from(b"Afloat".to_vec()).unwrap(),
            last_name: ShortString::try_from(b"User".to_vec()).unwrap(),
            email: LongString::try_from(b"Afloatuser@gmail.com".to_vec()).unwrap(),
            state: 1,
        };



        // Add users
        assert_ok!(Afloat::sign_up(RawOrigin::Signed(user1.clone()).into(), args.clone()));
        assert_ok!(Afloat::sign_up(RawOrigin::Signed(user2.clone()).into(), args.clone()));

        // Ensure users exist
        assert!(UserInfo::<Test>::contains_key(user1));
        assert!(UserInfo::<Test>::contains_key(user2));

        // Kill storage with admin
        assert_ok!(Afloat::kill_storage(RawOrigin::Signed(admin.clone()).into()));

        // Ensure users no longer exist
        assert!(!UserInfo::<Test>::contains_key(user1));
        assert!(!UserInfo::<Test>::contains_key(user2));

        // Ensure admin and owner still exists
        assert!(UserInfo::<Test>::contains_key(admin));
        assert!(UserInfo::<Test>::contains_key(owner));

        // Add users again
        assert_ok!(Afloat::sign_up(RawOrigin::Signed(user1.clone()).into(), args.clone()));
        assert_ok!(Afloat::sign_up(RawOrigin::Signed(user2.clone()).into(), args.clone()));

    });
}

#[test]
fn kill_storage_fails_for_non_admin() {
    new_test_ext().execute_with(|| {
        let user = new_account(3);

        // Attempt to kill storage with non-admin user
        assert_noop!(
            Afloat::kill_storage(RawOrigin::Signed(user.clone()).into()),
            Error::<Test>::Unauthorized
        );
    });
}

#[test]
fn set_afloat_balance_works(){
    new_test_ext().execute_with(|| {
        let user = new_account(3);
        let other_user = new_account(4);

        Balances::make_free_balance_be(&user, 100);
        Balances::make_free_balance_be(&other_user, 100);

        let args = SignUpArgs::BuyerOrSeller {
            first_name: ShortString::try_from(b"Afloat".to_vec()).unwrap(),
            last_name: ShortString::try_from(b"User".to_vec()).unwrap(),
            email: LongString::try_from(b"Afloatuser@gmail.com".to_vec()).unwrap(),
            state: 1,
        };
        
        assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args.clone()));

        assert_ok!(Afloat::set_afloat_balance(RawOrigin::Signed(1).into(), other_user.clone(), 10000));
        assert_eq!(Afloat::do_get_afloat_balance(other_user.clone()), 10000);
    });

}


#[test]
fn create_tax_credit_works() {
    new_test_ext().execute_with(|| {
        let user = new_account(3);
        let other_user = new_account(4);

        Balances::make_free_balance_be(&user, 100);
        Balances::make_free_balance_be(&other_user, 100);

        let args = SignUpArgs::BuyerOrSeller {
            first_name: ShortString::try_from(b"Afloat".to_vec()).unwrap(),
            last_name: ShortString::try_from(b"User".to_vec()).unwrap(),
            email: LongString::try_from(b"Afloatuser@gmail.com".to_vec()).unwrap(),
            state: 1,
        };

        assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args.clone()));
        assert_ok!(Afloat::sign_up(RawOrigin::Signed(other_user.clone()).into(), args.clone()));


        assert_ok!(Afloat::create_tax_credit(
            RawOrigin::Signed(user.clone()).into(),
            dummy_description(),
            None,
            None,
        ));


    });
}

#[test]
fn create_sell_order_works() {
    new_test_ext().execute_with(|| {
        let user = new_account(3);
        let other_user = new_account(4);
        let item_id = 0;

        Balances::make_free_balance_be(&user, 100);
        Balances::make_free_balance_be(&other_user, 100);

        let args = SignUpArgs::BuyerOrSeller {
            first_name: ShortString::try_from(b"Afloat".to_vec()).unwrap(),
            last_name: ShortString::try_from(b"User".to_vec()).unwrap(),
            email: LongString::try_from(b"Afloatuser@gmail.com".to_vec()).unwrap(),
            state: 0,
        };

        assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args.clone()));
        assert_ok!(Afloat::sign_up(RawOrigin::Signed(other_user.clone()).into(), args.clone()));

        assert_ok!(Afloat::create_tax_credit(
            RawOrigin::Signed(3).into(),
            dummy_description(),
            None,
            None,
        ));

        assert_ok!(Afloat::create_sell_order(
            RawOrigin::Signed(user.clone()).into(),
            item_id,
            10000,
            10,
        ));

    });
    
}

#[test]
fn take_sell_order_works() {
    new_test_ext().execute_with(|| {
        let user = new_account(3);
        let other_user = new_account(4);
        let item_id = 0;

        Balances::make_free_balance_be(&user, 100);
        Balances::make_free_balance_be(&other_user, 100);
        assert_ok!(Afloat::set_afloat_balance(RuntimeOrigin::signed(1), 4, 100000));

        let args = SignUpArgs::BuyerOrSeller {
            first_name: ShortString::try_from(b"Afloat".to_vec()).unwrap(),
            last_name: ShortString::try_from(b"User".to_vec()).unwrap(),
            email: LongString::try_from(b"Afloatuser@gmail.com".to_vec()).unwrap(),
            state: 0,
        };

        assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args.clone()));
        assert_ok!(Afloat::sign_up(RawOrigin::Signed(other_user.clone()).into(), args.clone()));

        assert_ok!(Afloat::create_tax_credit(
            RawOrigin::Signed(user.clone()).into(),
            dummy_description(),
            None,
            None,
        ));

        assert_ok!(Afloat::create_sell_order(
            RawOrigin::Signed(user.clone()).into(),
            item_id,
            10000,
            10,
        ));

        let offer_id = GatedMarketplace::offers_by_item(0, 0).iter().next().unwrap().clone();

        assert_ok!(Afloat::take_sell_order(
            RawOrigin::Signed(other_user.clone()).into(),
            offer_id,
        ));

        assert_eq!(Afloat::do_get_afloat_balance(user.clone()), 9600); // 10000 - 400 (sell fee)
        assert_eq!(Afloat::do_get_afloat_balance(1), 400); // 400 (sell fee)
    });
    
}

#[test]
fn create_buy_order_works() {
    new_test_ext().execute_with(|| {
        let user = new_account(3);
        let other_user = new_account(4);
        let item_id = 0;

        Balances::make_free_balance_be(&user, 100);
        Balances::make_free_balance_be(&other_user, 100);
        assert_ok!(Afloat::set_afloat_balance(RuntimeOrigin::signed(1), 4, 100000));

        let args = SignUpArgs::BuyerOrSeller {
            first_name: ShortString::try_from(b"Afloat".to_vec()).unwrap(),
            last_name: ShortString::try_from(b"User".to_vec()).unwrap(),
            email: LongString::try_from(b"Afloatuser@gmail.com".to_vec()).unwrap(),
            state: 0,
        };

        assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args.clone()));
        assert_ok!(Afloat::sign_up(RawOrigin::Signed(other_user.clone()).into(), args.clone()));

        assert_ok!(Afloat::create_tax_credit(
            RawOrigin::Signed(user.clone()).into(),
            dummy_description(),
            None,
            None,
        ));

        assert_ok!(Afloat::create_buy_order(
            RawOrigin::Signed(other_user.clone()).into(),
            item_id,
            10000,
            10,
        ));

    });
    
}

#[test]
fn take_buy_order_works(){
    new_test_ext().execute_with(|| {
        let user = new_account(3);
        let other_user = new_account(4);
        let item_id = 0;

        Balances::make_free_balance_be(&user, 100);
        Balances::make_free_balance_be(&other_user, 100);
        assert_ok!(Afloat::set_afloat_balance(RuntimeOrigin::signed(1), 4, 100000));

        let args = SignUpArgs::BuyerOrSeller {
            first_name: ShortString::try_from(b"Afloat".to_vec()).unwrap(),
            last_name: ShortString::try_from(b"User".to_vec()).unwrap(),
            email: LongString::try_from(b"Afloatuser@gmail.com".to_vec()).unwrap(),
            state: 0,
        };

        assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args.clone()));
        assert_ok!(Afloat::sign_up(RawOrigin::Signed(other_user.clone()).into(), args.clone()));

        assert_ok!(Afloat::create_tax_credit(
            RawOrigin::Signed(user.clone()).into(),
            dummy_description(),
            None,
            None,
        ));

        assert_ok!(Afloat::create_buy_order(
            RawOrigin::Signed(other_user.clone()).into(),
            item_id,
            10000,
            10,
        ));

        let offer_id = GatedMarketplace::offers_by_item(0, 0).iter().next().unwrap().clone();

        assert_ok!(Afloat::take_buy_order(
            RawOrigin::Signed(user.clone()).into(),
            offer_id,
        ));

        assert_eq!(Afloat::do_get_afloat_balance(user.clone()), 9800); // 10000 - 200 (buy fee)
        assert_eq!(Afloat::do_get_afloat_balance(1), 200); // 200 (buy fee)

    });
    
}
