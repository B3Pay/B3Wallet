use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use crate::account::Account;
use crate::error::SignerError;
use crate::ledger::{config::Environment, public_keys::PublicKeys, subaccount::Subaccount};
use crate::types::{Accounts, AccountsStatus, Metadata};

#[derive(CandidType, Deserialize, Clone)]
pub struct State {
    dev_counter: u64,
    prod_counter: u64,
    stag_counter: u64,
    metadata: Metadata,
    accounts: Accounts,
}

impl Default for State {
    fn default() -> Self {
        State {
            dev_counter: 0,
            prod_counter: 0,
            stag_counter: 0,
            metadata: Metadata::new(),
            accounts: Accounts::new(),
        }
    }
}

impl State {
    pub fn init(&mut self) {
        let mut account = Account::new(Subaccount([0; 32]));

        account.update_name("Main Account".to_owned());

        self.accounts.insert("default".to_owned(), account);
    }

    pub fn new_subaccount(&self, opt_env: Option<Environment>) -> Subaccount {
        let env = opt_env.unwrap_or(Environment::Production);

        let counter = self.account_counter(&env);

        let subaccount = Subaccount::new(env.clone(), counter);

        subaccount
    }

    pub fn insert_account(&mut self, mut account: Account, opt_name: Option<String>) -> String {
        let default_name = match account.env() {
            Environment::Production => {
                self.prod_counter += 1;

                ["Account", &self.prod_counter.to_string()].join(" ")
            }
            Environment::Staging => {
                self.stag_counter += 1;

                ["Staging Account", &self.stag_counter.to_string()].join(" ")
            }
            Environment::Development => {
                self.dev_counter += 1;

                ["Dev Account", &self.dev_counter.to_string()].join(" ")
            }
        };

        if let Some(name) = opt_name {
            account.update_name(name);
        } else {
            account.update_name(default_name);
        }

        let id = account.id();

        self.accounts.insert(id.clone(), account);

        id
    }

    pub fn remove_account(&mut self, id: &String) -> Result<(), SignerError> {
        if id == "default" {
            return Err(SignerError::CannotRemoveDefaultAccount);
        }

        self.accounts.remove(id);

        Ok(())
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn update_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn remove_metadata(&mut self, key: &String) {
        self.metadata.remove(key);
    }

    pub fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn account(&self, id: &String) -> Result<&Account, SignerError> {
        self.accounts.get(id).ok_or(SignerError::AccountNotExists)
    }

    pub fn account_mut(&mut self, id: &String) -> Result<&mut Account, SignerError> {
        self.accounts
            .get_mut(id)
            .ok_or(SignerError::AccountNotExists)
    }

    pub fn accounts_public_keys(&self) -> Vec<PublicKeys> {
        self.accounts
            .iter()
            .map(|(_, account)| account.public_keys())
            .collect()
    }

    pub fn accounts(&self) -> Vec<Account> {
        self.accounts
            .iter()
            .map(|(_, account)| account.clone())
            .collect()
    }

    pub fn accounts_len(&self) -> usize {
        self.accounts.len()
    }

    pub fn accounts_counters(&self) -> AccountsStatus {
        AccountsStatus {
            dev_counter: self.dev_counter,
            prod_counter: self.prod_counter,
            stag_counter: self.stag_counter,
        }
    }

    pub fn account_counter(&self, env: &Environment) -> u64 {
        match env {
            Environment::Production => self.prod_counter,
            Environment::Staging => self.stag_counter,
            Environment::Development => self.dev_counter,
        }
    }

    pub fn reset(&mut self) {
        self.accounts.clear();
        self.dev_counter = 0;
        self.prod_counter = 0;
        self.stag_counter = 0;
    }
}
