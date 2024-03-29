use b3_utils::error::HelperError;
use candid::{CandidType, Deserialize};

#[rustfmt::skip]
#[derive(CandidType, Deserialize)]
pub enum SystemError {
    HelperError(HelperError),
    InstallCodeError(String),
    WasmInstallError(String),
    AppCanisterNotFound,
    AppCanisterAlreadyInstalled,
    AppCanisterNotInstalled,
    WalletCanisterRateError(String),
    WalletCanisterDoesNotExist(String),
    WalletCanisterAlreadyExists(String),
    CreateCanisterError(String),
    CanisterStatusError(String),
    CanisterIdNotFound,
}

use std::fmt;

#[rustfmt::skip]
impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SystemError::HelperError(e) => write!(f, "{}", e),
            SystemError::WasmInstallError(e) => write!(f, "Wasm install error: {}", e),
            SystemError::InstallCodeError(e) => write!(f, "Install code error: {}", e),
            SystemError::CreateCanisterError(e) => write!(f, "Create canister error: {}", e),
            SystemError::CanisterStatusError(e) => write!(f, "Wallet status error: {}", e),
            SystemError::CanisterIdNotFound => write!(f, "Canister id not found!"),
            SystemError::WalletCanisterRateError(e) => write!(f, "Wallet canister rate error: {}", e),
            SystemError::AppCanisterNotFound => write!(f, "Wallet Canister id not found!"),
            SystemError::WalletCanisterDoesNotExist(e) => write!(f, "Wallet does not exist: {}", e),
            SystemError::WalletCanisterAlreadyExists(e) => write!(f, "Wallet already exists: {}", e),
            SystemError::AppCanisterNotInstalled => write!(f, "Wallet canister not installed!"),
            SystemError::AppCanisterAlreadyInstalled => write!(f, "Wallet canister already installed!"),
        }
    }
}
