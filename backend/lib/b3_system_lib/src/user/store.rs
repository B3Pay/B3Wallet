use b3_utils::{
    memory::{init_stable_mem, types::DefaultStableBTreeMap},
    types::{CanisterId, UserId},
};

use std::cell::RefCell;

use super::{error::UserSystemError, user::User, UserState};

pub type UserMap = DefaultStableBTreeMap<UserId, User>;

// The UserState starts from 10 to 19 to avoid conflicts with the app's stable memory
thread_local! {
    static USERS: RefCell<UserState> = RefCell::new(
        UserState {
            users: init_stable_mem("user_map", 10).unwrap(),
        }
    );
}

// SIGNER

pub fn with_users<R>(f: impl FnOnce(&UserMap) -> R) -> R {
    USERS.with(|state| f(&state.borrow().users))
}

pub fn with_users_mut<R>(f: impl FnOnce(&mut UserMap) -> R) -> R {
    USERS.with(|state| f(&mut state.borrow_mut().users))
}

pub fn with_user_state<F, T>(user_id: UserId, f: F) -> Result<T, UserSystemError>
where
    F: FnOnce(User) -> T,
{
    with_users(|signers| {
        signers
            .get(&user_id)
            .ok_or(UserSystemError::UserNotFound)
            .map(f)
    })
}

pub fn with_user_state_mut<F, T>(user_id: &UserId, f: F) -> Result<T, UserSystemError>
where
    F: FnOnce(&mut User) -> T,
{
    with_users_mut(|signers| {
        signers
            .get(user_id)
            .ok_or(UserSystemError::UserNotFound)
            .map(|mut user| f(&mut user))
    })
}

pub fn with_user_app<F, T>(
    user_id: UserId,
    canister_id: &CanisterId,
    f: F,
) -> Result<T, UserSystemError>
where
    F: FnOnce(&CanisterId) -> T,
{
    with_user_state(user_id, |state| {
        state
            .canisters
            .iter()
            .find(|canister| canister == &canister_id)
            .ok_or(UserSystemError::WalletCanisterNotFound)
            .map(f)
    })?
}
