use crate::guard::caller_is_controller;
use b3_helper::{
    constants::CREATE_SIGNER_CANISTER_CYCLES,
    error::TrapError,
    revert,
    types::{CanisterId, SignerId, Version},
};
use b3_system_lib::{
    error::SystemError,
    store::with_hash_release,
    store::{
        with_signer_canister, with_signer_canister_mut, with_state, with_state_mut, with_users_mut,
    },
    types::SignerCanisters,
    types::{Release, SignerCanister},
};
use ic_cdk::{
    api::management_canister::main::CanisterInstallMode, export::candid::candid_method, query,
    update,
};

// QUERY CALLS

#[candid_method(query)]
#[query]
pub fn get_canister() -> SignerCanister {
    let user_id = ic_cdk::caller();

    with_signer_canister(&user_id, |c| c.clone()).unwrap_or_else(revert)
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub fn get_user_ids() -> Vec<SignerId> {
    with_state(|s| s.user_ids())
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub fn get_signer_canisters() -> SignerCanisters {
    with_state(|s| s.signer_canisters())
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub async fn get_canister_version(canister_id: CanisterId) -> Version {
    let signer = SignerCanister::from(canister_id);

    signer.version().await.unwrap_or_else(revert)
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub async fn get_canister_version_by_user(user_id: SignerId) -> Version {
    let signer = with_signer_canister(&user_id, |c| c.clone()).unwrap_or_else(revert);

    signer.version().await.unwrap_or_else(revert)
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub async fn get_canister_release(canister_id: CanisterId) -> Release {
    let signer = SignerCanister::from(canister_id);

    let wasm_hash = signer.wasm_hash().await.unwrap_or_else(revert);

    with_hash_release(wasm_hash, |r| r.clone()).unwrap_or_else(revert)
}

// UPDATE CALLS

#[update]
#[candid_method(update)]
pub async fn create_signer_canister() -> Result<SignerCanister, String> {
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
pub async fn install_signer_canister(
    canister_id: Option<CanisterId>,
) -> Result<SignerCanister, String> {
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
fn change_signer_canister(canister_id: CanisterId) {
    let user_id = ic_cdk::caller();

    with_signer_canister_mut(&user_id, |c| c.set_canister_id(canister_id)).unwrap_or_else(revert);
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn remove_signer_canister(user_id: SignerId) {
    with_state_mut(|s| s.remove_user(&user_id));
}

// TODO: remove this method
#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn reset_users() {
    with_users_mut(|s| s.clear());
}
