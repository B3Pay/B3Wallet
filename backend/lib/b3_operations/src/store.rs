use crate::{
    error::OperationError,
    pending::PendingOperation,
    processed::ProcessedOperation,
    state::OperationState,
    types::{ProcessedRequestMap, UserIds},
    user::{role::UserRole, UserState},
};
use b3_utils::types::{OperationId, UserId};
use std::cell::RefCell;

thread_local! {
    static STATE: RefCell<OperationState> = RefCell::default();
}
// STATE ----------------------------------------------------------------------

/// Get all state.
/// This will retrieve all states.
pub fn with_permit<T, F>(callback: F) -> T
where
    F: FnOnce(&OperationState) -> T,
{
    STATE.with(|states| {
        let state = states.borrow();

        callback(&state)
    })
}

/// Get all state mutably.
/// This will retrieve all states.
pub fn with_permit_mut<T, F>(callback: F) -> T
where
    F: FnOnce(&mut OperationState) -> T,
{
    STATE.with(|states| {
        let mut state = states.borrow_mut();

        callback(&mut state)
    })
}

// REQUEST ------------------------------------------------------------------------

/// Get Request.
pub fn with_pending<T, F>(request_id: &OperationId, callback: F) -> Result<T, OperationError>
where
    F: FnOnce(&PendingOperation) -> T,
{
    with_permit(|permit| permit.request(request_id).map(callback))
}

/// Get Request mutably.
pub fn with_pending_mut<T, F>(request_id: &OperationId, callback: F) -> Result<T, OperationError>
where
    F: FnOnce(&mut PendingOperation) -> T,
{
    with_permit_mut(|permit| permit.request_mut(&request_id).map(callback))
}

// CONFIRMED ------------------------------------------------------------------------

/// Get Confirmed.
pub fn with_processed_requests<T, F>(callback: F) -> T
where
    F: FnOnce(&ProcessedRequestMap) -> T,
{
    with_permit(|state| callback(&state.processed))
}

/// Get Confirmed mutably.
pub fn with_processed_mut<T, F>(callback: F) -> T
where
    F: FnOnce(&mut ProcessedRequestMap) -> T,
{
    with_permit_mut(|state| callback(&mut state.processed))
}

pub fn with_processed_request<T, F>(
    request_id: &OperationId,
    callback: F,
) -> Result<T, OperationError>
where
    F: FnOnce(&ProcessedOperation) -> T,
{
    with_permit(|state| state.processed(request_id).map(callback))
}

// SIGNERS ----------------------------------------------------------------------

/// Get a user.
pub fn with_user<T, F>(user_id: &UserId, callback: F) -> Result<T, OperationError>
where
    F: FnOnce(&UserState) -> T,
{
    with_permit(|permit| permit.user(user_id).map(callback))
}

/// Check if a user exists, and optionally check if it has a role.
pub fn with_user_check<F>(user_id: UserId, callback: F) -> Result<(), String>
where
    F: FnOnce(&UserState) -> bool,
{
    with_permit(|permit| {
        permit
            .users
            .get(&user_id)
            .ok_or(OperationError::UserNotFound(user_id.to_string()).to_string())
            .map(callback)
            .and_then(|result| {
                if result {
                    Ok(())
                } else {
                    Err(OperationError::UserNotFound(user_id.to_string()).to_string())
                }
            })
    })
}

/// Get all user with a role, admins is always included.
pub fn with_user_ids_by_role<T, F>(role: UserRole, callback: F) -> T
where
    F: FnOnce(&UserIds) -> T,
{
    with_permit(|permit| {
        let filtered_signers: UserIds = permit
            .users
            .iter()
            .filter(|(_, user)| user.has_role(role.to_owned()))
            .map(|(user_id, _)| user_id.clone())
            .collect();

        callback(&filtered_signers)
    })
}
