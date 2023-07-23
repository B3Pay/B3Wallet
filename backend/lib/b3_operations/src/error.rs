use b3_utils::types::OperationId;
use b3_wallet_lib::{
    error::WalletError,
    ledger::{error::LedgerError, evm::error::EvmError},
};
use candid::{CandidType, Deserialize};
use std::fmt;

#[rustfmt::skip]
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub enum OperationError {
    WalletError(WalletError),
    LedgerError(LedgerError),
    EvmError(EvmError),
    RequestAlreadySigned(String),
    RequestRejected,
    RequestExpired,
    RequestNotFound(OperationId),
    RequestAlreadyProcessed(OperationId),
    RequestRemovedByAdmin(String),
    SignerNotAllowed(String),
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
    InvalidChainId(u64, u64),
    InvalidAmount,
    InvalidWasmHash,
    InvalidController,
    InvalidTransaction,
    SneakyMessage,
    AccountNotFound,
    ChainNotFound(String, String),
    ChainIdNotInitialized
}

#[rustfmt::skip]
impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperationError::WalletError(err) => write!(f, "Wallet Error::{}", err),
            OperationError::LedgerError(err) => write!(f, "Ledger Error::{}", err),
            OperationError::EvmError(err) => write!(f, "Evm Error::{}", err),
            OperationError::ExecutionError(ref msg) => write!(f, "Execution Error::{}", msg),
            OperationError::AccountIsHidden => write!(f, "Account is hidden!"),
            OperationError::AccountIsNotHidden => write!(f, "Account is not hidden!"),
            OperationError::AccountDoesNotExist => write!(f, "Account does not exist!"),
            OperationError::InvalidThreshold => write!(f, "Invalid threshold!"),
            OperationError::AmountIsZero => write!(f, "Amount is zero!"),
            OperationError::FeeIsZero => write!(f, "Fee is zero!"),
            OperationError::InvalidRequest => write!(f, "Invalid request"),
            OperationError::SignerNotAllowed(ref msg) => write!(f, "{} is not allowed to perform this operation!", msg),
            OperationError::SignerNotFound(ref msg) => write!(f, "{} is not a signer!", msg),
            OperationError::SignerRoleNotFound(ref signer,ref role) => write!(f, "Signer {} does not have role {}", signer, role),
            OperationError::SignerRoleNotAuthorized(ref signer,ref role) => write!(f, "Signer {} is not authorized to perform {} operations", signer, role),
            OperationError::SignerAlreadyExists(ref signer) => write!(f, "Signer {} already exists!", signer),
            OperationError::SignerDoesNotExist(ref signer) => write!(f, "Signer {} does not exist!", signer),
            OperationError::RequestExpired => write!(f, "Request expired!"),
            OperationError::RequestRejected => write!(f, "Request rejected!"),
            OperationError::RequestNotFound(ref msg) => write!(f, "Request not found: {}", msg),
            OperationError::RequestAlreadySigned(ref signer) => write!(f, "Signer {} already signed", signer),
            OperationError::RequestAlreadyProcessed(ref request_id) => write!(f, "Request {} already processed!", request_id),
            OperationError::RequestRemovedByAdmin(ref signer) => write!(f, "Request removed by admin: {}", signer),
            OperationError::WasmNotSet => write!(f, "Wasm not set!"),
            OperationError::InvalidChainId(ref chain_id, ref expected_chain_id) => write!(f, "Invalid chain id! Expected: {}, got: {}", expected_chain_id, chain_id),
            OperationError::InvalidAmount => write!(f, "Invalid amount!"),
            OperationError::InvalidWasmHash => write!(f, "Invalid wasm hash!"),
            OperationError::InvalidController => write!(f, "Invalid controller!"),
            OperationError::InvalidTransaction => write!(f, "Invalid transaction!"),
            OperationError::SneakyMessage => write!(f, "Sneaky message, if you want to send transaction use 'send_transaction' method!"),
            OperationError::AccountNotFound => write!(f, "Account not found!"),
            OperationError::ChainNotFound(ref chain_name, ref chain_id) => write!(f, "Chain {} with id {} not found!", chain_name, chain_id),
            OperationError::ChainIdNotInitialized => write!(f, "Chain ID not initialized!"),
        }
    }
}

impl From<WalletError> for OperationError {
    fn from(err: WalletError) -> Self {
        OperationError::WalletError(err)
    }
}

impl From<LedgerError> for OperationError {
    fn from(err: LedgerError) -> Self {
        OperationError::LedgerError(err)
    }
}

impl From<EvmError> for OperationError {
    fn from(err: EvmError) -> Self {
        OperationError::EvmError(err)
    }
}
