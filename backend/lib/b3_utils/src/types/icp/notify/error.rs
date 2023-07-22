use crate::{currency::ICPToken, types::TransferBlockIndex};
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt;

#[rustfmt::skip]
#[derive(CandidType, Clone, Debug, Deserialize,Serialize, PartialEq, Eq)]
pub enum NotifyError {
    BadFee { expected_fee: ICPToken },
    InsufficientFunds { balance: ICPToken },
    TxTooOld { allowed_window_nanos: u64 },
    TxDuplicate { duplicate_of: TransferBlockIndex },
    Refunded { block_index: Option<u64>, reason: String},
    InvalidTransaction(String),
    Other { error_message: String, error_code: u64 },
    Processing,
    TransactionTooOld(u64),    
    TxCreatedInFuture,
}

#[rustfmt::skip]
impl fmt::Display for NotifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotifyError::TransactionTooOld(e) => write!(f, "Transaction too old: {}", e),
            NotifyError::InvalidTransaction(e) => write!(f, "Invalid transaction: {}", e),
            NotifyError::Processing => write!(f, "Processing!"),
            NotifyError::BadFee { expected_fee } => write!(f, "Invalid fee: Expected {} tokens", expected_fee),
            NotifyError::InsufficientFunds { balance } => write!(f, "Insufficient funds: Balance is {} tokens", balance),
            NotifyError::TxTooOld { allowed_window_nanos } => write!(f, "Transaction too old: Allowed window is {} nanoseconds", allowed_window_nanos),
            NotifyError::TxCreatedInFuture => write!(f, "Transaction created in the future"),
            NotifyError::TxDuplicate { duplicate_of } => write!(f, "Duplicate transaction: Duplicate of block index {}", duplicate_of),
            NotifyError::Other { error_message, error_code } => write!(f, "Other error {} {}", error_code, error_message),
            NotifyError::Refunded { block_index, reason } => match block_index {
                Some(index) => write!(f, "Transaction refunded: Refunded at block index {} due to {}", index, reason),
                None => write!(f, "Transaction refunded: Refunded due to {}", reason),
            },
        }
    }
}
