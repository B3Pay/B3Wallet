#[cfg(test)]
mod test {
    use crate::{mocks::id_mock, Environment, ICRCAccount, Subaccount};

    #[test]
    fn test_production_subaccount() {
        let subaccount = Subaccount::default();
        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ])
        );

        let subaccount = Subaccount::new(Environment::Production, 0);
        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ])
        );

        let subaccount = Subaccount::new(Environment::Production, 1);

        assert_eq!(subaccount.nonce(), 1);

        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
            ])
        );

        assert_eq!(subaccount.to_hex(), "1");

        let subaccount = "001".parse::<Subaccount>().unwrap();

        assert_eq!(
            subaccount,
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
            ])
        );

        let subaccount = Subaccount::new(Environment::Production, 512);

        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0
            ])
        )
    }

    #[test]
    fn test_development_subaccount() {
        let subaccount = Subaccount::new(Environment::Development, 0);
        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0
            ])
        );

        let subaccount = Subaccount::new(Environment::Development, 1);

        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 1
            ])
        );

        assert_eq!(subaccount.to_hex(), "ff0000000000000001");

        let subaccount = Subaccount::from_hex(
            &"0000000000000000000000000000000000000000000000ff0000000000000001",
        )
        .expect("Failed to parse subaccount");

        assert_eq!(subaccount, Subaccount::new(Environment::Development, 1));
    }

    #[test]
    fn test_staging_subaccount() {
        let subaccount = Subaccount::new(Environment::Staging, 0);
        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 170, 0, 0, 0, 0, 0, 0, 0, 0
            ])
        );

        let subaccount = Subaccount::new(Environment::Staging, 1);

        assert_eq!(
            subaccount.to_owned(),
            Subaccount([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
                0, 0, 170, 0, 0, 0, 0, 0, 0, 0, 1
            ])
        );

        assert_eq!(subaccount.to_hex(), "aa0000000000000001");
    }

    #[test]
    fn test_account_and_subaccount_with_loop() {
        for i in 0..30 {
            let env = match i % 3 {
                0 => Environment::Production,
                1 => Environment::Staging,
                2 => Environment::Development,
                _ => unreachable!(),
            };
            let nonce = i / 3;

            let subaccount = Subaccount::new(env.clone(), nonce);
            let account = ICRCAccount::new(id_mock(), Some(subaccount.clone()));

            assert_eq!(account.effective_subaccount(), &subaccount);
            println!("{}", account.to_text());

            let recover = ICRCAccount::from_text(&account.to_text()).unwrap();
            assert_eq!(recover.effective_subaccount().environment(), env);
            assert_eq!(recover.effective_subaccount().nonce(), nonce);

            assert_eq!(recover, account);
        }
    }
}
