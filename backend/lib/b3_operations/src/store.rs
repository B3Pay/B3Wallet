use crate::{
    error::OperationError,
    operation::Operation,
    pending::PendingOperation,
    processed::ProccessedState,
    state::OperationState,
    types::{ProcessedOperationMap, UserIds, UserMap},
    user::{state::UserState, User},
};
use b3_utils::types::{OperationId, UserId};
use std::cell::RefCell;

thread_local! {
    static PROCCESSED: RefCell<ProccessedState> = RefCell::default();
    static OPERATION: RefCell<OperationState> = RefCell::default();
    static USERS: RefCell<UserState> = RefCell::default();
}
// STATE ----------------------------------------------------------------------

/// Get all state.
/// This will retrieve all states.
pub fn with_operation<T, F>(callback: F) -> T
where
    F: FnOnce(&OperationState) -> T,
{
    OPERATION.with(|states| {
        let state = states.borrow();

        callback(&state)
    })
}

/// Get all state mutably.
/// This will retrieve all states.
pub fn with_operation_mut<T, F>(callback: F) -> T
where
    F: FnOnce(&mut OperationState) -> T,
{
    OPERATION.with(|states| {
        let mut state = states.borrow_mut();

        callback(&mut state)
    })
}

// REQUEST ------------------------------------------------------------------------

/// Get Request.
pub fn with_pending_operation<T, F>(
    request_id: &OperationId,
    callback: F,
) -> Result<T, OperationError>
where
    F: FnOnce(&PendingOperation) -> T,
{
    with_operation(|permit| permit.pending(request_id).map(callback))
}

/// Get Request mutably.
pub fn with_pending_operation_mut<T, F>(
    request_id: &OperationId,
    callback: F,
) -> Result<T, OperationError>
where
    F: FnOnce(&mut PendingOperation) -> T,
{
    with_operation_mut(|permit| permit.request_mut(&request_id).map(callback))
}

// PROCESSED ------------------------------------------------------------------------

/// Get Confirmed.
pub fn with_processed_operation<T, F>(callback: F) -> T
where
    F: FnOnce(&ProcessedOperationMap) -> T,
{
    PROCCESSED.with(|states| {
        let state = states.borrow();

        callback(&state.processed)
    })
}

// SIGNERS ----------------------------------------------------------------------

/// Get all users.
pub fn with_users<T, F>(callback: F) -> T
where
    F: FnOnce(&UserMap) -> T,
{
    USERS.with(|states| {
        let users = states.borrow();

        callback(&users.users())
    })
}

/// Get all users mutably.
pub fn with_users_mut<T, F>(callback: F) -> T
where
    F: FnOnce(&mut UserMap) -> T,
{
    USERS.with(|states| {
        let mut users = states.borrow_mut();

        callback(&mut users.users_mut())
    })
}

/// Get a user.
pub fn with_user<T, F>(user_id: &UserId, callback: F) -> Result<T, OperationError>
where
    F: FnOnce(&User) -> T,
{
    USERS.with(|states| {
        let users = states.borrow();

        users.user(user_id).map(callback)
    })
}

/// Check if a user exists, and if it does, run the callback.
pub fn with_verified_user<F>(user_id: UserId, callback: F) -> Result<(), String>
where
    F: FnOnce(&User) -> bool,
{
    with_user(&user_id, |user| {
        if callback(user) {
            Ok(())
        } else {
            Err(OperationError::UserNotAllowed(user_id.to_string()).to_string())
        }
    })
    .map_err(|err| err.to_string())?
}

/// Get all user with a role, admins is always included.
pub fn with_users_can_operate<T, F>(operation: Operation, callback: F) -> T
where
    F: FnOnce(&UserIds) -> T,
{
    with_users(|user_map| {
        let mut user_ids = UserIds::new();

        for (user_id, user) in user_map.iter() {
            if user.can_operate(operation.to_owned()) {
                user_ids.push(user_id.to_owned());
            }
        }

        callback(&user_ids)
    })
}
