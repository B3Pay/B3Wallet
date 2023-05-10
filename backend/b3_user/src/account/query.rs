use b3_user_lib::{
    account::Account,
    ledger::types::Addresses,
    request::SignRequest,
    signed::SignedTransaction,
    store::{with_account, with_ledger, with_state},
    types::{CanisterAllowances, CanisterId},
};
use ic_cdk::{export::candid::candid_method, query};

use crate::guards::ic_trap;

#[query]
#[candid_method(query)]
pub fn get_account(account_id: String) -> Account {
    with_account(account_id, |account| account.clone()).unwrap_or_else(|err| ic_trap(err))
}

#[query]
#[candid_method(query)]
pub fn get_account_count() -> usize {
    with_state(|s| s.accounts_len())
}

#[query]
#[candid_method(query)]
pub fn get_accounts() -> Vec<Account> {
    with_state(|s| s.accounts())
}

#[query]
#[candid_method(query)]
pub fn get_addresses(account_id: String) -> Addresses {
    with_ledger(account_id, |ledger| ledger.public_keys.get_addresses())
        .unwrap_or_else(|err| ic_trap(err))
}

#[query]
#[candid_method(query)]
pub fn get_signed_transaction(account_id: String) -> SignedTransaction {
    with_account(account_id, |account| account.signed.clone()).unwrap_or_else(|err| ic_trap(err))
}

#[query]
#[candid_method(query)]
pub fn get_connected_canisters(account_id: String) -> CanisterAllowances {
    with_account(account_id, |account| account.canisters.clone()).unwrap_or_else(|err| ic_trap(err))
}

#[query]
#[candid_method(query)]
pub fn get_sign_requests(account_id: String, canister: CanisterId) -> SignRequest {
    with_account(account_id, |account| {
        account
            .sign_requests(canister)
            .unwrap_or_else(|err| ic_trap(err))
    })
    .unwrap_or_else(|err| ic_trap(err))
}
