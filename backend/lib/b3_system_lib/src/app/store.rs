use std::cell::RefCell;

use b3_utils::wasm::WasmHash;
use b3_utils::{memory::init_stable_mem, wasm::Wasm};

use super::error::AppSystemError;
use super::{
    release::Release,
    state::{AppState, ReleaseMap, WasmMap},
};

// The AppState starts from 1 to 9 to avoid conflicts with the user's stable memory
thread_local! {
    static APP_STATE: RefCell<AppState> = RefCell::new(
        AppState {
            apps: init_stable_mem("app_map", 1).unwrap(),
            releases: init_stable_mem("release_map", 2).unwrap(),
            wasm_map: init_stable_mem("wasm_map", 3).unwrap(),
        }
    );
}

pub fn with_app_state<R>(f: impl FnOnce(&AppState) -> R) -> R {
    APP_STATE.with(|state| f(&*state.borrow()))
}

pub fn with_app_state_mut<R>(f: impl FnOnce(&mut AppState) -> R) -> R {
    APP_STATE.with(|state| f(&mut *state.borrow_mut()))
}

pub fn with_releases<R>(f: impl FnOnce(&ReleaseMap) -> R) -> R {
    with_app_state(|state| f(&state.releases))
}

pub fn with_releases_mut<R>(f: impl FnOnce(&mut ReleaseMap) -> R) -> R {
    with_app_state_mut(|state| f(&mut state.releases))
}

pub fn with_release<F, T>(hash: &WasmHash, f: F) -> Result<T, AppSystemError>
where
    F: FnOnce(Release) -> T,
{
    with_releases(|releases| {
        releases
            .get(hash)
            .ok_or(AppSystemError::ReleaseNotFound)
            .map(f)
    })
}

pub fn with_release_mut<F, T>(hash: &WasmHash, f: F) -> Result<T, AppSystemError>
where
    F: FnOnce(&mut Release) -> T,
{
    with_releases_mut(|releases| {
        releases
            .get(hash)
            .ok_or(AppSystemError::ReleaseNotFound)
            .map(|mut release| f(&mut release))
    })
}

pub fn with_latest_hash_release<F, T>(f: F) -> Result<T, AppSystemError>
where
    F: FnOnce((WasmHash, Release)) -> T,
{
    with_releases(|releases| releases.last_key_value())
        .ok_or(AppSystemError::ReleaseNotFound)
        .map(f)
}

// WASM
pub fn with_wasm_map<F, R>(f: F) -> R
where
    F: FnOnce(&WasmMap) -> R,
{
    with_app_state(|state| f(&state.wasm_map))
}

pub fn with_wasm_map_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut WasmMap) -> R,
{
    with_app_state_mut(|state| f(&mut state.wasm_map))
}

pub fn with_release_wasm<F, T>(hash: &String, f: F) -> Result<T, AppSystemError>
where
    F: FnOnce(Wasm) -> T,
{
    with_wasm_map(|wasm_map| {
        wasm_map
            .get(hash)
            .ok_or(AppSystemError::WasmNotFound)
            .map(f)
    })
}
