use b3_helper::types::ControllerId;
use b3_system_lib::store::{with_state, with_state_mut};
use ic_cdk::{caller, export::candid::candid_method, query, update};

#[candid_method(query)]
#[query(guard = "caller_is_controller")]
fn get_controllers() -> Vec<ControllerId> {
    with_state(|s| s.get_controllers())
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

pub fn caller_is_controller() -> Result<(), String> {
    let caller = caller();
    let controllers: Vec<ControllerId> = with_state(|s| s.get_controllers());

    if controllers.contains(&caller) {
        Ok(())
    } else {
        Err(format!(
            "Caller ({}) is not a controller of the system.",
            caller
        ))
    }
}
