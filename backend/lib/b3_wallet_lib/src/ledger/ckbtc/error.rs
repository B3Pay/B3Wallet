use crate::ledger::icrc::error::{ICRC1TransferError, IcrcError};
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::fmt;

#[rustfmt::skip]
#[enum_dispatch]
#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum CkbtcError {
    IcrcError,
    MinterError,
    ICRC1TransferError,
    UpdateBalanceError,
    SendToInvalidAddress(String),
    CkbtcSwapToBtcError(String),
    CkbtcGetBtcAddressError(String),
}

#[rustfmt::skip]
impl fmt::Display for CkbtcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CkbtcError::IcrcError(ref err) => write!(f, "Icrc Error::{}", err),
            CkbtcError::MinterError(ref err) => write!(f, "Minter Error::{}", err),
            CkbtcError::UpdateBalanceError(ref err) => write!(f, "Update balance Error::{}", err),
            CkbtcError::SendToInvalidAddress(ref address) => write!(f, "Send to invalid address: {}", address),
            CkbtcError::ICRC1TransferError(ref err) => write!(f, "Transfer Error::{}", err),
            CkbtcError::CkbtcSwapToBtcError(ref msg) => write!(f, "Ckbtc swap to btc Error::{}", msg),
            CkbtcError::CkbtcGetBtcAddressError(ref msg) => write!(f, "Ckbtc get btc address Error::{}", msg),
        }
    }
}

#[rustfmt::skip]
#[enum_dispatch]
#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum MinterError {
    RetrieveBtcError,
    UpdateBalanceError,
    CallError(String),
    GetBtcAddressError(String),
    GetWithdrawalAccountError(String),
    RetrieveBtcStatusError(String),
}

#[rustfmt::skip]
impl fmt::Display for MinterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MinterError::CallError(ref msg) => write!(f, "Call error: {}", msg),
            MinterError::GetBtcAddressError(ref msg) => write!(f, "Get btc address error: {}", msg),
            MinterError::GetWithdrawalAccountError(ref msg) => write!(f, "Get withdrawal account error: {}", msg),
            MinterError::UpdateBalanceError(ref msg) => write!(f, "Update balance error: {}", msg),
            MinterError::RetrieveBtcError(ref msg) => write!(f, "Retrieve btc error: {}", msg),
            MinterError::RetrieveBtcStatusError(ref msg) => write!(f, "Retrieve btc status error: {}", msg),
        }
    }
}

#[rustfmt::skip]
#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum UpdateBalanceError {
    TemporarilyUnavailable(String), 
    AlreadyProcessing,
    NoNewUtxos { current_confirmations: Option<u32>, required_confirmations: u32 },
    GenericError { error_code: u64, error_message: String }
}

#[rustfmt::skip]
impl fmt::Display for UpdateBalanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpdateBalanceError::TemporarilyUnavailable(ref message) => write!(f, "Service is temporarily unavailable due to: {}", message),
            UpdateBalanceError::AlreadyProcessing => write!(f, "An update balance operation is already in progress."),
            UpdateBalanceError::NoNewUtxos { current_confirmations, required_confirmations } => write!(f, "No new UTXOs available. Current confirmations: {}, required confirmations: {}.", current_confirmations.unwrap_or(0), required_confirmations),
            UpdateBalanceError::GenericError { error_code, error_message } => write!(f, "A generic error occurred. Code: {}, Message: {}.", error_code, error_message),
        }
    }
}

#[rustfmt::skip]
#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum RetrieveBtcError {
    AlreadyProcessing,
    AmountTooLow(u64),
    MalformedAddress(String),
    InsufficientFunds { balance: u64 },
    TemporarilyUnavailable(String),
    GenericError { error_message: String, error_code: u64 },
}

#[rustfmt::skip]
impl fmt::Display for RetrieveBtcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RetrieveBtcError::AlreadyProcessing => write!(f, "An update balance operation is already in progress."),
            RetrieveBtcError::AmountTooLow(ref amount) => write!(f, "Amount too low: {}", amount),
            RetrieveBtcError::MalformedAddress(ref address) => write!(f, "Malformed address: {}", address),
            RetrieveBtcError::InsufficientFunds { balance } => write!(f, "Insufficient funds, balance is {}", balance),
            RetrieveBtcError::TemporarilyUnavailable(ref message) => write!(f, "Service is temporarily unavailable due to: {}", message),
            RetrieveBtcError::GenericError { error_code, error_message } => write!(f, "A generic error occurred. Code: {}, Message: {}.", error_code, error_message),
        }
    }
}
