use ic_cdk::export::candid::{CandidType, Deserialize};

use crate::types::{BlockIndex, Tokens};

#[rustfmt::skip]
#[derive(CandidType, Deserialize)]
pub enum HelperError {
    CanisterStatusError(String),
    CreateCanisterError(String),
    ValidateSignerError(String),
    VersionError(String),
    InstallCodeError(String),
    WasmHashError(String),
    EncodeError(String),
    SignerNotAvailable,
    RateLimitExceeded,
    TxCreatedInFuture,
    InvalidAccountIdentifier,
    UpdateCanisterControllersError(String),
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
}

pub trait TrapError {
    fn to_string(self) -> String;
}

#[rustfmt::skip]
impl TrapError for HelperError {
    fn to_string(self) -> String {
        match self {
            HelperError::Processing => "::Processing!".to_string(),
            HelperError::ValidateSignerError(e) => ["::Get owner error: ", &e].concat(),
            HelperError::WasmHashError(e) => ["::Wasm hash error: ", &e].concat(),
            HelperError::InvalidTransaction(e) => ["::Invalid transaction: ", &e].concat(),
            HelperError::TransactionTooOld(e) => ["::Transaction too old:", & e.to_string()].concat(),
            HelperError::CreateCanisterError(e) => ["::Create canister error: ", &e].concat(),
            HelperError::EncodeError(e) => ["::Encode error: ", &e].concat(),
            HelperError::InstallCodeError(e) => ["::Install code error: ", &e].concat(),
            HelperError::VersionError(e) => ["::Version error: ", &e].concat(),
            HelperError::CanisterStatusError(e) => ["::Canister status error: ", &e].concat(),
            HelperError::SignerNotAvailable => "::Signer not available!".to_string(),
            HelperError::RateLimitExceeded => "::Rate limit exceeded, please try again later!".to_string(),
            HelperError::UpdateCanisterControllersError(e) => ["::Update canister controllers error: " , &e].concat(),
            HelperError::InvalidAccountIdentifier => "::Invalid account identifier!".to_string(),
            HelperError::BadFee { expected_fee } => ["::Invalid fee: Expected", &expected_fee.to_string(), "tokens"].join(" "),
            HelperError::InsufficientFunds { balance } => ["::Insufficient funds: Balance is", &balance.to_string(), "tokens"].join(" "),
            HelperError::TxTooOld { allowed_window_nanos } => ["::Transaction too old: Allowed window is", &allowed_window_nanos.to_string(), "nanoseconds"].join(" "),
            HelperError::TxCreatedInFuture => "::Transaction created in the future".to_string(),
            HelperError::TxDuplicate { duplicate_of } => ["::Duplicate transaction: Duplicate of block index ", &duplicate_of.to_string()].concat(),
            HelperError::Other { error_message, error_code } => ["::Other error", &error_code.to_string(), &error_message].join(" "),
            HelperError::Refunded { block_index, reason } => {
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
