use ::easy_hasher::easy_hasher::Hash;
use easy_hasher::easy_hasher;

use ic_cdk::api::management_canister::{
    main::{canister_status, CanisterStatusResponse},
    provisional::{CanisterId, CanisterIdRecord},
};
use std::fmt::Display;

use crate::error::helper_error::HelperError;

pub fn raw_keccak256(data: &[u8]) -> Hash {
    easy_hasher::raw_keccak256(data.to_vec())
}

pub fn sha2_sha256(data: &[u8]) -> Vec<u8> {
    let hash = easy_hasher::raw_sha256(data.to_vec());
    hash.to_vec()
}

pub fn vec_to_hex_string(data: &[u8]) -> String {
    hex::encode(data)
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
