#[cfg(test)]
mod tests {
    use candid::Principal;

    use crate::{Environment, ICRCAccount, Subaccount};

    #[test]
    fn test_account_display() {
        let account_1 = ICRCAccount::new(
            Principal::from_text("k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae")
                .unwrap(),
            None,
        );

        assert_eq!(
            account_1.to_string(),
            "k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae"
        );

        let account_2 = ICRCAccount::new(
            Principal::from_text("k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae")
                .unwrap(),
            Some(Subaccount::from_slice(&[0u8; 32]).unwrap()),
        );

        assert_eq!(
            account_2.to_string(),
            "k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae"
        );

        let account_3 = ICRCAccount::new(
            Principal::from_text("k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae")
                .unwrap(),
            Some(Subaccount::from_slice(&[1u8; 32]).unwrap()),
        );
        assert_eq!(
            account_3.to_string(),
            "k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae-7s4rpcq.101010101010101010101010101010101010101010101010101010101010101"
        );

        let mut slices = [0u8; 32];
        slices[31] = 0x01;

        let account_4 = ICRCAccount::new(
            Principal::from_text("k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae")
                .unwrap(),
            Some(Subaccount::from_slice(&slices).unwrap()),
        );

        assert_eq!(
            account_4.to_string(),
            "k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae-6cc627i.1"
        );

        let slices = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
            0x1d, 0x1e, 0x1f, 0x20,
        ];

        let account_5 = ICRCAccount::new(
            Principal::from_text("k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae")
                .unwrap(),
            Some(Subaccount::from_slice(&slices).unwrap()),
        );

        assert_eq!(
            account_5.to_string(),
            "k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae-dfxgiyy.102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20"
        );
    }

    #[test]
    fn test_account_parsing() {
        let account_1 = ICRCAccount::from_text(
            "k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae",
        )
        .unwrap();

        let expected_1 = ICRCAccount::new(
            Principal::from_text("k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae")
                .unwrap(),
            None,
        );

        assert_eq!(account_1, expected_1,);

        let account_2 = "k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae"
            .parse::<ICRCAccount>()
            .unwrap();

        let expected_2 = ICRCAccount::new(
            Principal::from_text("k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae")
                .unwrap(),
            Some(Subaccount([0u8; 32])),
        );

        assert_eq!(account_2, expected_2);

        let account_3 = ICRCAccount::from_text(
            "k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae-7s4rpcq.101010101010101010101010101010101010101010101010101010101010101"
        ).unwrap();

        let expected_3 = ICRCAccount::new(
            Principal::from_text("k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae")
                .unwrap(),
            Some(Subaccount([1u8; 32])),
        );

        assert_eq!(account_3, expected_3);

        let mut slices = [0u8; 32];
        slices[31] = 0x01;

        let account_4 = ICRCAccount::new(
            Principal::from_text("k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae")
                .unwrap(),
            Some(Subaccount(slices)),
        );

        assert_eq!(
            account_4,
            "k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae-6cc627i.1"
                .parse::<ICRCAccount>()
                .unwrap()
        );

        let slices = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
            0x1d, 0x1e, 0x1f, 0x20,
        ];

        let account_5 = ICRCAccount::new(
            Principal::from_text("k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae")
                .unwrap(),
            Some(Subaccount(slices)),
        );

        assert_eq!(
            account_5,
            "k2t6j-2nvnp-4zjm3-25dtz-6xhaa-c7boj-5gayf-oj3xs-i43lp-teztq-6ae-dfxgiyy.102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20"
                .parse::<ICRCAccount>()
                .unwrap()
        );
    }

    const TEST_PRINCIPAL: Principal = Principal::from_slice(&[
        0, 0, 0, 0, 0, 0, 0, 7, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    #[test]
    fn test_subaccount_derivation_path() {
        let subaccount = Subaccount::new(Environment::Production, 0);
        let account = ICRCAccount::new(TEST_PRINCIPAL, None);

        assert_eq!(account.effective_subaccount(), &subaccount);
        println!("{}", account.to_text());

        let recover = ICRCAccount::from_text(&account.to_text()).unwrap();
        assert_eq!(
            recover.effective_subaccount().environment(),
            Environment::Production
        );
        println!("{:?}", recover);
        assert_eq!(recover, account);

        let subaccount = Subaccount::new(Environment::Production, 0);
        let account = ICRCAccount::new(TEST_PRINCIPAL, Some(subaccount.clone()));

        assert_eq!(account.effective_subaccount(), &subaccount);
        println!("{}", account.to_text());

        let recover = ICRCAccount::from_text(&account.to_text()).unwrap();
        assert_eq!(
            recover.effective_subaccount().environment(),
            Environment::Production
        );
        println!("{:?}", recover);
        assert_eq!(recover, account);

        let subaccount = Subaccount::new(Environment::Production, 1);
        let account = ICRCAccount::new(TEST_PRINCIPAL, Some(subaccount.clone()));

        assert_eq!(account.effective_subaccount(), &subaccount);
        println!("{}", account.to_text());

        let recover = ICRCAccount::from_text(&account.to_text()).unwrap();
        assert_eq!(
            recover.effective_subaccount().environment(),
            Environment::Production
        );
        println!("{:?}", recover);
        assert_eq!(recover, account);

        let subaccount = Subaccount::new(Environment::Production, 256);
        let account = ICRCAccount::new(TEST_PRINCIPAL, Some(subaccount.clone()));

        assert_eq!(account.effective_subaccount(), &subaccount);
        println!("{}", account.to_text());

        let recover = ICRCAccount::from_text(&account.to_text()).unwrap();
        assert_eq!(
            recover.effective_subaccount().environment(),
            Environment::Production
        );
        println!("{:?}", recover);
        assert_eq!(recover, account);

        let subaccount = Subaccount::new(Environment::Staging, 512);
        let account = ICRCAccount::new(TEST_PRINCIPAL, Some(subaccount.clone()));

        assert_eq!(account.effective_subaccount(), &subaccount);
        println!("{}", account.to_text());

        let recover = ICRCAccount::from_text(&account.to_text()).unwrap();
        assert_eq!(
            recover.effective_subaccount().environment(),
            Environment::Staging
        );
        println!("{:?}", recover);
        assert_eq!(recover, account);

        let subaccount = Subaccount::new(Environment::Development, 400);
        let account = ICRCAccount::new(TEST_PRINCIPAL, Some(subaccount.clone()));

        assert_eq!(account.effective_subaccount(), &subaccount);
        println!("{}", account.to_text());

        let recover = ICRCAccount::from_text(&account.to_text()).unwrap();
        assert_eq!(
            recover.effective_subaccount().environment(),
            Environment::Development
        );
        println!("{:?}", recover);
        assert_eq!(recover, account);

        let subaccount = Subaccount::new(Environment::Development, 1024);
        let account = ICRCAccount::new(TEST_PRINCIPAL, Some(subaccount.clone()));

        assert_eq!(account.effective_subaccount(), &subaccount);
        println!("{}", account.to_text());

        let recover = ICRCAccount::from_text(&account.to_text()).unwrap();
        assert_eq!(
            recover.effective_subaccount().environment(),
            Environment::Development
        );
        println!("{:?}", recover);
        assert_eq!(recover, account);
    }
}
