use account::Account;
use error::SignerError;
use ledger::ledger::Ledger;
use state::State;
use std::cell::RefCell;

pub mod account;
pub mod allowance;
pub mod error;
pub mod evm_tx;
pub mod ledger;
pub mod request;
pub mod signed;
pub mod state;
pub mod types;
pub mod utils;

thread_local! {
    pub static STATE: RefCell<State> = RefCell::default();
}

/// Get all state.
/// This will retrieve all states.
pub fn with_state<T, F>(callback: F) -> T
where
    F: FnOnce(&State) -> T,
{
    STATE.with(|states| {
        let state = states.borrow();

        callback(&state)
    })
}

/// Get all state mutably.
/// This will retrieve all states.
pub fn with_state_mut<T, F>(callback: F) -> T
where
    F: FnOnce(&mut State) -> T,
{
    STATE.with(|states| {
        let mut state = states.borrow_mut();

        callback(&mut state)
    })
}

/// Retrieve an account.
/// This accepts a callback function that will be called with a reference to the account data.
pub fn with_account<T, F>(account_id: String, callback: F) -> Result<T, SignerError>
where
    F: FnOnce(&Account) -> T,
{
    STATE.with(|states| {
        let state = states.borrow();

        state.account(&account_id).map(callback)
    })
}

/// Retrieve an account mutably.
/// This accepts a callback function that will be called with a mutable reference to the account data.
pub fn with_account_mut<T, F>(account_id: String, callback: F) -> Result<T, SignerError>
where
    F: FnOnce(&mut Account) -> T,
{
    STATE.with(|states| {
        let mut state = states.borrow_mut();

        state.account_mut(&account_id).map(callback)
    })
}

pub fn with_ledger<T, F>(account_id: String, callback: F) -> Result<T, SignerError>
where
    F: FnOnce(&Ledger) -> T,
{
    STATE.with(|states| {
        let state = states.borrow();

        state
            .account(&account_id)
            .map(|account| callback(&account.ledger))
    })
}

pub fn with_ledger_mut<T, F>(account_id: String, callback: F) -> Result<T, SignerError>
where
    F: FnOnce(&mut Ledger) -> T,
{
    STATE.with(|states| {
        let mut state = states.borrow_mut();

        state
            .account_mut(&account_id)
            .map(|account| callback(&mut account.ledger))
    })
}
