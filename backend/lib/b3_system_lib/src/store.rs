use crate::error::SystemError;
use b3_utils::{
    api::bugs::AppBugs, memory::init_stable_mem_refcell, memory::types::DefaultStableBTreeMap,
    principal::StoredPrincipal,
};

use std::cell::RefCell;

pub type BugMap = DefaultStableBTreeMap<StoredPrincipal, AppBugs>;

thread_local! {
    static BUG_MAP: RefCell<BugMap> = init_stable_mem_refcell("bug_map", 11).unwrap();
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
