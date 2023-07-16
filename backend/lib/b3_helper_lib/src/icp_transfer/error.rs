use super::TransferBlockIndex;
use crate::icp_token::ICPToken;

use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};

use std::fmt;

#[derive(CandidType, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub enum ICPTransferError {
    BadFee { expected_fee: ICPToken },
    InsufficientFunds { balance: ICPToken },
    TxTooOld { allowed_window_nanos: u64 },
    TxCreatedInFuture,
    TxDuplicate { duplicate_of: TransferBlockIndex },
}

#[rustfmt::skip]
impl fmt::Display for ICPTransferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ICPTransferError::BadFee { expected_fee } => write!(f, "Bad fee, expected at least {} tokens", expected_fee),
            ICPTransferError::InsufficientFunds { balance } => write!(f, "Insufficient funds, balance is {}", balance),
            ICPTransferError::TxTooOld { allowed_window_nanos } => write!(f, "Transaction is too old, allowed window is {} nanoseconds", allowed_window_nanos),
            ICPTransferError::TxCreatedInFuture => write!(f, "Transaction created in the future"),
            ICPTransferError::TxDuplicate { duplicate_of } => write!(f, "Duplicate transaction, duplicate of {}", duplicate_of)
        }
    }
}
