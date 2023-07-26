use b3_utils::{types::WalletAccountsNonce, Environment};

pub trait NonceTrait {
    fn reset(&mut self);
    fn total(&self) -> u64;
    fn account(&self, environment: &Environment) -> u64;
    fn increment(&mut self, environment: Environment) -> u64;
    fn generate_next_name(&mut self, environment: Environment) -> String;
}

impl NonceTrait for WalletAccountsNonce {
    fn total(&self) -> u64 {
        self.development + self.production + self.staging
    }

    fn account(&self, environment: &Environment) -> u64 {
        match environment {
            Environment::Development => self.development,
            Environment::Production => self.production,
            Environment::Staging => self.staging,
        }
    }

    fn increment(&mut self, environment: Environment) -> u64 {
        match environment {
            Environment::Development => {
                self.development += 1;
                self.development
            }
            Environment::Production => {
                self.production += 1;
                self.production
            }
            Environment::Staging => {
                self.staging += 1;
                self.staging
            }
        }
    }

    /// Increment the account counter and return the new name based on the environment
    fn generate_next_name(&mut self, environment: Environment) -> String {
        let counter = self.increment(environment.clone()).to_string();

        environment.to_name(counter)
    }

    fn reset(&mut self) {
        self.production = 0;
        self.staging = 0;
        self.development = 0;
    }
}
