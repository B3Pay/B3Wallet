mod canister;
mod control;
mod guards;
mod release;
mod state;
mod store;
mod types;
mod wasm;

use canister::canister_status;
use control::new_user_control;
use guards::caller_is_controller;
use ic_cdk::export::{candid::candid_method, Principal};
use ic_cdk::{caller, id, init, post_upgrade, pre_upgrade, query, trap, update};
use state::State;
use store::{
    with_latest_release, with_release, with_release_mut, with_releases, with_releases_mut,
    with_state, with_state_mut,
};
use types::{
    Blob, CanisterStatus, ControllerId, Controllers, LoadRelease, Release, ReleaseArgs, Releases,
    UserControl, UserControlId, UserId,
};

#[init]
#[candid_method(init)]
pub fn init() {
    // TODO: Remove this function and get owner from argument.
    let owner = Principal::anonymous();
    let manager = caller();

    with_state_mut(|s| {
        s.add_controller(owner);
        s.add_controller(manager);
    });
}

#[candid_method(query)]
#[query]
pub fn get_user_control() -> Option<UserControl> {
    let user = caller();

    with_state(|s| s.get_user_control(&user))
}

#[candid_method(query)]
#[query]
pub fn get_user_control_id(user: Principal) -> Option<UserControlId> {
    with_state(|s| s.get_user_control_id(&user))
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub fn get_user_ids() -> Vec<UserId> {
    with_state(|s| s.get_user_ids())
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub fn get_controllers() -> Controllers {
    with_state(|s| s.controllers.clone())
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub fn get_release(index: usize) -> Release {
    with_release(index, |r| r.clone()).unwrap_or_else(|e| trap(&e))
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub async fn get_canister_status(canister_id: Principal) -> Result<CanisterStatus, String> {
    canister_status(canister_id).await
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn add_controller(controller_id: ControllerId) {
    with_state_mut(|s| {
        s.add_controller(controller_id);
    });
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn remove_controller(controller_id: ControllerId) {
    with_state_mut(|s| {
        s.remove_controller(controller_id);
    });
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn remove_user_control(user: UserId) {
    with_state_mut(|s| {
        s.remove_user_control(&user);
    });
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn load_release(blob: Blob, release_args: ReleaseArgs) -> Result<LoadRelease, String> {
    let version = release_args.version.clone();

    let release_index =
        with_releases_mut(|rs| match rs.iter().position(|r| r.version == version) {
            Some(index) => index,
            None => {
                let release = Release::new(release_args);
                rs.push(release);

                rs.len() - 1
            }
        });

    let total = with_release_mut(release_index, |r| r.load_wasm(&blob))
        .unwrap_or_else(|e| trap(&e))
        .unwrap_or_else(|e| trap(&e));

    let chunks = blob.len();

    Ok(LoadRelease {
        version,
        chunks,
        total,
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn remove_latest_release() {
    with_releases_mut(|rs| {
        rs.pop();
    });
}

#[update]
#[candid_method(update)]
pub async fn create_user_control() -> Result<UserControl, String> {
    let user = caller();
    let system = id();

    let user_control = with_state(|s| s.get_user_control(&user));

    match user_control {
        Some(user_control) => Ok(user_control),
        None => new_user_control(user, system).await,
    }
}

#[candid_method(query)]
#[query]
fn get_releases() -> Releases {
    with_releases(|r| r.clone())
}

#[candid_method(query)]
#[query]
fn get_latest_release() -> Release {
    with_latest_release(|r| r.clone()).unwrap_or_else(|e| trap(&e))
}

#[pre_upgrade]
pub fn pre_upgrade() {
    with_state(|s| {
        ic_cdk::storage::stable_save((s.clone(),)).unwrap();
    });
}

#[post_upgrade]
pub fn post_upgrade() {
    let (s_prev,): (State,) = ic_cdk::storage::stable_restore().unwrap();
    with_state_mut(|s| {
        *s = s_prev;
    });
}

#[cfg(test)]
#[test]
fn generate_candid() {
    use std::io::Write;

    ic_cdk::export::candid::export_service!();

    let candid = __export_service();

    let mut file = std::fs::File::create("./b3_system.did").unwrap();

    file.write_all(candid.as_bytes()).unwrap();

    assert!(true);
}
