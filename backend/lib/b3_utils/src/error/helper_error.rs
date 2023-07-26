use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(CandidType, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum HelperError {
    InvalidReleaseName(String),
    InvalidSubaccount(String),
    ValidateSignerError(String),
    WasmHashError(String),
    CreateCanisterError(String),
    EncodeError(String),
    InstallCodeError(String),
    VersionError(String),
    CanisterStatusError(String),
    SignerNotAvailable,
    RateLimitExceeded,
    UpdateCanisterControllersError(String),
    InvalidAccountIdentifier,
}

#[rustfmt::skip]
impl fmt::Display for HelperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HelperError::InvalidReleaseName(e) => write!(f, "Invalid release name: {}", e),
            HelperError::InvalidSubaccount(e) => write!(f, "Invalid subaccount: {}", e),
            HelperError::ValidateSignerError(e) => write!(f, "Get owner error: {}", e),
            HelperError::WasmHashError(e) => write!(f, "Wasm hash error: {}", e),
            HelperError::CreateCanisterError(e) => write!(f, "Create canister error: {}", e),
            HelperError::EncodeError(e) => write!(f, "Encode error: {}", e),
            HelperError::InstallCodeError(e) => write!(f, "Install code error: {}", e),
            HelperError::VersionError(e) => write!(f, "Version error: {}", e),
            HelperError::CanisterStatusError(e) => write!(f, "Canister status error: {}", e),
            HelperError::SignerNotAvailable => write!(f, "Signer not available!"),
            HelperError::RateLimitExceeded => write!(f, "Rate limit exceeded, please try again later!"),
            HelperError::UpdateCanisterControllersError(e) => write!(f, "Update canister controllers error: {}", e),
            HelperError::InvalidAccountIdentifier => write!(f, "Invalid account identifier!"),
        }
    }
}
