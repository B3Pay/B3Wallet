use std::cell::RefCell;

use crate::types::SignerId;

thread_local! {
    static STATE: RefCell<SignerId> = RefCell::new(SignerId::anonymous());
}

pub fn with_owner<F, R>(f: F) -> R
where
    F: FnOnce(&SignerId) -> R,
{
    STATE.with(|states| {
        let state = states.borrow();
        f(&state)
    })
}

pub fn with_owner_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut SignerId) -> R,
{
    STATE.with(|states| {
        let mut state = states.borrow_mut();
        f(&mut state)
    })
}

pub fn caller_is_owner() -> Result<(), String> {
    let caller_id = ic_cdk::caller();

    with_owner(|owner_id| {
        if caller_id == *owner_id {
            Ok(())
        } else {
            Err("Error::Caller is not owner!".to_string())
        }
    })
}
