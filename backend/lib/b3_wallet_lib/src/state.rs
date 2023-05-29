use crate::counter::CounterTrait;
use crate::error::WalletError;
use crate::ledger::keys::Keys;
use crate::ledger::subaccount::SubaccountTrait;
use crate::types::{WalletAccountMap, WalletAccountView};
use crate::{account::WalletAccount, types::AccountId};
use b3_helper_lib::types::{AccountsCounter, Environment, Subaccount};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct WalletState {
    pub accounts: WalletAccountMap,
    pub counters: AccountsCounter,
}

impl Default for WalletState {
    fn default() -> Self {
        WalletState {
            accounts: WalletAccountMap::default(),
            counters: AccountsCounter::default(),
        }
    }
}

impl WalletState {
    // Init Functions
    pub fn init_wallet(&mut self) {
        if self.accounts_len() > 0 {
            return;
        }

        let mut account = WalletAccount::from(Subaccount::default());

        account.rename("Main Account".to_owned());

        self.accounts.insert("default".to_owned(), account);
    }

    pub fn new_subaccount(&self, opt_env: Option<Environment>) -> Subaccount {
        let env = opt_env.unwrap_or(Environment::Production);

        let counter = self.account_counter(&env);

        Subaccount::new(env, counter)
    }

    pub fn insert_account(&mut self, mut account: WalletAccount, opt_name: Option<String>) {
        if let Some(name) = opt_name {
            account.rename(name);
        } else {
            let env = account.environment();

            let name = self.counters.generate_next_name(env);

            account.rename(name);
        }

        let id = account.id();

        self.accounts.insert(id.clone(), account);
    }

    pub fn counters(&self) -> &AccountsCounter {
        &self.counters
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

    pub fn accounts_public_keys(&self) -> Vec<&Keys> {
        self.accounts
            .iter()
            .map(|(_, account)| account.public_keys())
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

    pub fn account_status(&self) -> AccountsCounter {
        self.counters.clone().into()
    }

    pub fn account_counter(&self, env: &Environment) -> u64 {
        self.counters.account(env)
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

        if self.counters.account(&subaccount.environment()) <= subaccount.nonce() {
            return Err(WalletError::WalletAccountCounterMismatch);
        }

        let name = subaccount.name();

        let account = WalletAccount::from(subaccount);

        self.insert_account(account, Some(name));

        Ok(())
    }

    pub fn reset(&mut self) {
        self.accounts.clear();
        self.counters.reset();

        self.init_wallet();
    }
}
