use crate::guard::caller_is_signer;
use b3_helper::revert;
use b3_wallet_lib::{
    request::Request,
    store::{with_confirmed_request, with_request_mut, with_signer_ids_by_role, with_state_mut},
    types::RequestId,
};
use ic_cdk::{export::candid::candid_method, query, update};

// QUERY

#[query]
#[candid_method(query)]
pub fn get_confirmed(request_id: RequestId) -> Request {
    with_confirmed_request(request_id, |confirmed| confirmed.clone()).unwrap_or_else(revert)
}

// UPDATE

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn confirm_request(request_id: RequestId) -> Request {
    let caller = ic_cdk::caller();

    let request = with_request_mut(request_id, |request| {
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
        with_state_mut(|s| s.confirm_request(request_id)).unwrap_or_else(revert);

        request.execute().unwrap_or_else(revert);
    }

    request
}
