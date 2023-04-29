pub mod canister;
pub mod control;

use candid::{CandidType, Deserialize, Principal};
use control::{
    new_user_control, ControllerId, Controllers, LoadRelease, Releases, UserControl, UserId,
};
use ic_cdk::export::candid::candid_method;
use ic_cdk::{caller, id, init, post_upgrade, pre_upgrade, query, update};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Default, CandidType, Deserialize, Clone)]
pub struct State {
    releases: Releases,
    controllers: Controllers,
    user_controlls: HashMap<UserId, UserControl>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

pub fn is_controller(caller: UserId, controllers: &Controllers) -> bool {
    controllers
        .iter()
        .any(|(&controller, _)| controller.eq(&caller))
}

pub fn caller_is_controller() -> Result<(), String> {
    let caller = caller();
    let controllers: Controllers = STATE.with(|state| state.borrow().controllers.clone());

    if is_controller(caller, &controllers) {
        Ok(())
    } else {
        Err(format!(
            "Caller ({}) is not a controller of the system.",
            caller
        ))
    }
}

#[init]
#[candid_method(init)]
pub fn init() {
    // TODO: Remove this function and get owner from argument.
    let owner = Principal::anonymous();
    let manager = caller();

    STATE.with(|s| {
        let state = &mut *s.borrow_mut();

        state.add_controller(owner);
        state.add_controller(manager);
    });
}

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
pub fn get_controllers() -> Controllers {
    STATE.with(|s| s.borrow().controllers.clone())
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn add_controller(controller_id: ControllerId) {
    STATE.with(|s| {
        let state = &mut *s.borrow_mut();

        state.add_controller(controller_id);
    });
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn remove_controller(controller_id: ControllerId) {
    STATE.with(|s| {
        let state = &mut *s.borrow_mut();

        state.remove_controller(controller_id);
    });
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn load_release(blob: Vec<u8>, version: String) -> LoadRelease {
    STATE.with(|s| {
        s.borrow_mut().update_release(&blob, version);
    });

    let total: usize = STATE.with(|state| state.borrow().releases.wasm.len());

    LoadRelease {
        total,
        chunks: blob.len(),
    }
}

#[candid_method(update)]
#[update(guard = "caller_is_controller")]
fn reset_release() {
    STATE.with(|s| {
        let state = &mut *s.borrow_mut();

        state.remove_release();
    });
}

#[update]
#[candid_method(update)]
pub async fn create_user_control() -> Result<UserControl, String> {
    let user = caller();
    let console = id();

    let user_control = STATE.with(|state| {
        let state = state.borrow();

        state.get_user_control(&user)
    });

    match user_control {
        Some(user_control) => Ok(user_control),
        None => new_user_control(&user, &console).await,
    }
}

#[candid_method(query)]
#[query]
fn get_releases_version() -> String {
    STATE.with(|s| s.borrow().releases.version.clone().unwrap_or_default())
}

#[pre_upgrade]
pub fn pre_upgrade() {
    STATE.with(|s| {
        ic_cdk::storage::stable_save((s.borrow().clone(),)).unwrap();
    });
}

#[post_upgrade]
pub fn post_upgrade() {
    let (s_prev,): (State,) = ic_cdk::storage::stable_restore().unwrap();
    STATE.with(|s| {
        *s.borrow_mut() = s_prev;
    });
}

#[cfg(test)]
#[test]
fn generate_candid() {
    use std::io::Write;

    ic_cdk::export::candid::export_service!();

    let candid = format!("{}", __export_service());

    let mut file = std::fs::File::create("./b3_system.did").unwrap();

    file.write_all(candid.as_bytes()).unwrap();

    assert!(true);
}
