#[cfg(test)]
mod test {
    use crate::{
        partition::{main::timer::TimerEntry, with_partition_manager_mut, MainPartition},
        NanoTimeStamp,
    };
    use b3_stable_structures::Memory;

    #[test]
    fn test_init_main_partition() {
        let main_partition = with_partition_manager_mut(|pm| MainPartition::init(pm));

        assert_eq!(main_partition.backup_details().len, 0);
        assert_eq!(main_partition.timer_details().len, 0);
    }

    #[test]
    fn test_core_backup_partition() {
        let mut main_partition = with_partition_manager_mut(|pm| MainPartition::init(pm));

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
        println!("{:?}", main_partition.get_backup());

        assert_eq!(state_bytes, main_partition.get_backup());
    }

    #[test]
    fn test_timer_partition() {
        let mut main_partition = with_partition_manager_mut(|pm| MainPartition::init(pm));

        let timer = main_partition.timer();

        assert_eq!(timer.len(), 0);

        main_partition
            .push_timer(&TimerEntry {
                id: 1,
                time: NanoTimeStamp(2),
            })
            .unwrap();

        let timer = main_partition.timer_mut();

        assert_eq!(timer.len(), 1);

        let timer_entry = timer.peek().unwrap();

        assert_eq!(timer_entry.id, 1);
        assert_eq!(timer_entry.time, 2.into());
    }
}
