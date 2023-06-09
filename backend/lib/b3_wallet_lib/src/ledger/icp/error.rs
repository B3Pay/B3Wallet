use enum_dispatch::enum_dispatch;
use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::fmt;

#[rustfmt::skip]
#[enum_dispatch]
#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum IcpError {
    CallError(String),
}

#[rustfmt::skip]
impl fmt::Display for IcpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IcpError::CallError(ref msg) => write!(f, "Call error: {}", msg),
        }
    }
}
