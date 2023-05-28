use crate::permit::{caller_is_admin, caller_is_signer};
use b3_helper_lib::{
    revert,
    types::{
        AccountIdentifier, AccountsCounter, BlockIndex, CanisterId, Environment, Memo,
        NotifyTopUpResult, Subaccount, Tokens,
    },
};
use b3_wallet_lib::{
    account::WalletAccount,
    error::WalletError,
    ledger::{btc::network::BtcNetwork, chains::Chains, types::AddressMap},
    store::{
        with_account, with_account_mut, with_ledger, with_ledger_mut, with_wallet, with_wallet_mut,
    },
    types::WalletAccountView,
};
use ic_cdk::{
    api::management_canister::bitcoin::{GetUtxosResponse, Satoshi, UtxoFilter},
    export::candid::candid_method,
    query, update,
};

// QUERY

#[query]
#[candid_method(query)]
pub fn get_account(account_id: String) -> WalletAccount {
    with_account(&account_id, |account| account.clone()).unwrap_or_else(revert)
}

#[query]
#[candid_method(query)]
pub fn get_account_count() -> usize {
    with_wallet(|s| s.accounts_len())
}

#[query]
#[candid_method(query)]
pub fn get_account_counters() -> AccountsCounter {
    with_wallet(|s| s.counters().clone())
}

#[query]
#[candid_method(query)]
pub fn get_accounts() -> Vec<WalletAccount> {
    with_wallet(|s| s.accounts())
}

#[query]
#[candid_method(query)]
pub fn get_account_views() -> Vec<WalletAccountView> {
    with_wallet(|s| s.account_views())
}

#[query]
#[candid_method(query)]
pub fn get_account_view(account_id: String) -> WalletAccountView {
    with_account(&account_id, |account| account.view()).unwrap_or_else(revert)
}

#[query]
#[candid_method(query)]
pub fn get_addresses(account_id: String) -> AddressMap {
    with_ledger(&account_id, |ledger| ledger.keys.addresses().clone()).unwrap_or_else(revert)
}

// UPDATE

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn account_create(env: Option<Environment>, name: Option<String>) {
    let subaccount = with_wallet(|s| s.new_subaccount(env));

    let new_account = WalletAccount::from(subaccount);

    with_wallet_mut(|s| s.insert_account(new_account, name));
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn account_rename(account_id: String, name: String) {
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
    with_wallet_mut(|s| s.remove_account(&account_id)).unwrap_or_else(revert);
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn account_remove_address(account_id: String, chains: Chains) {
    with_ledger_mut(&account_id, |ledger| ledger.keys.remove_address(chains))
        .unwrap_or_else(revert)
        .unwrap_or_else(revert);
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn account_restore(env: Environment, index: u64) {
    let subaccount = Subaccount::new(env, index);

    with_wallet_mut(|s| s.restore_account(subaccount)).unwrap_or_else(revert);
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn account_request_public_key(account_id: String) {
    let ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(revert);

    if ledger.keys.is_ecdsa_set() {
        revert(WalletError::EcdsaPublicKeyAlreadySet)
    }

    let ecdsa = ledger.ecdsa_public_key().await.unwrap_or_else(revert);

    with_ledger_mut(&account_id, |ledger| ledger.set_ecdsa_public_key(ecdsa))
        .unwrap_or_else(revert)
        .unwrap_or_else(revert);
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn account_icp_balance(account_id: String, owner: Option<CanisterId>) -> Tokens {
    let ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(revert);

    let tokens = ledger.account_balance(owner).await;

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

    let ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(revert);

    let result = ledger
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
pub async fn account_btc_utxos(
    account_id: String,
    network: BtcNetwork,
    filter: Option<UtxoFilter>,
) -> GetUtxosResponse {
    let ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(revert);

    let utxos = ledger.bitcoin_get_utxos(network.into(), filter).await;

    match utxos {
        Ok(utxos) => utxos,
        Err(err) => revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn account_btc_fees(network: BtcNetwork, num_blocks: u8) -> u64 {
    let rate = network.fee_rate(num_blocks).await;

    match rate {
        Ok(rate) => rate,
        Err(err) => revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn account_balance_btc(
    account_id: String,
    network: BtcNetwork,
    min_confirmations: Option<u32>,
) -> Satoshi {
    let ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(revert);

    let balance = ledger
        .bitcoin_balance(network.into(), min_confirmations)
        .await;

    match balance {
        Ok(balance) => balance,
        Err(err) => revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn account_send_btc(
    account_id: String,
    network: BtcNetwork,
    to: String,
    amount: Satoshi,
) -> String {
    let ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(revert);

    let result = ledger.bitcoin_transfer(network, &to, amount).await;

    match result {
        Ok(result) => result.to_string(),
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
pub async fn account_generate_address(account_id: String, network: Chains) {
    with_ledger_mut(&account_id, |ledger| ledger.keys.generate_address(network))
        .unwrap_or_else(revert)
        .unwrap_or_else(revert);
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn reset_wallet() {
    with_wallet_mut(|s| s.reset());
}
