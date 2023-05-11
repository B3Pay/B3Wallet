use super::utils::new_signer;
use crate::guards::caller_is_controller;
use crate::store::{with_state, with_state_mut};

use b3_shared::types::{Signer, UserId};
use ic_cdk::export::candid::candid_method;
use ic_cdk::{caller, update};

#[update]
#[candid_method(update)]
pub async fn create_signer() -> Result<Signer, String> {
    let user = caller();

    let signer = with_state(|s| s.get_signer(&user));

    match signer {
        Some(signer) => Ok(signer),
        None => new_signer(user).await,
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn remove_signer(user: UserId) {
    with_state_mut(|s| {
        s.remove_signer(&user);
    });
}
