#[cfg(test)]
mod test {
    use crate::{
        memory::base::{timer::TimerEntry, with_base_partition, with_base_partition_mut},
        NanoTimeStamp,
    };
    use b3_stable_structures::Memory;

    #[test]
    fn test_init_main_partition() {
        with_base_partition(|base_partition| {
            assert_eq!(base_partition.backup_details().len, 0);
            assert_eq!(base_partition.timer_details().len, 0);
            assert_eq!(base_partition.details().len(), 2);
        });
    }

    #[test]
    fn test_core_backup_partition() {
        with_base_partition_mut(|base_partition| {
            let backup = base_partition.backup();

            assert_eq!(backup.size(), 0);

            let state_bytes = [1, 2, 3, 4].to_vec();

            base_partition.set_backup(state_bytes.clone());

            let backup = base_partition.backup();

            assert_eq!(backup.size(), 1);

            assert_eq!(state_bytes, base_partition.get_backup());
        });
    }

    #[test]
    fn test_timer_partition() {
        with_base_partition_mut(|base_partition| {
            let timer = base_partition.timer();

            assert_eq!(timer.len(), 0);

            base_partition
                .push_timer(&TimerEntry {
                    id: 1,
                    time: NanoTimeStamp(2),
                })
                .unwrap();

            let timer = base_partition.timer();

            assert_eq!(timer.len(), 1);

            let timer_entry = timer.peek().unwrap();

            assert_eq!(timer_entry.id, 1);
            assert_eq!(timer_entry.time, 2.into());
        });
    }
}
