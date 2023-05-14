use b3_helper::types::{AccountsCounter, Environment};
use ic_cdk::export::{candid::CandidType, serde::Deserialize};

use crate::types::RequestId;

impl From<WalletCounters> for AccountsCounter {
    fn from(status: WalletCounters) -> Self {
        status.account
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct WalletCounters {
    pub account: AccountsCounter,
    pub request: RequestId,
}

impl WalletCounters {
    pub fn new() -> Self {
        WalletCounters {
            account: AccountsCounter::default(),
            request: 0,
        }
    }

    pub fn total(&self) -> u64 {
        self.account.development + self.account.production + self.account.staging
    }

    pub fn account(&self, environment: &Environment) -> u64 {
        match environment {
            Environment::Development => self.account.development,
            Environment::Production => self.account.production,
            Environment::Staging => self.account.staging,
        }
    }

    pub fn increment(&mut self, environment: Environment) -> u64 {
        match environment {
            Environment::Development => {
                self.account.development += 1;
                self.account.development
            }
            Environment::Production => {
                self.account.production += 1;
                self.account.production
            }
            Environment::Staging => {
                self.account.staging += 1;
                self.account.staging
            }
        }
    }

    pub fn decrement(&mut self, environment: Environment) {
        match environment {
            Environment::Development => self.account.development -= 1,
            Environment::Production => self.account.production -= 1,
            Environment::Staging => self.account.staging -= 1,
        }
    }
    /// Increment the account counter and return the new name based on the environment
    pub fn generate_next_account_name(&mut self, environment: Environment) -> String {
        let counter = self.increment(environment.clone()).to_string();

        match environment {
            Environment::Development => ["Development Account", &counter].join(" "),
            Environment::Production => ["Account", &counter].join(" "),
            Environment::Staging => ["Staging Account", &counter].join(" "),
        }
    }
    /// increment the request counter and return the new value
    pub fn generate_next_request_id(&mut self) -> RequestId {
        self.request += 1;

        self.request
    }
}