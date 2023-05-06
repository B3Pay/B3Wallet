use ic_cdk::api::call::CallResult;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::account::Account;
use crate::error::SignerError;
use crate::ledger::config::Environment;
use crate::ledger::keys::Keys;
use crate::ledger::subaccount::Subaccount;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct State {
    pub dev_counter: u64,
    pub prod_counter: u64,
    pub accounts: BTreeMap<String, Account>,
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
    pub fn insert_account(
        &mut self,
        mut account: Account,
        opt_name: Option<String>,
    ) -> CallResult<String> {
        let default_name = match account.env() {
            Environment::Production => {
                if self.prod_counter == 255 {
                    Err(SignerError::MaximumProductionAccountsReached)?;
                }

                self.prod_counter += 1;

                ["Account", &self.prod_counter.to_string()].join(" ")
            }
            _ => {
                if self.dev_counter == 255 {
                    Err(SignerError::MaximumDevelopmentAccountsReached)?;
                }

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

        Ok(id)
    }

    pub fn new_subaccount(&self, opt_env: Option<Environment>) -> CallResult<Subaccount> {
        if self.accounts.len() == 512 {
            Err(SignerError::MaximumAccountsReached)?;
        }

        let env = opt_env.unwrap_or(Environment::Production);

        let counter = self.account_counter(&env);

        let subaccount = Subaccount::new(env.clone(), counter);

        Ok(subaccount)
    }

    pub fn account_counter(&self, env: &Environment) -> u64 {
        match env {
            Environment::Production => self.prod_counter,
            _ => self.dev_counter,
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

    pub fn reset(&mut self) {
        self.accounts.clear();
        self.dev_counter = 0;
        self.prod_counter = 0;
    }
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}
