use ic_cdk::export::candid::{CandidType, Deserialize};

use crate::{tokens::Tokens, types::BlockIndex};

#[derive(Debug, Clone, PartialEq)]
pub enum ICRCAccountError {
    InvalidFormat,
    BadChecksum,
    NotCanonical,
    HexDecode(String),
    Malformed(String),
    InvalidPrincipal(String),
    InvalidSubaccount(String),
}

#[rustfmt::skip]
impl ErrorTrait for ICRCAccountError {
    fn to_string(self) -> String {
        match self {
            ICRCAccountError::BadChecksum => "Bad checksum".to_string(),
            ICRCAccountError::NotCanonical => "Not canonical".to_string(),
            ICRCAccountError::HexDecode(e) => ["Hex decode error: ", &e].concat(),
            ICRCAccountError::Malformed(e) => ["Malformed account: ", &e].concat(),
            ICRCAccountError::InvalidFormat => "Invalid account format".to_string(),
            ICRCAccountError::InvalidPrincipal(e) => ["Invalid principal: ", &e].concat(),
            ICRCAccountError::InvalidSubaccount(e) => ["Invalid subaccount: ", &e].concat(),
        }
    }
}

#[rustfmt::skip]
#[derive(CandidType, Deserialize, Debug)]
pub enum SubaccountError {
    HexError(String),
    SliceError(String),
    Base32Error(String),
    InvalidSubaccount(String),
}

impl ErrorTrait for SubaccountError {
    fn to_string(self) -> String {
        match self {
            SubaccountError::InvalidSubaccount(e) => ["InvalidSubaccount: ", &e].concat(),
            SubaccountError::Base32Error(e) => ["::Subaccount base32 error: ", &e].concat(),
            SubaccountError::SliceError(e) => ["::Subaccount slice error: ", &e].concat(),
            SubaccountError::HexError(e) => ["::Subaccount hex error: ", &e].concat(),
        }
    }
}

#[rustfmt::skip]
#[derive(CandidType, Deserialize, Debug)]
pub enum HelperError {
    CanisterStatusError(String),
    CreateCanisterError(String),
    ValidateSignerError(String),
    InvalidSubaccount(String),
    VersionError(String),
    InstallCodeError(String),
    WasmHashError(String),
    EncodeError(String),
    SignerNotAvailable,
    RateLimitExceeded,
    InvalidAccountIdentifier,
    UpdateCanisterControllersError(String),
}

pub trait ErrorTrait {
    fn to_string(self) -> String;
}

#[rustfmt::skip]
impl ErrorTrait for HelperError {
    fn to_string(self) -> String {
        match self {
            HelperError::InvalidSubaccount(e) => ["::Invalid subaccount: ", &e].concat(),
            HelperError::ValidateSignerError(e) => ["::Get owner error: ", &e].concat(),
            HelperError::WasmHashError(e) => ["::Wasm hash error: ", &e].concat(),
            HelperError::CreateCanisterError(e) => ["::Create canister error: ", &e].concat(),
            HelperError::EncodeError(e) => ["::Encode error: ", &e].concat(),
            HelperError::InstallCodeError(e) => ["::Install code error: ", &e].concat(),
            HelperError::VersionError(e) => ["::Version error: ", &e].concat(),
            HelperError::CanisterStatusError(e) => ["::Canister status error: ", &e].concat(),
            HelperError::SignerNotAvailable => "::Signer not available!".to_string(),
            HelperError::RateLimitExceeded => "::Rate limit exceeded, please try again later!".to_string(),
            HelperError::UpdateCanisterControllersError(e) => ["::Update canister controllers error: " , &e].concat(),
            HelperError::InvalidAccountIdentifier => "::Invalid account identifier!".to_string(),
        }
    }
}

pub enum TransferError {
    BadFee { expected_fee: Tokens },
    InsufficientFunds { balance: Tokens },
    TxTooOld { allowed_window_nanos: u64 },
    TxCreatedInFuture,
    TxDuplicate { duplicate_of: BlockIndex },
}

#[rustfmt::skip]
impl ErrorTrait for TransferError {
    fn to_string(self) -> String {
        match self {
            TransferError::BadFee { expected_fee } => {
                format!("Bad fee, expected at least {} tokens", expected_fee)
            }
            TransferError::InsufficientFunds { balance } => {
                format!("Insufficient funds, balance is {}", balance)
            }
            TransferError::TxTooOld {
                allowed_window_nanos,
            } => format!(
                "Transaction is too old, allowed window is {} nanoseconds",
                allowed_window_nanos
            ),
            TransferError::TxCreatedInFuture => "Transaction created in the future".to_string(),
            TransferError::TxDuplicate { duplicate_of } => {
                format!("Duplicate transaction, duplicate of {}", duplicate_of)
            }
        }
    }
}

#[rustfmt::skip]
pub enum NotifyError {
    BadFee { expected_fee: Tokens },
    InsufficientFunds { balance: Tokens },
    TxTooOld { allowed_window_nanos: u64 },
    TxDuplicate { duplicate_of: BlockIndex },
    Refunded { block_index: Option<u64>, reason: String},
    InvalidTransaction(String),
    Other {
        error_message: String,
        error_code: u64,
    },
    Processing,
    TransactionTooOld(u64),    
    TxCreatedInFuture,
}

#[rustfmt::skip]
impl ErrorTrait for NotifyError {
    fn to_string(self) -> String {
        match self {
            NotifyError::TransactionTooOld(e) => ["::Transaction too old:", & e.to_string()].concat(),
            NotifyError::InvalidTransaction(e) => ["::Invalid transaction: ", &e].concat(),
            NotifyError::Processing => "::Processing!".to_string(),
            NotifyError::BadFee { expected_fee } => ["::Invalid fee: Expected", &expected_fee.to_string(), "tokens"].join(" "),
            NotifyError::InsufficientFunds { balance } => ["::Insufficient funds: Balance is", &balance.to_string(), "tokens"].join(" "),
            NotifyError::TxTooOld { allowed_window_nanos } => ["::Transaction too old: Allowed window is", &allowed_window_nanos.to_string(), "nanoseconds"].join(" "),
            NotifyError::TxCreatedInFuture => "::Transaction created in the future".to_string(),
            NotifyError::TxDuplicate { duplicate_of } => ["::Duplicate transaction: Duplicate of block index ", &duplicate_of.to_string()].concat(),
            NotifyError::Other { error_message, error_code } => ["::Other error", &error_code.to_string(), &error_message].join(" "),
            NotifyError::Refunded { block_index, reason } => {
                if let Some(index) = block_index {
                    [
                        "::Transaction refunded: Refunded at block index",
                        &index.to_string(),
                        "due to",
                        &reason,
                    ]
                    .join(" ")
                } else {
                    ["::Transaction refunded: Refunded due to ", &reason].concat()
                }
            }
        }
    }
}
