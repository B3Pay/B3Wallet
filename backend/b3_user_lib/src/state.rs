use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::collections::BTreeMap;

use crate::account::Account;
use crate::error::SignerError;
use crate::ledger::{config::Environment, public_keys::PublicKeys, subaccount::Subaccount};

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct State {
    dev_counter: u64,
    prod_counter: u64,
    stag_counter: u64,
    accounts: BTreeMap<String, Account>,
}

impl Default for State {
    fn default() -> Self {
        State {
            dev_counter: 0,
            prod_counter: 0,
            stag_counter: 0,
            accounts: BTreeMap::new(),
        }
    }
}

impl State {
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
            account.update_name(name)
        } else {
            account.update_name(default_name);
        }

        let id = account.id();

        self.accounts.insert(id.clone(), account);

        id
    }

    pub fn new_subaccount(&self, opt_env: Option<Environment>) -> Subaccount {
        let env = opt_env.unwrap_or(Environment::Production);

        let counter = self.account_counter(&env);

        let subaccount = Subaccount::new(env.clone(), counter);

        subaccount
    }

    pub fn account_counter(&self, env: &Environment) -> u64 {
        match env {
            Environment::Production => self.prod_counter,
            Environment::Staging => self.stag_counter,
            Environment::Development => self.dev_counter,
        }
    }

    pub fn account(&self, id: &String) -> Result<&Account, SignerError> {
        self.accounts.get(id).ok_or(SignerError::AccountNotExists)
    }

    pub fn account_mut(&mut self, id: &String) -> Result<&mut Account, SignerError> {
        self.accounts
            .get_mut(id)
            .ok_or(SignerError::AccountNotExists)
    }

    pub fn account_keys(&self) -> Vec<PublicKeys> {
        self.accounts
            .iter()
            .map(|(_, account)| account.keys())
            .collect()
    }

    pub fn accounts(&self) -> Vec<Account> {
        self.accounts
            .iter()
            .map(|(_, account)| account.clone())
            .collect()
    }

    pub fn accounts_len(&self) -> u8 {
        self.accounts.len() as u8
    }

    pub fn reset(&mut self) {
        self.accounts.clear();
        self.dev_counter = 0;
        self.prod_counter = 0;
        self.stag_counter = 0;
    }
}
