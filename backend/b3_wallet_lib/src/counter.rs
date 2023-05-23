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
    account: AccountsCounter,
    request: RequestId,
}

impl WalletCounters {
    pub fn new() -> Self {
        WalletCounters {
            account: AccountsCounter::default(),
            request: 0,
        }
    }

    pub fn total_account(&self) -> u64 {
        self.account.development + self.account.production + self.account.staging
    }

    pub fn account(&self, environment: &Environment) -> u64 {
        match environment {
            Environment::Development => self.account.development,
            Environment::Production => self.account.production,
            Environment::Staging => self.account.staging,
        }
    }

    pub fn increment_account(&mut self, environment: Environment) -> u64 {
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

    /// Increment the account counter and return the new name based on the environment
    pub fn generate_next_account_name(&mut self, environment: Environment) -> String {
        let counter = self.increment_account(environment.clone()).to_string();

        environment.to_name(counter)
    }

    pub fn increment_request(&mut self) -> RequestId {
        self.request += 1;

        self.request
    }

    pub fn request(&self) -> RequestId {
        self.request
    }

    /// increment the request counter and return the new value
    pub fn generate_next_request_id(&mut self) -> RequestId {
        self.request += 1;

        self.request
    }
}
