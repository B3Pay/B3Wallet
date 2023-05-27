use crate::permit::caller_is_signer;
use b3_helper_lib::{revert, types::RequestId};
use b3_permit_lib::{
    confirmed::ConfirmedRequest,
    store::{
        with_confirmed_request, with_pending_mut, with_permit, with_permit_mut,
        with_signer_ids_by_role,
    },
    types::ConfirmedRequestMap,
};
use ic_cdk::{export::candid::candid_method, query, update};

// QUERY

#[query]
#[candid_method(query)]
pub fn get_confirmed(request_id: RequestId) -> ConfirmedRequest {
    with_confirmed_request(&request_id, |confirmed| confirmed.clone()).unwrap_or_else(revert)
}

#[query]
#[candid_method(query)]
pub fn get_confirmed_requests() -> ConfirmedRequestMap {
    with_permit(|s| s.confirmed.clone())
}

// UPDATE

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn confirm_request(request_id: RequestId) -> ConfirmedRequest {
    let caller = ic_cdk::caller();

    let request = with_pending_mut(&request_id, |request| {
        request.sign(caller).unwrap_or_else(revert);
        request.clone()
    })
    .unwrap_or_else(revert);

    let is_confirmed = with_signer_ids_by_role(request.role(), |signer_ids| {
        signer_ids
            .iter()
            .all(|signer_id| request.is_signed(signer_id))
    });

    if is_confirmed {
        let confirmed = request.execute().await;

        with_permit_mut(|s| s.insert_confirmed(request_id, confirmed.clone()))
            .unwrap_or_else(revert);

        return confirmed;
    }

    request.into()
}
