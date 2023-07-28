use crate::permit::{caller_is_admin, caller_is_signer};
use b3_operations::{
    error::OperationError,
    processed::ProcessedOperation,
    response::Response,
    store::{with_operation, with_operation_mut, with_pending_operation_mut},
    types::ProcessedOperations,
};
use b3_utils::{revert, types::OperationId};
use candid::candid_method;
use ic_cdk::{query, update};

// QUERY

#[candid_method(query)]
#[query(guard = "caller_is_signer")]
pub fn get_processed_list() -> ProcessedOperations {
    with_operation(|s| s.processed_list())
}

// UPDATE

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn response(
    request_id: OperationId,
    answer: Response,
) -> Result<ProcessedOperation, String> {
    let caller = ic_cdk::caller();

    let request = with_pending_operation_mut(&request_id, |request| {
        if request.is_expired() {
            return request.clone();
        }

        request.response(caller, answer).unwrap_or_else(revert);

        request.clone()
    })
    .unwrap_or_else(revert);

    if request.is_failed() {
        let processed = ProcessedOperation::from(request);

        if let Err(err) = with_operation_mut(|s| s.insert_processed(request_id, processed.clone()))
        {
            return Err(err.to_string());
        }

        return Ok(processed);
    }

    if request.is_confirmed() {
        let processed = request.execute().await;

        if let Err(err) = with_operation_mut(|s| s.insert_processed(request_id, processed.clone()))
        {
            return Err(err.to_string());
        }

        return Ok(processed);
    }

    Ok(request.into())
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn process_request(request_id: OperationId) {
    let caller = ic_cdk::caller();

    with_operation_mut(|s| {
        let mut processed: ProcessedOperation = s
            .processed(&request_id)
            .unwrap_or_else(revert)
            .clone()
            .into();

        processed.fail(OperationError::RequestRemovedByAdmin(caller.to_string()));

        s.insert_processed(request_id, processed)
            .unwrap_or_else(revert);
    });
}
