use std::cell::RefCell;

use b3_utils::memory::init_stable_mem_refcell;
use b3_utils::memory::types::DefaultStableBTreeMap;
use b3_utils::wasm::Wasm;
use b3_utils::wasm::WasmHash;

use super::error::AppSystemError;
use super::release::Release;
use super::types::AppId;
use super::App;

pub type ReleaseMap = DefaultStableBTreeMap<WasmHash, Release>;
pub type AppMap = DefaultStableBTreeMap<AppId, App>;
pub type WasmMap = DefaultStableBTreeMap<WasmHash, Wasm>;

// The AppState starts from 1 to 9 to avoid conflicts with the user's stable memory
thread_local! {
    static APP_MAP: RefCell<AppMap> = init_stable_mem_refcell("app_map", 1).unwrap();
    static RELEASE_MAP: RefCell<ReleaseMap> = init_stable_mem_refcell("release_map", 2).unwrap();
    static WASM_MAP: RefCell<WasmMap>  = init_stable_mem_refcell("wasm_map", 3).unwrap();
}

// APPS
pub fn with_apps<R>(f: impl FnOnce(&AppMap) -> R) -> R {
    APP_MAP.with(|state| f(&*state.borrow()))
}

pub fn with_apps_mut<R>(f: impl FnOnce(&mut AppMap) -> R) -> R {
    APP_MAP.with(|state| f(&mut *state.borrow_mut()))
}

pub fn with_app<F, T>(app_id: &AppId, f: F) -> Result<T, AppSystemError>
where
    F: FnOnce(App) -> T,
{
    with_apps(|state| state.get(&app_id).ok_or(AppSystemError::AppNotFound).map(f))
}

pub fn with_app_mut<F, T>(app_id: &AppId, f: F) -> Result<T, AppSystemError>
where
    F: FnOnce(&mut App) -> T,
{
    with_apps_mut(|state| {
        state
            .get(app_id)
            .ok_or(AppSystemError::AppNotFound)
            .map(|mut app| f(&mut app))
    })
}

// RELEASES
pub fn with_releases<R>(f: impl FnOnce(&ReleaseMap) -> R) -> R {
    RELEASE_MAP.with(|state| f(&state.borrow()))
}

pub fn with_releases_mut<R>(f: impl FnOnce(&mut ReleaseMap) -> R) -> R {
    RELEASE_MAP.with(|state| f(&mut state.borrow_mut()))
}

pub fn with_release<F, T>(wasm_hash: &WasmHash, f: F) -> Result<T, AppSystemError>
where
    F: FnOnce(Release) -> T,
{
    with_releases(|releases| {
        releases
            .get(wasm_hash)
            .ok_or(AppSystemError::ReleaseNotFound)
            .map(f)
    })
}

pub fn with_release_mut<F, T>(wasm_hash: &WasmHash, f: F) -> Result<T, AppSystemError>
where
    F: FnOnce(&mut Release) -> T,
{
    with_releases_mut(|releases| {
        releases
            .get(wasm_hash)
            .ok_or(AppSystemError::ReleaseNotFound)
            .map(|mut release| f(&mut release))
    })
}

// WASM
pub fn with_wasms<F, R>(f: F) -> R
where
    F: FnOnce(&WasmMap) -> R,
{
    WASM_MAP.with(|state| f(&*state.borrow()))
}

pub fn with_wasms_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut WasmMap) -> R,
{
    WASM_MAP.with(|state| f(&mut *state.borrow_mut()))
}

pub fn with_wasm<F, T>(wasm_hash: &WasmHash, f: F) -> Result<T, AppSystemError>
where
    F: FnOnce(Wasm) -> T,
{
    with_wasms(|wasm_map| {
        wasm_map
            .get(wasm_hash)
            .ok_or(AppSystemError::WasmNotFound)
            .map(f)
    })
}

pub fn with_wasm_mut<F, T>(wasm_hash: &WasmHash, f: F) -> Result<T, AppSystemError>
where
    F: FnOnce(&mut Wasm) -> T,
{
    with_wasms_mut(|wasm_map| {
        wasm_map
            .get(wasm_hash)
            .ok_or(AppSystemError::WasmNotFound)
            .map(|mut wasm| f(&mut wasm))
    })
}
