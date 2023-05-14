use crate::{
    account::WalletAccount,
    error::SignerError,
    ledger::ledger::Ledger,
    signed::SignedTransaction,
    signer::{Roles, Signer},
    state::State,
    types::{RequestId, SignerMap},
};
use b3_helper::{
    error::TrapError,
    types::{SignerId, Wasm},
};
use std::cell::RefCell;

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
    static WASM: RefCell<Wasm> = RefCell::new(Wasm::default());
    static SIGNER: RefCell<SignerMap> = RefCell::new(SignerMap::new());
}

// STATE ----------------------------------------------------------------------

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

// SIGNED ------------------------------------------------------------------------

/// Get all signed.
pub fn with_confirmed<T, F>(request_id: RequestId, callback: F) -> Result<T, SignerError>
where
    F: FnOnce(&SignedTransaction) -> T,
{
    with_state(|state| state.confirmed(request_id).map(callback))
}

/// Get all signed mutably.
pub fn with_confirmed_mut<T, F>(request_id: RequestId, callback: F) -> Result<T, SignerError>
where
    F: FnOnce(&mut SignedTransaction) -> T,
{
    with_state_mut(|state| state.confirmed_mut(request_id).map(callback))
}

// ACCOUNTS ----------------------------------------------------------------------

/// Retrieve an account.
/// This accepts a callback function that will be called with a reference to the account data.
pub fn with_account<T, F>(account_id: &String, callback: F) -> Result<T, SignerError>
where
    F: FnOnce(&WalletAccount) -> T,
{
    with_state(|state| state.account(account_id).map(callback))
}

/// Retrieve an account mutably.
/// This accepts a callback function that will be called with a mutable reference to the account data.
pub fn with_account_mut<T, F>(account_id: &String, callback: F) -> Result<T, SignerError>
where
    F: FnOnce(&mut WalletAccount) -> T,
{
    with_state_mut(|state| state.account_mut(account_id).map(callback))
}

pub fn with_ledger<T, F>(account_id: &String, callback: F) -> Result<T, SignerError>
where
    F: FnOnce(&Ledger) -> T,
{
    with_state(|state| {
        state
            .account(account_id)
            .map(|account| callback(&account.ledger))
    })
}

pub fn with_ledger_mut<T, F>(account_id: &String, callback: F) -> Result<T, SignerError>
where
    F: FnOnce(&mut Ledger) -> T,
{
    with_state_mut(|state| {
        state
            .account_mut(account_id)
            .map(|account| callback(&mut account.ledger))
    })
}

// SIGNERS ----------------------------------------------------------------------

/// Get Signers.
pub fn with_signers<T>(f: impl FnOnce(&SignerMap) -> T) -> T {
    SIGNER.with(|signers| f(&signers.borrow()))
}

/// Get Signers mutably.
pub fn with_signers_mut<T>(f: impl FnOnce(&mut SignerMap) -> T) -> T {
    SIGNER.with(|signers| f(&mut signers.borrow_mut()))
}

/// Get a signer.
pub fn with_signer<T, F>(signer_id: SignerId, callback: F) -> Result<T, SignerError>
where
    F: FnOnce(&Signer) -> T,
{
    with_signers(|signers| {
        signers
            .get(&signer_id)
            .ok_or(SignerError::SignerNotFound(signer_id.to_string()))
            .map(callback)
    })
}

/// Check if a signer exists, and optionally check if it has a role.
pub fn check_signer(signer_id: SignerId, opt_role: Option<Roles>) -> Result<(), String> {
    with_signer(signer_id, |signer| {
        if let Some(role) = opt_role {
            if !signer.has_role(role.clone()) {
                return Err(SignerError::SignerRoleNotFound(
                    signer_id.to_string(),
                    role.to_string(),
                )
                .to_string());
            }
        }

        Ok(())
    })
    .map_err(|err| err.to_string())?
}

// WASM ----------------------------------------------------------------------

/// Get wasm.
pub fn with_wasm<T, F>(callback: F) -> T
where
    F: FnOnce(&Wasm) -> T,
{
    WASM.with(|wasm| {
        let wasm = wasm.borrow();

        callback(&wasm)
    })
}

/// Get wasm mutably.
pub fn with_wasm_mut<T, F>(callback: F) -> T
where
    F: FnOnce(&mut Wasm) -> T,
{
    WASM.with(|wasm| {
        let mut wasm = wasm.borrow_mut();

        callback(&mut wasm)
    })
}
