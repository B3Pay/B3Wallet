use crate::{
    error::OperationError,
    operation::Operation,
    operation::OperationState,
    pending::PendingOperation,
    processed::ProccessedState,
    role::RoleState,
    types::UserIds,
    user::{state::UserState, User},
};
use b3_utils::{principal::StoredPrincipal, types::OperationId};
use std::cell::RefCell;

thread_local! {
    static PROCCESSED: RefCell<ProccessedState> = RefCell::default();
    static OPERATION: RefCell<OperationState> = RefCell::default();
    static USERS: RefCell<UserState> = RefCell::default();
    static ROLES: RefCell<RoleState> = RefCell::default();
}

// STATE ----------------------------------------------------------------------

/// Get all state.
/// This will retrieve all states.
pub fn with_operation<T, F>(callback: F) -> T
where
    F: FnOnce(&OperationState) -> T,
{
    OPERATION.with(|states| callback(&states.borrow()))
}

/// Get all state mutably.
/// This will retrieve all states.
pub fn with_operation_mut<T, F>(callback: F) -> T
where
    F: FnOnce(&mut OperationState) -> T,
{
    OPERATION.with(|states| callback(&mut states.borrow_mut()))
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
    F: FnOnce(&ProccessedState) -> T,
{
    PROCCESSED.with(|states| callback(&states.borrow()))
}

/// Get Confirmed.
pub fn with_processed_operation_mut<T, F>(callback: F) -> T
where
    F: FnOnce(&mut ProccessedState) -> T,
{
    PROCCESSED.with(|states| callback(&mut states.borrow_mut()))
}

// SIGNERS ----------------------------------------------------------------------

/// Get all users.
pub fn with_users<T, F>(callback: F) -> T
where
    F: FnOnce(&UserState) -> T,
{
    USERS.with(|states| callback(&states.borrow()))
}

/// Get all users mutably.
pub fn with_users_mut<T, F>(callback: F) -> T
where
    F: FnOnce(&mut UserState) -> T,
{
    USERS.with(|states| callback(&mut states.borrow_mut()))
}

/// Get a user.
pub fn with_user<T, F>(user_id: &StoredPrincipal, callback: F) -> Result<T, OperationError>
where
    F: FnOnce(&User) -> T,
{
    USERS.with(|states| {
        let users = states.borrow();

        users.user(user_id).map(callback)
    })
}

/// Check if a user exists, and if it does, run the callback.
pub fn with_verified_user<F>(user_id: StoredPrincipal, callback: F) -> Result<(), String>
where
    F: FnOnce(&User) -> bool,
{
    with_user(&user_id, |user| {
        if callback(user) {
            Ok(())
        } else {
            Err(OperationError::UserNotAllowed(user_id).to_string())
        }
    })
    .map_err(|err| err.to_string())?
}

/// Get all users who can operate.
pub fn with_users_who_can_operate<T, F>(operation: &Operation, callback: F) -> T
where
    F: FnOnce(&UserIds) -> T,
{
    with_users(|user_map| {
        let mut user_ids = UserIds::new();

        for (user_id, user) in user_map.iter() {
            if user.can_operate(operation) {
                user_ids.push(user_id.to_owned());
            }
        }

        callback(&user_ids)
    })
}

pub fn with_roles<T, F>(callback: F) -> T
where
    F: FnOnce(&RoleState) -> T,
{
    ROLES.with(|states| callback(&states.borrow()))
}

pub fn with_roles_mut<T, F>(callback: F) -> T
where
    F: FnOnce(&mut RoleState) -> T,
{
    ROLES.with(|states| callback(&mut states.borrow_mut()))
}
