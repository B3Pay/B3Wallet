use crate::guard::caller_is_controller;
use b3_helper_lib::{
    constants::CREATE_SIGNER_CANISTER_CYCLES,
    error::ErrorTrait,
    revert,
    types::{CanisterId, SignerId, Version},
};
use b3_system_lib::{
    error::SystemError,
    store::with_hash_release,
    store::{
        with_state, with_state_mut, with_users_mut, with_wallet_canister, with_wallet_canister_mut,
    },
    types::WalletCanisters,
    types::{Release, WalletCanister},
};
use ic_cdk::{
    api::management_canister::main::CanisterInstallMode, export::candid::candid_method, query,
    update,
};

// QUERY CALLS

#[candid_method(query)]
#[query]
pub fn get_canister() -> WalletCanister {
    let user_id = ic_cdk::caller();

    with_wallet_canister(&user_id, |c| c.clone()).unwrap_or_else(revert)
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub fn get_user_ids() -> Vec<SignerId> {
    with_state(|s| s.user_ids())
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub fn get_wallet_canisters() -> WalletCanisters {
    with_state(|s| s.wallet_canisters())
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub async fn get_canister_version(canister_id: CanisterId) -> Version {
    let wallet = WalletCanister::from(canister_id);

    wallet.version().await.unwrap_or_else(revert)
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub async fn get_canister_version_by_user(user_id: SignerId) -> Version {
    let wallet = with_wallet_canister(&user_id, |c| c.clone()).unwrap_or_else(revert);

    wallet.version().await.unwrap_or_else(revert)
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub async fn get_wallet_release(canister_id: CanisterId) -> Release {
    let wallet = WalletCanister::from(canister_id);

    let wasm_hash = wallet.wasm_hash().await.unwrap_or_else(revert);

    with_hash_release(wasm_hash, |r| r.clone()).unwrap_or_else(revert)
}

// UPDATE CALLS

#[update]
#[candid_method(update)]
pub async fn create_wallet_canister() -> Result<WalletCanister, String> {
    let user_id = ic_cdk::caller();
    let system_id = ic_cdk::id();

    let mut wallet_canister = with_state_mut(|s| s.init_user(user_id)).unwrap_or_else(revert);

    wallet_canister
        .create_with_cycles(vec![user_id, system_id], CREATE_SIGNER_CANISTER_CYCLES)
        .await
        .unwrap_or_else(revert);

    with_state_mut(|s| s.add_user(user_id, wallet_canister.clone()));

    let install_arg_result = with_state_mut(|s| {
        s.get_latest_install_args(user_id, Some(system_id), CanisterInstallMode::Install)
    });

    match install_arg_result {
        Ok(install_arg) => {
            // Install the code.
            let install_result = wallet_canister.install_code(install_arg).await;

            // Update the controllers, and remove this canister as a controller.
            // and get full control of the canister to the user.
            let update_result = wallet_canister.update_controllers(vec![user_id]).await;

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
pub async fn install_wallet_canister(
    canister_id: Option<CanisterId>,
) -> Result<WalletCanister, String> {
    let system_id = ic_cdk::id();
    let user_id = ic_cdk::caller();

    let mut wallet_canister =
        with_state_mut(|s| s.get_or_init_user(user_id, canister_id)).unwrap_or_else(revert);

    let install_arg_result = with_state_mut(|s| {
        s.get_latest_install_args(user_id, Some(system_id), CanisterInstallMode::Install)
    });

    match install_arg_result {
        Ok(install_arg) => {
            let status = wallet_canister.status().await;

            if status.is_ok() {
                revert(SystemError::WalletCanisterAlreadyInstalled)
            }

            // Install the code.
            let install_result = wallet_canister.install_code(install_arg).await;

            // Update the controllers, and remove this canister as a controller.
            // and get full control of the canister to the user.
            let update_result = wallet_canister.update_controllers(vec![user_id]).await;

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

    if !is_valid {
        revert(SystemError::InvalidWalletCanister)
    } else {
        with_state_mut(|s| s.get_or_init_user(user_id, Some(canister_id))).unwrap_or_else(revert);
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
