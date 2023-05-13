use b3_helper::{b3_revert, types::CanisterId};
use b3_signer_lib::{
    account::SignerAccount,
    ledger::types::Addresses,
    request::EvmSignRequest,
    signed::SignedTransaction,
    store::{with_account, with_ledger, with_state},
    types::CanisterAllowances,
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
pub fn get_sign_requests(account_id: String, canister: CanisterId) -> EvmSignRequest {
    with_account(account_id, |account| {
        account
            .sign_requests(canister)
            .unwrap_or_else(|err| b3_revert(err))
    })
    .unwrap_or_else(|err| b3_revert(err))
}
