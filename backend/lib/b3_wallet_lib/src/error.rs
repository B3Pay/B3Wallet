use b3_helper_lib::error::HelperError;
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
    HelperError(HelperError),
    UnknownError,
    WasmNotLoaded,
    ExecutionError(String),
    SignerAlreadyExists(String),
    SignerDoesNotExist(String),
    UpdateSettingsError(String),
    NotifyTopUpError(String),
    CannotRemoveDefaultAccount,
    WalletAccountNotExists,
    WalletAlreadyInitialized,
    WalletAccountAlreadyExists,
    WalletAccountCounterMismatch,
    TooManyControllers,
    ControllerAlreadyExists,
    CannotRemoveSelf,
    UpdateCanisterControllersError(String),
}

#[rustfmt::skip]
impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WalletError::BitcoinError(ref err) => write!(f, "Bitcoin Error::{}", err),
            WalletError::EvmError(ref err) => write!(f, "EVM Error::{}", err),
            WalletError::CkbtcError(ref err) => write!(f, "CKBTC Error::{}", err),
            WalletError::IcrcError(ref err) => write!(f, "ICRC Error::{}", err),
            WalletError::IcpError(ref err) => write!(f, "ICP Error::{}", err),
            WalletError::LedgerError(ref err) => write!(f, "Ledger Error::{}", err),
            WalletError::HelperError(ref err) => write!(f, "Helper Error::{}", err),
            WalletError::ExecutionError(ref msg) => write!(f, "Execution Error::{}", msg),
            WalletError::NotifyTopUpError(ref msg) => write!(f, "Notify top up Error::{}", msg),
            WalletError::UpdateSettingsError(ref msg) => write!(f, "Update settings Error::{}", msg),
            WalletError::UnknownError => write!(f, "Unknown Error!"),
            WalletError::WasmNotLoaded => write!(f, "Wasm not loaded!"),
            WalletError::SignerAlreadyExists(ref msg) => write!(f, "Signer ({}) already exists!", msg),
            WalletError::SignerDoesNotExist(ref msg) => write!(f, "Signer ({}) does not exist!", msg),
            WalletError::CannotRemoveDefaultAccount => write!(f, "Cannot remove default account!"),
            WalletError::WalletAlreadyInitialized => write!(f, "Wallet already initialized!"),
            WalletError::WalletAccountNotExists => write!(f, "Wallet account does not exist!"),
            WalletError::WalletAccountAlreadyExists => write!(f, "Wallet account already exists!"),
            WalletError::WalletAccountCounterMismatch => write!(f, "Wallet account nonce mismatch!"),
            WalletError::ControllerAlreadyExists => write!(f, "Controller already exists!"),
            WalletError::TooManyControllers => write!(f, "Too many controllers!"),
            WalletError::CannotRemoveSelf => write!(f, "Cannot remove self!"),
            WalletError::UpdateCanisterControllersError(ref msg) => write!(f, "Update canister controllers Error::{}", msg),
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
