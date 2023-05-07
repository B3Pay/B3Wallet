use ic_cdk::{api::call::RejectionCode, export::candid::CandidType};

/// Represents errors that can occur when working with the state.
#[derive(CandidType, Debug, PartialEq)]
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
    InvalidMessageLength,
    MissingEcdsaPublicKey,
    CallerIsNotOwner,
    MaximumAccountsReached,
    MaximumDevelopmentAccountsReached,
    MaximumProductionAccountsReached,
    PublicKeyAlreadyExists,
    InvalidEcdsaPublicKey,
    InvalidAccountIdentifier,
    AccountNotExists,
    RequestNotExists,
    InvalidAddress,
}

impl From<SignerError> for (RejectionCode, String) {
    fn from(error: SignerError) -> Self {
        match error {
            SignerError::InvalidMsg(msg) => (
                RejectionCode::CanisterError,
                ["Invalid message ", &msg].concat(),
            ),
            SignerError::InvalidTx(msg) => (
                RejectionCode::CanisterError,
                ["Invalid transaction ", &msg].concat(),
            ),
            SignerError::InvalidSignature(msg) => (
                RejectionCode::CanisterError,
                ["Invalid signature ", &msg].concat(),
            ),
            SignerError::LedgerError(msg) => (
                RejectionCode::CanisterError,
                ["Ledger error ", &msg].concat(),
            ),
            SignerError::GenerateError(msg) => (
                RejectionCode::CanisterError,
                ["Generate error ", &msg].concat(),
            ),
            SignerError::CyclesMintingError(msg) => (
                RejectionCode::CanisterError,
                ["Cycles minting error ", &msg].concat(),
            ),
            SignerError::PublicKeyError(msg) => (
                RejectionCode::CanisterError,
                ["Public key error ", &msg].concat(),
            ),
            SignerError::CanisterStatusError(msg) => (
                RejectionCode::CanisterError,
                ["Canister status error ", &msg].concat(),
            ),
            SignerError::SignError(msg) => {
                (RejectionCode::CanisterError, ["Sign error ", &msg].concat())
            }
            SignerError::UnknownError => (RejectionCode::Unknown, "Unknown error".to_string()),
            SignerError::MissingEcdsaPublicKey => (
                RejectionCode::CanisterError,
                "Missing public key".to_string(),
            ),
            SignerError::InvalidAccountIdentifier => (
                RejectionCode::CanisterReject,
                "Invalid account identifier".to_string(),
            ),
            SignerError::CallerIsNotOwner => (
                RejectionCode::CanisterReject,
                "Caller is not owner".to_string(),
            ),
            SignerError::MaximumAccountsReached => (
                RejectionCode::CanisterError,
                "Maximum accounts reached".to_string(),
            ),
            SignerError::MaximumDevelopmentAccountsReached => (
                RejectionCode::CanisterError,
                "Maximum development accounts reached".to_string(),
            ),
            SignerError::MaximumProductionAccountsReached => (
                RejectionCode::CanisterError,
                "Maximum production accounts reached".to_string(),
            ),
            SignerError::InvalidEcdsaPublicKey => (
                RejectionCode::CanisterError,
                "Invalid ECDSA public key".to_string(),
            ),
            SignerError::AccountNotExists => (
                RejectionCode::CanisterError,
                "Account not found".to_string(),
            ),
            SignerError::RequestNotExists => (
                RejectionCode::CanisterError,
                "Setting not found".to_string(),
            ),
            SignerError::InvalidAddress => {
                (RejectionCode::CanisterError, "Invalid address".to_string())
            }
            SignerError::PublicKeyAlreadyExists => (
                RejectionCode::CanisterError,
                "Public key already exists".to_string(),
            ),
            SignerError::InvalidMessageLength => (
                RejectionCode::CanisterError,
                "Invalid message length".to_string(),
            ),
        }
    }
}
