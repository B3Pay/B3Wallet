use crate::{ledger::config::Environment, types::EcdsaKeyId};
use candid::Principal;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::mem::size_of;

use super::{config::Config, identifier::AccountIdentifier};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Subaccount(pub [u8; 32]);

impl Default for Subaccount {
    fn default() -> Self {
        Subaccount([0; 32])
    }
}

impl From<&Principal> for Subaccount {
    fn from(principal: &Principal) -> Self {
        let mut subaccount = [0; size_of::<Subaccount>()];
        let principal_id = principal.as_slice();

        subaccount[0] = principal_id.len().try_into().unwrap();
        subaccount[1..1 + principal_id.len()].copy_from_slice(principal_id);

        Subaccount(subaccount)
    }
}

impl Subaccount {
    pub fn new(env: Environment, index: u64) -> Self {
        let mut subaccount = [0; 32];

        match env {
            Environment::Production => subaccount[0] = 32,
            Environment::Staging => subaccount[0] = 16,
            Environment::Development => subaccount[0] = 8,
        }

        if index >= 255 {
            let count = ((index - 1) / 255) as usize;
            subaccount[1..=count].fill(255);
            subaccount[count + 1] = (index - (count as u64) * 255 - 1) as u8;
        } else {
            subaccount[1] = index as u8;
        }

        Subaccount(subaccount)
    }

    pub fn account_identifier(&self) -> AccountIdentifier {
        let canister = ic_cdk::id();

        AccountIdentifier::new(&canister, self)
    }

    pub fn env(&self) -> Environment {
        match self.0[0] {
            16 => Environment::Staging,
            8 => Environment::Development,
            _ => Environment::Production,
        }
    }

    pub fn index(&self) -> u64 {
        self.0[1..].iter().fold(0, |acc, x| acc + *x as u64)
    }

    pub fn id(&self) -> String {
        let index = self.index();

        let env_str = match self.env() {
            Environment::Production => "account",
            Environment::Staging => "staging_account",
            Environment::Development => "development_account",
        };

        [env_str, &index.to_string()].join("_")
    }

    pub fn derivation_path(&self) -> Vec<Vec<u8>> {
        vec![self.0.to_vec()]
    }

    pub fn config(&self) -> Config {
        Config::from(self)
    }

    pub fn key_id(&self) -> EcdsaKeyId {
        self.config().key_id()
    }

    pub fn key_id_with_cycles_and_path(&self) -> (EcdsaKeyId, u64, Vec<Vec<u8>>) {
        let config = self.config();

        (
            config.key_id(),
            config.sign_cycles(),
            self.derivation_path(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subaccount() {
        let subaccount = Subaccount::new(Environment::Production, 1);
        assert_eq!(subaccount.env(), Environment::Production);
        assert_eq!(subaccount.index(), 1);
        assert_eq!(subaccount.id(), "account_1");

        let subaccount = Subaccount::new(Environment::Staging, 1);
        assert_eq!(subaccount.env(), Environment::Staging);
        assert_eq!(subaccount.index(), 1);
        assert_eq!(subaccount.id(), "staging_account_1");

        let subaccount = Subaccount::new(Environment::Development, 1);
        assert_eq!(subaccount.env(), Environment::Development);
        assert_eq!(subaccount.index(), 1);
        assert_eq!(subaccount.id(), "development_account_1");
    }

    #[test]
    fn test_subaccount_from_principal() {
        let principal = Principal::from_text("rno2w-sqaaa-aaaaa-aaacq-cai").unwrap();
        let subaccount = Subaccount::from(&principal);
        assert_eq!(subaccount.env(), Environment::Production);
        assert_eq!(subaccount.index(), 7);
        assert_eq!(subaccount.id(), "account_7");
    }

    #[test]
    fn test_subaccount_derivation_path() {
        let subaccount = Subaccount::new(Environment::Production, 0);
        assert_eq!(
            subaccount.derivation_path(),
            vec![vec![
                32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ]]
        );

        let subaccount = Subaccount::new(Environment::Production, 1);
        assert_eq!(
            subaccount.derivation_path(),
            vec![vec![
                32, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ]]
        );

        let subaccount = Subaccount::new(Environment::Production, 256);
        assert_eq!(subaccount.env(), Environment::Production);
        assert_eq!(
            subaccount.derivation_path(),
            vec![vec![
                32, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0
            ]]
        );

        let subaccount = Subaccount::new(Environment::Staging, 512);
        assert_eq!(subaccount.env(), Environment::Staging);
        assert_eq!(
            subaccount.derivation_path(),
            vec![vec![
                16, 255, 255, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0
            ]]
        );

        let subaccount = Subaccount::new(Environment::Development, 1024);
        assert_eq!(subaccount.env(), Environment::Development);
        assert_eq!(
            subaccount.derivation_path(),
            vec![vec![
                8, 255, 255, 255, 255, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0
            ]]
        );
    }
}
