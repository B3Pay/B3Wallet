use crate::guards::caller_is_controller;
use b3_shared::error::TrapError;
use b3_shared::{b3_canister_status, types::CanisterStatus};
use b3_system_lib::store::with_state;
use ic_cdk::export::candid::candid_method;
use ic_cdk::trap;
use ic_cdk::{api::time, query, update};

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
pub async fn status() -> CanisterStatus {
    let canister_id = ic_cdk::id();

    let version = version();

    let canister_status = b3_canister_status(canister_id)
        .await
        .unwrap_or_else(|e| trap(&e.to_string()));

    let account_counter = with_state(|s| s.get_number_of_signers());
    let status_at = time();

    CanisterStatus {
        canister_id,
        version,
        status_at,
        canister_status,
        account_counter,
    }
}

#[candid_method(query)]
#[query]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
