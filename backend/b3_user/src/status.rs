use crate::guards::caller_is_owner;

use b3_user_lib::{error::SignerError, types::CanisterStatus, with_state};
use ic_cdk::{
    api::{
        call::CallResult,
        management_canister::{main::canister_status, provisional::CanisterIdRecord},
        time,
    },
    export::candid::candid_method,
    query, update,
};

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn status() -> CallResult<CanisterStatus> {
    let canister_id = ic_cdk::id();

    let version = version();

    let (canister_status,) = canister_status(CanisterIdRecord { canister_id })
        .await
        .map_err(|e| SignerError::CanisterStatusError(e.1))?;

    let accounts_status = with_state(|state| state.accounts_counters());
    let status_at = time();

    let status = CanisterStatus {
        canister_id,
        version,
        status_at,
        canister_status,
        accounts_status,
    };

    Ok(status)
}

#[candid_method(query)]
#[query]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
