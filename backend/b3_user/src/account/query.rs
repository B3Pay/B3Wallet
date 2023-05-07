use b3_user_lib::{
    account::Account,
    ledger::types::Addresses,
    request::SignRequest,
    signed::SignedTransaction,
    types::{CanisterAllowances, CanisterId},
    with_account, with_ledger, with_state,
};
use ic_cdk::{
    api::call::CallResult,
    export::candid::{candid_method, export_service},
    query,
};

#[query]
#[candid_method(query)]
pub fn get_account(account_id: String) -> CallResult<Account> {
    with_account(account_id, |account| Ok(account.clone()))?
}

#[query]
#[candid_method(query)]
pub fn get_number_of_accounts() -> u8 {
    with_state(|s| s.accounts_len())
}

#[query]
#[candid_method(query)]
pub fn get_accounts() -> Vec<Account> {
    with_state(|s| s.accounts())
}

#[query]
#[candid_method(query)]
pub fn account_addresses(account_id: String) -> CallResult<Addresses> {
    with_ledger(account_id, |ledger| Ok(ledger.public_keys.get_addresses()))?
}

#[query]
#[candid_method(query)]
pub fn account_signed_transaction(account_id: String) -> CallResult<SignedTransaction> {
    let signed = with_account(account_id, |account| account.signed.clone())?;

    Ok(signed)
}

#[query]
#[candid_method(query)]
pub fn account_connected_canisters(account_id: String) -> CallResult<CanisterAllowances> {
    with_account(account_id, |account| Ok(account.canisters.clone()))?
}

#[query]
#[candid_method(query)]
pub fn account_sign_requests(account_id: String, canister: CanisterId) -> CallResult<SignRequest> {
    let request = with_account(account_id, |account| {
        account.requests.get(&canister).unwrap().clone()
    })?;

    Ok(request)
}

#[query(name = "__get_candid_interface_tmp_hack")]
pub fn export_candid() -> String {
    export_service!();
    __export_service()
}
