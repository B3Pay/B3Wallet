pub mod canister;
pub mod constants;
pub mod error;
pub mod identifier;
pub mod subaccount;
pub mod types;

use error::{SharedError, TrapError};
use ic_cdk::api::management_canister::{
    main::{canister_status, CanisterStatusResponse},
    provisional::{CanisterId, CanisterIdRecord},
};
use sha2::{Digest, Sha256};

pub fn b3_sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn b3_sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);

    hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

pub async fn b3_canister_status(
    canister_id: CanisterId,
) -> Result<CanisterStatusResponse, SharedError> {
    let (status,) = canister_status(CanisterIdRecord { canister_id })
        .await
        .map_err(|e| SharedError::CanisterStatusError(e.1))?;

    Ok(status)
}

pub fn b3_trap<E: TrapError>(err: E) -> ! {
    ic_cdk::trap(&err.to_string())
}
