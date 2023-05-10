use ic_cdk::caller;

use crate::{
    store::with_state,
    types::{ControllerId, UserId},
};

pub fn is_controller(caller: UserId, controllers: &Vec<ControllerId>) -> bool {
    controllers.contains(&caller)
}

pub fn caller_is_controller() -> Result<(), String> {
    let caller = caller();
    let controllers: Vec<ControllerId> = with_state(|s| s.get_controllers());

    if is_controller(caller, &controllers) {
        Ok(())
    } else {
        Err(format!(
            "Caller ({}) is not a controller of the system.",
            caller
        ))
    }
}
