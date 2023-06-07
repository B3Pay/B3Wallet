use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};
use std::fmt;

use crate::constants::{DEVELOPMENT_PREFIX, STAGING_PREFIX};

#[derive(CandidType, Deserialize, Serialize, Clone, PartialEq, Default, Debug)]
pub enum Environment {
    Development,
    Staging,
    #[default]
    Production,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Environment::Development => write!(f, "Development"),
            Environment::Staging => write!(f, "Staging"),
            Environment::Production => write!(f, "Production"),
        }
    }
}

impl Environment {
    pub fn prefix(&self) -> u8 {
        match self {
            Environment::Production => 0,
            Environment::Staging => STAGING_PREFIX,
            Environment::Development => DEVELOPMENT_PREFIX,
        }
    }

    pub fn to_name(&self, counter: String) -> String {
        match self {
            Environment::Development => ["Development", "Account", &counter].join(" "),
            Environment::Production => ["Account", &counter].join(" "),
            Environment::Staging => ["Staging", "Account", &counter].join(" "),
        }
    }
}
