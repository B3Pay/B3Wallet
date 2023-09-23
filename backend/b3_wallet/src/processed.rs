use crate::permit::caller_is_signer;
use b3_operations::{
    processed::ProcessedOperation,
    response::Response,
    store::{with_pending_operation_mut, with_processed_operation, with_processed_operation_mut},
    types::ProcessedOperations,
};
use b3_utils::{revert, types::OperationId};
use ic_cdk::{query, update};

// QUERY

#[query(guard = "caller_is_signer")]
pub fn get_processed_list() -> ProcessedOperations {
    with_processed_operation(|s| s.processed_list())
}

// UPDATE

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

        if let Err(err) = with_processed_operation_mut(|s| s.add(request_id, processed.clone())) {
            return Err(err.to_string());
        }

        return Ok(processed);
    }

    if request.is_confirmed() {
        let processed = request.execute().await;

        if let Err(err) = with_processed_operation_mut(|s| s.add(request_id, processed.clone())) {
            return Err(err.to_string());
        }

        return Ok(processed);
    }

    Ok(request.into())
}
