pub mod account;
pub mod base32;
pub mod constants;
pub mod environment;
pub mod error;
pub mod identifier;
pub mod owner;
pub mod release;
pub mod subaccount;
pub mod time;
pub mod tokens;
pub mod types;
pub mod wasm;

#[cfg(test)]
pub mod mocks;

use ::easy_hasher::easy_hasher::Hash;
use easy_hasher::easy_hasher;
use error::HelperError;
use ic_cdk::api::management_canister::{
    main::{canister_status, CanisterStatusResponse},
    provisional::{CanisterId, CanisterIdRecord},
};
use std::fmt::Display;
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

pub async fn ic_canister_status(
    canister_id: CanisterId,
) -> Result<CanisterStatusResponse, HelperError> {
    let (status,) = canister_status(CanisterIdRecord { canister_id })
        .await
        .map_err(|e| HelperError::CanisterStatusError(e.1))?;

    Ok(status)
}

pub fn revert<T, E: Display>(err: E) -> T {
    ic_cdk::trap(&format!("Error::{}", err));
}
