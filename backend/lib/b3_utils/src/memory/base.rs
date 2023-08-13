use b3_stable_structures::Memory;

use super::types::PartitionDetail;
use super::StableMemory;

mod test;

pub mod backup;
pub mod timer;

mod store;
pub use store::*;

use backup::MainBackupType;
use timer::MainTimerType;

pub struct BasePartition {
    backup: MainBackupType,
    timer: MainTimerType,
}

impl BasePartition {
    /// Initializes the core partition.
    /// The core partition is composed of 3 sub-partitions:
    /// - __backup 0
    /// - __timer 1
    ///
    /// The backup partition is used to store the backup state of the canister.
    /// The events_data and events_index partitions are used to store the events of the canister.
    ///
    /// The backup partition is initialized with the value "[PartitionManager]: initialized!".
    pub fn init(partition_manager: &mut StableMemory) -> Self {
        let backup = partition_manager.create("__backup", 0).unwrap();

        let timer = partition_manager.init_min_heap("__timer", 1).unwrap();

        Self { backup, timer }
    }

    pub fn details(&self) -> Vec<PartitionDetail> {
        vec![self.backup_details(), self.timer_details()]
    }

    pub fn backup_details(&self) -> PartitionDetail {
        PartitionDetail {
            name: "__backup".to_string(),
            len: self.backup.size(),
        }
    }

    pub fn timer_details(&self) -> PartitionDetail {
        PartitionDetail {
            name: "__timer".to_string(),
            len: self.timer.len(),
        }
    }
}
