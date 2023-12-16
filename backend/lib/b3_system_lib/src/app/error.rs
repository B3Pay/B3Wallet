use b3_utils::error::HelperError;
use candid::{CandidType, Deserialize};

#[rustfmt::skip]
#[derive(CandidType, Deserialize)]
pub enum AppSystemError {
    HelperError(HelperError),
    AppIdMismatch,
    InvalidSigner,
    ValidateSignerError(String),
    UpdateCanisterControllersError(String),
    VersionError(String),
    RateLimitExceeded,
    InvalidReleaseName(String),
    InvalidWalletCanister,
    InvalidAccountIdentifier,
    ReleaseNotFound,
    ReleaseNameNotFound,
    ReleaseAlreadyExists,
    WasmNotFound,
    WasmAlreadyLoaded,
    UserAlreadyExists,
    NoCanisterAvailable,
    OwnerMismatch { owner: String, user: String },
    UpdateControllersError(String),
    InstallArgError(String),
    EncodeError(String),
    WasmGetError(String),
    WasmHashError(String),
    InstallCodeError(String),
    WasmInstallError(String),
    WalletCanisterNotFound,
    WalletCanisterAlreadyInstalled,
    WalletCanisterRateError(String),
    WalletCanisterDoesNotExist(String),
    WalletCanisterAlreadyExists(String),
    CreateCanisterError(String),
    CanisterStatusError(String),
    AppNotFound,
    AppAlreadyExists,
    AppIsDeprecated,
    CanisterIdNotFound,
}

use std::fmt;

#[rustfmt::skip]
impl fmt::Display for AppSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppSystemError::HelperError(e) => write!(f, "{}", e),
            AppSystemError::AppIdMismatch => write!(f, "App id mismatch!"),
            AppSystemError::AppIsDeprecated => write!(f, "App is deprecated!"),
            AppSystemError::InvalidSigner => write!(f, "Invalid user!"),
            AppSystemError::ValidateSignerError(e) => write!(f, "Validate user error: {}", e),
            AppSystemError::UpdateCanisterControllersError(e) => write!(f, "Update canister controllers error: {}", e),
            AppSystemError::VersionError(e) => write!(f, "Version error: {}", e),
            AppSystemError::RateLimitExceeded => write!(f, "Rate limit exceeded!"),
            AppSystemError::InvalidWalletCanister => write!(f, "Invalid wallet canister!"),
            AppSystemError::OwnerMismatch { owner, user } => write!(f, "Owner mismatch: {} != {}", owner, user),
            AppSystemError::InstallArgError(e) => write!(f, "Install arg error: {}", e),
            AppSystemError::UpdateControllersError(e) => write!(f, "Update controllers error: {}", e),
            AppSystemError::WasmInstallError(e) => write!(f, "Wasm install error: {}", e),
            AppSystemError::InvalidReleaseName(e) => write!(f, "Invalid release name: {}", e),
            AppSystemError::InvalidAccountIdentifier => write!(f, "Invalid account identifier!"),
            AppSystemError::ReleaseNotFound => write!(f, "Release not found!"),
            AppSystemError::ReleaseNameNotFound => write!(f, "Release name not found!"),
            AppSystemError::UserAlreadyExists => write!(f, "User already exists!"),
            AppSystemError::NoCanisterAvailable => write!(f, "No canister available!"),
            AppSystemError::ReleaseAlreadyExists => write!(f, "Release already exists!"),
            AppSystemError::WasmNotFound => write!(f, "Wasm not found!"),
            AppSystemError::WasmAlreadyLoaded => write!(f, "Wasm already loaded!"),
            AppSystemError::WasmGetError(e) => write!(f, "Wasm get error: {}", e),
            AppSystemError::WasmHashError(e) => write!(f, "Wasm hash error: {}", e),
            AppSystemError::EncodeError(e) => write!(f, "Encode error: {}", e),
            AppSystemError::InstallCodeError(e) => write!(f, "Install code error: {}", e),
            AppSystemError::CreateCanisterError(e) => write!(f, "Create canister error: {}", e),
            AppSystemError::CanisterStatusError(e) => write!(f, "Wallet status error: {}", e),
            AppSystemError::AppNotFound => write!(f, "Application not found!"),
            AppSystemError::AppAlreadyExists => write!(f, "Application already exists!"),
            AppSystemError::CanisterIdNotFound => write!(f, "Canister id not found!"),
            AppSystemError::WalletCanisterRateError(e) => write!(f, "Wallet canister rate error: {}", e),
            AppSystemError::WalletCanisterNotFound => write!(f, "Wallet Canister id not found!"),
            AppSystemError::WalletCanisterDoesNotExist(e) => write!(f, "Wallet does not exist: {}", e),
            AppSystemError::WalletCanisterAlreadyExists(e) => write!(f, "Wallet already exists: {}", e),
            AppSystemError::WalletCanisterAlreadyInstalled => write!(f, "Wallet canister already installed!"),
        }
    }
}
