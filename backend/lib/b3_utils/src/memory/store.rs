use std::cell::RefCell;

use super::{
    backup::BackupPartition, error::StableMemoryError, traits::InitMemory, StableMemoryManager,
};

thread_local! {
    pub static STABLE_MEMORY: RefCell<StableMemoryManager> = RefCell::new(StableMemoryManager::init())
}

pub fn with_stable_mem<F, R>(f: F) -> R
where
    F: FnOnce(&StableMemoryManager) -> R,
{
    STABLE_MEMORY.with(|states| {
        let state = states.borrow();
        f(&state)
    })
}

pub fn with_stable_mem_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut StableMemoryManager) -> R,
{
    STABLE_MEMORY.with(|states| {
        let mut state = states.borrow_mut();
        f(&mut state)
    })
}

pub fn with_backup_mem<F, R>(f: F) -> R
where
    F: FnOnce(&BackupPartition) -> R,
{
    with_stable_mem(|pm| {
        let bp = pm.backup();
        f(&bp)
    })
}

pub fn with_backup_mem_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut BackupPartition) -> R,
{
    with_stable_mem_mut(|pm| {
        let mut bp = pm.backup_mut();
        f(&mut bp)
    })
}

pub fn with_stable_mem_by_name<F, R>(name: &str, f: F) -> R
where
    F: FnOnce(&super::types::DefaultVM) -> R,
{
    with_stable_mem(|pm| {
        let memory = pm
            .memory(name)
            .expect(&format!("Unable to find memory with name: {}", name));
        f(&memory)
    })
}

pub fn init_stable_mem<F: InitMemory<F>>(name: &str, id: u8) -> Result<F, StableMemoryError> {
    with_stable_mem_mut(|pm| pm.init_memory(name, id))
}

pub fn init_stable_mem_refcell<F: InitMemory<F>>(
    name: &str,
    id: u8,
) -> Result<RefCell<F>, StableMemoryError> {
    let memory = with_stable_mem_mut(|pm| pm.init_memory(name, id))?;
    Ok(RefCell::new(memory))
}
