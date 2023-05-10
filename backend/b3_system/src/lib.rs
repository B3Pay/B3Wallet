mod canister;
mod control;
mod guards;
mod release;
mod state;
mod store;
mod types;

use canister::canister_status;
use control::new_user_control;
use guards::caller_is_controller;
use ic_cdk::export::{candid::candid_method, Principal};
use ic_cdk::{caller, id, init, post_upgrade, pre_upgrade, query, update};
use state::State;
use store::{with_state, with_state_mut, with_wasm_map, with_wasm_map_mut};
use types::{
    CanisterStatus, ControllerId, Controllers, UserControl, UserControlId, UserId, WasmMap,
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
pub async fn get_canister_status(canister_id: Principal) -> Result<CanisterStatus, String> {
    canister_status(canister_id).await
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

#[pre_upgrade]
pub fn pre_upgrade() {
    let wasm = with_wasm_map(|m| m.clone());

    let state = with_state(|s| s.clone());

    ic_cdk::storage::stable_save((state, wasm)).unwrap();
}

#[post_upgrade]
pub fn post_upgrade() {
    let (s_prev, w_prev): (State, WasmMap) = ic_cdk::storage::stable_restore().unwrap();
    with_state_mut(|s| {
        *s = s_prev;
    });

    with_wasm_map_mut(|m| {
        *m = w_prev;
    });
}

#[cfg(test)]
mod tests {
    use super::types::*;

    use ic_cdk::export::Principal;

    #[test]
    fn generate_candid() {
        use std::io::Write;

        ic_cdk::export::candid::export_service!();

        let candid = __export_service();

        let mut file = std::fs::File::create("./b3_system.did").unwrap();

        file.write_all(candid.as_bytes()).unwrap();

        assert!(true);
    }
}
