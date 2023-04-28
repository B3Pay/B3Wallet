use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use ic_cdk::trap;
use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::account::Account;
use crate::config::Environment;
use crate::ecdsa::Ecdsa;
use crate::keys::Keys;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct State {
    dev_counter: u8,
    prod_counter: u8,
    accounts: BTreeMap<String, Account>,
}

impl Default for State {
    fn default() -> Self {
        State {
            dev_counter: 0,
            prod_counter: 0,
            accounts: BTreeMap::new(),
        }
    }
}

impl State {
    pub fn insert_account(&mut self, mut account: Account, name: Option<String>) -> String {
        let default_name = match account.env() {
            Environment::Production => {
                if self.prod_counter == 255 {
                    trap("Maximum number of production accounts reached!");
                }

                self.prod_counter += 1;

                format!("Account {}", self.prod_counter)
            }
            _ => {
                if self.dev_counter == 255 {
                    trap("Maximum number of development accounts reached!");
                }

                self.dev_counter += 1;

                format!("Dev Account {}", self.dev_counter)
            }
        };

        if let Some(name) = name {
            account.update_name(name)
        } else {
            account.update_name(default_name);
        }

        let id = account.id();

        self.accounts.insert(id.clone(), account);

        id
    }

    pub fn ecdsa_path(&self, env: Environment, index: u8) -> Ecdsa {
        let mut path = Vec::new();

        if env == Environment::Production {
            path.extend_from_slice(&index.to_be_bytes());
        } else {
            path.extend_from_slice(&0_u8.to_be_bytes());
            path.extend_from_slice(&index.to_be_bytes());
        }

        Ecdsa::new(path, env.clone())
    }

    pub fn new_ecdsa_path(&self, env: Option<Environment>) -> Ecdsa {
        if self.accounts.len() == 512 {
            trap("Maximum number of accounts reached!");
        }

        let env = env.unwrap_or(Environment::Production);

        let counter = self.account_counter(&env);

        self.ecdsa_path(env, counter)
    }

    pub fn account_counter(&self, env: &Environment) -> u8 {
        match env {
            Environment::Production => self.prod_counter,
            _ => self.dev_counter,
        }
    }

    pub fn account(&self, id: &String) -> Option<Account> {
        self.accounts.get(id).cloned()
    }

    pub fn account_mut(&mut self, id: &String) -> Option<&mut Account> {
        self.accounts.get_mut(id)
    }

    pub fn account_keys(&self) -> Vec<Keys> {
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
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}
