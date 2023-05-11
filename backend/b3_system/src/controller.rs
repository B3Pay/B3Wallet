use crate::guards::caller_is_controller;
use b3_shared::types::{ControllerId, UserId};
use b3_system_lib::store::{with_state, with_state_mut};
use ic_cdk::export::candid::candid_method;
use ic_cdk::{query, update};

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
