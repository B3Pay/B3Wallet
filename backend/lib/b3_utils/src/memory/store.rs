use std::cell::RefCell;

use super::StableMemory;

thread_local! {
    pub static STABLE_MEMORY: RefCell<StableMemory> = RefCell::new(StableMemory::init())
}

pub fn with_stable_memory<F, R>(f: F) -> R
where
    F: FnOnce(&StableMemory) -> R,
{
    STABLE_MEMORY.with(|states| {
        let state = states.borrow();
        f(&state)
    })
}

pub fn with_stable_memory_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut StableMemory) -> R,
{
    STABLE_MEMORY.with(|states| {
        let mut state = states.borrow_mut();
        f(&mut state)
    })
}
