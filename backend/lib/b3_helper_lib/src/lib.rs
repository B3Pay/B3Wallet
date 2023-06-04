pub mod account;
pub mod base32;
pub mod constants;
pub mod error;
pub mod impls;
pub mod mocks;
pub mod types;
pub mod wasm;

use ::easy_hasher::easy_hasher::Hash;
use easy_hasher::easy_hasher;
use error::{HelperError, TrapError};
use ic_cdk::api::management_canister::{
    main::{canister_status, CanisterStatusResponse},
    provisional::{CanisterId, CanisterIdRecord},
};
use types::WasmHash;

pub fn raw_keccak256(data: &[u8]) -> Hash {
    easy_hasher::raw_keccak256(data.to_vec())
}

pub fn sha2_sha256(data: &[u8]) -> Vec<u8> {
    let hash = easy_hasher::raw_sha256(data.to_vec());
    hash.to_vec()
}

pub fn sha2_sha256_wasm_hash(data: &[u8]) -> WasmHash {
    let hash = easy_hasher::raw_sha256(data.to_vec());
    let bytes = hash.to_vec();

    let mut wasm_hash: WasmHash = [0; 32];
    wasm_hash.copy_from_slice(&bytes[0..32]);
    wasm_hash
}

pub fn sha2_sha256_wasm_hash_string(data: &[u8]) -> String {
    let hash = easy_hasher::raw_sha256(data.to_vec());
    hash.to_hex_string()
}

pub async fn b3_canister_status(
    canister_id: CanisterId,
) -> Result<CanisterStatusResponse, HelperError> {
    let (status,) = canister_status(CanisterIdRecord { canister_id })
        .await
        .map_err(|e| HelperError::CanisterStatusError(e.1))?;

    Ok(status)
}

pub fn revert<T, E: TrapError>(err: E) -> T {
    ic_cdk::trap(&err.to_string());
}
