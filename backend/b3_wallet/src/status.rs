use crate::permit::caller_is_signer;
use b3_helper_lib::revert;
use b3_helper_lib::time::NanoTimeStamp;
use b3_helper_lib::{b3_canister_status, types::WalletCanisterStatus};
use b3_wallet_lib::store::with_wallet;
use ic_cdk::export::candid::candid_method;
use ic_cdk::{query, update};

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn status() -> WalletCanisterStatus {
    let canister_id = ic_cdk::api::id();

    let version = version();

    let canister_status = b3_canister_status(canister_id).await.unwrap_or_else(revert);

    let account_status = with_wallet(|s| s.account_status());
    let status_at = NanoTimeStamp::now();

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
pub fn canister_cycle_balance() -> u128 {
    ic_cdk::api::canister_balance128()
}

#[query]
#[candid_method(query)]
pub fn canister_version() -> u64 {
    ic_cdk::api::canister_version()
}

#[query]
#[candid_method(query)]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[query]
#[candid_method(query)]
pub fn name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}
