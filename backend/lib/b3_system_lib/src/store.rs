use b3_utils::{
    ledger::types::Bug,
    memory::{init_stable_mem_refcell, types::DefaultVMVec},
    types::{CanisterId, UserId},
    wasm::{Wasm, WasmHash},
};
use std::str::FromStr;

use crate::{
    error::SystemError,
    release::names::ReleaseNames,
    types::{Release, ReleaseMap, Releases, State, UserMap, WasmMap},
    user::UserState,
    wallet::WalletCanister,
};
use std::cell::RefCell;

thread_local! {
    static WASM_MAP: RefCell<WasmMap> = init_stable_mem_refcell("wasm_map", 10).unwrap();

    static BUGS: RefCell<DefaultVMVec<Bug>> = init_stable_mem_refcell("bugs", 11).unwrap();

    static STATE: RefCell<State> = RefCell::new(State::default());
}

// STATE

pub fn with_state<F, R>(f: F) -> R
where
    F: FnOnce(&State) -> R,
{
    STATE.with(|state| f(&state.borrow()))
}

pub fn with_state_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut State) -> R,
{
    STATE.with(|state| f(&mut state.borrow_mut()))
}

// RELEASE

pub fn with_release_map<F, R>(f: F) -> R
where
    F: FnOnce(&ReleaseMap) -> R,
{
    with_state(|state| f(&state.releases))
}

pub fn with_release_map_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut ReleaseMap) -> R,
{
    with_state_mut(|state| f(&mut state.releases))
}

pub fn with_releases<F, T>(name: &str, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&Releases) -> T,
{
    let release_name = ReleaseNames::from_str(name)?;

    with_release_map(|releases| {
        releases
            .get(&release_name)
            .ok_or(SystemError::ReleaseNameNotFound)
            .map(f)
    })
}

pub fn with_releases_mut<F, T>(release_name: ReleaseNames, f: F) -> T
where
    F: FnOnce(&mut Releases) -> T,
{
    with_release_map_mut(|releases| {
        let releases = releases.entry(release_name).or_default();
        f(releases)
    })
}

pub fn with_release<F, T>(name: &str, index: usize, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&Release) -> T,
{
    with_releases(name, |releases| {
        releases
            .get(index)
            .ok_or(SystemError::ReleaseNotFound)
            .map(f)
    })?
}

pub fn with_release_mut<F, T>(name: &str, index: usize, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&mut Release) -> T,
{
    let release_name = ReleaseNames::from_str(name)?;

    with_releases_mut(release_name, |releases| {
        releases
            .get_mut(index)
            .ok_or(SystemError::ReleaseNotFound)
            .map(f)
    })
}

pub fn with_version_release<F, T>(name: &str, version: String, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&Release) -> T,
{
    with_releases(name, |releases| {
        releases
            .iter()
            .find(|release| release.version == version)
            .ok_or(SystemError::ReleaseNotFound)
            .map(f)
    })?
}

pub fn with_version_release_mut<F, T>(
    release_name: ReleaseNames,
    version: String,
    f: F,
) -> Result<T, SystemError>
where
    F: FnOnce(&mut Release) -> T,
{
    with_releases_mut(release_name, |releases| {
        releases
            .iter_mut()
            .find(|release| release.version == version)
            .ok_or(SystemError::ReleaseNotFound)
            .map(f)
    })
}

pub fn with_hash_release<F, T>(name: &str, hash: WasmHash, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&Release) -> T,
{
    with_releases(name, |releases| {
        releases
            .iter()
            .find(|release| release.hash == hash)
            .ok_or(SystemError::ReleaseNotFound)
            .map(f)
    })?
}

pub fn with_latest_release<F, T>(name: &str, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&Release) -> T,
{
    with_releases(name, |releases| {
        releases.last().ok_or(SystemError::ReleaseNotFound).map(f)
    })?
}

// SIGNER

pub fn with_users<F, R>(f: F) -> R
where
    F: FnOnce(&UserMap) -> R,
{
    with_state(|state| f(&state.users))
}

pub fn with_users_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut UserMap) -> R,
{
    with_state_mut(|state| f(&mut state.users))
}

pub fn with_user_state<F, T>(user_id: &UserId, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&UserState) -> T,
{
    with_users(|signers| signers.get(user_id).ok_or(SystemError::UserNotFound).map(f))
}

pub fn with_user_state_mut<F, T>(user_id: &UserId, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&mut UserState) -> T,
{
    with_users_mut(|signers| {
        signers
            .get_mut(user_id)
            .ok_or(SystemError::UserNotFound)
            .map(f)
    })
}

pub fn with_wallet_canister<F, T>(user_id: &UserId, index: usize, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&WalletCanister) -> T,
{
    with_user_state(user_id, |state| {
        state
            .canisters
            .get(index)
            .ok_or(SystemError::WalletCanisterNotFound)
            .map(f)
    })?
}

pub fn with_wallet_canister_mut<F, T>(
    user_id: &UserId,
    index: usize,
    f: F,
) -> Result<T, SystemError>
where
    F: FnOnce(&mut WalletCanister) -> T,
{
    with_user_state_mut(user_id, |state| {
        state
            .canisters
            .get_mut(index)
            .ok_or(SystemError::WalletCanisterNotFound)
            .map(f)
    })?
}

// WASM

pub fn with_wasm_map<F, R>(f: F) -> R
where
    F: FnOnce(&WasmMap) -> R,
{
    WASM_MAP.with(|wasm| f(&wasm.borrow()))
}

pub fn with_wasm_map_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut WasmMap) -> R,
{
    WASM_MAP.with(|wasm| f(&mut wasm.borrow_mut()))
}

pub fn with_release_wasm<F, T>(version: &String, f: F) -> Result<T, SystemError>
where
    F: FnOnce(Wasm) -> T,
{
    with_wasm_map(|wasm_map| {
        wasm_map
            .get(version)
            .ok_or(SystemError::WasmNotFound)
            .map(f)
    })
}

pub fn with_bugs<F, R>(f: F) -> R
where
    F: FnOnce(&DefaultVMVec<Bug>) -> R,
{
    BUGS.with(|bugs| f(&bugs.borrow()))
}

pub fn with_bugs_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut DefaultVMVec<Bug>) -> R,
{
    BUGS.with(|bugs| f(&mut bugs.borrow_mut()))
}

pub fn with_bug<F, T>(canister_id: &CanisterId, f: F) -> Result<T, SystemError>
where
    F: FnOnce(Bug) -> T,
{
    with_bugs(|bugs| {
        bugs.iter()
            .find(|bug| bug.canister_id == *canister_id)
            .ok_or(SystemError::BugNotFound)
            .map(f)
    })
}
