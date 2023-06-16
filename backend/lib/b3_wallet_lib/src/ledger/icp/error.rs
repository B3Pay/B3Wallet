use b3_helper_lib::error::{NotifyError, TransferError};
use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::fmt;

#[rustfmt::skip]
#[enum_dispatch]
#[derive(CandidType, Clone, Debug, Deserialize, PartialEq)]
pub enum IcpError {
    CallError(String),
    TopUpPending(String),
    TransferError,
    NotifyError
}

#[rustfmt::skip]
impl fmt::Display for IcpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IcpError::TransferError(ref err) => write!(f, "Transfer error: {}", err),
            IcpError::NotifyError(ref err) => write!(f, "Notify error: {}", err),
            IcpError::CallError(ref msg) => write!(f, "Call error: {}", msg),
            IcpError::TopUpPending(ref msg) => write!(f, "Top up pending: {}", msg),
        }
    }
}
