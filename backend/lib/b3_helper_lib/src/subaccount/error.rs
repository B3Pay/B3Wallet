use candid::CandidType;
use serde::Deserialize;
use std::fmt;

#[derive(CandidType, Deserialize, Debug)]
pub enum SubaccountError {
    NotPrincipal,
    HexError(String),
    LengthError(usize),
    Base32Error(String),
}

#[rustfmt::skip]
impl fmt::Display for SubaccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubaccountError::NotPrincipal => write!(f, "The subaccount is not a principal"),
            SubaccountError::Base32Error(e) => write!(f, "Subaccount base32 error: {}", e),
            SubaccountError::LengthError(len) => write!(f, "The slice length is not 32 bytes: {}", len),
            SubaccountError::HexError(e) => write!(f, "Subaccount hex error: {}", e),
        }
    }
}
