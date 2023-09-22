use crate::{
    account::WalletAccount,
    error::WalletError,
    ledger::{chain::Chain, ledger::Ledger, types::ChainEnum},
    setting::WalletSettings,
    state::WalletState,
    types::AccountId,
};
use b3_utils::memory::{init_stable_mem_cell, types::DefaultVMCell};
use std::cell::RefCell;

thread_local! {
    static STATE: RefCell<DefaultVMCell<WalletState>> = init_stable_mem_cell("wallet", 10).unwrap();
}

// STATE ----------------------------------------------------------------------

/// Get all state.
/// This will retrieve all states.
pub fn with_wallet<T, F>(callback: F) -> T
where
    F: FnOnce(&WalletState) -> T,
{
    STATE.with(|states| {
        let state = states.borrow();

        callback(state.get())
    })
}

/// Get all state mutably.
/// This will retrieve all states.
pub fn with_wallet_mut<T, F>(callback: F) -> T
where
    F: FnOnce(&mut WalletState) -> T,
{
    STATE.with(|states| {
        let mut state = states.borrow_mut();

        callback(state.get_mut())
    })
}

// SETTINGS ----------------------------------------------------------------------

/// Retrieve Setting.
/// This accepts a callback function that will be called with a reference to the setting data.
pub fn with_setting<T, F>(callback: F) -> T
where
    F: FnOnce(&WalletSettings) -> T,
{
    with_wallet(|states| callback(&states.settings))
}

/// Retrieve Setting mutably.
/// This accepts a callback function that will be called with a mutable reference to the setting data.
pub fn with_setting_mut<T, F>(callback: F) -> T
where
    F: FnOnce(&mut WalletSettings) -> T,
{
    with_wallet_mut(|states| callback(&mut states.settings))
}

// ACCOUNTS ----------------------------------------------------------------------

/// Retrieve an account.
/// This accepts a callback function that will be called with a reference to the account data.
pub fn with_account<T, F>(account_id: &AccountId, callback: F) -> Result<T, WalletError>
where
    F: FnOnce(&WalletAccount) -> T,
{
    with_wallet(|state| state.account(account_id).map(callback))
}

/// Retrieve an account mutably.
/// This accepts a callback function that will be called with a mutable reference to the account data.
pub fn with_account_mut<T, F>(account_id: &AccountId, callback: F) -> Result<T, WalletError>
where
    F: FnOnce(&mut WalletAccount) -> T,
{
    with_wallet_mut(|state| state.account_mut(account_id).map(callback))
}

// LEDGER ----------------------------------------------------------------------
/// Retrieve a ledger.
/// This accepts a callback function that will be called with a reference to the ledger data.
/// This will retrieve the ledger from the account.
pub fn with_ledger<T, F>(account_id: &AccountId, callback: F) -> Result<T, WalletError>
where
    F: FnOnce(&Ledger) -> T,
{
    with_wallet(|state| {
        state
            .account(account_id)
            .map(|account| callback(&account.ledger()))
    })
}

/// Retrieve a ledger mutably.
/// This accepts a callback function that will be called with a mutable reference to the ledger data.
/// This will retrieve the ledger from the account.
pub fn with_ledger_mut<T, F>(account_id: &AccountId, callback: F) -> Result<T, WalletError>
where
    F: FnOnce(&mut Ledger) -> T,
{
    with_wallet_mut(|state| {
        state
            .account_mut(account_id)
            .map(|account| callback(&mut account.ledger_mut()))
    })
}

// CHAINS ----------------------------------------------------------------------
/// Retrieve a chain.
/// This accepts a callback function that will be called with a reference to the chain data.
/// This will retrieve the chain from the account's ledger.
pub fn with_chain<T, F>(
    account_id: &AccountId,
    chain_type: &ChainEnum,
    callback: F,
) -> Result<T, WalletError>
where
    F: FnOnce(&Chain) -> T,
{
    with_ledger(account_id, |ledger| {
        ledger
            .chain(chain_type)
            .map(|chain| callback(chain))
            .map_err(WalletError::LedgerError)
    })?
}

/// Retrieve a chain mutably.
/// This accepts a callback function that will be called with a mutable reference to the chain data.
/// This will retrieve the chain from the account's ledger.
pub fn with_chain_mut<T, F>(
    account_id: &AccountId,
    chain_type: ChainEnum,
    callback: F,
) -> Result<T, WalletError>
where
    F: FnOnce(&mut Chain) -> T,
{
    with_ledger_mut(account_id, |ledger| {
        ledger
            .chain_mut(chain_type)
            .map(|chain| callback(chain))
            .map_err(WalletError::LedgerError)
    })?
}
