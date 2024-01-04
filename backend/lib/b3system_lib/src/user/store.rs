use b3_utils::{
    memory::{init_stable_mem_refcell, types::DefaultStableBTreeMap},
    types::UserId,
};

use std::cell::RefCell;

use super::{error::UserSystemError, user::User};

pub type UserMap = DefaultStableBTreeMap<UserId, User>;

// The UserState starts from 10 to 19 to avoid conflicts with the app's stable memory
thread_local! {
    static USER_MAP: RefCell<UserMap> = init_stable_mem_refcell("user_map", 10).unwrap();
}

pub fn with_users<R>(f: impl FnOnce(&UserMap) -> R) -> R {
    USER_MAP.with(|state| f(&*state.borrow()))
}

pub(crate) fn with_users_mut<R>(f: impl FnOnce(&mut UserMap) -> R) -> R {
    USER_MAP.with(|state| f(&mut *state.borrow_mut()))
}

pub(crate) fn with_user<F, T>(user_id: &UserId, f: F) -> Result<T, UserSystemError>
where
    F: FnOnce(User) -> T,
{
    with_users(|users| {
        users
            .get(user_id)
            .ok_or(UserSystemError::UserNotFound)
            .map(f)
    })
}
