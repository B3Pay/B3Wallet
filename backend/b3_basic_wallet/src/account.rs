use b3_helper_lib::{
    amount::Amount,
    environment::Environment,
    owner::caller_is_owner,
    revert,
    subaccount::Subaccount,
    tokens::Tokens,
    types::{AccountsNonce, BlockIndex, CanisterId, Cycles, NotifyTopUpResult},
};
use b3_wallet_lib::ledger::{
    chain::ChainTrait,
    ckbtc::types::UtxoStatus,
    types::{BtcPending, PendingEnum, SendResult},
};
use b3_wallet_lib::{
    account::WalletAccount,
    ledger::{
        btc::network::BtcNetwork,
        ckbtc::{minter::Minter, types::RetrieveBtcStatus},
        subaccount::SubaccountTrait,
        types::{AddressMap, Balance, ChainEnum},
    },
    store::{
        with_account, with_account_mut, with_chain, with_chain_mut, with_ledger, with_ledger_mut,
        with_wallet, with_wallet_mut,
    },
    types::{AccountId, WalletAccountView},
};
use ic_cdk::{
    api::management_canister::bitcoin::Satoshi, export::candid::candid_method, query, update,
};

// QUERY ---------------------------------------------------------------------

#[candid_method(query)]
#[query(guard = "caller_is_owner")]
pub fn get_account(account_id: AccountId) -> WalletAccount {
    with_account(&account_id, |account| account.clone()).unwrap_or_else(revert)
}

#[candid_method(query)]
#[query(guard = "caller_is_owner")]
pub fn get_account_count() -> usize {
    with_wallet(|s| s.accounts_len())
}

#[candid_method(query)]
#[query(guard = "caller_is_owner")]
pub fn get_account_counters() -> AccountsNonce {
    with_wallet(|s| s.counters().clone())
}

#[candid_method(query)]
#[query(guard = "caller_is_owner")]
pub fn get_account_views() -> Vec<WalletAccountView> {
    with_wallet(|s| s.account_views())
}

#[candid_method(query)]
#[query(guard = "caller_is_owner")]
pub fn get_account_view(account_id: AccountId) -> WalletAccountView {
    with_account(&account_id, |account| account.view()).unwrap_or_else(revert)
}

#[candid_method(query)]
#[query(guard = "caller_is_owner")]
pub fn get_addresses(account_id: AccountId) -> AddressMap {
    with_ledger(&account_id, |ledger| ledger.address_map().clone()).unwrap_or_else(revert)
}

#[candid_method(query)]
#[query(guard = "caller_is_owner")]
pub async fn retrieve_btc_status(
    network: BtcNetwork,
    block_index: BlockIndex,
) -> RetrieveBtcStatus {
    let minter = Minter(network);

    minter
        .retrieve_btc_status(block_index)
        .await
        .unwrap_or_else(revert)
}

// UPDATE ---------------------------------------------------------------------
#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn account_update_balance(account_id: AccountId, network: BtcNetwork) -> Vec<UtxoStatus> {
    let ckbtc = with_chain(&account_id, &ChainEnum::CKBTC(network), |chain| {
        chain.ckbtc()
    })
    .unwrap_or_else(revert)
    .unwrap_or_else(revert);

    let reuslt = ckbtc.update_balance().await.unwrap_or_else(revert);

    match reuslt {
        Ok(utxos) => utxos,
        Err(err) => revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub fn account_create(env: Option<Environment>, name: Option<String>) {
    let subaccount = with_wallet(|s| s.new_subaccount(env));

    let new_account = WalletAccount::from(subaccount);

    with_wallet_mut(|s| s.insert_account(new_account, name));
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub fn account_rename(account_id: AccountId, name: String) {
    with_account_mut(&account_id, |a| a.rename(name)).unwrap_or_else(revert)
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub fn account_hide(account_id: AccountId) {
    with_account_mut(&account_id, |a| a.hide()).unwrap_or_else(revert)
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub fn account_remove(account_id: AccountId) {
    with_wallet_mut(|s| s.remove_account(&account_id)).unwrap_or_else(revert);
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub fn account_remove_address(account_id: AccountId, chain: ChainEnum) {
    with_ledger_mut(&account_id, |ledger| ledger.remove_address(chain))
        .unwrap_or_else(revert)
        .unwrap_or_else(revert);
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub fn account_restore(env: Environment, nonce: u64) {
    let subaccount = Subaccount::new(env, nonce);

    with_wallet_mut(|s| s.restore_account(subaccount)).unwrap_or_else(revert);
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn account_balance(account_id: AccountId, chain: ChainEnum) -> Balance {
    let ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(revert);

    let balance = ledger.balance(chain).await;

    match balance {
        Ok(balance) => balance,
        Err(err) => revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn account_send(
    account_id: AccountId,
    chain: ChainEnum,
    to: String,
    amount: Amount,
) -> SendResult {
    let ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(revert);

    ledger.send(&chain, to, amount).await.unwrap_or_else(revert)
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn account_check_pending(
    account_id: AccountId,
    chain_enum: ChainEnum,
    pending_index: usize,
) {
    let chain = with_chain(&account_id, &chain_enum, |chain| chain.clone()).unwrap_or_else(revert);

    let result = chain.check_pending(pending_index).await;

    match result {
        Ok(_) => {
            with_chain_mut(&account_id, chain_enum, |chain| {
                chain.remove_pending(pending_index)
            })
            .unwrap_or_else(revert);
        }
        Err(err) => revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn account_add_pending(account_id: AccountId, chain: ChainEnum, pending: PendingEnum) {
    with_chain_mut(&account_id, chain, |chain| chain.add_pending(pending)).unwrap_or_else(revert);
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn account_remove_pending(account_id: AccountId, chain: ChainEnum, pending_index: usize) {
    with_chain_mut(&account_id, chain, |chain| {
        chain.remove_pending(pending_index)
    })
    .unwrap_or_else(revert);
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn account_swap_btc_to_ckbtc(
    account_id: AccountId,
    network: BtcNetwork,
    amount: Satoshi,
) -> BtcPending {
    let btc = with_chain(&account_id, &ChainEnum::BTC(network), |chain| chain.btc())
        .unwrap_or_else(revert)
        .unwrap_or_else(revert);

    let result = btc.swap_to_ckbtc(amount).await;

    match result {
        Ok(pending) => {
            with_chain_mut(&account_id, ChainEnum::BTC(network), |chain| {
                chain.add_pending(pending.clone().into())
            })
            .unwrap_or_else(revert);

            pending
        }
        Err(err) => revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn account_swap_ckbtc_to_btc(
    account_id: AccountId,
    network: BtcNetwork,
    retrieve_address: String,
    amount: Satoshi,
) -> BlockIndex {
    let ckbtc = with_chain(&account_id, &ChainEnum::CKBTC(network), |chain| {
        chain.ckbtc()
    })
    .unwrap_or_else(revert)
    .unwrap_or_else(revert);

    let result = ckbtc.swap_to_btc(retrieve_address, amount).await;

    match result {
        Ok(result) => {
            let block_index = result.block_index;

            with_chain_mut(&account_id, ChainEnum::CKBTC(network), |chain| {
                let pending = PendingEnum::new_ckbtc(block_index, None);

                chain.add_pending(pending)
            })
            .unwrap_or_else(revert);

            block_index
        }
        Err(err) => revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn account_top_up_and_notify(
    account_id: AccountId,
    amount: Tokens,
    canister_id: Option<CanisterId>,
) -> Result<Cycles, String> {
    let icp = with_chain(&account_id, &ChainEnum::ICP, |chain| chain.icp())
        .unwrap_or_else(revert)
        .unwrap_or_else(revert);

    let canister_id = canister_id.unwrap_or(ic_cdk::id());

    let block_index = icp.top_up(canister_id, amount).await.unwrap_or_else(revert);

    let notify_result = icp.notify_top_up(canister_id, block_index).await.unwrap();

    match notify_result {
        NotifyTopUpResult::Ok(cycles) => Ok(cycles),
        NotifyTopUpResult::Err(err) => {
            with_chain_mut(&account_id, ChainEnum::ICP, |chain| {
                let pending = PendingEnum::new_icp(block_index, canister_id.to_string());

                chain.add_pending(pending)
            })
            .unwrap();

            Err(err.to_string())
        }
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn account_create_address(account_id: AccountId, chain_enum: ChainEnum) {
    let mut ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(revert);

    let ecdsa = match chain_enum {
        ChainEnum::BTC(_) | ChainEnum::EVM(_) => {
            if !ledger.is_public_key_set() {
                let ecdsa = ledger
                    .subaccount
                    .ecdsa_public_key()
                    .await
                    .unwrap_or_else(revert);

                ledger
                    .set_ecdsa_public_key(ecdsa.clone())
                    .unwrap_or_else(revert);

                Some(ecdsa)
            } else {
                None
            }
        }
        _ => None,
    };

    let chain = ledger
        .new_chain(chain_enum.clone())
        .await
        .unwrap_or_else(revert);

    // Check if the chain is ckbtc and update balance for any pending balance
    if chain_enum.is_ckbtc() {
        // We don't care about the result. If it fails, it will be retried later
        let _ = chain.ckbtc().unwrap().update_balance();
    }

    with_ledger_mut(&account_id, |ledger| {
        if let Some(ecdsa) = ecdsa {
            ledger.set_ecdsa_public_key(ecdsa).unwrap_or_else(revert);
        }

        ledger.insert_chain(chain_enum, chain)
    })
    .unwrap_or_else(revert);
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub async fn account_btc_fees(network: BtcNetwork, num_blocks: u8) -> u64 {
    let rate = network.fee_rate(num_blocks).await;

    match rate {
        Ok(rate) => rate,
        Err(err) => revert(err),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_owner")]
pub fn reset_accounts() {
    with_wallet_mut(|s| s.reset_accounts());
}
