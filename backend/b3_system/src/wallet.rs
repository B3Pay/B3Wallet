use std::{str::FromStr, vec};

use crate::guard::caller_is_controller;
use b3_system_lib::{
    error::SystemError,
    store::{
        with_state, with_state_mut, with_user_state, with_user_state_mut, with_users_mut,
        with_wallet_canister,
    },
    types::Canisters,
    types::UserStates,
    user::UserState,
    wallet::WalletCanister,
};
use b3_utils::{
    constants::CREATE_WALLET_CANISTER_CYCLES,
    release::ReleaseTypes,
    revert,
    types::{CanisterId, UserId, WalletCanisterInitArgs, WalletVersion},
};
use candid::candid_method;
use ic_cdk::{api::management_canister::main::CanisterInstallMode, query, update};

// QUERY CALLS

#[candid_method(query)]
#[query]
fn get_states() -> UserState {
    let user_id = ic_cdk::caller();

    with_state(|s| s.user_state(user_id)).unwrap_or_else(revert)
}

#[candid_method(query)]
#[query]
fn get_create_canister_wallet_cycle() -> u128 {
    CREATE_WALLET_CANISTER_CYCLES
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
fn get_user_ids() -> Vec<UserId> {
    with_state(|s| s.user_ids())
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
fn get_user_states() -> UserStates {
    with_state(|s| s.user_states())
}

#[query]
#[candid_method(query)]
fn get_canisters() -> Canisters {
    let user_id = ic_cdk::caller();

    with_user_state(&user_id, |s| s.canisters())
        .unwrap_or_else(revert)
        .unwrap_or_else(revert)
}

// UPDATE CALLS

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
async fn get_canister_version(canister_id: CanisterId) -> WalletVersion {
    let wallet = WalletCanister(canister_id);

    wallet.version().await.unwrap_or_else(revert)
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
async fn get_canister_version_by_user(user_id: UserId, index: usize) -> WalletVersion {
    let wallet = with_wallet_canister(&user_id, index, |w| w.clone()).unwrap_or_else(revert);

    wallet.version().await.unwrap_or_else(revert)
}

#[update]
#[candid_method(update)]
async fn create_wallet_canister(name: String) -> Result<UserState, String> {
    let owner_id = ic_cdk::caller();
    let system_id = ic_cdk::id();

    let release_name = ReleaseTypes::from_str(&name).unwrap_or_else(revert);

    let mut user_state = with_state_mut(|s| s.init_user(owner_id)).unwrap_or_else(revert);

    let wallet_canister = user_state
        .create_with_cycles(vec![owner_id, system_id], CREATE_WALLET_CANISTER_CYCLES)
        .await
        .unwrap_or_else(revert);

    with_state_mut(|s| s.add_user(owner_id, user_state.clone()));

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
                (Ok(_), Ok(_)) => Ok(user_state),
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
    canister_id: CanisterId,
) -> Result<UserState, String> {
    let system_id = ic_cdk::id();
    let owner_id = ic_cdk::caller();

    let release_name = ReleaseTypes::from_str(&name).unwrap_or_else(revert);

    let user_state =
        with_state_mut(|s| s.get_or_init_user(owner_id, Some(canister_id))).unwrap_or_else(revert);

    let init_args = WalletCanisterInitArgs {
        owner_id,
        system_id,
    };

    let install_arg_result = with_state_mut(|s| {
        s.get_latest_install_args(release_name, CanisterInstallMode::Install, init_args)
    });

    match install_arg_result {
        Ok(install_arg) => {
            let wallet_canister = WalletCanister(canister_id);

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
                (Ok(_), Ok(_)) => Ok(user_state),
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

    let wallet_canister = WalletCanister(canister_id);

    let is_valid = wallet_canister
        .validate_signer(user_id)
        .await
        .unwrap_or_else(revert);

    if is_valid {
        with_state_mut(|s| s.get_or_init_user(user_id, Some(canister_id))).unwrap_or_else(revert);
    } else {
        revert(SystemError::InvalidSigner)
    }
}

#[update]
#[candid_method(update)]
fn change_wallet_canister(canister_id: CanisterId, index: usize) {
    let user_id = ic_cdk::caller();

    with_user_state_mut(&user_id, |s| s.change_canister(index, canister_id)).unwrap_or_else(revert);
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn remove_wallet_canister(user_id: UserId) {
    with_state_mut(|s| s.remove_user(&user_id));
}

// TODO: remove this method
#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn reset_users() {
    with_users_mut(|s| s.clear());
}
