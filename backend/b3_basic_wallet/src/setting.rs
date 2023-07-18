use b3_helper_lib::owner::{caller_is_owner, with_owner_mut};
use b3_helper_lib::revert;
use b3_helper_lib::types::{
    ControllerId, Metadata, SignerId, WalletController, WalletControllerMap,
};
use b3_wallet_lib::store::with_wallet_mut;
use b3_wallet_lib::{
    setting::WalletSettings,
    store::{with_setting, with_setting_mut},
};

use candid::candid_method;
use ic_cdk::{query, update};

#[candid_method(query)]
#[query(guard = "caller_is_owner")]
fn setting() -> WalletSettings {
    let settings = with_setting(|s| s.clone());

    settings
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
async fn change_owner(new_owner: SignerId) {
    with_owner_mut(|o| *o = new_owner);
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
async fn add_controller_and_update(
    controller_id: ControllerId,
    name: String,
    metadata: Option<Metadata>,
) {
    let controller = WalletController::new(name, metadata);

    let mut settings = with_setting(|s| s.clone());

    settings
        .add_controller_and_update(controller_id, controller)
        .await
        .unwrap_or_else(revert);

    with_wallet_mut(|w| w.set_setting(settings));
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
async fn update_controller(controller_map: WalletControllerMap) -> WalletControllerMap {
    let mut settings = with_setting(|s| s.clone());

    settings
        .update_controller_and_update(controller_map)
        .await
        .unwrap_or_else(revert);

    with_wallet_mut(|w| w.set_setting(settings));

    with_setting(|s| s.controllers().clone())
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
async fn update_settings() {
    let mut settings = with_setting(|s| s.clone());

    settings.update_settings().await.unwrap_or_else(revert);

    with_wallet_mut(|w| w.set_setting(settings));
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
async fn refresh_settings() {
    let mut settings = with_setting(|s| s.clone());

    settings.refresh_settings().await.unwrap_or_else(revert);

    with_wallet_mut(|w| w.set_setting(settings));
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
fn add_setting(key: String, value: String) {
    with_setting_mut(|s| s.add_metadata(key, value));
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
fn remove_setting(key: String) {
    with_setting_mut(|s| s.remove_metadata(&key));
}
