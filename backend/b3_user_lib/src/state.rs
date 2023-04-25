use ic_cdk::export::{candid::CandidType, serde::Deserialize, Principal};
use ic_cdk::trap;
use std::cell::RefCell;

use crate::account::Account;
use crate::config::Environment;
use crate::keys::Keys;
use crate::subaccount::Subaccount;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct State {
    subaccount: Subaccount,
    accounts: Vec<Account>,
    dev_counter: u8,
    prod_counter: u8,
}

impl Default for State {
    fn default() -> Self {
        State {
            subaccount: Subaccount::default(),
            accounts: Vec::with_capacity(512),
            prod_counter: 0,
            dev_counter: 0,
        }
    }
}

impl State {
    pub fn init(&mut self, caller: Principal) {
        if self.accounts.len() > 0 {
            trap("State already initialized!");
        }

        self.subaccount = Subaccount::new(&caller);
    }

    pub fn insert_account(&mut self, mut account: Account, name: Option<String>) -> u8 {
        if self.accounts.len() == self.accounts.capacity() {
            trap("Maximum number of accounts reached!");
        }

        let id = match account.env() {
            Environment::Production => {
                self.prod_counter += 1;

                self.prod_counter
            }
            _ => {
                self.dev_counter += 1;

                self.dev_counter
            }
        };

        if let Some(name) = name {
            account.update_name(name)
        } else {
            account.update_name(format!("Account {}", id));
        }

        self.accounts.push(account);

        self.accounts.len() as u8
    }

    pub fn drivation_path(&self, env: &Environment, id: u8) -> Vec<u8> {
        self.subaccount.derive_hd_path(env, id)
    }

    pub fn new_drivation_path(&self, env: &Environment) -> Vec<u8> {
        if self.accounts.len() == self.accounts.capacity() {
            trap("Maximum number of accounts reached!");
        }

        let id = self.next_account_id(env);

        self.subaccount.derive_hd_path(env, id)
    }

    pub fn next_account_id(&self, env: &Environment) -> u8 {
        match env {
            Environment::Production => self.prod_counter + 1,
            _ => self.dev_counter + 1,
        }
    }

    pub fn account(&self, index: u8) -> Option<Account> {
        self.accounts.get(index as usize).cloned()
    }

    pub fn account_key(&self, id: String) -> Option<Keys> {
        self.accounts
            .iter()
            .find(|account| account.id() == id)
            .map(|account| account.keys())
    }

    pub fn account_keys(&self) -> Vec<Keys> {
        self.accounts.iter().map(|account| account.keys()).collect()
    }

    pub fn accounts_len(&self) -> u8 {
        self.accounts.len() as u8
    }

    pub fn accounts(&self) -> Vec<Account> {
        self.accounts.clone()
    }
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}
