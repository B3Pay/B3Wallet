use b3_shared::error::TrapError;
use ic_cdk::export::candid::{CandidType, Deserialize};

#[rustfmt::skip]
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
    InvalidTransaction(String),
    TransactionTooOld(u64),
    Processing,
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

#[rustfmt::skip]
impl TrapError for SignerError {
    fn to_string(self) -> String {
        match self {
            SignerError::UnknownError => "Unknown error".to_string(),
            SignerError::InvalidTx(msg) => ["Invalid transaction: ", &msg].concat(),
            SignerError::InvalidMsg(msg) => ["Invalid message: ", &msg].concat(),
            SignerError::InvalidSignature(msg) => ["Invalid signature: ", &msg].concat(),
            SignerError::SignError(msg) => ["Sign error: ", &msg].concat(),
            SignerError::LedgerError(msg) => ["Ledger error: ", &msg].concat(),
            SignerError::GenerateError(msg) => ["Generation error: ", &msg].concat(),
            SignerError::PublicKeyError(msg) => ["Public key error: ", &msg].concat(),
            SignerError::CyclesMintingError(msg) => ["Cycles minting error: ", &msg].concat(),
            SignerError::CanisterStatusError(msg) => ["Canister status error: ", &msg].concat(),
            SignerError::UpdateSettingsError(msg) => ["Update settings error: ", &msg].concat(),
            SignerError::InvalidTransaction(msg) => ["Invalid transaction: ", &msg].concat(),
            SignerError::TransactionTooOld(nanos) => ["Transaction too old: {} nanoseconds", &nanos.to_string()].concat(),
            SignerError::Processing => "Processing error".to_string(),
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
