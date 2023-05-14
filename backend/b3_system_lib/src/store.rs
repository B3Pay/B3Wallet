use b3_helper::types::{SignerId, Version, Wasm, WasmHash};

use crate::{
    error::SystemError,
    types::{Release, Releases, SignerCanister, State, UserMap, WasmMap},
};
use std::cell::RefCell;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
    static WASM: RefCell<WasmMap> = RefCell::new(WasmMap::default());
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

pub fn with_releases<F, R>(f: F) -> R
where
    F: FnOnce(&Releases) -> R,
{
    with_state(|state| f(&state.releases))
}

pub fn with_releases_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut Releases) -> R,
{
    with_state_mut(|state| f(&mut state.releases))
}

pub fn with_release<F, T>(index: usize, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&Release) -> T,
{
    with_releases(|releases| {
        releases
            .get(index)
            .ok_or(SystemError::ReleaseNotFound)
            .map(f)
    })
}

pub fn with_release_mut<F, T>(index: usize, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&mut Release) -> T,
{
    with_releases_mut(|releases| {
        releases
            .get_mut(index)
            .ok_or(SystemError::ReleaseNotFound)
            .map(f)
    })
}

pub fn with_version_release<F, T>(version: Version, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&Release) -> T,
{
    with_releases(|releases| {
        releases
            .iter()
            .find(|release| release.version == version)
            .ok_or(SystemError::ReleaseNotFound)
            .map(f)
    })
}

pub fn with_version_release_mut<F, T>(version: Version, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&mut Release) -> T,
{
    with_releases_mut(|releases| {
        releases
            .iter_mut()
            .find(|release| release.version == version)
            .ok_or(SystemError::ReleaseNotFound)
            .map(f)
    })
}

pub fn with_hash_release<F, T>(hash: WasmHash, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&Release) -> T,
{
    with_releases(|releases| {
        releases
            .iter()
            .find(|release| release.hash == hash)
            .ok_or(SystemError::ReleaseNotFound)
            .map(f)
    })
}

pub fn with_latest_release<F, T>(f: F) -> Result<T, SystemError>
where
    F: FnOnce(&Release) -> T,
{
    with_releases(|releases| releases.last().ok_or(SystemError::ReleaseNotFound).map(f))
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

pub fn with_signer_canister<F, T>(user_id: &SignerId, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&SignerCanister) -> T,
{
    with_users(|signers| {
        signers
            .get(user_id)
            .ok_or(SystemError::SignerCanisterNotFound)
            .map(f)
    })
}

pub fn with_signer_canister_mut<F, T>(user_id: &SignerId, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&mut SignerCanister) -> T,
{
    with_users_mut(|signers| {
        signers
            .get_mut(user_id)
            .ok_or(SystemError::SignerCanisterNotFound)
            .map(f)
    })
}

// WASM

pub fn with_wasm_map<F, R>(f: F) -> R
where
    F: FnOnce(&WasmMap) -> R,
{
    WASM.with(|wasm| f(&wasm.borrow()))
}

pub fn with_wasm_map_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut WasmMap) -> R,
{
    WASM.with(|wasm| f(&mut wasm.borrow_mut()))
}

pub fn with_wasm<F, T>(version: &Version, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&Wasm) -> T,
{
    with_wasm_map(|wasm_map| {
        wasm_map
            .get(version)
            .ok_or(SystemError::WasmNotFound)
            .map(f)
    })
}

pub fn with_wasm_mut<F, T>(version: &Version, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&mut Wasm) -> T,
{
    with_wasm_map_mut(|wasm_map| {
        wasm_map
            .get_mut(version)
            .ok_or(SystemError::WasmNotFound)
            .map(f)
    })
}
