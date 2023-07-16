use crate::{icp_token::ICPToken, types::BlockIndex};
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
};
use std::fmt;

#[derive(CandidType, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub enum TransferError {
    BadFee { expected_fee: ICPToken },
    InsufficientFunds { balance: ICPToken },
    TxTooOld { allowed_window_nanos: u64 },
    TxCreatedInFuture,
    TxDuplicate { duplicate_of: BlockIndex },
}

#[rustfmt::skip]
impl fmt::Display for TransferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransferError::BadFee { expected_fee } => write!(f, "Bad fee, expected at least {} tokens", expected_fee),
            TransferError::InsufficientFunds { balance } => write!(f, "Insufficient funds, balance is {}", balance),
            TransferError::TxTooOld { allowed_window_nanos } => write!(f, "Transaction is too old, allowed window is {} nanoseconds", allowed_window_nanos),
            TransferError::TxCreatedInFuture => write!(f, "Transaction created in the future"),
            TransferError::TxDuplicate { duplicate_of } => write!(f, "Duplicate transaction, duplicate of {}", duplicate_of)
        }
    }
}
