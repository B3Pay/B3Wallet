mod guards;
mod release;
mod signer;
mod state;
mod store;
mod types;

use b3_shared::types::{ControllerId, UserId};
use guards::caller_is_controller;
use ic_cdk::export::{candid::candid_method, Principal};
use ic_cdk::{caller, init, post_upgrade, pre_upgrade, query, update};
use store::{with_state, with_state_mut, with_wasm_map, with_wasm_map_mut};
use types::{State, WasmMap};

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
#[query(guard = "caller_is_controller")]
pub fn get_user_ids() -> Vec<UserId> {
    with_state(|s| s.get_user_ids())
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
    use b3_shared::types::*;

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
