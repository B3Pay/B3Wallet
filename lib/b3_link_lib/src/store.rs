use crate::{
    confirmed::ConfirmedRequest,
    error::RequestError,
    pending::PendingRequest,
    signer::{Roles, Signer},
    state::LinkState,
    types::{ConfirmedRequestMap, RequestId},
};
use b3_helper_lib::{error::TrapError, types::SignerId};
use std::cell::RefCell;

thread_local! {
    static STATE: RefCell<LinkState> = RefCell::default();
}
// STATE ----------------------------------------------------------------------

/// Get all state.
/// This will retrieve all states.
pub fn with_link<T, F>(callback: F) -> T
where
    F: FnOnce(&LinkState) -> T,
{
    STATE.with(|states| {
        let state = states.borrow();

        callback(&state)
    })
}

/// Get all state mutably.
/// This will retrieve all states.
pub fn with_link_mut<T, F>(callback: F) -> T
where
    F: FnOnce(&mut LinkState) -> T,
{
    STATE.with(|states| {
        let mut state = states.borrow_mut();

        callback(&mut state)
    })
}

// REQUEST ------------------------------------------------------------------------

/// Get Request.
pub fn with_pending<T, F>(request_id: &RequestId, callback: F) -> Result<T, RequestError>
where
    F: FnOnce(&PendingRequest) -> T,
{
    with_link(|link| link.request(request_id).map(callback))
}

/// Get Request mutably.
pub fn with_pending_mut<T, F>(request_id: &RequestId, callback: F) -> Result<T, RequestError>
where
    F: FnOnce(&mut PendingRequest) -> T,
{
    with_link_mut(|link| link.request_mut(&request_id).map(callback))
}

// CONFIRMED ------------------------------------------------------------------------

/// Get Confirmed.
pub fn with_confirmed_requests<T, F>(callback: F) -> T
where
    F: FnOnce(&ConfirmedRequestMap) -> T,
{
    with_link(|state| callback(&state.confirmed))
}

/// Get Confirmed mutably.
pub fn with_confirmed_mut<T, F>(callback: F) -> T
where
    F: FnOnce(&mut ConfirmedRequestMap) -> T,
{
    with_link_mut(|state| callback(&mut state.confirmed))
}

pub fn with_confirmed_request<T, F>(request_id: &RequestId, callback: F) -> Result<T, RequestError>
where
    F: FnOnce(&ConfirmedRequest) -> T,
{
    with_link(|state| state.confirmed(request_id).map(callback))
}

// SIGNERS ----------------------------------------------------------------------

/// Get a signer.
pub fn with_signer<T, F>(signer_id: &SignerId, callback: F) -> Result<T, RequestError>
where
    F: FnOnce(&Signer) -> T,
{
    with_link(|link| link.signer(signer_id).map(callback))
}

/// Check if a signer exists, and optionally check if it has a role.
pub fn with_signer_check<F>(signer_id: SignerId, callback: F) -> Result<(), String>
where
    F: FnOnce(&Signer) -> bool,
{
    with_link(|link| {
        link.signers
            .get(&signer_id)
            .ok_or(RequestError::SignerNotFound(signer_id.to_string()).to_string())
            .map(callback)
            .and_then(|result| {
                if result {
                    Ok(())
                } else {
                    Err(RequestError::SignerNotFound(signer_id.to_string()).to_string())
                }
            })
    })
}

/// Get all link with a role, admins is always included.
pub fn with_signer_ids_by_role<T, F>(role: Roles, callback: F) -> T
where
    F: FnOnce(&Vec<SignerId>) -> T,
{
    with_link(|link| {
        let filtered_signers: Vec<SignerId> = link
            .signers
            .iter()
            .filter(|(_, signer)| signer.has_role(role))
            .map(|(signer_id, _)| signer_id.clone())
            .collect();

        callback(&filtered_signers)
    })
}
