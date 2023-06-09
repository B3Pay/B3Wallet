use ic_cdk::export::{candid::CandidType, serde::Deserialize};
use std::fmt;

#[rustfmt::skip]
#[derive(CandidType, Clone, Debug, Deserialize, PartialEq, Eq)]
pub enum EvmError {
    NotSignedTransaction,
    InvalidTransactionType,
    InvalidMessage(String),
    InvalidPublicKey(String),
    InvalidRecoveryId(String),
    InvalidSignature(String)
}

#[rustfmt::skip]
impl fmt::Display for EvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvmError::NotSignedTransaction => write!(f, "Not signed transaction"),
            EvmError::InvalidTransactionType => write!(f, "Invalid transaction type"),
            EvmError::InvalidMessage(msg) => write!(f, "Invalid message: {}", msg),
            EvmError::InvalidPublicKey(msg) => write!(f, "Invalid public key: {}", msg),
            EvmError::InvalidRecoveryId(msg) => write!(f, "Invalid recovery id: {}", msg),
            EvmError::InvalidSignature(msg) => write!(f, "Invalid signature: {}", msg),
        }
    }
}
