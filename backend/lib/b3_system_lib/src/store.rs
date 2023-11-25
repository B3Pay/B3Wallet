use crate::{
    error::SystemError,
    types::{Release, Releases, WalletBugs},
    user::UserState,
    wallet::WalletCanister,
};
use b3_utils::{
    ledger::types::WalletVersion,
    memory::types::DefaultStableBTreeMap,
    memory::{init_stable_mem, init_stable_mem_refcell},
    principal::StoredPrincipal,
    types::UserId,
    wasm::{Wasm, WasmHash},
};

use std::cell::RefCell;

pub struct State {
    pub users: UserMap,
    pub releases: Releases,
}

pub type UserMap = DefaultStableBTreeMap<UserId, UserState>;
pub type WasmMap = DefaultStableBTreeMap<WalletVersion, Wasm>;
pub type BugMap = DefaultStableBTreeMap<StoredPrincipal, WalletBugs>;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(
        State {
            users: init_stable_mem("users", 1).unwrap(),
            releases: init_stable_mem("releases", 2).unwrap(),
        }
    );
    static WASM_MAP: RefCell<WasmMap> = init_stable_mem_refcell("wasm_map", 10).unwrap();
    static BUGS: RefCell<BugMap> = init_stable_mem_refcell("bug_map", 11).unwrap();
}

// STATE

pub fn with_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|state| f(&state.borrow()))
}

pub fn with_state_mut<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|state| f(&mut state.borrow_mut()))
}

// RELEASE

pub fn with_releases<R>(f: impl FnOnce(&Releases) -> R) -> R {
    with_state(|state| f(&state.releases))
}

pub fn with_releases_mut<R>(f: impl FnOnce(&mut Releases) -> R) -> R {
    with_state_mut(|state| f(&mut state.releases))
}

pub fn with_release<F, T>(index: u64, f: F) -> Result<T, SystemError>
where
    F: FnOnce(Release) -> T,
{
    with_releases(|releases| {
        releases
            .get(index)
            .ok_or(SystemError::ReleaseNotFound)
            .map(f)
    })
}

pub fn with_release_mut<F, T>(index: u64, f: F) -> Result<T, SystemError>
where
    F: FnOnce(Release) -> T,
{
    with_releases_mut(|releases| {
        releases
            .get(index)
            .ok_or(SystemError::ReleaseNotFound)
            .map(f)
    })
}

pub fn with_version_release<F, T>(version: String, f: F) -> Result<T, SystemError>
where
    F: FnOnce(Release) -> T,
{
    with_releases(|releases| {
        releases
            .iter()
            .find(|release| release.version == version)
            .ok_or(SystemError::ReleaseNotFound)
            .map(f)
    })
}

pub fn with_version_release_mut<F, T>(version: String, f: F) -> Result<T, SystemError>
where
    F: FnOnce(Release) -> T,
{
    with_releases_mut(|releases| {
        releases
            .iter()
            .find(|release| release.version == version)
            .ok_or(SystemError::ReleaseNotFound)
            .map(f)
    })
}

pub fn with_hash_release<F, T>(hash: WasmHash, f: F) -> Result<T, SystemError>
where
    F: FnOnce(Release) -> T,
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
    F: FnOnce(Release) -> T,
{
    with_releases(|releases| {
        releases
            .iter()
            .last()
            .ok_or(SystemError::ReleaseNotFound)
            .map(f)
    })
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
    F: FnOnce(UserState) -> T,
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
    F: FnOnce(UserState) -> T,
{
    with_users_mut(|signers| signers.get(user_id).ok_or(SystemError::UserNotFound).map(f))
}

pub fn with_wallet_canister<F, T>(user_id: UserId, index: usize, f: F) -> Result<T, SystemError>
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
    BUGS.with(|bugs| f(&*bugs.borrow()))
}

pub fn with_bugs_mut<R>(f: impl FnOnce(&mut BugMap) -> R) -> R {
    BUGS.with(|bugs| f(&mut *bugs.borrow_mut()))
}

pub fn with_canister_bugs<F, T>(canister_id: &StoredPrincipal, f: F) -> Result<T, SystemError>
where
    F: FnOnce(WalletBugs) -> T,
{
    with_bugs(|bugs| {
        bugs.get(canister_id)
            .ok_or(SystemError::BugsNotFound)
            .map(f)
    })
}

pub fn with_canister_bugs_mut<F, T>(canister_id: &StoredPrincipal, f: F) -> Result<T, SystemError>
where
    F: FnOnce(WalletBugs) -> T,
{
    with_bugs_mut(|bugs| {
        bugs.get(canister_id)
            .ok_or(SystemError::BugsNotFound)
            .map(f)
    })
}

pub fn with_release_bugs<F, T>(canister_id: &StoredPrincipal, f: F) -> Result<T, SystemError>
where
    F: FnOnce(WalletBugs) -> T,
{
    with_bugs(|bugs| {
        bugs.get(canister_id)
            .ok_or(SystemError::BugsNotFound)
            .map(f)
    })
}

pub fn with_release_bugs_mut<F, T>(canister_id: &StoredPrincipal, f: F) -> Result<T, SystemError>
where
    F: FnOnce(WalletBugs) -> T,
{
    with_bugs_mut(|bugs| {
        bugs.get(canister_id)
            .ok_or(SystemError::BugsNotFound)
            .map(f)
    })
}
