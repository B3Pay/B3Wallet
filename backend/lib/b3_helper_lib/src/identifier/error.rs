use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};
use std::fmt;

#[derive(CandidType, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum AccountIdentifierError {
    InvalidLength,
    InvalidAccountIdentifier,
}

#[rustfmt::skip]
impl fmt::Display for AccountIdentifierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountIdentifierError::InvalidLength => write!(f, "Invalid length"),
            AccountIdentifierError::InvalidAccountIdentifier => write!(f, "Invalid account identifier")
        }
    }
}
