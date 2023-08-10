#[cfg(test)]
mod test {
    use crate::partition::{
        main::timer::TimerEntry, MainPartition, PartitionManager, PARTITION_MANAGER,
    };
    use b3_stable_structures::Memory;

    #[test]
    fn test_init_main_partition() {
        let main_partition = PARTITION_MANAGER.with(|pm| MainPartition::init(&mut pm.borrow_mut()));

        assert_eq!(main_partition.backup_details().size, 0);
        assert_eq!(main_partition.timer_details().size, 0);
    }

    #[test]
    fn test_core_backup_partition() {
        let mut main_partition =
            PARTITION_MANAGER.with(|pm| MainPartition::init(&mut pm.borrow_mut()));

        let backup = main_partition.backup();

        assert_eq!(backup.size(), 0);

        let mut state_bytes = vec![0u8; 4];

        state_bytes.copy_from_slice(&[1, 2, 3, 4]);

        main_partition.set_backup(state_bytes);

        let backup = main_partition.backup();

        assert_eq!(backup.size(), 1);

        let mut len_bytes = [0u8; 4];
        backup.read(0, &mut len_bytes);
        let state_len = u32::from_le_bytes(len_bytes);

        assert_eq!(state_len, 4);

        let mut state_bytes = vec![0; state_len as usize];
        backup.read(4, &mut state_bytes);

        println!("{:?}", state_bytes);
    }

    #[test]
    fn test_timer_partition() {
        let mut partition_manager = PartitionManager::init();

        let mut main_partition = MainPartition::init(&mut partition_manager);

        let timer = main_partition.timer();

        assert_eq!(timer.len(), 0);

        main_partition
            .push_timer(&TimerEntry { id: 1, time: 2 })
            .unwrap();

        let timer = main_partition.timer_mut();

        assert_eq!(timer.len(), 1);

        let timer_entry = timer.peek().unwrap();

        assert_eq!(timer_entry.id, 1);
        assert_eq!(timer_entry.time, 2);
    }
}
