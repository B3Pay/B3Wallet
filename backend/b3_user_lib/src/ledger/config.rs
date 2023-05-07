use ic_cdk::export::candid::{CandidType, Deserialize};

use super::types::{EcdsaCurve, EcdsaKeyId};

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq, Default)]
pub enum Environment {
    #[default]
    Development,
    Staging,
    Production,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Config {
    pub key_name: String,
    pub sign_cycles: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self::from(Environment::Development)
    }
}

impl From<Environment> for Config {
    fn from(env: Environment) -> Self {
        if env == Environment::Production {
            Self {
                key_name: "key_1".to_string(),
                sign_cycles: 26_153_846_153,
            }
        } else if env == Environment::Staging {
            Self {
                key_name: "test_key_1".to_string(),
                sign_cycles: 10_000_000_000,
            }
        } else {
            Self {
                key_name: "dfx_test_key".to_string(),
                sign_cycles: 0,
            }
        }
    }
}

impl Config {
    pub fn key_name(&self) -> String {
        self.key_name.clone()
    }

    pub fn sign_cycles(&self) -> u64 {
        self.sign_cycles
    }

    pub fn key_id(&self) -> EcdsaKeyId {
        EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: self.key_name.clone(),
        }
    }
}
