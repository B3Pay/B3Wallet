use b3_helper_lib::types::RequestId;
use b3_wallet_lib::error::WalletError;
use ic_cdk::export::candid::{CandidType, Deserialize};
use std::fmt;

#[rustfmt::skip]
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub enum RequestError {
    WalletError(WalletError),
    RequestRejected,
    RequestExpired,
    RequestAlreadySigned(String),
    RequestNotFound(RequestId),
    RequestAlreadyProcessed(RequestId),
    SignerNotFound(String),
    SignerAlreadyExists(String),
    SignerDoesNotExist(String),
    SignerRoleNotFound(String, String),
    SignerRoleNotAuthorized(String, String),
    InvalidRequest,
    ExecutionError(String),
    AmountIsZero,
    FeeIsZero,
    InvalidThreshold,
    AccountIsHidden,
    AccountIsNotHidden,
    AccountDoesNotExist,
    WasmNotSet,
    InvalidAmount,
    InvalidWasmHash,
    InvalidController,
    ChainIdNotInitialized
}

#[rustfmt::skip]
impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestError::WalletError(err) => write!(f, "::Wallet error: {}", err),
            RequestError::AccountIsHidden => write!(f, "::Account is hidden!"),
            RequestError::AccountIsNotHidden => write!(f, "::Account is not hidden!"),
            RequestError::AccountDoesNotExist => write!(f, "::Account does not exist!"),
            RequestError::InvalidThreshold => write!(f, "::Invalid threshold!"),
            RequestError::AmountIsZero => write!(f, "::Amount is zero!"),
            RequestError::FeeIsZero => write!(f, "::Fee is zero!"),
            RequestError::InvalidRequest => write!(f, "::Invalid request"),
            RequestError::ExecutionError(ref msg) => write!(f, "::Execution error: {}", msg),
            RequestError::SignerNotFound(ref msg) => write!(f, "::{} is not a signer!", msg),
            RequestError::SignerRoleNotFound(ref signer,ref role) => write!(f, "::Signer {} does not have role {}", signer, role),
            RequestError::SignerRoleNotAuthorized(ref signer,ref role) => write!(f, "::Signer {} is not authorized to perform {} operations", signer, role),
            RequestError::SignerAlreadyExists(ref signer) => write!(f, "::Signer {} already exists!", signer),
            RequestError::SignerDoesNotExist(ref signer) => write!(f, "::Signer {} does not exist!", signer),
            RequestError::RequestExpired => write!(f, "::Request expired!"),
            RequestError::RequestRejected => write!(f, "::Request rejected!"),
            RequestError::RequestNotFound(ref msg) => write!(f, "::Request not found: {}", msg),
            RequestError::RequestAlreadySigned(ref signer) => write!(f, "::Signer {} already signed", signer),
            RequestError::RequestAlreadyProcessed(ref request_id) => write!(f, "::Request {} already processed!", request_id),
            RequestError::WasmNotSet => write!(f, "::Wasm not set!"),
            RequestError::InvalidAmount => write!(f, "::Invalid amount!"),
            RequestError::InvalidWasmHash => write!(f, "::Invalid wasm hash!"),
            RequestError::InvalidController => write!(f, "::Invalid controller!"),
            RequestError::ChainIdNotInitialized => write!(f, "::Chain ID not initialized!"),
        }
    }
}

impl From<WalletError> for RequestError {
    fn from(err: WalletError) -> Self {
        RequestError::WalletError(err)
    }
}
