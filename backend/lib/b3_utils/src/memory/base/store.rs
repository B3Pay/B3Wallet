use std::cell::RefCell;

use crate::memory::with_stable_memory_mut;

use super::BasePartition;

thread_local! {
    static BASE_PARTITION: RefCell<BasePartition> = RefCell::new(with_stable_memory_mut(|pm| BasePartition::init(pm)));
}

pub fn with_base_partition<F, R>(f: F) -> R
where
    F: FnOnce(&BasePartition) -> R,
{
    BASE_PARTITION.with(|states| {
        let state = states.borrow();
        f(&state)
    })
}

pub fn with_base_partition_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut BasePartition) -> R,
{
    BASE_PARTITION.with(|states| {
        let mut state = states.borrow_mut();
        f(&mut state)
    })
}
