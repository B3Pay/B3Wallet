use crate::guards::caller_is_owner;
use b3_shared::error::TrapError;
use b3_shared::{b3_canister_status, types::CanisterStatus};
use b3_signer_lib::store::with_state;
use ic_cdk::export::candid::candid_method;
use ic_cdk::trap;
use ic_cdk::{api::time, query, update};

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn status() -> CanisterStatus {
    let canister_id = ic_cdk::id();

    let version = version();

    let canister_status = b3_canister_status(canister_id)
        .await
        .unwrap_or_else(|e| trap(&e.to_string()));

    let accounts_status = with_state(|state| state.accounts_counters());
    let status_at = time();

    CanisterStatus {
        canister_id,
        version,
        status_at,
        canister_status,
        accounts_status,
    }
}

#[candid_method(query)]
#[query]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
