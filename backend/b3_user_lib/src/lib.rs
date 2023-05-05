use account::Account;
use error::SignerError;
use state::{State, STATE};

pub mod account;
pub mod allowance;
pub mod error;
pub mod ledger;
pub mod request;
pub mod signed;
pub mod state;
pub mod transaction;
pub mod types;
pub mod utils;

/// Get all state.
/// This will retrieve all states.
pub fn with_state<T, F>(callback: F) -> Result<T, SignerError>
where
    F: FnOnce(&State) -> T,
{
    STATE.with(|states| {
        let state = states.borrow();

        Ok(callback(&state))
    })
}

/// Get all state mutably.
/// This will retrieve all states.
pub fn with_state_mut<T, F>(callback: F) -> Result<T, SignerError>
where
    F: FnOnce(&mut State) -> T,
{
    STATE.with(|states| {
        let mut state = states.borrow_mut();

        Ok(callback(&mut state))
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
