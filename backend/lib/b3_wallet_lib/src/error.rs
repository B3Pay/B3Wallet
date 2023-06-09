use ic_cdk::export::candid::{CandidType, Deserialize};
use std::fmt;

use crate::ledger::{
    btc::error::BitcoinError, ckbtc::error::CkbtcError, error::LedgerError, evm::error::EvmError,
    icp::error::IcpError, icrc::error::IcrcError,
};

#[rustfmt::skip]
#[derive(CandidType, Clone, Deserialize, Debug, PartialEq)]
pub enum WalletError {
    BitcoinError(BitcoinError),
    CkbtcError(CkbtcError),
    IcrcError(IcrcError),
    EvmError(EvmError),
    IcpError(IcpError),
    LedgerError(LedgerError),
    UnknownError,
    InvalidRequest,
    InvalidNetwork,
    MissingAddress,
    ChainNotFound,
    CkbtcNotInitialized,
    InvalidTx(String),
    InvalidMsg(String),
    SignError(String),
    GenerateError(String),
    PublicKeyError(String),
    MissingWitnessScript,
    MissingSighashType,
    BitcoinGetAddressError,
    NoUtxos,
    MinterError(String),
    ICRC1Error(String),
    ICRC1CallError(String),
    CkbtcPendingBalance(String),
    CkbtcUpdateBalance(String),
    CkbtcSwapToBtcError(String),
    CyclesMintingError(String),
    CanisterStatusError(String),
    UpdateSettingsError(String),
    SignerRoleNotAuthorized(String),
    SignerRoleNotFound(String, String),
    SignerNotFound(String),
    SignerAlreadyExists(String),
    SignerDoesNotExist(String),
    TransactionTooOld(u64),
    AlreadySigned(String),
    ExecutionError(String),
    NotifyTopUpError(String),
    RecoverableSignatureError(String),
    DeadlineExceeded,
    Processing,
    InvalidMessageLength,
    CallerIsNotOwner,
    CannotRemoveDefaultAccount,
    EcdsaPublicKeyAlreadySet,
    EcdsaPublicKeyError(String),
    MissingEcdsaPublicKey,
    InvalidEcdsaPublicKey,
    InvalidAccountIdentifier,
    WalletAccountNotExists,
    RequestNotExists,
    InvalidToken,
    InvalidAddress,
    InvalidNetworkAddress,
    NotSignedTransaction,
    InvalidController,
    WalletAccountAlreadyExists,
    WalletAccountCounterMismatch,
    WasmNotLoaded,
    ChainTypeMismatch
}

#[rustfmt::skip]
impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WalletError::BitcoinError(ref err) => write!(f, "::Bitcoin error: {}", err),
            WalletError::EvmError(ref err) => write!(f, "::EVM error: {}", err),
            WalletError::CkbtcError(ref err) => write!(f, "::CKBTC error: {}", err),
            WalletError::IcrcError(ref err) => write!(f, "::ICRC error: {}", err),
            WalletError::IcpError(ref err) => write!(f, "::ICP error: {}", err),
            WalletError::UnknownError => write!(f, "::Unknown error"),
            WalletError::InvalidToken => write!(f, "::Invalid token"),
            WalletError::InvalidRequest => write!(f, "::Invalid request"),
            WalletError::InvalidNetwork => write!(f, "::Invalid network"),
            WalletError::MissingAddress => write!(f, "::Missing address"),
            WalletError::ChainNotFound => write!(f, "::Chain not found"),
            WalletError::CkbtcNotInitialized => write!(f, "::CKBTC not initialized"),
            WalletError::InvalidTx(ref msg) => write!(f, "::Invalid transaction: {}", msg),
            WalletError::InvalidMsg(ref msg) => write!(f, "::Invalid message: {}", msg),
            WalletError::SignError(ref msg) => write!(f, "::Sign error: {}", msg),
            WalletError::LedgerError(ref msg) => write!(f, "::Ledger error: {}", msg),
            WalletError::GenerateError(ref msg) => write!(f, "::Generation error: {}", msg),
            WalletError::PublicKeyError(ref msg) => write!(f, "::Public key error: {}", msg),
            WalletError::MissingWitnessScript => write!(f, "::Missing witness script"),
            WalletError::MissingSighashType => write!(f, "::Missing sighash type"),
            WalletError::BitcoinGetAddressError => write!(f, "::Bitcoin get address error"),
            WalletError::NoUtxos => write!(f, "::No UTXOs"),
            WalletError::MinterError(ref msg) => write!(f, "::Minter error: {}", msg),
            WalletError::ICRC1Error(ref msg) => write!(f, "::ICRC1 error: {}", msg),
            WalletError::ICRC1CallError(ref msg) => write!(f, "::ICRC1 call error: {}", msg),
            WalletError::CkbtcPendingBalance(ref msg) => write!(f, "::Pending CKBTC balance failed: {}", msg),
            WalletError::CkbtcUpdateBalance(ref msg) => write!(f, "::Update CKBTC balance failed: {}", msg),
            WalletError::CkbtcSwapToBtcError(ref msg) => write!(f, "::Swap CKBTC to BTC failed: {}", msg),
            WalletError::CyclesMintingError(ref msg) => write!(f, "::Cycles minting error: {}", msg),
            WalletError::CanisterStatusError(ref msg) => write!(f, "::Canister status error: {}", msg),
            WalletError::UpdateSettingsError(ref msg) => write!(f, "::Update settings error: {}", msg),
            WalletError::SignerRoleNotAuthorized(ref msg) => write!(f, "::Signer {} is not authorized to sign!", msg),
            WalletError::SignerRoleNotFound(ref signer, ref role) => write!(f, "::Signer {} does not have role {}", signer, role),
            WalletError::SignerNotFound(ref msg) => write!(f, "::{} is not a signer!", msg),
            WalletError::SignerAlreadyExists(ref msg) => write!(f, "::Signer {} already exists!", msg),
            WalletError::SignerDoesNotExist(ref msg) => write!(f, "::Signer {} does not exist!", msg),
            WalletError::TransactionTooOld(nanos) => write!(f, "::Transaction too old: {} nanoseconds", nanos),
            WalletError::AlreadySigned(ref msg) => write!(f, "::Signer {} already signed", msg),
            WalletError::ExecutionError(ref msg) => write!(f, "::Execution error: {}", msg),
            WalletError::NotifyTopUpError(ref msg) => write!(f, "::Notify top up error: {}", msg),
            WalletError::RecoverableSignatureError(ref msg) => write!(f, "::Recoverable signature error: {}", msg),
            WalletError::DeadlineExceeded => write!(f, "::Deadline exceeded!"),
            WalletError::Processing => write!(f, "::Processing error"),
            WalletError::InvalidMessageLength => write!(f, "::Invalid message length"),
            WalletError::CallerIsNotOwner => write!(f, "::Caller is not the owner"),
            WalletError::CannotRemoveDefaultAccount => write!(f, "::Cannot remove default account!"),
            WalletError::EcdsaPublicKeyAlreadySet => write!(f, "::Public key already exists"),
            WalletError::EcdsaPublicKeyError(ref msg) => write!(f, "::ECDSA public key error: {}", msg),
            WalletError::MissingEcdsaPublicKey => write!(f, "::Missing ECDSA public key"),
            WalletError::InvalidEcdsaPublicKey => write!(f, "::Invalid ECDSA public key!"),
            WalletError::InvalidAccountIdentifier => write!(f, "::Invalid account identifier!"),
            WalletError::WalletAccountNotExists => write!(f, "::Wallet account does not exist!"),
            WalletError::RequestNotExists => write!(f, "::Request does not exist!"),
            WalletError::InvalidAddress => write!(f, "::Invalid address!"),
            WalletError::InvalidNetworkAddress => write!(f, "::Invalid network address"),
            WalletError::NotSignedTransaction => write!(f, "::Not signed transaction!"),
            WalletError::InvalidController => write!(f, "::Invalid controller!"),
            WalletError::WalletAccountAlreadyExists => write!(f, "::Wallet account already exists!"),
            WalletError::WalletAccountCounterMismatch => write!(f, "::Wallet account counter mismatch!"),
            WalletError::WasmNotLoaded => write!(f, "::Wasm not loaded!"),
            WalletError::ChainTypeMismatch => write!(f, "::Chain type mismatch!"),
        }
    }
}
impl From<LedgerError> for WalletError {
    fn from(error: LedgerError) -> Self {
        WalletError::LedgerError(error)
    }
}

impl From<BitcoinError> for WalletError {
    fn from(error: BitcoinError) -> Self {
        WalletError::BitcoinError(error)
    }
}

impl From<EvmError> for WalletError {
    fn from(error: EvmError) -> Self {
        WalletError::EvmError(error)
    }
}

impl From<CkbtcError> for WalletError {
    fn from(value: CkbtcError) -> Self {
        WalletError::CkbtcError(value)
    }
}

impl From<IcrcError> for WalletError {
    fn from(value: IcrcError) -> Self {
        WalletError::IcrcError(value)
    }
}

impl From<IcpError> for WalletError {
    fn from(value: IcpError) -> Self {
        WalletError::IcpError(value)
    }
}
