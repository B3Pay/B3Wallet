use super::types::ICRCTimestamp;
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{
    candid::{CandidType, Nat},
    serde::Deserialize,
};
use std::fmt;

#[rustfmt::skip]
#[enum_dispatch]
#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum IcrcError {
    ICRC1TransferError,
    CallError(String),
}

impl fmt::Display for IcrcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IcrcError::ICRC1TransferError(ref err) => write!(f, "ICRC1 transfer error: {}", err),
            IcrcError::CallError(ref msg) => write!(f, "Call error: {}", msg),
        }
    }
}

#[rustfmt::skip]
#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum ICRC1TransferError {
    BadFee { expected_fee: Nat },
    BadBurn { min_burn_amount: Nat },
    InsufficientFunds { balance: Nat },
    TooOld,
    CreatedInFuture { ledger_time: ICRCTimestamp },
    Duplicate { duplicate_of: Nat },
    TemporarilyUnavailable,
    GenericError { error_code: Nat, message: String },
}

#[rustfmt::skip]
impl fmt::Display for ICRC1TransferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ICRC1TransferError::BadFee { expected_fee } => write!(f, "Bad fee: expected {}", expected_fee),
            ICRC1TransferError::BadBurn { min_burn_amount } => write!(f, "Bad burn: minimum burn amount is {}", min_burn_amount),
            ICRC1TransferError::InsufficientFunds { balance } => write!(f, "Insufficient funds: balance is {}", balance),
            ICRC1TransferError::TooOld => write!(f, "Transaction is too old"),
            ICRC1TransferError::CreatedInFuture { ledger_time } => write!(f, "Transaction created in the future: {}", ledger_time),
            ICRC1TransferError::Duplicate { duplicate_of } => write!(f, "Duplicate transaction: duplicate of {}", duplicate_of),
            ICRC1TransferError::TemporarilyUnavailable => write!(f, "Temporarily unavailable"),
            ICRC1TransferError::GenericError { error_code, message } => write!(f, "Generic error: {} - {}", error_code, message),
        }
    }
}
