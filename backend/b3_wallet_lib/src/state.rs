use crate::account::WalletAccount;
use crate::counter::WalletCounters;
use crate::error::WalletError;
use crate::ledger::keys::Keys;
use crate::ledger::subaccount::SubaccountTrait;
use crate::types::{AccountId, ConfirmedRequestMap, PendingRequestMap, WalletAccountMap};
use b3_helper::types::{AccountsCounter, Environment, Subaccount};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

#[derive(CandidType, Deserialize, Clone)]
pub struct State {
    pub accounts: WalletAccountMap,
    pub counters: WalletCounters,
    pub pending_requests: PendingRequestMap,
    pub confirmed_requests: ConfirmedRequestMap,
}

impl Default for State {
    fn default() -> Self {
        State {
            confirmed_requests: ConfirmedRequestMap::new(),
            pending_requests: PendingRequestMap::new(),
            accounts: WalletAccountMap::new(),
            counters: WalletCounters::new(),
        }
    }
}

impl State {
    // Init Functions
    pub fn init_wallet(&mut self) {
        if self.counters.total_account() > 0 {
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

    pub fn insert_account(
        &mut self,
        mut account: WalletAccount,
        opt_name: Option<String>,
    ) -> AccountId {
        if let Some(name) = opt_name {
            account.rename(name);
        } else {
            let env = account.environment();

            let name = self.counters.generate_next_account_name(env);

            account.rename(name);
        }

        let id = account.id();

        self.accounts.insert(id.clone(), account);

        id
    }

    pub fn counters(&self) -> WalletCounters {
        self.counters.clone()
    }

    pub fn account(&self, id: &String) -> Result<&WalletAccount, WalletError> {
        self.accounts
            .get(id)
            .ok_or(WalletError::WalletAccountNotExists)
    }

    pub fn account_mut(&mut self, id: &String) -> Result<&mut WalletAccount, WalletError> {
        self.accounts
            .get_mut(id)
            .ok_or(WalletError::WalletAccountNotExists)
    }

    pub fn accounts_public_keys(&self) -> Vec<Keys> {
        self.accounts
            .iter()
            .map(|(_, account)| account.public_keys())
            .collect()
    }

    pub fn accounts(&self) -> Vec<WalletAccount> {
        self.accounts
            .iter()
            .map(|(_, account)| account.clone())
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

    pub fn restore_account(
        &mut self,
        subaccount: Subaccount,
    ) -> Result<WalletAccount, WalletError> {
        if self.accounts.contains_key(&subaccount.id()) {
            return Err(WalletError::WalletAccountAlreadyExists);
        }

        if self.counters.account(&subaccount.environment()) <= subaccount.nonce() {
            return Err(WalletError::WalletAccountCounterMismatch);
        }

        let name = subaccount.name();

        let account = WalletAccount::from(subaccount);

        let id = self.insert_account(account, Some(name));

        self.account(&id).map(|account| account.clone())
    }
}
