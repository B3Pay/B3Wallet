use crate::guards::caller_is_controller;
use b3_shared::b3_trap;
use b3_shared::types::{Canister, CanisterId, UserId, Version};
use b3_system_lib::error::SystemError;
use b3_system_lib::{store::with_state, types::Signers};
use ic_cdk::export::{candid::candid_method, Principal};
use ic_cdk::{caller, query};

#[candid_method(query)]
#[query]
pub fn get_signer() -> Option<Canister> {
    let user = caller();

    with_state(|s| s.get_signer(&user))
}

#[candid_method(query)]
#[query]
pub fn get_canister_id(user: Principal) -> Option<CanisterId> {
    with_state(|s| s.get_canister_id(&user))
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub async fn get_signer_version(user: UserId) -> Version {
    let signer = with_state(|s| s.get_signer(&user));

    match signer {
        Some(signer) => signer.version().await.unwrap_or_else(|e| b3_trap(e)),
        None => b3_trap(SystemError::SignerNotFound(user.to_string())),
    }
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub fn get_signers() -> Signers {
    with_state(|s| s.controllers.clone())
}
