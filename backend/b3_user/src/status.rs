use crate::{
    guards::{caller_is_owner, ic_trap},
    types::{CanisterId, CanisterStatus},
};

use b3_user_lib::{error::SignerError, with_state};
use ic_cdk::{
    api::{
        management_canister::{
            main::{canister_status, CanisterStatusResponse},
            provisional::CanisterIdRecord,
        },
        time,
    },
    export::candid::candid_method,
    query, update,
};

pub async fn ic_canister_status(
    canister_id: CanisterId,
) -> Result<CanisterStatusResponse, SignerError> {
    let (status,) = canister_status(CanisterIdRecord { canister_id })
        .await
        .map_err(|e| SignerError::CanisterStatusError(e.1))?;

    Ok(status)
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn status() -> CanisterStatus {
    let canister_id = ic_cdk::id();

    let version = version();

    let canister_status = ic_canister_status(canister_id)
        .await
        .unwrap_or_else(|e| ic_trap(e));

    let accounts_status = with_state(|state| state.accounts_counters());
    let status_at = time();

    let status = CanisterStatus {
        canister_id,
        version,
        status_at,
        canister_status,
        accounts_status,
    };

    status
}

#[candid_method(query)]
#[query]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
