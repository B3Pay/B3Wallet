use crate::permit::{caller_is_admin, caller_is_signer};
use b3_operations::{
    error::OperationError,
    processed::processed::ProcessedRequest,
    store::{with_pending_mut, with_permit, with_permit_mut, with_processed_request},
    types::{ProcessedRequestList, Response},
};
use b3_utils::{revert, types::RequestId};
use candid::candid_method;
use ic_cdk::{query, update};

// QUERY

#[candid_method(query)]
#[query(guard = "caller_is_signer")]
pub fn get_processed(request_id: RequestId) -> ProcessedRequest {
    with_processed_request(&request_id, |processed| processed.clone()).unwrap_or_else(revert)
}

#[candid_method(query)]
#[query(guard = "caller_is_signer")]
pub fn get_processed_list() -> ProcessedRequestList {
    with_permit(|s| s.processed_list())
}

// UPDATE

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn response(request_id: RequestId, answer: Response) -> Result<ProcessedRequest, String> {
    let caller = ic_cdk::caller();

    let request = with_pending_mut(&request_id, |request| {
        if request.is_expired() {
            return request.clone();
        }

        request.response(caller, answer).unwrap_or_else(revert);

        request.clone()
    })
    .unwrap_or_else(revert);

    if request.is_failed() {
        let processed = ProcessedRequest::from(request);

        if let Err(err) = with_permit_mut(|s| s.insert_processed(request_id, processed.clone())) {
            return Err(err.to_string());
        }

        return Ok(processed);
    }

    if request.is_confirmed() {
        let processed = request.execute().await;

        if let Err(err) = with_permit_mut(|s| s.insert_processed(request_id, processed.clone())) {
            return Err(err.to_string());
        }

        return Ok(processed);
    }

    Ok(request.into())
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn process_request(request_id: RequestId) {
    let caller = ic_cdk::caller();

    with_permit_mut(|s| {
        let mut processed: ProcessedRequest =
            s.request(&request_id).unwrap_or_else(revert).clone().into();

        processed.fail(OperationError::RequestRemovedByAdmin(caller.to_string()));

        s.insert_processed(request_id, processed)
            .unwrap_or_else(revert);
    });
}
