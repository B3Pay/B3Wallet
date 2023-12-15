use b3_utils::{
    memory::init_stable_mem,
    types::{CanisterId, CanisterIds, UserId},
};

use std::cell::RefCell;

use super::{error::UserSystemError, user::User, UserState};

// The UserState starts from 10 to 19 to avoid conflicts with the app's stable memory
thread_local! {
    static USER_STATE: RefCell<UserState> = RefCell::new(
        UserState {
            users: init_stable_mem("user_map", 10).unwrap(),
        }
    );
}

pub fn with_user_state<R>(f: impl FnOnce(&UserState) -> R) -> R {
    USER_STATE.with(|state| f(&*state.borrow()))
}

pub fn with_user_state_mut<R>(f: impl FnOnce(&mut UserState) -> R) -> R {
    USER_STATE.with(|state| f(&mut *state.borrow_mut()))
}

pub fn with_user<F, T>(user_id: UserId, f: F) -> Result<T, UserSystemError>
where
    F: FnOnce(User) -> T,
{
    with_user_state(|signers| {
        signers
            .user(&user_id)
            .ok_or(UserSystemError::UserNotFound)
            .map(f)
    })
}

pub fn with_user_mut<F, T>(user_id: &UserId, f: F) -> Result<T, UserSystemError>
where
    F: FnOnce(&mut User) -> T,
{
    with_user_state_mut(|signers| {
        signers
            .user(user_id)
            .ok_or(UserSystemError::UserNotFound)
            .map(|mut user| f(&mut user))
    })
}

pub fn with_user_apps<F, T>(user_id: UserId, f: F) -> Result<T, UserSystemError>
where
    F: FnOnce(CanisterIds) -> T,
{
    with_user(user_id, |state| f(state.canisters()))
}

pub fn with_user_app<F, T>(
    user_id: UserId,
    canister_id: &CanisterId,
    f: F,
) -> Result<T, UserSystemError>
where
    F: FnOnce(&CanisterId) -> T,
{
    with_user(user_id, |state| {
        state
            .canisters()
            .iter()
            .find(|canister| canister == &canister_id)
            .ok_or(UserSystemError::WalletCanisterNotFound)
            .map(f)
    })?
}
