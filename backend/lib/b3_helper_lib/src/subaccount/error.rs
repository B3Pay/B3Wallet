use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::fmt;

#[derive(CandidType, Deserialize, Debug)]
pub enum SubaccountError {
    HexError(String),
    SliceError(String),
    Base32Error(String),
    InvalidSubaccount(String),
    InvalidSubaccountLength(usize),
}

#[rustfmt::skip]
impl fmt::Display for SubaccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubaccountError::InvalidSubaccountLength(len) => write!(f, "InvalidSubaccountLength: {}", len),
            SubaccountError::InvalidSubaccount(e) => write!(f, "InvalidSubaccount: {}", e),
            SubaccountError::Base32Error(e) => write!(f, "Subaccount base32 error: {}", e),
            SubaccountError::SliceError(e) => write!(f, "Subaccount slice error: {}", e),
            SubaccountError::HexError(e) => write!(f, "Subaccount hex error: {}", e),
        }
    }
}
