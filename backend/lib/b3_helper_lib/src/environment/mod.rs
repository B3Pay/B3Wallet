mod test;

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
            Environment::Development => write!(f, "development"),
            Environment::Staging => write!(f, "staging"),
            Environment::Production => write!(f, "production"),
        }
    }
}

impl Environment {
    pub fn from_identifier(identifier: u8) -> Self {
        identifier.into()
    }

    pub fn identifier(&self) -> u8 {
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

impl From<u8> for Environment {
    fn from(value: u8) -> Self {
        match value {
            0 => Environment::Production,
            STAGING_PREFIX => Environment::Staging,
            DEVELOPMENT_PREFIX => Environment::Development,
            _ => Environment::Production,
        }
    }
}
