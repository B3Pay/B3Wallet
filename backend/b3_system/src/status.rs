use crate::guard::caller_is_controller;
use b3_system_lib::store::with_state;
use b3_utils::ic_canister_status;
use b3_utils::revert;
use b3_utils::types::SystemCanisterStatus;
use b3_utils::NanoTimeStamp;
use candid::candid_method;
use ic_cdk::{query, update};

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
pub async fn status() -> SystemCanisterStatus {
    let canister_id = ic_cdk::id();

    let version = version();

    let canister_status = ic_canister_status(canister_id).await.unwrap_or_else(revert);

    let user_status = with_state(|s| s.number_of_users());
    let status_at = NanoTimeStamp::now();

    SystemCanisterStatus {
        canister_id,
        version,
        status_at,
        user_status,
        canister_status,
    }
}

#[candid_method(query)]
#[query]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
