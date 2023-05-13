use crate::guards::caller_is_signer;
use b3_helper::{
    b3_revert,
    types::{
        AccountIdentifier, BlockIndex, CanisterId, Environment, Memo, NotifyTopUpResult,
        SignerAllowanceArgs, Tokens,
    },
};
use b3_signer_lib::{
    account::SignerAccount,
    error::SignerError,
    ledger::{network::Network, types::Ecdsa},
    signed::SignedTransaction,
    state::State,
    store::{
        with_account, with_account_mut, with_ledger, with_ledger_mut, with_state, with_state_mut,
    },
};
use ic_cdk::{export::candid::candid_method, update};

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn create_account(env: Option<Environment>, name: Option<String>) -> SignerAccount {
    let subaccount = with_state(|s| s.new_subaccount(env));

    let new_account: SignerAccount = subaccount.into();

    let id = with_state_mut(|s| s.insert_account(new_account, name));

    with_account(id, |account| account.clone()).unwrap_or_else(|err| b3_revert(err))
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn rename_account(account_id: String, name: String) -> String {
    with_account_mut(account_id, |s| s.update_name(name)).unwrap_or_else(|err| b3_revert(err))
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn hide_account(account_id: String) {
    with_state_mut(|s| s.hide_account(&account_id)).unwrap_or_else(|err| b3_revert(err))
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn remove_account(account_id: String) {
    with_state_mut(|s| s.remove_account(&account_id)).unwrap_or_else(|err| b3_revert(err))
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn request_public_key(account_id: String) -> Ecdsa {
    let ledger = with_ledger(account_id.clone(), |ledger| ledger.clone())
        .unwrap_or_else(|err| b3_revert(err));

    if ledger.public_keys.is_ecdsa_set() {
        b3_revert(SignerError::PublicKeyAlreadyExists);
    }

    let ecdsa = ledger
        .ecdsa_public_key()
        .await
        .unwrap_or_else(|err| b3_revert(err));

    let result = with_ledger_mut(account_id, |ledger| {
        ledger.public_keys.set_ecdsa(ecdsa.clone())
    })
    .unwrap_or_else(|err| b3_revert(err));

    match result {
        Ok(_) => ecdsa,
        Err(err) => b3_revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn send_icp(
    account_id: String,
    to: String,
    amount: Tokens,
    fee: Option<Tokens>,
    memo: Option<Memo>,
) -> BlockIndex {
    let to = AccountIdentifier::try_from(to).unwrap_or_else(|err| b3_revert(err));

    let account =
        with_account(account_id, |account| account.clone()).unwrap_or_else(|err| b3_revert(err));

    let result = account
        .ledger
        .transfer(to, amount, fee, memo)
        .await
        .unwrap_or_else(|err| b3_revert(err));

    match result {
        Ok(result) => result,
        Err(err) => b3_revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn top_up_and_notify(
    account_id: String,
    amount: Tokens,
    canister_id: Option<CanisterId>,
    fee: Option<Tokens>,
) -> u128 {
    let ledger =
        with_ledger(account_id, |ledger| ledger.clone()).unwrap_or_else(|err| b3_revert(err));

    let canister_id = canister_id.unwrap_or(ic_cdk::id());

    let result = ledger
        .topup_and_notify_top_up(canister_id, amount, fee)
        .await
        .unwrap_or_else(|err| b3_revert(err));

    match result {
        NotifyTopUpResult::Ok(result) => result,
        NotifyTopUpResult::Err(err) => b3_revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn generate_address(account_id: String, network: Network) -> String {
    let result = with_ledger_mut(account_id, |ledger| {
        ledger.public_keys.generate_address(network)
    })
    .unwrap_or_else(|err| b3_revert(err));

    match result {
        Ok(result) => result,
        Err(err) => b3_revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn request_sign_message(account_id: String, message_hash: Vec<u8>) -> Vec<u8> {
    let ledger =
        with_ledger(account_id, |ledger| ledger.clone()).unwrap_or_else(|err| b3_revert(err));

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
        with_account(account_id, |account| account.clone()).unwrap_or_else(|err| b3_revert(err));

    account
        .sign_eth_transaction(hex_raw_tx, chain_id)
        .await
        .unwrap_or_else(|err| b3_revert(err))
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn request_balance(account_id: String) -> Tokens {
    let account =
        with_account(account_id, |account| account.clone()).unwrap_or_else(|err| b3_revert(err));

    account
        .ledger
        .account_balance()
        .await
        .unwrap_or_else(|err| b3_revert(err))
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn reset_accounts() -> State {
    with_state_mut(|s| s.reset());

    with_state(|s| s.clone())
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn request_allowance(
    account_id: String,
    canister: CanisterId,
    allowance: SignerAllowanceArgs,
) -> () {
    with_account_mut(account_id, |account| {
        account.insert_canister(canister, allowance)
    })
    .unwrap_or_else(|err| b3_revert(err))
}
