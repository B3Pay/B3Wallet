use crate::{
    account::WalletAccount,
    error::WalletError,
    ledger::ledger::Ledger,
    request::Request,
    signer::{Roles, Signer},
    state::State,
    types::{ConfirmedRequests, RequestId, SignerMap},
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

// REQUEST ------------------------------------------------------------------------

/// Get Request.
pub fn with_request<T, F>(request_id: RequestId, callback: F) -> Result<T, WalletError>
where
    F: FnOnce(&Request) -> T,
{
    with_state(|state| state.request(request_id).map(callback))
}

/// Get Request mutably.
pub fn with_request_mut<T, F>(request_id: RequestId, callback: F) -> Result<T, WalletError>
where
    F: FnOnce(&mut Request) -> T,
{
    with_state_mut(|state| state.request_mut(request_id).map(callback))
}

// CONFIRMED ------------------------------------------------------------------------

/// Get Confirmed.
pub fn with_confirmed_requests<T, F>(callback: F) -> T
where
    F: FnOnce(&ConfirmedRequests) -> T,
{
    with_state(|state| callback(state.confirmed_requests()))
}

/// Get Confirmed mutably.
pub fn with_confirmed_requests_mut<T, F>(callback: F) -> T
where
    F: FnOnce(&mut ConfirmedRequests) -> T,
{
    with_state_mut(|state| callback(state.confirmed_requests_mut()))
}

pub fn with_confirmed_request<T, F>(request_id: RequestId, callback: F) -> Result<T, WalletError>
where
    F: FnOnce(&Request) -> T,
{
    with_state(|state| state.confirmed(request_id).map(callback))
}

// ACCOUNTS ----------------------------------------------------------------------

/// Retrieve an account.
/// This accepts a callback function that will be called with a reference to the account data.
pub fn with_account<T, F>(account_id: &String, callback: F) -> Result<T, WalletError>
where
    F: FnOnce(&WalletAccount) -> T,
{
    with_state(|state| state.account(account_id).map(callback))
}

/// Retrieve an account mutably.
/// This accepts a callback function that will be called with a mutable reference to the account data.
pub fn with_account_mut<T, F>(account_id: &String, callback: F) -> Result<T, WalletError>
where
    F: FnOnce(&mut WalletAccount) -> T,
{
    with_state_mut(|state| state.account_mut(account_id).map(callback))
}

pub fn with_ledger<T, F>(account_id: &String, callback: F) -> Result<T, WalletError>
where
    F: FnOnce(&Ledger) -> T,
{
    with_state(|state| {
        state
            .account(account_id)
            .map(|account| callback(&account.ledger))
    })
}

pub fn with_ledger_mut<T, F>(account_id: &String, callback: F) -> Result<T, WalletError>
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
pub fn with_signer<T, F>(signer_id: SignerId, callback: F) -> Result<T, WalletError>
where
    F: FnOnce(&Signer) -> T,
{
    with_signers(|signers| {
        signers
            .get(&signer_id)
            .ok_or(WalletError::SignerNotFound(signer_id.to_string()))
            .map(callback)
    })
}

/// Get all signers with a role, admins is always included.
pub fn with_role_signer_ids<T, F>(role: Roles, callback: F) -> T
where
    F: FnOnce(&Vec<SignerId>) -> T,
{
    with_signers(|signers| {
        let filtered_signers: Vec<SignerId> = signers
            .iter()
            .filter(|(_, signer)| signer.has_role(role))
            .map(|(signer_id, _)| signer_id.clone())
            .collect();

        callback(&filtered_signers)
    })
}

/// Check if a signer exists, and optionally check if it has a role.
pub fn with_check_signer(signer_id: SignerId, opt_role: Option<Roles>) -> Result<(), String> {
    with_signer(signer_id, |signer| {
        if let Some(role) = opt_role {
            if !signer.has_role(role.clone()) {
                return Err(WalletError::SignerRoleNotFound(
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
