use b3_operations::{
    role::Role,
    store::{
        with_operation, with_operation_mut, with_user, with_users, with_users_mut,
        with_verified_user,
    },
    types::UserMap,
    user::User,
};
use b3_utils::types::UserId;
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
pub fn validate_signer(signer_id: UserId) -> bool {
    with_user(&signer_id, |_| true).is_ok()
}

#[query(guard = "caller_is_admin")]
pub fn get_signers() -> UserMap {
    with_users(|u| u.users().clone())
}

#[update(guard = "caller_is_admin")]
pub fn signer_add(signer_id: UserId, role: Role) -> UserMap {
    let signer = User::from(role);

    with_users_mut(|users| {
        users.add(signer_id.clone(), signer);

        users.get_users()
    })
}

#[update(guard = "caller_is_admin")]
pub fn signer_remove(signer_id: UserId) -> UserMap {
    with_users_mut(|users| {
        users.remove(&signer_id);

        users.get_users()
    })
}
