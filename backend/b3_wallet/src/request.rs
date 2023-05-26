use crate::signer::{caller_is_admin, caller_is_signer};
use b3_helper_lib::{
    revert,
    types::{Deadline, RequestId},
};
use b3_permit_lib::{
    pending::{
        inner::{account::RenameAccountRequest, setting::UpdateCanisterSettingsRequest},
        Request, RequestArgs,
    },
    signer::Roles,
    store::{with_permit, with_permit_mut},
    types::PendingRequestList,
};
use b3_wallet_lib::store::{with_account, with_ledger};
use ic_cdk::{export::candid::candid_method, query, update};

// QUERY ---------------------------------------------------------------------

#[query]
#[candid_method(query)]
pub fn get_requests() -> PendingRequestList {
    with_permit(|s| s.requests())
}

// UPDATE ---------------------------------------------------------------------
#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_maker(request: Request, deadline: Option<Deadline>) -> RequestId {
    let request_args = RequestArgs::new(Roles::Admin, request.into(), deadline);

    with_permit_mut(|s| {
        let new_request = s.new_request(request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_update_settings(
    request: UpdateCanisterSettingsRequest,
    deadline: Option<Deadline>,
) -> RequestId {
    request.validate_request().unwrap_or_else(revert);

    let request_args = RequestArgs::new(Roles::Admin, request.into(), deadline);

    with_permit_mut(|s| {
        let new_request = s.new_request(request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_account_rename(
    request: RenameAccountRequest,
    deadline: Option<Deadline>,
) -> RequestId {
    let request_args = RequestArgs::new(Roles::Admin, request.into(), deadline);

    with_permit_mut(|s| {
        let new_request = s.new_request(request_args);
        s.insert_new_request(new_request)
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
        .sign_evm_transaction(hex_raw_tx, chain_id)
        .await
        .unwrap_or_else(revert)
}
