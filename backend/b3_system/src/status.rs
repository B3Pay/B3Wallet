use crate::guard::caller_is_controller;
use b3_helper_lib::revert;
use b3_helper_lib::time::NanoTimeStamp;
use b3_helper_lib::{ic_canister_status, types::SystemCanisterStatus};
use b3_system_lib::store::with_state;
use ic_cdk::{
    export::candid::candid_method,
    {query, update},
};

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
