use b3_operations::{
    store::{with_operation, with_operation_mut, with_verified_user},
    types::UserMap,
    user::{role::UserRole, User},
};
use b3_utils::types::UserId;
use candid::candid_method;
use ic_cdk::{query, update};

pub fn caller_is_canister_or_admin() -> Result<(), String> {
    let caller_id = ic_cdk::caller();

    with_verified_user(caller_id, |signer| signer.is_canister_or_admin())
}

pub fn caller_is_admin() -> Result<(), String> {
    let caller_id = ic_cdk::caller();

    with_verified_user(caller_id, |signer| signer.is_admin())
}

pub fn _caller_is_user() -> Result<(), String> {
    let caller_id = ic_cdk::caller();

    with_verified_user(caller_id, |signer| signer.is_user())
}

pub fn caller_is_signer() -> Result<(), String> {
    let caller_id = ic_cdk::caller();

    with_verified_user(caller_id, |_| true)
}

#[query]
#[candid_method(query)]
pub fn validate_signer(signer_id: UserId) -> bool {
    with_operation(|u| u.users.contains_key(&signer_id))
}

#[query(guard = "caller_is_admin")]
#[candid_method(query)]
pub fn get_signers() -> UserMap {
    with_operation(|u| u.users.clone())
}

#[update(guard = "caller_is_admin")]
#[candid_method(update)]
pub fn signer_add(signer_id: UserId, role: UserRole) -> UserMap {
    let signer = User::from(role);

    with_operation_mut(|u| {
        u.users.insert(signer_id.clone(), signer);

        u.users.clone()
    })
}

#[update(guard = "caller_is_admin")]
#[candid_method(update)]
pub fn signer_remove(signer_id: UserId) -> UserMap {
    with_operation_mut(|u| {
        u.users.remove(&signer_id);

        u.users.clone()
    })
}
