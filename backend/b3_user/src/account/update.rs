use crate::guards::caller_is_owner;
use b3_user_lib::{
    account::Account,
    error::SignerError,
    ledger::identifier::AccountIdentifier,
    ledger::{
        config::Environment,
        network::Network,
        types::{Ecdsa, Memo, NotifyTopUpResult, Tokens, TransferResult},
    },
    signed::SignedTransaction,
    types::{CanisterId, SetAllowance},
    with_account, with_account_mut, with_ledger, with_ledger_mut, with_state, with_state_mut,
};

use ic_cdk::{api::call::CallResult, export::candid::candid_method, update};

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub fn account_request_allowance(
    account_id: String,
    canister: CanisterId,
    allowance: SetAllowance,
) -> CallResult<()> {
    with_account_mut(account_id, |account| {
        account.insert_canister(canister, allowance)
    })?;

    Ok(())
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn create_account(env: Option<Environment>, name: Option<String>) -> CallResult<Account> {
    let subaccount = with_state(|s| s.new_subaccount(env));

    let new_account = Account::new(subaccount);

    let id = with_state_mut(|s| s.insert_account(new_account, name));

    let account = with_account(id, |account| account.clone())?;

    Ok(account)
}
#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub fn rename_account(account_id: String, name: String) -> CallResult<String> {
    let new_name = with_account_mut(account_id, |s| s.update_name(name))?;

    Ok(new_name)
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn account_request_public_key(account_id: String) -> CallResult<Ecdsa> {
    let ledger = with_ledger(account_id.clone(), |ledger| {
        if ledger.public_keys.is_available() {
            Err(SignerError::PublicKeyAlreadyExists)
        } else {
            Ok(ledger.clone())
        }
    })??;

    let ecdsa = ledger.ecdsa_public_key().await?;

    with_ledger_mut(account_id, |ledger| {
        ledger.public_keys.set_ecdsa(ecdsa.clone())
    })??;

    Ok(ecdsa)
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn account_transfer_icp(
    account_id: String,
    amount: Tokens,
    to: String,
    fee: Option<Tokens>,
    memo: Option<Memo>,
) -> CallResult<TransferResult> {
    let to = AccountIdentifier::try_from(to)?;

    let account = with_account(account_id, |account| account.clone())?;

    let result = account.ledger.transfer(amount, to, fee, memo).await?;

    Ok(result)
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn account_topup_and_notify(
    account_id: String,
    amount: Tokens,
    canister_id: Option<CanisterId>,
    fee: Option<Tokens>,
) -> CallResult<NotifyTopUpResult> {
    let ledger = with_ledger(account_id, |ledger| ledger.clone())?;

    let canister_id = canister_id.unwrap_or(ic_cdk::id());

    let result = ledger
        .topup_and_notify_top_up(amount, canister_id, fee)
        .await?;

    Ok(result)
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn generate_account_address(account_id: String, network: Network) -> CallResult<String> {
    let address = with_ledger_mut(account_id, |ledger| {
        ledger.public_keys.generate_address(network)
    })??;

    Ok(address)
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn request_account_sign_message(
    account_id: String,
    message_hash: Vec<u8>,
) -> CallResult<Vec<u8>> {
    let ledger = with_ledger(account_id, |ledger| ledger.clone())?;

    let signature = ledger.sign_with_ecdsa(message_hash).await?;

    Ok(signature)
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn request_account_sign_transaction(
    account_id: String,
    hex_raw_tx: Vec<u8>,
    chain_id: u64,
) -> CallResult<SignedTransaction> {
    let account = with_account(account_id, |account| account.clone())?;

    let signed = account.sign_transaction(hex_raw_tx, chain_id).await?;

    Ok(signed)
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn request_account_balance(account_id: String) -> CallResult<Tokens> {
    let account = with_account(account_id, |account| account.clone())?;

    let balance = account.ledger.account_balance().await?;

    Ok(balance)
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn reset_accounts() {
    with_state_mut(|s| s.reset());
}
