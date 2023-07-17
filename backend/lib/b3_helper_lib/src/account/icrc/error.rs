use candid::CandidType;
use serde::Deserialize;
use std::fmt;

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub enum ICRCAccountError {
    InvalidFormat,
    BadChecksum,
    NotCanonical,
    HexDecode(String),
    Malformed(String),
    InvalidPrincipal(String),
    InvalidSubaccount(String),
}

impl fmt::Display for ICRCAccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ICRCAccountError::BadChecksum => write!(f, "Bad checksum"),
            ICRCAccountError::NotCanonical => write!(f, "Not canonical"),
            ICRCAccountError::HexDecode(e) => write!(f, "Hex decode error: {}", e),
            ICRCAccountError::Malformed(e) => write!(f, "Malformed account: {}", e),
            ICRCAccountError::InvalidFormat => write!(f, "Invalid account format"),
            ICRCAccountError::InvalidPrincipal(e) => write!(f, "Invalid principal: {}", e),
            ICRCAccountError::InvalidSubaccount(e) => write!(f, "Invalid subaccount: {}", e),
        }
    }
}
