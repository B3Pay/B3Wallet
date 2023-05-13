use crate::guards::caller_is_owner;
use b3_helper::error::TrapError;
use b3_helper::{b3_canister_status, types::SignerCanisterStatus};
use b3_signer_lib::store::with_state;
use ic_cdk::export::candid::candid_method;
use ic_cdk::trap;
use ic_cdk::{api::time, query, update};

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn status() -> SignerCanisterStatus {
    let canister_id = ic_cdk::id();

    let version = version();

    let canister_status = b3_canister_status(canister_id)
        .await
        .unwrap_or_else(|e| trap(&e.to_string()));

    let account_status = with_state(|s| s.account_status());
    let status_at = time();

    SignerCanisterStatus {
        canister_id,
        version,
        status_at,
        canister_status,
        account_status,
    }
}

#[candid_method(query)]
#[query]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
