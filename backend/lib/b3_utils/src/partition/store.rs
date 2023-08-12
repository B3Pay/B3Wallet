use std::cell::RefCell;

use super::PartitionManager;

thread_local! {
    pub static PARTITION_MANAGER: RefCell<PartitionManager> = RefCell::new(PartitionManager::init())
}

pub fn with_partition_manager<F, R>(f: F) -> R
where
    F: FnOnce(&PartitionManager) -> R,
{
    PARTITION_MANAGER.with(|states| {
        let state = states.borrow();
        f(&state)
    })
}

pub fn with_partition_manager_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut PartitionManager) -> R,
{
    PARTITION_MANAGER.with(|states| {
        let mut state = states.borrow_mut();
        f(&mut state)
    })
}

#[cfg(test)]
mod test {
    use crate::partition::PartitionManager;

    #[test]
    fn test_init_partition_manager() {
        let partition_manager = PartitionManager::init();

        assert_eq!(partition_manager.partitions.len(), 0);
    }

    #[test]
    fn test_create_partition() {
        let mut partition_manager = PartitionManager::init();

        let partition = partition_manager.create_partition("test", 1);

        assert!(partition.is_ok());

        let partition = partition_manager.create_partition("test", 2);

        assert!(partition.is_err());

        let partition = partition_manager.create_partition("test2", 1);

        assert!(partition.is_err());
    }
}
