#[cfg(test)]
mod test {
    use b3_utils::{nonce::Nonce, Environment};

    use crate::{account::WalletAccount, nonces::NonceTrait, state::WalletState};

    #[test]
    fn test_init_wallet() {
        let mut state = WalletState::new();

        state.init_accounts();

        assert_eq!(state.accounts_len(), 1);

        let account = state.account(&"-default".to_owned()).unwrap();

        assert_eq!(account.name(), "Main Account");

        let subaccount = account.subaccount();

        assert_eq!(subaccount.environment(), Environment::Production);

        assert_eq!(subaccount.nonce(), 0);
    }

    #[test]
    fn test_new_subaccount() {
        let state = WalletState::new();

        let subaccount = state.new_subaccount(None);

        assert_eq!(subaccount.environment(), Environment::Production);
    }

    #[test]
    fn test_insert_account() {
        let mut state = WalletState::new();

        let subaccount = state.new_subaccount(None);

        let account = WalletAccount::from(subaccount);

        state.insert_account(account, None);

        assert_eq!(state.accounts_len(), 2);
    }

    #[test]
    fn test_counters() {
        let state = WalletState::new();

        let nonces = state.counters();

        assert_eq!(nonces.account(&Environment::Production), Nonce(0));
    }

    #[test]
    fn test_account() {
        let mut state = WalletState::new();

        let subaccount = state.new_subaccount(None);

        let account = WalletAccount::from(subaccount);

        state.insert_account(account, None);

        let account = state.account(&"-default".to_owned()).unwrap();

        assert_eq!(account.name(), "Main Account");
    }

    #[test]
    fn test_account_mut() {
        let mut state = WalletState::new();

        let subaccount = state.new_subaccount(None);

        let account = WalletAccount::from(subaccount);

        state.insert_account(account, None);

        let account = state.account_mut(&"-default".to_owned()).unwrap();

        account.rename("Test Account".to_owned());

        assert_eq!(account.name(), "Test Account");
    }

    #[test]
    fn test_accounts_public_keys() {
        let mut state = WalletState::new();

        let subaccount = state.new_subaccount(None);

        let account = WalletAccount::from(subaccount);

        state.insert_account(account, None);

        let public_keys = state.accounts_public_keys();

        assert_eq!(public_keys.len(), 2);
    }

    #[test]
    fn test_account_views() {
        let mut state = WalletState::new();

        let subaccount = state.new_subaccount(Some(Environment::Development));

        let account = WalletAccount::from(subaccount);

        state.insert_account(account, None);

        let account_views = state.account_views();

        assert_eq!(account_views.len(), 2);

        assert_eq!(account_views[1].name, "Development Account 1");

        assert_eq!(account_views[1].environment, Environment::Development);
    }

    #[test]
    fn test_accounts_len() {
        let mut state = WalletState::new();

        assert_eq!(state.accounts_len(), 1);

        let subaccount = state.new_subaccount(None);

        let account = WalletAccount::from(subaccount);

        state.insert_account(account, None);

        assert_eq!(state.accounts_len(), 2);
    }

    #[test]
    fn test_account_status() {
        let state = WalletState::new();

        let counters = state.account_status();

        assert_eq!(counters.account(&Environment::Production), Nonce(0));
    }

    #[test]
    fn test_account_counter() {
        let mut state = WalletState::new();

        let nonce = state.account_nonce(&Environment::Production);

        assert_eq!(nonce.get(), 0);

        let new_account = state.new_subaccount(None);

        let account = WalletAccount::from(new_account);

        state.insert_account(account, None);

        let nonce = state.account_nonce(&Environment::Production);

        assert_eq!(nonce, Nonce(1));
    }
}
