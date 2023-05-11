use b3_shared::error::TrapError;
use ic_cdk::export::candid::{CandidType, Deserialize};

#[rustfmt::skip]
#[derive(CandidType, Deserialize)]
pub enum SystemError {
    InvalidAccountIdentifier,
    ReleaseNotFound,
    ReleaseAlreadyExists,
    CanisterIdNotFound,
    WasmNotFound,
    WasmAlreadyLoaded,
    UpdateControllersError(String),
    InstallArgError(String),
    EncodeError(String),
    WasmGetError(String),
    InstallCodeError(String),
    WasmInstallError(String),
    SignerNotFound(String),
    SignerAlreadyExists(String),
    CreateCanisterError(String),
    CanisterStatusError(String),
}

#[rustfmt::skip]
impl TrapError for SystemError {
    fn to_string(self) -> String {
        match self {
            SystemError::InstallArgError(e) => ["Install arg error: ", &e].concat(),
            SystemError::UpdateControllersError(e) => ["Update controllers error: ", &e].concat(),
            SystemError::WasmInstallError(e) => ["Wasm install error: ", &e].concat(),
            SystemError::SignerNotFound(e) => ["Signer not found: ", &e].concat(),
            SystemError::CanisterStatusError(e) => ["Canister status error: ", &e].concat(),
            SystemError::InvalidAccountIdentifier => "Invalid account identifier!".to_string(),
            SystemError::ReleaseNotFound => "Release not found!".to_string(),
            SystemError::ReleaseAlreadyExists => "Release already exists!".to_string(),
            SystemError::WasmNotFound => "Wasm not found!".to_string(),
            SystemError::CanisterIdNotFound => "Canister id not found!".to_string(),
            SystemError::WasmAlreadyLoaded => "Wasm already loaded!".to_string(),
            SystemError::WasmGetError(e) => ["Wasm get error: ", &e].concat(),
            SystemError::EncodeError(e) => ["Encode error: ", &e].concat(),
            SystemError::CreateCanisterError(e) => ["Create canister error: ", &e].concat(),
            SystemError::InstallCodeError(e) => ["Install code error: ", &e].concat(),
            SystemError::SignerAlreadyExists(e) => ["Signer already exists: ", &e].concat(),
        }
    }
}
