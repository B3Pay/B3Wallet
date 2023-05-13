use crate::guards::caller_is_controller;
use b3_helper::{
    b3_revert,
    constants::CREATE_SIGNER_CANISTER_CYCLES,
    error::TrapError,
    types::{CanisterId, SignerCanister, UserId},
};
use b3_system_lib::store::{with_signer_canister_mut, with_state_mut, with_users_mut};
use ic_cdk::{
    api::management_canister::main::CanisterInstallMode, caller, export::candid::candid_method,
    update,
};

#[update]
#[candid_method(update)]
pub async fn create_signer_canister() -> Result<SignerCanister, String> {
    let user = caller();
    let system_id = ic_cdk::id();

    let mut signer_canister =
        with_state_mut(|s| s.init_user(user)).unwrap_or_else(|err| b3_revert(err));

    signer_canister
        .create_with_cycles(vec![user, system_id], CREATE_SIGNER_CANISTER_CYCLES)
        .await
        .unwrap_or_else(|err| b3_revert(err));

    with_state_mut(|s| s.add_user(user, signer_canister.clone()));

    let install_arg_result =
        with_state_mut(|s| s.get_latest_install_args(user, CanisterInstallMode::Install));

    match install_arg_result {
        Ok(install_arg) => {
            // Install the code.
            let install_result = signer_canister.install_code(install_arg).await;

            // Update the controllers, and remove this canister as a controller.
            let update_result = signer_canister.update_controllers(vec![user]).await;

            match (install_result, update_result) {
                (Ok(_), Ok(_)) => Ok(signer_canister),
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
    let user = caller();

    let mut signer_canister = with_state_mut(|s| s.get_or_init_user(user, canister_id))
        .unwrap_or_else(|err| b3_revert(err));

    let install_arg_result =
        with_state_mut(|s| s.get_latest_install_args(user, CanisterInstallMode::Install));

    match install_arg_result {
        Ok(install_arg) => {
            let install_result = signer_canister.install_code(install_arg).await;

            match install_result {
                Ok(_) => Ok(signer_canister),
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

#[update]
#[candid_method(update)]
fn change_signer_canister(canister_id: CanisterId) {
    let user = caller();

    with_signer_canister_mut(&user, |c| c.set_canister_id(canister_id))
        .unwrap_or_else(|e| b3_revert(e));
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn remove_signer_canister(user: UserId) {
    with_state_mut(|s| s.remove_user(&user));
}

// TODO: remove this method
#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn reset_users() {
    with_users_mut(|s| s.clear());
}
