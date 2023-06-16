use b3_helper_lib::owner::caller_is_owner;
use b3_helper_lib::revert;
use b3_helper_lib::time::NanoTimeStamp;
use b3_helper_lib::types::InititializeWalletArgs;
use b3_helper_lib::{ic_canister_status, types::WalletCanisterStatus};
use b3_wallet_lib::error::WalletError;
use b3_wallet_lib::setting::WalletSettings;
use b3_wallet_lib::store::{with_wallet, with_wallet_mut};
use ic_cdk::export::candid::candid_method;
use ic_cdk::{query, update};

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn init_wallet(args: InititializeWalletArgs) {
    if with_wallet(|w| w.is_initialised()) {
        return revert(WalletError::WalletAlreadyInitialized);
    }

    let mut setting = WalletSettings::new(args.controllers, args.metadata);

    setting.update_settings().await.unwrap_or_else(revert);

    with_wallet_mut(|w| w.init_wallet(setting));
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn status() -> WalletCanisterStatus {
    let canister_id = ic_cdk::api::id();

    let version = version();
    let name = name();

    let canister_status = ic_canister_status(canister_id).await.unwrap_or_else(revert);

    let account_status = with_wallet(|s| s.account_status());
    let status_at = NanoTimeStamp::now();

    WalletCanisterStatus {
        canister_id,
        name,
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
