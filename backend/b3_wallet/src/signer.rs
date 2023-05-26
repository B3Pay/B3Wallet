use b3_helper_lib::types::SignerId;
use b3_link_lib::{
    signer::{Roles, Signer},
    store::{with_link, with_link_mut, with_signer_check},
    types::SignerMap,
};
use candid::candid_method;
use ic_cdk::{query, update};

pub fn caller_is_canister_or_admin() -> Result<(), String> {
    let caller_id = ic_cdk::caller();

    with_signer_check(caller_id, |signer| signer.is_canister_or_admin())
}

pub fn caller_is_admin() -> Result<(), String> {
    let caller_id = ic_cdk::caller();

    with_signer_check(caller_id, |signer| signer.is_admin())
}

pub fn _caller_is_user() -> Result<(), String> {
    let caller_id = ic_cdk::caller();

    with_signer_check(caller_id, |signer| signer.is_user())
}

pub fn caller_is_signer() -> Result<(), String> {
    let caller_id = ic_cdk::caller();

    with_signer_check(caller_id, |_| true)
}

#[query]
#[candid_method(query)]
pub fn get_signers() -> SignerMap {
    with_link(|u| u.signers.clone())
}

#[update(guard = "caller_is_admin")]
#[candid_method(update)]
pub fn signer_add(signer_id: SignerId, role: Roles) -> SignerMap {
    let signer = Signer::from(role);

    with_link_mut(|u| {
        u.signers.insert(signer_id.clone(), signer);

        u.signers.clone()
    })
}

#[update(guard = "caller_is_admin")]
#[candid_method(update)]
pub fn signer_remove(signer_id: SignerId) -> SignerMap {
    with_link_mut(|u| {
        u.signers.remove(&signer_id);

        u.signers.clone()
    })
}
