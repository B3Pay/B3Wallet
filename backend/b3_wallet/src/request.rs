use crate::guard::caller_is_signer;
use b3_helper::revert;
use b3_wallet_lib::{
    request::{inter::RenameAccountRequest, sign::SignRequest, RequestArgs},
    signer::Roles,
    store::{with_account, with_ledger, with_state, with_state_mut},
    types::{PendingRequestMap, RequestId},
};
use ic_cdk::{export::candid::candid_method, query, update};

// QUERY

#[query]
#[candid_method(query)]
pub fn get_requests() -> PendingRequestMap {
    with_state(|s| s.requests())
}

// UPDATE

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn request_account_rename(account_id: String, name: String) -> RequestId {
    let rename_request: SignRequest = RenameAccountRequest::new(account_id, name).into();

    let request_args = RequestArgs::new(Roles::Admin, rename_request);

    with_state_mut(|s| {
        let new_request = s.new_request(request_args, None);
        s.insert_request(new_request)
    })
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
