use b3_utils::{nonce::Nonce, Environment};
use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Default, Serialize, Deserialize, Clone)]
pub struct WalletAccountsNonce {
    development: Nonce,
    production: Nonce,
    staging: Nonce,
}

impl WalletAccountsNonce {
    pub fn new() -> Self {
        Self {
            development: Nonce::zero(),
            production: Nonce::zero(),
            staging: Nonce::zero(),
        }
    }

    pub fn total(&self) -> u64 {
        (self.development + self.production + self.staging).get()
    }

    pub fn account(&self, environment: &Environment) -> Nonce {
        match environment {
            Environment::Development => self.development,
            Environment::Production => self.production,
            Environment::Staging => self.staging,
        }
    }

    pub fn increment(&mut self, environment: Environment) -> Nonce {
        match environment {
            Environment::Development => self.development.add_64(1),
            Environment::Production => self.production.add_64(1),
            Environment::Staging => self.staging.add_64(1),
        }
    }

    /// Increment the account counter and return the new name based on the environment
    pub fn generate_next_name(&mut self, environment: Environment) -> String {
        let nonce = self.increment(environment);

        environment.to_name(nonce.get())
    }

    pub fn reset(&mut self) {
        self.development = Nonce::zero();
        self.production = Nonce::zero();
        self.staging = Nonce::zero();
    }
}
