use b3_helper_lib::account::ICRCAccountError;
use ic_cdk::export::candid::{CandidType, Deserialize};
use std::fmt;

use crate::ledger::{
    btc::error::BitcoinError, ckbtc::error::CkbtcError, evm::error::EvmError, icp::error::IcpError,
    icrc::error::IcrcError,
};

#[rustfmt::skip]
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub enum LedgerError {
    BitcoinError(BitcoinError),
    CkbtcError(CkbtcError),
    IcrcError(IcrcError),
    EvmError(EvmError),
    IcpError(IcpError),
    CallError(String),
    ICRCAccountError(ICRCAccountError),
    GenerateError(String),
    EcdsaPublicKeyError(String),
    MissingAddress,
    MissingEcdsaPublicKey,
    InvalidEcdsaPublicKey,
    EcdsaPublicKeyAlreadySet,
    InvalidMessageLength,
}

#[rustfmt::skip]
impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LedgerError::InvalidMessageLength => write!(f, "Invalid message length"),
            LedgerError::MissingAddress => write!(f, "Missing address"),
            LedgerError::MissingEcdsaPublicKey => write!(f, "Missing ecdsa public key"),
            LedgerError::InvalidEcdsaPublicKey => write!(f, "Invalid ecdsa public key"),
            LedgerError::EcdsaPublicKeyAlreadySet => write!(f, "Ecdsa public key already set"),
            LedgerError::BitcoinError(ref err) => write!(f, "Bitcoin error: {}", err),
            LedgerError::ICRCAccountError(ref err) => write!(f, "ICRC account error: {}", err),
            LedgerError::EvmError(ref err) => write!(f, "EVM error: {}", err),
            LedgerError::CkbtcError(ref err) => write!(f, "CKBTC error: {}", err),
            LedgerError::IcrcError(ref err) => write!(f, "ICRC error: {}", err),
            LedgerError::IcpError(ref err) => write!(f, "ICP error: {}", err),
            LedgerError::CallError(ref msg) => write!(f, "Call error: {}", msg),
            LedgerError::GenerateError(ref msg) => write!(f, "Generate error: {}", msg),
            LedgerError::EcdsaPublicKeyError(ref msg) => write!(f, "Ecdsa public key error: {}", msg),
        }
    }
}
