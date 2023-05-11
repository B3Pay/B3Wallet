use b3_shared::types::Version;

use crate::types::{Release, Releases, State, SystemWasm, WasmMap};
use std::cell::RefCell;

// STATE

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State::default());
}

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

pub fn with_release<F, T>(index: usize, f: F) -> Result<T, String>
where
    F: FnOnce(&Release) -> T,
{
    with_releases(|releases| {
        releases
            .get(index)
            .ok_or("Release not found!".to_string())
            .map(f)
    })
}

pub fn with_release_mut<F, T>(index: usize, f: F) -> Result<T, String>
where
    F: FnOnce(&mut Release) -> T,
{
    with_releases_mut(|releases| {
        releases
            .get_mut(index)
            .ok_or("Release not found!".to_string())
            .map(f)
    })
}

pub fn with_version_release<F, T>(version: Version, f: F) -> Result<T, String>
where
    F: FnOnce(&Release) -> T,
{
    with_releases(|releases| {
        releases
            .iter()
            .find(|release| release.version == version)
            .ok_or("Release not found!".to_string())
            .map(f)
    })
}

pub fn with_version_release_mut<F, T>(version: Version, f: F) -> Result<T, String>
where
    F: FnOnce(&mut Release) -> T,
{
    with_releases_mut(|releases| {
        releases
            .iter_mut()
            .find(|release| release.version == version)
            .ok_or("Release not found!".to_string())
            .map(f)
    })
}

pub fn with_latest_release<F, T>(f: F) -> Result<T, String>
where
    F: FnOnce(&Release) -> T,
{
    with_releases(|releases| {
        releases
            .last()
            .ok_or("No releases found!".to_string())
            .map(f)
    })
}

// WASM

thread_local! {
    pub static WASM: RefCell<WasmMap> = RefCell::new(WasmMap::default());
}

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

pub fn with_wasm<F, T>(version: &Version, f: F) -> Result<T, String>
where
    F: FnOnce(&SystemWasm) -> T,
{
    with_wasm_map(|wasm_map| {
        wasm_map
            .get(version)
            .ok_or("Wasm not found!".to_string())
            .map(f)
    })
}

pub fn with_wasm_mut<F, T>(version: &Version, f: F) -> Result<T, String>
where
    F: FnOnce(&mut SystemWasm) -> T,
{
    with_wasm_map_mut(|wasm_map| {
        wasm_map
            .get_mut(version)
            .ok_or("Wasm not found!".to_string())
            .map(f)
    })
}
