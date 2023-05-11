use ic_cdk::export::candid::{CandidType, Deserialize};

use crate::types::{BlockIndex, Tokens};

#[rustfmt::skip]
#[derive(CandidType, Deserialize)]
pub enum SharedError {
    CanisterStatusError(String),
    CreateCanisterError(String),
    VersionError(String),
    InstallCodeError(String),
    EncodeError(String),
    SignerNotAvailable,
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
impl TrapError for SharedError {
    fn to_string(self) -> String {
        match self {
            SharedError::Processing => "Processing!".to_string(),
            SharedError::InvalidTransaction(e) => ["Invalid transaction: ", &e].concat(),
            SharedError::TransactionTooOld(e) => ["Transaction too old:", & e.to_string()].concat(),
            SharedError::CreateCanisterError(e) => ["Create canister error: ", &e].concat(),
            SharedError::EncodeError(e) => ["Encode error: ", &e].concat(),
            SharedError::InstallCodeError(e) => ["Install code error: ", &e].concat(),
            SharedError::VersionError(e) => ["Version error: ", &e].concat(),
            SharedError::CanisterStatusError(e) => ["Canister status error: ", &e].concat(),
            SharedError::SignerNotAvailable => "Signer not available!".to_string(),
            SharedError::UpdateCanisterControllersError(e) => ["Update canister controllers error: " , &e].concat(),
            SharedError::InvalidAccountIdentifier => "Invalid account identifier!".to_string(),
            SharedError::BadFee { expected_fee } => ["Invalid fee: Expected", &expected_fee.to_string(), "tokens"].join(" "),
            SharedError::InsufficientFunds { balance } => ["Insufficient funds: Balance is", &balance.to_string(), "tokens"].join(" "),
            SharedError::TxTooOld { allowed_window_nanos } => ["Transaction too old: Allowed window is", &allowed_window_nanos.to_string(), "nanoseconds"].join(" "),
            SharedError::TxCreatedInFuture => "Transaction created in the future".to_string(),
            SharedError::TxDuplicate { duplicate_of } => ["Duplicate transaction: Duplicate of block index ", &duplicate_of.to_string()].concat(),
            SharedError::Other { error_message, error_code } => ["Other error", &error_code.to_string(), &error_message].join(" "),
            SharedError::Refunded { block_index, reason } => {
                if let Some(index) = block_index {
                    [
                        "Transaction refunded: Refunded at block index",
                        &index.to_string(),
                        "due to",
                        &reason,
                    ]
                    .join(" ")
                } else {
                    ["Transaction refunded: Refunded due to ", &reason].concat()
                }
            }
        }
    }
}
