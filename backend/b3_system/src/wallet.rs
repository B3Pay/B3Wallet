use std::{str::FromStr, vec};

use crate::guard::caller_is_controller;
use b3_helper_lib::{
    constants::CREATE_SIGNER_CANISTER_CYCLES,
    release::ReleaseName,
    revert,
    types::{CanisterId, SignerId, Version, WalletCanisterInitArgs},
};
use b3_system_lib::{
    error::SystemError,
    store::{
        with_state, with_state_mut, with_users_mut, with_wallet_canister, with_wallet_canister_mut,
    },
    types::WalletCanister,
    types::WalletCanisters,
};
use ic_cdk::{
    api::management_canister::main::CanisterInstallMode, export::candid::candid_method, query,
    update,
};

// QUERY CALLS

#[candid_method(query)]
#[query]
fn get_canister() -> WalletCanister {
    let user_id = ic_cdk::caller();

    with_wallet_canister(&user_id, |c| c.clone()).unwrap_or_else(revert)
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
fn get_user_ids() -> Vec<SignerId> {
    with_state(|s| s.user_ids())
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
fn get_wallet_canisters() -> WalletCanisters {
    with_state(|s| s.wallet_canisters())
}

// UPDATE CALLS

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
async fn get_canister_version(canister_id: CanisterId) -> Version {
    let wallet = WalletCanister::from(canister_id);

    wallet.version().await.unwrap_or_else(revert)
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
async fn get_canister_version_by_user(user_id: SignerId) -> Version {
    let wallet = with_wallet_canister(&user_id, |c| c.clone()).unwrap_or_else(revert);

    wallet.version().await.unwrap_or_else(revert)
}

#[update]
#[candid_method(update)]
async fn create_wallet_canister(name: String) -> Result<WalletCanister, String> {
    let owner_id = ic_cdk::caller();
    let system_id = ic_cdk::id();

    let release_name = ReleaseName::from_str(&name).unwrap_or_else(revert);

    let mut wallet_canister = with_state_mut(|s| s.init_user(owner_id)).unwrap_or_else(revert);

    wallet_canister
        .create_with_cycles(vec![owner_id, system_id], CREATE_SIGNER_CANISTER_CYCLES)
        .await
        .unwrap_or_else(revert);

    with_state_mut(|s| s.add_user(owner_id, wallet_canister.clone()));

    let init_args = WalletCanisterInitArgs {
        owner_id,
        system_id,
    };

    let install_arg_result = with_state_mut(|s| {
        s.get_latest_install_args(release_name, CanisterInstallMode::Install, init_args)
    });

    match install_arg_result {
        Ok(install_arg) => {
            // Install the code.
            let install_result = wallet_canister.install_code(install_arg).await;

            // Update the controllers, and add canister id as controller of itself.
            // this enables the canister to update itself.
            let update_result = wallet_canister
                .add_controllers(vec![owner_id, system_id])
                .await;

            match (install_result, update_result) {
                (Ok(_), Ok(_)) => Ok(wallet_canister),
                (Err(err), _) => Err(err.to_string()),
                (_, Err(err)) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

#[update]
#[candid_method(update)]
async fn install_wallet_canister(
    name: String,
    canister_id: Option<CanisterId>,
) -> Result<WalletCanister, String> {
    let system_id = ic_cdk::id();
    let owner_id = ic_cdk::caller();

    let release_name = ReleaseName::from_str(&name).unwrap_or_else(revert);

    let mut wallet_canister =
        with_state_mut(|s| s.get_or_init_user(owner_id, canister_id)).unwrap_or_else(revert);

    let init_args = WalletCanisterInitArgs {
        owner_id,
        system_id,
    };

    let install_arg_result = with_state_mut(|s| {
        s.get_latest_install_args(release_name, CanisterInstallMode::Install, init_args)
    });

    match install_arg_result {
        Ok(install_arg) => {
            let status = wallet_canister.status().await;

            if status.is_ok() {
                revert(SystemError::WalletCanisterAlreadyInstalled)
            }

            // Install the code.
            let install_result = wallet_canister.install_code(install_arg).await;

            // Update the controllers, and add the user and canister id as controller of itself.
            // this enables the canister to update itself.
            let update_result = wallet_canister
                .add_controllers(vec![owner_id, system_id])
                .await;

            match (install_result, update_result) {
                (Ok(_), Ok(_)) => Ok(wallet_canister),
                (Err(err), _) => Err(err.to_string()),
                (_, Err(err)) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

#[update]
#[candid_method(update)]
async fn add_wallet_canister(canister_id: CanisterId) {
    let user_id = ic_cdk::caller();

    let wallet_canister = WalletCanister::from(canister_id);

    let is_valid = wallet_canister
        .validate_signer(user_id)
        .await
        .unwrap_or_else(revert);

    if is_valid {
        with_state_mut(|s| s.get_or_init_user(user_id, Some(canister_id))).unwrap_or_else(revert);
    } else {
        revert(SystemError::InvalidWalletCanister)
    }
}

#[update]
#[candid_method(update)]
fn change_wallet_canister(canister_id: CanisterId) {
    let user_id = ic_cdk::caller();

    with_wallet_canister_mut(&user_id, |c| c.add_canister_id(canister_id)).unwrap_or_else(revert);
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn remove_wallet_canister(user_id: SignerId) {
    with_state_mut(|s| s.remove_user(&user_id));
}

// TODO: remove this method
#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn reset_users() {
    with_users_mut(|s| s.clear());
}
