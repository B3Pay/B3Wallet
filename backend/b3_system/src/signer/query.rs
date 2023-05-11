use crate::guards::caller_is_controller;
use crate::store::with_state;
use crate::types::Signers;
use b3_shared::types::{Signer, SignerId};
use ic_cdk::export::{candid::candid_method, Principal};
use ic_cdk::{caller, query};

#[candid_method(query)]
#[query]
pub fn get_signer() -> Option<Signer> {
    let user = caller();

    with_state(|s| s.get_signer(&user))
}

#[candid_method(query)]
#[query]
pub fn get_signer_id(user: Principal) -> Option<SignerId> {
    with_state(|s| s.get_signer_id(&user))
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub fn get_signers() -> Signers {
    with_state(|s| s.controllers.clone())
}
