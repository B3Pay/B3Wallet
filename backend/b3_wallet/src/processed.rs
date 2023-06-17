use crate::permit::caller_is_signer;
use b3_helper_lib::{revert, types::RequestId};
use b3_permit_lib::{
    processed::processed::ProcessedRequest,
    store::{with_pending_mut, with_permit, with_permit_mut, with_processed_request},
    types::{ProcessedRequestList, Response},
};
use ic_cdk::{export::candid::candid_method, query, update};

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
pub async fn response(request_id: RequestId, answer: Response) -> ProcessedRequest {
    let caller = ic_cdk::caller();

    let request = with_pending_mut(&request_id, |request| {
        request.response(caller, answer).unwrap_or_else(revert);
        request.clone()
    })
    .unwrap_or_else(revert);

    if request.is_rejected() || request.is_expired() {
        let processed = ProcessedRequest::from(request);

        with_permit_mut(|s| s.insert_processed(request_id, processed.clone()))
            .unwrap_or_else(revert);

        return processed;
    }

    if request.is_confirmed() {
        let processed = request.execute().await;

        with_permit_mut(|s| s.insert_processed(request_id, processed.clone()))
            .unwrap_or_else(revert);

        return processed;
    }

    request.into()
}
