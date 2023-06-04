use crate::permit::caller_is_signer;
use b3_helper_lib::revert;
use b3_helper_lib::{b3_canister_status, types::WalletCanisterStatus};
use b3_wallet_lib::store::with_wallet;
use ic_cdk::export::candid::candid_method;
use ic_cdk::{api::time, query, update};

#[cfg(test)]
use b3_helper_lib::mocks::ic_cdk_id;
#[cfg(not(test))]
use ic_cdk::api::id as ic_cdk_id;

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn status() -> WalletCanisterStatus {
    let canister_id = ic_cdk_id();

    let version = version();

    let canister_status = b3_canister_status(canister_id).await.unwrap_or_else(revert);

    let account_status = with_wallet(|s| s.account_status());
    let status_at = time();

    WalletCanisterStatus {
        canister_id,
        version,
        status_at,
        canister_status,
        account_status,
    }
}

#[query]
#[candid_method(query)]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
