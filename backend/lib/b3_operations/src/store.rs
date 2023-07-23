use crate::{
    error::OperationError,
    pending::PendingOperation,
    processed::ProcessedOperation,
    signer::{roles::SignerRoles, Signer},
    state::OperationState,
    types::{ProcessedRequestMap, SignerIds},
};
use b3_utils::types::{OperationId, SignerId};
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

/// Get a signer.
pub fn with_signer<T, F>(signer_id: &SignerId, callback: F) -> Result<T, OperationError>
where
    F: FnOnce(&Signer) -> T,
{
    with_permit(|permit| permit.signer(signer_id).map(callback))
}

/// Check if a signer exists, and optionally check if it has a role.
pub fn with_signer_check<F>(signer_id: SignerId, callback: F) -> Result<(), String>
where
    F: FnOnce(&Signer) -> bool,
{
    with_permit(|permit| {
        permit
            .signers
            .get(&signer_id)
            .ok_or(OperationError::SignerNotFound(signer_id.to_string()).to_string())
            .map(callback)
            .and_then(|result| {
                if result {
                    Ok(())
                } else {
                    Err(OperationError::SignerNotFound(signer_id.to_string()).to_string())
                }
            })
    })
}

/// Get all signer with a role, admins is always included.
pub fn with_signer_ids_by_role<T, F>(role: SignerRoles, callback: F) -> T
where
    F: FnOnce(&SignerIds) -> T,
{
    with_permit(|permit| {
        let filtered_signers: SignerIds = permit
            .signers
            .iter()
            .filter(|(_, signer)| signer.has_role(role))
            .map(|(signer_id, _)| signer_id.clone())
            .collect();

        callback(&filtered_signers)
    })
}
