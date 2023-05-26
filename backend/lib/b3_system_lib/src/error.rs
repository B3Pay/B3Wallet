use b3_helper_lib::error::TrapError;
use ic_cdk::export::candid::{CandidType, Deserialize};

#[rustfmt::skip]
#[derive(CandidType, Deserialize)]
pub enum SystemError {
    InvalidAccountIdentifier,
    ReleaseNotFound,
    ReleaseAlreadyExists,
    WasmNotFound,
    WasmAlreadyLoaded,
    UserAlreadyExists,
    UserNotFound,
    OwnerMismatch { owner: String, user: String },
    UpdateControllersError(String),
    InstallArgError(String),
    EncodeError(String),
    WasmGetError(String),
    InstallCodeError(String),
    WasmInstallError(String),
    WalletCanisterNotFound,
    WalletCanisterAlreadyInstalled,
    WalletCanisterRateError(String),
    WalletCanisterDoesNotExist(String),
    WalletCanisterAlreadyExists(String),
    CreateCanisterError(String),
    CanisterStatusError(String),
}

#[rustfmt::skip]
impl TrapError for SystemError {
    fn to_string(self) -> String {
        match self {
            SystemError::OwnerMismatch { owner, user } => ["Owner mismatch:", &owner, "!=", &user].join(" "),
            SystemError::InstallArgError(e) => ["Install arg error: ", &e].concat(),
            SystemError::UpdateControllersError(e) => ["Update controllers error: ", &e].concat(),
            SystemError::WasmInstallError(e) => ["Wasm install error: ", &e].concat(),
            SystemError::InvalidAccountIdentifier => "Invalid account identifier!".to_string(),
            SystemError::ReleaseNotFound => "Release not found!".to_string(),
            SystemError::UserAlreadyExists => "User already exists!".to_string(),
            SystemError::UserNotFound => "User not found!".to_string(),
            SystemError::ReleaseAlreadyExists => "Release already exists!".to_string(),
            SystemError::WasmNotFound => "Wasm not found!".to_string(),
            SystemError::WasmAlreadyLoaded => "Wasm already loaded!".to_string(),
            SystemError::WasmGetError(e) => ["Wasm get error: ", &e].concat(),
            SystemError::EncodeError(e) => ["Encode error: ", &e].concat(),
            SystemError::CreateCanisterError(e) => ["Create canister error: ", &e].concat(),
            SystemError::InstallCodeError(e) => ["Install code error: ", &e].concat(),
            SystemError::CanisterStatusError(e) => ["Wallet status error: ", &e].concat(),
            SystemError::WalletCanisterRateError(e) => ["Wallet canister rate error: ", &e].concat(),
            SystemError::WalletCanisterNotFound => "Wallet Canister id not found!".to_string(),
            SystemError::WalletCanisterDoesNotExist(e) => ["Wallet does not exist: ", &e].concat(),
            SystemError::WalletCanisterAlreadyExists(e) => ["Wallet already exists: ", &e].concat(),
            SystemError::WalletCanisterAlreadyInstalled => "Wallet canister already installed!".to_string(),
        }
    }
}
