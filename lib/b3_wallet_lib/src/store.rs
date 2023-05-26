use crate::{
    account::WalletAccount, error::WalletError, ledger::Ledger, state::WalletState,
    types::AccountId,
};
use std::cell::RefCell;

thread_local! {
    static STATE: RefCell<WalletState> = RefCell::default();
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

        callback(&state)
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

        callback(&mut state)
    })
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
