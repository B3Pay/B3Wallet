use crate::permit::{caller_is_admin, caller_is_signer};
use b3_helper_lib::{
    revert,
    types::{Controller, ControllerId, ControllerMap, Metadata},
};
use b3_permit_lib::{store::with_permit, types::WalletSettingsAndSigners};
use b3_wallet_lib::store::{with_setting, with_setting_mut, with_wallet_mut};
use ic_cdk::{export::candid::candid_method, query, update};

#[candid_method(query)]
#[query(guard = "caller_is_signer")]
fn setting_and_signer() -> WalletSettingsAndSigners {
    let settings = with_setting(|s| s.clone());
    let signers = with_permit(|s| s.signers.clone());

    WalletSettingsAndSigners { settings, signers }
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
async fn add_controller_and_update(
    controller_id: ControllerId,
    name: String,
    metadata: Option<Metadata>,
) {
    let controller = Controller::new(name, metadata);

    let mut settings = with_setting(|s| s.clone());

    settings
        .add_controller_and_update(controller_id, controller)
        .await
        .unwrap_or_else(revert);

    with_wallet_mut(|w| w.set_setting(settings));
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
async fn update_controller(controller_map: ControllerMap) -> ControllerMap {
    let mut settings = with_setting(|s| s.clone());

    settings
        .update_controller_and_update(controller_map)
        .await
        .unwrap_or_else(revert);

    with_wallet_mut(|w| w.set_setting(settings));

    with_setting(|s| s.controllers().clone())
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
async fn update_settings() {
    let mut settings = with_setting(|s| s.clone());

    settings.update_settings().await.unwrap_or_else(revert);

    with_wallet_mut(|w| w.set_setting(settings));
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
async fn refresh_settings() {
    let mut settings = with_setting(|s| s.clone());

    settings.refresh_settings().await.unwrap_or_else(revert);

    with_wallet_mut(|w| w.set_setting(settings));
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
fn add_setting_metadata(key: String, value: String) {
    with_setting_mut(|s| s.add_metadata(key, value));
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
fn remove_setting_metadata(key: String) {
    with_setting_mut(|s| s.remove_metadata(&key));
}
