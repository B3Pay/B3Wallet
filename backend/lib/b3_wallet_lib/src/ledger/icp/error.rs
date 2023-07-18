use b3_helper_lib::types::{ICPTransferError, NotifyError};
use candid::{CandidType, Deserialize};
use enum_dispatch::enum_dispatch;
use std::fmt;

#[rustfmt::skip]
#[enum_dispatch]
#[derive(CandidType, Clone, Debug, Deserialize, PartialEq)]
pub enum IcpError {
    CallError(String),
    TopUpPending(String),
    ICPTransferError,
    NotifyError
}

#[rustfmt::skip]
impl fmt::Display for IcpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IcpError::ICPTransferError(ref err) => write!(f, "ICP Transfer error: {}", err),
            IcpError::NotifyError(ref err) => write!(f, "Notify error: {}", err),
            IcpError::CallError(ref msg) => write!(f, "Call error: {}", msg),
            IcpError::TopUpPending(ref msg) => write!(f, "Top up pending: {}", msg),
        }
    }
}
