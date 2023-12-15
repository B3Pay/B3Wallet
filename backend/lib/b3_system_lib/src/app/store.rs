use std::cell::RefCell;

use b3_utils::{memory::init_stable_mem, wasm::Wasm};

use crate::{error::SystemError, types::ReleaseVersion};

use super::{release::Release, AppState, ReleaseMap, WasmMap};

// The AppState starts from 1 to 10 to avoid conflicts with the user's stable memory
thread_local! {
    static STATE: RefCell<AppState> = RefCell::new(
        AppState {
            apps: init_stable_mem("app_map", 1).unwrap(),
            releases: init_stable_mem("release_map", 2).unwrap(),
            wasm_map: init_stable_mem("wasm_map", 3).unwrap(),
        }
    );
}

pub fn with_app_state<R>(f: impl FnOnce(&AppState) -> R) -> R {
    STATE.with(|state| f(&*state.borrow()))
}

pub fn with_state_mut<R>(f: impl FnOnce(&mut AppState) -> R) -> R {
    STATE.with(|state| f(&mut *state.borrow_mut()))
}

pub fn with_releases<R>(f: impl FnOnce(&ReleaseMap) -> R) -> R {
    with_app_state(|state| f(&state.releases))
}

pub fn with_releases_mut<R>(f: impl FnOnce(&mut ReleaseMap) -> R) -> R {
    with_state_mut(|state| f(&mut state.releases))
}

pub fn with_release<F, T>(version: &ReleaseVersion, f: F) -> Result<T, SystemError>
where
    F: FnOnce(Release) -> T,
{
    with_releases(|releases| {
        releases
            .get(version)
            .ok_or(SystemError::ReleaseNotFound)
            .map(f)
    })
}

pub fn with_release_mut<F, T>(version: &ReleaseVersion, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&mut Release) -> T,
{
    with_releases_mut(|releases| {
        releases
            .get(version)
            .ok_or(SystemError::ReleaseNotFound)
            .map(|mut release| f(&mut release))
    })
}

pub fn with_latest_version_release<F, T>(f: F) -> Result<T, SystemError>
where
    F: FnOnce((ReleaseVersion, Release)) -> T,
{
    with_releases(|releases| releases.last_key_value())
        .ok_or(SystemError::ReleaseNotFound)
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
    with_state_mut(|state| f(&mut state.wasm_map))
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
