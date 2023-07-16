use b3_helper_lib::owner::{caller_is_owner, with_owner};
use b3_helper_lib::revert;
use b3_helper_lib::timestamp::NanoTimeStamp;
use b3_helper_lib::types::{SignerId, WalletInititializeArgs};
use b3_helper_lib::wasm::with_wasm;
use b3_helper_lib::{ic_canister_status, types::WalletCanisterStatus};
use b3_wallet_lib::error::WalletError;
use b3_wallet_lib::setting::WalletSettings;
use b3_wallet_lib::store::{with_wallet, with_wallet_mut};
use ic_cdk::api::management_canister::main::{
    install_code, uninstall_code, CanisterInstallMode, InstallCodeArgument,
};
use ic_cdk::api::management_canister::provisional::CanisterIdRecord;
use ic_cdk::export::candid::candid_method;
use ic_cdk::{query, update};

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn init_wallet(args: WalletInititializeArgs) {
    if with_wallet(|w| w.is_initialised()) {
        return revert(WalletError::WalletAlreadyInitialized);
    }

    let mut setting = WalletSettings::new(args.controllers, args.metadata);

    setting.update_settings().await.unwrap_or_else(revert);

    with_wallet_mut(|w| w.init_wallet(setting));
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
async fn upgrage_wallet() {
    let canister_id = ic_cdk::id();
    let wasm_module = with_wasm(|w| {
        if w.is_empty() {
            return revert(WalletError::WasmNotLoaded);
        }
        w.get()
    });

    let args = InstallCodeArgument {
        canister_id,
        wasm_module,
        arg: Vec::new(),
        mode: CanisterInstallMode::Upgrade,
    };

    install_code(args).await.unwrap();
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn uninstall_wallet() {
    let canister_id = ic_cdk::id();

    let args = CanisterIdRecord { canister_id };

    uninstall_code(args).await.unwrap();
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
pub fn validate_signer(signer_id: SignerId) -> bool {
    with_owner(|o| o.eq(&signer_id))
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
