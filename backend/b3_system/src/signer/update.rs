use crate::guards::caller_is_controller;
use b3_shared::b3_trap;
use b3_shared::constants::CREATE_SIGNER_CANISTER_CYCLES;
use b3_system_lib::error::SystemError;
use b3_system_lib::store::{with_state, with_state_mut};

use b3_shared::types::{Canister, CanisterId, CanisterStatus, UserId};
use ic_cdk::api::management_canister::main::CanisterInstallMode;
use ic_cdk::export::candid::candid_method;
use ic_cdk::{caller, update};

#[update]
#[candid_method(update)]
pub async fn create_signer_canister() -> Canister {
    let user = caller();

    let signer = with_state(|s| s.get_signer(&user));

    match signer {
        Some(signer) => signer,
        None => {
            let sysmte_id = ic_cdk::id();
            let mut signer = Canister::from(user);

            signer
                .create_with_cycles(vec![user, sysmte_id], CREATE_SIGNER_CANISTER_CYCLES)
                .await
                .unwrap_or_else(|e| b3_trap(e));

            with_state_mut(|s| {
                s.add_signer(user, signer.clone());
            });

            let install_arg = with_state_mut(|state| state.get_latest_install_args(user))
                .unwrap_or_else(|e| b3_trap(e));

            signer
                .install_code(&install_arg, CanisterInstallMode::Install)
                .await
                .unwrap_or_else(|e| b3_trap(e));

            signer
        }
    }
}

#[update]
#[candid_method(update)]
pub async fn install_signer_canister(canister_id: Option<CanisterId>) -> Canister {
    let user = caller();

    let mut signer = match with_state(|s| s.get_signer(&user)) {
        Some(signer) => signer,
        None => {
            if let Some(canister_id) = canister_id {
                let mut new_signer = Canister::from(user);
                new_signer.set_canister_id(canister_id);
                new_signer
            } else {
                b3_trap(SystemError::SignerNotFound(user.to_string()));
            }
        }
    };

    let version = signer.version().await;

    if version.is_ok() {
        return signer;
    }

    let install_arg = with_state_mut(|state| state.get_latest_install_args(user))
        .unwrap_or_else(|err| b3_trap(err));

    signer
        .install_code(&install_arg, CanisterInstallMode::Install)
        .await
        .unwrap_or_else(|err| b3_trap(err));

    signer
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn remove_signer(user: UserId) {
    with_state_mut(|s| {
        s.remove_signer(&user);
    });
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
pub async fn get_signer_status(user: UserId) -> CanisterStatus {
    let signer = with_state(|s| s.get_signer(&user));

    match signer {
        Some(signer) => signer.status().await.unwrap_or_else(|e| b3_trap(e)),
        None => b3_trap(SystemError::SignerNotFound(user.to_string())),
    }
}
