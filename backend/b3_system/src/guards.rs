use ic_cdk::caller;

use b3_shared::types::ControllerId;
use b3_system_lib::store::with_state;

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
