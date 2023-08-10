use b3_stable_structures::Memory;

use self::backup::MainBackupType;
use self::timer::MainTimerType;

use super::types::PartitionDetail;
use super::PartitionManager;

mod backup;

mod test;
mod timer;

pub struct MainPartition {
    backup: MainBackupType,
    timer: MainTimerType,
}

impl MainPartition {
    /// Initializes the core partition.
    /// The core partition is composed of 3 sub-partitions:
    /// - __backup 0
    /// - __timer 3
    ///
    /// The backup partition is used to store the backup state of the canister.
    /// The events_data and events_index partitions are used to store the events of the canister.
    ///
    /// The backup partition is initialized with the value "[PartitionManager]: initialized!".
    pub fn init(partition_manager: &mut PartitionManager) -> Self {
        let backup = partition_manager.create_partition("__backup", 0).unwrap();

        let timer = partition_manager.init_stable_heap("__timer", 3).unwrap();

        Self { backup, timer }
    }

    pub fn partitions_details(&self) -> Vec<PartitionDetail> {
        vec![self.backup_details(), self.timer_details()]
    }

    pub fn backup_details(&self) -> PartitionDetail {
        PartitionDetail {
            name: "__backup".to_string(),
            size: self.backup.size(),
        }
    }

    pub fn timer_details(&self) -> PartitionDetail {
        PartitionDetail {
            name: "__timer".to_string(),
            size: self.timer.len(),
        }
    }
}
