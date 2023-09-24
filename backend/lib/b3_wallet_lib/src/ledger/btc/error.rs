use candid::{CandidType, Deserialize};
use std::fmt;

#[rustfmt::skip]
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub enum BitcoinError {
    NoUtxos,
    FeeTooHigh(u64, u64),
    Signature(String),
    GetBalance(String),
    GetUtxos(String),
    SendTransaction(String),
    SendRawTransaction(String),
    PublicKeyError(String),
    GetCurrentFeePercentiles(String),
    GetFeeRate(String),
    InsufficientBalance(u64, u64),
    SwapToCkbtc(String),
    InvalidPublicKey(String),
    InvalidAddress(String),
    InvalidFeePercentile(String),
    InvalidNetworkAddress(String),
    InvalidChain(String),
}

#[rustfmt::skip]
impl fmt::Display for BitcoinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BitcoinError::NoUtxos => write!(f, "No utxos"),
            BitcoinError::FeeTooHigh(fee, amount) => write!(f, "Fee too high: {} > {}", fee, amount),
            BitcoinError::Signature(msg) => write!(f, "Error: {}", msg),
            BitcoinError::GetBalance(msg) => write!(f, "Get balance error: {}", msg),
            BitcoinError::GetUtxos(msg) => write!(f, "Get utxos error: {}", msg),
            BitcoinError::SendRawTransaction(msg) => write!(f, "Send raw transaction error: {}", msg),
            BitcoinError::PublicKeyError(msg) => write!(f, "Public key error: {}", msg),
            BitcoinError::GetCurrentFeePercentiles(msg) => write!(f, "Get current fee percentiles error: {}", msg),
            BitcoinError::SendTransaction(msg) => write!(f, "Send transaction error: {}", msg),
            BitcoinError::GetFeeRate(msg) => write!(f, "Get fee rate error: {}", msg),
            BitcoinError::InsufficientBalance(balance, amount) => write!(f, "Insufficient balance: {} < {}", balance, amount),
            BitcoinError::SwapToCkbtc(msg) => write!(f, "Swap to ckbtc error: {}", msg),
            BitcoinError::InvalidChain(msg) => write!(f, "Invalid chain: {}", msg),
            BitcoinError::InvalidPublicKey(msg) => write!(f, "Invalid public key: {}", msg),
            BitcoinError::InvalidAddress(msg) => write!(f, "Invalid address: {}", msg),
            BitcoinError::InvalidFeePercentile(msg) => write!(f, "Invalid fee percentile: {}", msg),
            BitcoinError::InvalidNetworkAddress(msg) => write!(f, "Invalid network address: {}", msg),
        }
    }
}
