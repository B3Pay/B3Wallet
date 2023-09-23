use b3_utils::{ledger::types::WalletAccountsNonce, nonce::Nonce, Environment};

pub trait NonceTrait {
    fn new() -> Self;
    fn total(&self) -> u64;
    fn account(&self, environment: &Environment) -> Nonce;
    fn increment(&mut self, environment: Environment) -> Nonce;
    fn generate_next_name(&mut self, environment: Environment) -> String;
    fn reset(&mut self);
}

impl NonceTrait for WalletAccountsNonce {
    fn new() -> Self {
        Self {
            development: Nonce::zero(),
            production: Nonce::zero(),
            staging: Nonce::zero(),
        }
    }

    fn total(&self) -> u64 {
        (self.development + self.production + self.staging).get()
    }

    fn account(&self, environment: &Environment) -> Nonce {
        match environment {
            Environment::Development => self.development,
            Environment::Production => self.production,
            Environment::Staging => self.staging,
        }
    }

    fn increment(&mut self, environment: Environment) -> Nonce {
        match environment {
            Environment::Development => self.development.add_64(1),
            Environment::Production => self.production.add_64(1),
            Environment::Staging => self.staging.add_64(1),
        }
    }

    /// Increment the account counter and return the new name based on the environment
    fn generate_next_name(&mut self, environment: Environment) -> String {
        let nonce = self.increment(environment.clone());

        environment.to_name(nonce.get())
    }

    fn reset(&mut self) {
        self.development = Nonce::zero();
        self.production = Nonce::zero();
        self.staging = Nonce::zero();
    }
}
