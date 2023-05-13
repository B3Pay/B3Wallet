use b3_helper::b3_revert;
use b3_signer_lib::{
    account::SignerAccount,
    ledger::types::Addresses,
    request::sign::SignRequest,
    signed::SignedTransaction,
    store::{with_account, with_ledger, with_state},
    types::{CanisterAllowances, RequestId},
};
use ic_cdk::{export::candid::candid_method, query};

#[query]
#[candid_method(query)]
pub fn get_account(account_id: String) -> SignerAccount {
    with_account(account_id, |account| account.clone()).unwrap_or_else(|err| b3_revert(err))
}

#[query]
#[candid_method(query)]
pub fn get_account_count() -> usize {
    with_state(|s| s.accounts_len())
}

#[query]
#[candid_method(query)]
pub fn get_accounts() -> Vec<SignerAccount> {
    with_state(|s| s.accounts())
}

#[query]
#[candid_method(query)]
pub fn get_addresses(account_id: String) -> Addresses {
    with_ledger(account_id, |ledger| ledger.public_keys.addresses())
        .unwrap_or_else(|err| b3_revert(err))
}

#[query]
#[candid_method(query)]
pub fn get_signed_transaction(account_id: String) -> SignedTransaction {
    with_account(account_id, |account| account.signed.clone()).unwrap_or_else(|err| b3_revert(err))
}

#[query]
#[candid_method(query)]
pub fn get_connected_canisters(account_id: String) -> CanisterAllowances {
    with_account(account_id, |account| account.canisters.clone())
        .unwrap_or_else(|err| b3_revert(err))
}

#[query]
#[candid_method(query)]
pub fn get_sign_requests(account_id: String, request_id: RequestId) -> SignRequest {
    with_account(account_id, |account| {
        account
            .sign_request(request_id)
            .unwrap_or_else(|err| b3_revert(err))
    })
    .unwrap_or_else(|err| b3_revert(err))
}
