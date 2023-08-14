use crate::Environment;
use candid::{CandidType, Deserialize};

mod types;

use types::{VetKDCurve, VetKDKeyId};

#[derive(CandidType, Deserialize, Clone)]
pub struct VetKDConfig {
    pub key_name: String,
    pub sign_cycles: u64,
}

impl Default for VetKDConfig {
    fn default() -> Self {
        Self::from(Environment::Development)
    }
}

impl From<Environment> for VetKDConfig {
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

impl VetKDConfig {
    pub fn key_name(&self) -> String {
        self.key_name.clone()
    }

    pub fn sign_cycles(&self) -> u64 {
        self.sign_cycles
    }

    pub fn key_id(&self) -> VetKDKeyId {
        VetKDKeyId {
            curve: VetKDCurve::Bls12_381,
            name: self.key_name.clone(),
        }
    }
}
