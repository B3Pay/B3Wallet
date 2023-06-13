use b3_helper_lib::types::RequestId;
use b3_wallet_lib::{
    error::WalletError,
    ledger::{error::LedgerError, evm::error::EvmError},
};
use ic_cdk::export::candid::{CandidType, Deserialize};
use std::fmt;

#[rustfmt::skip]
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub enum PermitError {
    WalletError(WalletError),
    LedgerError(LedgerError),
    EvmError(EvmError),
    RequestAlreadySigned(String),
    RequestRejected,
    RequestExpired,
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
    InvalidChainId(u64, u64),
    InvalidAmount,
    InvalidWasmHash,
    InvalidController,
    InvalidTransaction,
    SneakyMessage,
    ChainIdNotInitialized
}

#[rustfmt::skip]
impl fmt::Display for PermitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PermitError::WalletError(err) => write!(f, "Wallet Error::{}", err),
            PermitError::LedgerError(err) => write!(f, "Ledger Error::{}", err),
            PermitError::EvmError(err) => write!(f, "Evm Error::{}", err),
            PermitError::ExecutionError(ref msg) => write!(f, "Execution Error::{}", msg),
            PermitError::AccountIsHidden => write!(f, "Account is hidden!"),
            PermitError::AccountIsNotHidden => write!(f, "Account is not hidden!"),
            PermitError::AccountDoesNotExist => write!(f, "Account does not exist!"),
            PermitError::InvalidThreshold => write!(f, "Invalid threshold!"),
            PermitError::AmountIsZero => write!(f, "Amount is zero!"),
            PermitError::FeeIsZero => write!(f, "Fee is zero!"),
            PermitError::InvalidRequest => write!(f, "Invalid request"),
            PermitError::SignerNotFound(ref msg) => write!(f, "{} is not a signer!", msg),
            PermitError::SignerRoleNotFound(ref signer,ref role) => write!(f, "Signer {} does not have role {}", signer, role),
            PermitError::SignerRoleNotAuthorized(ref signer,ref role) => write!(f, "Signer {} is not authorized to perform {} operations", signer, role),
            PermitError::SignerAlreadyExists(ref signer) => write!(f, "Signer {} already exists!", signer),
            PermitError::SignerDoesNotExist(ref signer) => write!(f, "Signer {} does not exist!", signer),
            PermitError::RequestExpired => write!(f, "Request expired!"),
            PermitError::RequestRejected => write!(f, "Request rejected!"),
            PermitError::RequestNotFound(ref msg) => write!(f, "Request not found: {}", msg),
            PermitError::RequestAlreadySigned(ref signer) => write!(f, "Signer {} already signed", signer),
            PermitError::RequestAlreadyProcessed(ref request_id) => write!(f, "Request {} already processed!", request_id),
            PermitError::WasmNotSet => write!(f, "Wasm not set!"),
            PermitError::InvalidChainId(ref chain_id, ref expected_chain_id) => write!(f, "Invalid chain id! Expected: {}, got: {}", expected_chain_id, chain_id),
            PermitError::InvalidAmount => write!(f, "Invalid amount!"),
            PermitError::InvalidWasmHash => write!(f, "Invalid wasm hash!"),
            PermitError::InvalidController => write!(f, "Invalid controller!"),
            PermitError::InvalidTransaction => write!(f, "Invalid transaction!"),
            PermitError::SneakyMessage => write!(f, "Sneaky message, if you want to send transaction use 'send_transaction' method!"),
            PermitError::ChainIdNotInitialized => write!(f, "Chain ID not initialized!"),
        }
    }
}

impl From<WalletError> for PermitError {
    fn from(err: WalletError) -> Self {
        PermitError::WalletError(err)
    }
}

impl From<LedgerError> for PermitError {
    fn from(err: LedgerError) -> Self {
        PermitError::LedgerError(err)
    }
}

impl From<EvmError> for PermitError {
    fn from(err: EvmError) -> Self {
        PermitError::EvmError(err)
    }
}
