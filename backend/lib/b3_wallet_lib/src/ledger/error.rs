use b3_utils::ledger::ICRCAccountError;
use candid::{CandidType, Deserialize};
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
    PublicKeyError(String),
    SignatureError(String),
    ICRCAccountError(ICRCAccountError),
    GenerateError(String),
    EcdsaPublicKeyError(String),
    BtcTxIdError(String),
    PendingIndexError(usize),
    UpdateBalanceError(String),
    InvalidAmountError(String),
    InvalidChain,
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
            LedgerError::SignatureError(ref msg) => write!(f, "Signature Error::{}", msg),
            LedgerError::EcdsaPublicKeyAlreadySet => write!(f, "Ecdsa public key already set"),
            LedgerError::InvalidChain => write!(f, "Invalid chain"),
            LedgerError::BitcoinError(ref err) => write!(f, "Bitcoin error: {}", err),
            LedgerError::ICRCAccountError(ref err) => write!(f, "ICRC account error: {}", err),
            LedgerError::PublicKeyError(ref msg) => write!(f, "Public key error: {}", msg),
            LedgerError::PendingIndexError(ref msg) => write!(f, "Pending index error: {}", msg),
            LedgerError::BtcTxIdError(ref msg) => write!(f, "Btc tx id error: {}", msg),
            LedgerError::EvmError(ref err) => write!(f, "EVM error: {}", err),
            LedgerError::CkbtcError(ref err) => write!(f, "CKBTC error: {}", err),
            LedgerError::IcrcError(ref err) => write!(f, "ICRC error: {}", err),
            LedgerError::IcpError(ref err) => write!(f, "ICP error: {}", err),
            LedgerError::CallError(ref msg) => write!(f, "Call error: {}", msg),
            LedgerError::GenerateError(ref msg) => write!(f, "Generate error: {}", msg),
            LedgerError::UpdateBalanceError(ref msg) => write!(f, "Update balance error: {}", msg),
            LedgerError::EcdsaPublicKeyError(ref msg) => write!(f, "Ecdsa public key error: {}", msg),
            LedgerError::InvalidAmountError(ref msg) => write!(f, "Invalid amount error: {}", msg),
        }
    }
}
