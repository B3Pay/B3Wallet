use crate::error::WalletError;
use crate::ledger::ledger::Ledger;
use crate::nonces::NonceTrait;
use crate::setting::WalletSettings;
use crate::types::{WalletAccountMap, WalletAccountView};
use crate::{account::WalletAccount, types::AccountId};
use b3_helper_lib::environment::Environment;
use b3_helper_lib::subaccount::Subaccount;
use b3_helper_lib::types::AccountsNonce;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct WalletState {
    pub accounts: WalletAccountMap,
    pub nonces: AccountsNonce,
    pub settings: WalletSettings,
}

impl Default for WalletState {
    fn default() -> Self {
        WalletState {
            nonces: AccountsNonce::default(),
            settings: WalletSettings::default(),
            accounts: WalletAccountMap::default(),
        }
    }
}

impl WalletState {
    // Init Functions
    pub fn init_wallet(&mut self, setting: WalletSettings) {
        self.init_accounts();
        self.init_setting(setting);
    }

    pub fn init_setting(&mut self, setting: WalletSettings) {
        if self.settings.initialised {
            return;
        }

        self.settings = setting;
        self.settings.initialised = true;
    }

    pub fn init_accounts(&mut self) {
        if self.accounts_len() > 0 {
            return;
        }

        let mut account = WalletAccount::from(Subaccount::default());

        account.rename("Main Account".to_owned());

        self.accounts.insert("default".to_owned(), account);

        self.nonces.increment(Environment::Production);
    }

    pub fn is_initialised(&self) -> bool {
        self.settings.initialised
    }

    pub fn set_setting(&mut self, setting: WalletSettings) {
        self.settings = setting;
    }

    // Account Functions

    pub fn new_subaccount(&self, opt_env: Option<Environment>) -> Subaccount {
        let env = opt_env.unwrap_or(Environment::Production);

        let nonce = self.account_nonce(&env);

        Subaccount::new(env, nonce)
    }

    pub fn insert_account(&mut self, mut account: WalletAccount, opt_name: Option<String>) {
        if let Some(name) = opt_name {
            account.rename(name);
        } else {
            let env = account.environment();

            let name = self.nonces.generate_next_name(env);

            account.rename(name);
        }

        let id = account.id();

        self.accounts.insert(id.clone(), account);
    }

    pub fn counters(&self) -> &AccountsNonce {
        &self.nonces
    }

    pub fn account(&self, id: &AccountId) -> Result<&WalletAccount, WalletError> {
        self.accounts
            .get(id)
            .ok_or(WalletError::WalletAccountNotExists)
    }

    pub fn account_mut(&mut self, id: &AccountId) -> Result<&mut WalletAccount, WalletError> {
        self.accounts
            .get_mut(id)
            .ok_or(WalletError::WalletAccountNotExists)
    }

    pub fn accounts_public_keys(&self) -> Vec<&Ledger> {
        self.accounts
            .iter()
            .map(|(_, account)| account.ledger())
            .collect()
    }

    pub fn account_views(&self) -> Vec<WalletAccountView> {
        self.accounts
            .values()
            .map(WalletAccountView::from)
            .collect()
    }

    pub fn accounts_len(&self) -> usize {
        self.accounts.len()
    }

    pub fn account_status(&self) -> AccountsNonce {
        self.nonces.clone().into()
    }

    pub fn account_nonce(&self, env: &Environment) -> u64 {
        self.nonces.account(env)
    }

    pub fn remove_account(&mut self, id: &String) -> Result<(), WalletError> {
        if id == "default" {
            return Err(WalletError::CannotRemoveDefaultAccount);
        }

        self.accounts
            .remove(id)
            .ok_or(WalletError::WalletAccountNotExists)?;

        Ok(())
    }

    pub fn restore_account(&mut self, subaccount: Subaccount) -> Result<(), WalletError> {
        if self.accounts.contains_key(&subaccount.id()) {
            return Err(WalletError::WalletAccountAlreadyExists);
        }

        if self.nonces.account(&subaccount.environment()) <= subaccount.nonce() {
            return Err(WalletError::WalletAccountCounterMismatch);
        }

        let name = subaccount.name();

        let account = WalletAccount::from(subaccount);

        self.insert_account(account, Some(name));

        Ok(())
    }

    pub fn reset_accounts(&mut self) {
        self.accounts.clear();
        self.nonces.reset();

        self.init_accounts();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use b3_helper_lib::environment::Environment;

    #[test]
    fn test_init_wallet() {
        let mut state = WalletState::default();

        state.init_accounts();

        assert_eq!(state.accounts_len(), 1);

        let account = state.account(&"default".to_owned()).unwrap();

        assert_eq!(account.name(), "Main Account");

        let subaccount = account.subaccount();

        assert_eq!(subaccount.environment(), Environment::Production);

        assert_eq!(subaccount.nonce(), 0);
    }

    #[test]
    fn test_new_subaccount() {
        let state = WalletState::default();

        let subaccount = state.new_subaccount(None);

        assert_eq!(subaccount.environment(), Environment::Production);
    }

    #[test]
    fn test_insert_account() {
        let mut state = WalletState::default();

        let subaccount = state.new_subaccount(None);

        let account = WalletAccount::from(subaccount);

        state.insert_account(account, None);

        assert_eq!(state.accounts_len(), 1);
    }

    #[test]
    fn test_counters() {
        let state = WalletState::default();

        let counters = state.counters();

        assert_eq!(counters.account(&Environment::Production), 0);
    }

    #[test]
    fn test_account() {
        let mut state = WalletState::default();

        let subaccount = state.new_subaccount(None);

        let account = WalletAccount::from(subaccount);

        state.insert_account(account, None);

        let account = state.account(&"default".to_owned()).unwrap();

        assert_eq!(account.name(), "Account 1");
    }

    #[test]
    fn test_account_mut() {
        let mut state = WalletState::default();

        let subaccount = state.new_subaccount(None);

        let account = WalletAccount::from(subaccount);

        state.insert_account(account, None);

        let account = state.account_mut(&"default".to_owned()).unwrap();

        account.rename("Test Account".to_owned());

        assert_eq!(account.name(), "Test Account");
    }

    #[test]
    fn test_accounts_public_keys() {
        let mut state = WalletState::default();

        let subaccount = state.new_subaccount(None);

        let account = WalletAccount::from(subaccount);

        state.insert_account(account, None);

        let public_keys = state.accounts_public_keys();

        assert_eq!(public_keys.len(), 1);
    }

    #[test]
    fn test_account_views() {
        let mut state = WalletState::default();

        let subaccount = state.new_subaccount(Some(Environment::Development));

        let account = WalletAccount::from(subaccount);

        state.insert_account(account, None);

        let account_views = state.account_views();

        assert_eq!(account_views.len(), 1);

        assert_eq!(account_views[0].name, "Development Account 1");

        assert_eq!(account_views[0].environment, Environment::Development);
    }

    #[test]
    fn test_accounts_len() {
        let mut state = WalletState::default();

        let subaccount = state.new_subaccount(None);

        let account = WalletAccount::from(subaccount);

        state.insert_account(account, None);

        assert_eq!(state.accounts_len(), 1);
    }

    #[test]
    fn test_account_status() {
        let state = WalletState::default();

        let counters = state.account_status();

        assert_eq!(counters.account(&Environment::Production), 0);
    }

    #[test]
    fn test_account_counter() {
        let mut state = WalletState::default();

        let nonce = state.account_nonce(&Environment::Production);

        assert_eq!(nonce, 0);

        state.init_accounts();

        let nonce = state.account_nonce(&Environment::Production);

        assert_eq!(nonce, 1);
    }
}
