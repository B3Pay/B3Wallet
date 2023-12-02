use crate::{
    app::App,
    error::SystemError,
    release::Release,
    state::State,
    types::{AppId, ReleaseVersion},
    user::User,
};
use b3_utils::{
    api::{bugs::AppBugs, AppVersion},
    memory::types::DefaultStableBTreeMap,
    memory::{init_stable_mem, init_stable_mem_refcell},
    principal::StoredPrincipal,
    types::{CanisterId, UserId},
    wasm::Wasm,
};

use std::cell::RefCell;

pub type UserMap = DefaultStableBTreeMap<UserId, User>;
pub type ReleaseMap = DefaultStableBTreeMap<ReleaseVersion, Release>;
pub type AppMap = DefaultStableBTreeMap<AppId, App>;

pub type WasmMap = DefaultStableBTreeMap<AppVersion, Wasm>;
pub type BugMap = DefaultStableBTreeMap<StoredPrincipal, AppBugs>;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(
        State {
            users: init_stable_mem("user_map", 1).unwrap(),
            apps: init_stable_mem("product_map", 2).unwrap(),
            releases: init_stable_mem("release_map", 3).unwrap(),
        }
    );
    static WASM_MAP: RefCell<WasmMap> = init_stable_mem_refcell("wasm_map", 10).unwrap();
    static BUG_MAP: RefCell<BugMap> = init_stable_mem_refcell("bug_map", 11).unwrap();
}

// STATE

pub fn with_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|state| f(&*state.borrow()))
}

pub fn with_state_mut<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|state| f(&mut *state.borrow_mut()))
}

// RELEASE

pub fn with_releases<R>(f: impl FnOnce(&ReleaseMap) -> R) -> R {
    with_state(|state| f(&state.releases))
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

// SIGNER

pub fn with_users<R>(f: impl FnOnce(&UserMap) -> R) -> R {
    with_state(|state| f(&state.users))
}

pub fn with_users_mut<R>(f: impl FnOnce(&mut UserMap) -> R) -> R {
    with_state_mut(|state| f(&mut state.users))
}

pub fn with_user_state<F, T>(user_id: UserId, f: F) -> Result<T, SystemError>
where
    F: FnOnce(User) -> T,
{
    with_users(|signers| {
        signers
            .get(&user_id)
            .ok_or(SystemError::UserNotFound)
            .map(f)
    })
}

pub fn with_user_state_mut<F, T>(user_id: &UserId, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&mut User) -> T,
{
    with_users_mut(|signers| {
        signers
            .get(user_id)
            .ok_or(SystemError::UserNotFound)
            .map(|mut user| f(&mut user))
    })
}

pub fn with_user_canister<F, T>(
    user_id: UserId,
    canister_id: &CanisterId,
    f: F,
) -> Result<T, SystemError>
where
    F: FnOnce(&CanisterId) -> T,
{
    with_user_state(user_id, |state| {
        state
            .canisters
            .iter()
            .find(|canister| canister == &canister_id)
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

pub fn with_bugs<R>(f: impl FnOnce(&BugMap) -> R) -> R {
    BUG_MAP.with(|bugs| f(&bugs.borrow()))
}

pub fn with_bugs_mut<R>(f: impl FnOnce(&mut BugMap) -> R) -> R {
    BUG_MAP.with(|bugs| f(&mut *bugs.borrow_mut()))
}

pub fn with_canister_bugs<F, T>(canister_id: &StoredPrincipal, f: F) -> Result<T, SystemError>
where
    F: FnOnce(AppBugs) -> T,
{
    with_bugs(|bugs| {
        bugs.get(canister_id)
            .ok_or(SystemError::BugsNotFound)
            .map(f)
    })
}

pub fn with_canister_bugs_mut<F, T>(canister_id: &StoredPrincipal, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&mut AppBugs) -> T,
{
    with_bugs_mut(|bugs| {
        bugs.get(canister_id)
            .ok_or(SystemError::BugsNotFound)
            .map(|mut bugs| f(&mut bugs))
    })
}

pub fn with_release_bugs<F, T>(canister_id: &StoredPrincipal, f: F) -> Result<T, SystemError>
where
    F: FnOnce(AppBugs) -> T,
{
    with_bugs(|bugs| {
        bugs.get(canister_id)
            .ok_or(SystemError::BugsNotFound)
            .map(f)
    })
}

pub fn with_release_bugs_mut<F, T>(canister_id: &StoredPrincipal, f: F) -> Result<T, SystemError>
where
    F: FnOnce(&mut AppBugs) -> T,
{
    with_bugs_mut(|bugs| {
        bugs.get(canister_id)
            .ok_or(SystemError::BugsNotFound)
            .map(|mut bugs| f(&mut bugs))
    })
}
