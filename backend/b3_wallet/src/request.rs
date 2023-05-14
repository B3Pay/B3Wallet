use crate::guard::caller_is_signer;
use b3_helper::b3_revert;
use b3_wallet_lib::{
    request::sign::SignRequest,
    signed::SignedTransaction,
    state::State,
    store::{with_account, with_confirmed, with_ledger, with_state, with_state_mut},
    types::RequestId,
};
use ic_cdk::{export::candid::candid_method, query, update};

#[query]
#[candid_method(query)]
pub fn get_signed_transaction(request_id: RequestId) -> SignedTransaction {
    with_confirmed(request_id, |tx| tx.clone()).unwrap_or_else(|err| b3_revert(err))
}

#[query]
#[candid_method(query)]
pub fn get_sign_requests(account_id: String, request_id: RequestId) -> SignRequest {
    todo!()
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn request_sign_message(account_id: String, message_hash: Vec<u8>) -> Vec<u8> {
    let ledger =
        with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(|err| b3_revert(err));

    ledger
        .sign_with_ecdsa(message_hash)
        .await
        .unwrap_or_else(|err| b3_revert(err))
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn request_sign_transaction(
    account_id: String,
    hex_raw_tx: Vec<u8>,
    chain_id: u64,
) -> SignedTransaction {
    let account =
        with_account(&account_id, |account| account.clone()).unwrap_or_else(|err| b3_revert(err));

    account
        .sign_eth_transaction(hex_raw_tx, chain_id)
        .await
        .unwrap_or_else(|err| b3_revert(err))
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn reset_accounts() -> State {
    with_state_mut(|s| s.reset());

    with_state(|s| s.clone())
}
