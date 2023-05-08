use ic_cdk::export::candid::{CandidType, Deserialize};

use crate::ledger::types::{BlockIndex, Tokens};

#[derive(CandidType, Deserialize)]
pub enum SignerError {
    UnknownError,
    InvalidTx(String),
    InvalidMsg(String),
    InvalidSignature(String),
    SignError(String),
    LedgerError(String),
    GenerateError(String),
    PublicKeyError(String),
    CyclesMintingError(String),
    CanisterStatusError(String),
    UpdateSettingsError(String),
    BadFee {
        expected_fee: Tokens,
    },
    InsufficientFunds {
        balance: Tokens,
    },
    TxTooOld {
        allowed_window_nanos: u64,
    },
    TxCreatedInFuture,
    TxDuplicate {
        duplicate_of: BlockIndex,
    },
    Refunded {
        block_index: Option<u64>,
        reason: String,
    },
    InvalidTransaction(String),
    Other {
        error_message: String,
        error_code: u64,
    },
    Processing,
    TransactionTooOld(u64),
    InvalidMessageLength,
    MissingEcdsaPublicKey,
    CallerIsNotOwner,
    CannotRemoveDefaultAccount,
    PublicKeyAlreadyExists,
    InvalidEcdsaPublicKey,
    InvalidAccountIdentifier,
    AccountNotExists,
    RequestNotExists,
    InvalidAddress,
}
impl SignerError {
    pub fn to_string(&self) -> String {
        match self {
            SignerError::UnknownError => "Unknown error".to_string(),
            SignerError::InvalidTx(msg) => ["Invalid transaction: ", msg].concat(),
            SignerError::InvalidMsg(msg) => ["Invalid message: ", msg].concat(),
            SignerError::InvalidSignature(msg) => ["Invalid signature: ", msg].concat(),
            SignerError::SignError(msg) => ["Sign error: ", msg].concat(),
            SignerError::LedgerError(msg) => ["Ledger error: ", msg].concat(),
            SignerError::GenerateError(msg) => ["Generation error: ", msg].concat(),
            SignerError::PublicKeyError(msg) => ["Public key error: ", msg].concat(),
            SignerError::CyclesMintingError(msg) => ["Cycles minting error: ", msg].concat(),
            SignerError::CanisterStatusError(msg) => ["Canister status error: ", msg].concat(),
            SignerError::UpdateSettingsError(msg) => ["Update settings error: ", msg].concat(),
            SignerError::BadFee { expected_fee } => {
                ["Invalid fee: Expected {} tokens", &expected_fee.to_string()].concat()
            }
            SignerError::InsufficientFunds { balance } => [
                "Insufficient funds: Balance is {} tokens",
                &balance.to_string(),
            ]
            .concat(),
            SignerError::TxTooOld {
                allowed_window_nanos,
            } => [
                "Transaction too old: Allowed window is",
                &allowed_window_nanos.to_string(),
                "nanoseconds",
            ]
            .join(" "),
            SignerError::TxCreatedInFuture => "Transaction created in the future".to_string(),
            SignerError::TxDuplicate { duplicate_of } => [
                "Duplicate transaction: Duplicate of block index ",
                &duplicate_of.to_string(),
            ]
            .concat(),
            SignerError::Refunded {
                block_index,
                reason,
            } => {
                if let Some(index) = block_index {
                    [
                        "Transaction refunded: Refunded at block index",
                        &index.to_string(),
                        "due to",
                        reason,
                    ]
                    .join(" ")
                } else {
                    ["Transaction refunded: Refunded due to ", reason].concat()
                }
            }
            SignerError::InvalidTransaction(msg) => ["Invalid transaction: ", msg].concat(),
            SignerError::Other {
                error_message,
                error_code,
            } => [
                "Other error (Code {}): ",
                &error_code.to_string(),
                error_message,
            ]
            .concat(),
            SignerError::Processing => "Processing error".to_string(),
            SignerError::TransactionTooOld(nanos) => {
                ["Transaction too old: {} nanoseconds", &nanos.to_string()].concat()
            }
            SignerError::InvalidMessageLength => "Invalid message length".to_string(),
            SignerError::MissingEcdsaPublicKey => "Missing ECDSA public key".to_string(),
            SignerError::CallerIsNotOwner => "Caller is not the owner".to_string(),
            SignerError::CannotRemoveDefaultAccount => "Cannot remove default account".to_string(),
            SignerError::PublicKeyAlreadyExists => "Public key already exists".to_string(),
            SignerError::InvalidEcdsaPublicKey => "Invalid ECDSA public key".to_string(),
            SignerError::InvalidAccountIdentifier => "Invalid account identifier".to_string(),
            SignerError::AccountNotExists => "Account does not exist".to_string(),
            SignerError::RequestNotExists => "Request does not exist".to_string(),
            SignerError::InvalidAddress => "Invalid address".to_string(),
        }
    }
}
