use crate::guard::caller_is_signer;
use b3_helper::revert;
use b3_wallet_lib::{
    request::Request,
    store::{
        with_account, with_confirmed_request, with_ledger, with_request, with_role_signer_ids,
        with_state, with_state_mut,
    },
    types::{RequestId, RequestMap},
};
use ic_cdk::{export::candid::candid_method, query, update};

// QUERY

#[query]
#[candid_method(query)]
pub fn get_confirmed(request_id: RequestId) -> Request {
    with_confirmed_request(request_id, |confirmed| confirmed.clone()).unwrap_or_else(revert)
}

#[query]
#[candid_method(query)]
pub fn get_requests() -> RequestMap {
    with_state(|s| s.requests())
}

// UPDATE

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn request_confirm(request_id: RequestId) -> Request {
    let caller = ic_cdk::caller();

    let mut request = with_request(request_id, |request| request.clone()).unwrap_or_else(revert);

    request.sign(caller).unwrap_or_else(revert);

    let is_confirmed = with_role_signer_ids(request.role(), |role_signer_ids| {
        role_signer_ids
            .iter()
            .all(|signer_id| request.is_signed(signer_id))
    });

    if is_confirmed {
        with_state_mut(|s| s.confirm_request(request_id)).unwrap_or_else(revert);
    }

    request
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn request_sign_message(account_id: String, message_hash: Vec<u8>) -> Vec<u8> {
    let ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(revert);

    ledger
        .sign_with_ecdsa(message_hash)
        .await
        .unwrap_or_else(revert)
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn request_sign_transaction(
    account_id: String,
    hex_raw_tx: Vec<u8>,
    chain_id: u64,
) -> Vec<u8> {
    let account = with_account(&account_id, |account| account.clone()).unwrap_or_else(revert);

    account
        .sign_eth_transaction(hex_raw_tx, chain_id)
        .await
        .unwrap_or_else(revert)
}
