use crate::signer::caller_is_signer;
use b3_helper::{
    revert,
    types::{
        AccountIdentifier, BlockIndex, CanisterId, Environment, Memo, NotifyTopUpResult,
        Subaccount, Tokens,
    },
};
use b3_wallet_lib::{
    account::WalletAccount,
    counter::WalletCounters,
    error::WalletError,
    ledger::{network::Network, types::AddressMap},
    store::{
        with_account, with_account_mut, with_ledger, with_ledger_mut, with_state, with_state_mut,
    },
};
use ic_cdk::{export::candid::candid_method, query, update};

// QUERY

#[query]
#[candid_method(query)]
pub fn get_account(account_id: String) -> WalletAccount {
    with_account(&account_id, |account| account.clone()).unwrap_or_else(revert)
}

#[query]
#[candid_method(query)]
pub fn get_account_count() -> usize {
    with_state(|s| s.accounts_len())
}

#[query]
#[candid_method(query)]
pub fn get_account_counters() -> WalletCounters {
    with_state(|s| s.counters())
}

#[query]
#[candid_method(query)]
pub fn get_accounts() -> Vec<WalletAccount> {
    with_state(|s| s.accounts())
}

#[query]
#[candid_method(query)]
pub fn get_addresses(account_id: String) -> AddressMap {
    with_ledger(&account_id, |ledger| ledger.keys.addresses()).unwrap_or_else(revert)
}

// UPDATE

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn account_create(env: Option<Environment>, name: Option<String>) -> WalletAccount {
    let subaccount = with_state(|s| s.new_subaccount(env));

    let new_account = WalletAccount::from(subaccount);

    let id = with_state_mut(|s| s.insert_account(new_account, name));

    with_account(&id, |account| account.clone()).unwrap_or_else(revert)
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn account_rename(account_id: String, name: String) -> String {
    with_account_mut(&account_id, |a| a.rename(name)).unwrap_or_else(revert)
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn account_hide(account_id: String) {
    with_account_mut(&account_id, |a| a.hide()).unwrap_or_else(revert)
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn account_remove(account_id: String) {
    with_state_mut(|s| s.remove_account(&account_id)).unwrap_or_else(revert)
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn account_restore(env: Environment, index: u64) -> WalletAccount {
    let subaccount = Subaccount::new(env, index);

    with_state_mut(|s| s.restore_account(subaccount)).unwrap_or_else(revert)
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn account_request_public_key(account_id: String) -> AddressMap {
    let ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(revert);

    if ledger.keys.is_ecdsa_set() {
        revert(WalletError::EcdsaPublicKeyAlreadySet)
    }

    let ecdsa = ledger.ecdsa_public_key().await.unwrap_or_else(revert);

    let result = with_ledger_mut(&account_id, |ledger| ledger.keys.set_ecdsa(ecdsa.clone()))
        .unwrap_or_else(revert);

    match result {
        Ok(addresses) => addresses,
        Err(err) => revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn account_icp_balance(account_id: String, owner: Option<CanisterId>) -> Tokens {
    let account = with_account(&account_id, |account| account.clone()).unwrap_or_else(revert);

    let tokens = account.ledger.account_balance(owner).await;

    match tokens {
        Ok(tokens) => tokens,
        Err(err) => revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn account_send_icp(
    account_id: String,
    to: String,
    amount: Tokens,
    fee: Option<Tokens>,
    memo: Option<Memo>,
) -> BlockIndex {
    let to = AccountIdentifier::try_from(to).unwrap_or_else(revert);

    let account = with_account(&account_id, |account| account.clone()).unwrap_or_else(revert);

    let result = account
        .ledger
        .transfer(to, amount, fee, memo)
        .await
        .unwrap_or_else(revert);

    match result {
        Ok(result) => result,
        Err(err) => revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn account_top_up_and_notify(
    account_id: String,
    amount: Tokens,
    canister_id: Option<CanisterId>,
    fee: Option<Tokens>,
) -> u128 {
    let ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(revert);

    let canister_id = canister_id.unwrap_or(ic_cdk::id());

    let result = ledger
        .topup_and_notify_top_up(canister_id, amount, fee)
        .await
        .unwrap_or_else(revert);

    match result {
        NotifyTopUpResult::Ok(result) => result,
        NotifyTopUpResult::Err(err) => revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn account_generate_address(account_id: String, network: Network) -> String {
    let result = with_ledger_mut(&account_id, |ledger| ledger.keys.generate_address(network))
        .unwrap_or_else(revert);

    match result {
        Ok(result) => result,
        Err(err) => revert(err),
    }
}
