#[cfg(test)]
mod test {
    use crate::memory::{with_backup_mem, with_backup_mem_mut};

    #[test]
    fn test_init_main_partition() {
        with_backup_mem(|bp| {
            assert_eq!(bp.len(), 0);
        });
    }

    #[test]
    fn test_core_backup_partition() {
        with_backup_mem_mut(|backup| {
            assert_eq!(backup.len(), 0);

            let state_bytes = [1, 2, 3, 4].to_vec();

            backup.set_backup(state_bytes.clone());

            assert_eq!(backup.len(), 1);

            assert_eq!(state_bytes, backup.get_backup());
        });
    }
}
