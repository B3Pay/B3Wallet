use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{DefaultMemoryImpl, Memory};

mod test;

pub mod error;
use error::StableMemoryError;

pub mod backup;
pub mod timer;

mod store;
pub use store::*;

pub mod partitions;
pub mod traits;
pub mod types;

use types::{DefaultVM, DefaultVMHeap};

use self::backup::BackupPartition;
use self::partitions::{PartitionName, Partitions};
use self::traits::{InitMemory, InitMemoryArg, MemoryType};
use self::types::PartitionDetail;

pub struct StableMemoryManager {
    memory_manager: MemoryManager<DefaultMemoryImpl>,
    backup: BackupPartition,
    partitions: Partitions,
}

impl StableMemoryManager {
    pub fn init() -> Self {
        let memory_manager = MemoryManager::init(DefaultMemoryImpl::default());
        let partitions_vm = memory_manager.get(MemoryId::new(254));
        let partitions = Partitions::init(partitions_vm);

        let backup_vm = memory_manager.get(MemoryId::new(0));
        let backup = BackupPartition::init(backup_vm);

        Self {
            memory_manager,
            partitions,
            backup,
        }
    }

    fn check_partition(&self, name: &PartitionName, id: u8) -> Result<(), StableMemoryError> {
        match self.partitions.get(&name) {
            Some(existing_id) if existing_id != id => {
                return Err(StableMemoryError::IdAlreadyUsed(name.to_string()));
            }
            Some(_) | None => {}
        }

        for (memory_name, memory_id) in self.partitions.iter() {
            if memory_id == id && memory_name != name.to_owned() {
                return Err(StableMemoryError::IdAlreadyUsed(memory_name.to_string()));
            }
        }

        Ok(())
    }

    pub fn create(&mut self, name: &str, id: u8) -> Result<DefaultVM, StableMemoryError> {
        let name = PartitionName::from(name);

        self.check_partition(&name, id)?;

        self.partitions.insert(name.clone(), id);

        let memory = self
            .memory(&name.to_string())
            .ok_or(StableMemoryError::UnableToCreateMemory(name.to_string()))?;

        Ok(memory)
    }

    pub fn get(&self, id: u8) -> DefaultVM {
        self.memory_manager.get(MemoryId::new(id))
    }

    pub fn backup(&self) -> &BackupPartition {
        &self.backup
    }

    pub fn backup_mut(&mut self) -> &mut BackupPartition {
        &mut self.backup
    }

    pub fn partition_details(&self) -> Vec<PartitionDetail> {
        self.partitions
            .iter()
            .map(|(name, id)| PartitionDetail {
                id,
                name: name.to_string(),
                size: self.get(id).size(),
            })
            .collect()
    }

    pub fn partition(&self, name: &str) -> Option<u8> {
        self.partitions.get(&name.into())
    }

    pub fn partitions(&self) -> &Partitions {
        &self.partitions
    }

    pub fn memory(&self, name: &str) -> Option<DefaultVM> {
        let memory_id = self.partitions.get(&name.into())?;

        let vm = self.memory_manager.get(MemoryId::new(memory_id));

        Some(vm)
    }

    pub fn memory_manager(&self) -> &MemoryManager<DefaultMemoryImpl> {
        &self.memory_manager
    }

    pub fn init_memory<T: InitMemory<T>>(
        &mut self,
        name: &str,
        id: u8,
    ) -> Result<T, StableMemoryError> {
        let init_arg = match T::memory_type() {
            MemoryType::Vec => {
                let memory = self.create(name, id)?;

                InitMemoryArg::Single(memory)
            }
            MemoryType::Map => {
                let memory = self.create(name, id)?;

                InitMemoryArg::Single(memory)
            }
            MemoryType::Log => {
                let index_memory = self.create(&format!("{}_index", name), id)?;
                let data_memory = self.create(&format!("{}_data", name), id + 1)?;

                InitMemoryArg::Double(index_memory, data_memory)
            }
            MemoryType::Cell => {
                let memory = self.create(name, id)?;

                InitMemoryArg::Single(memory)
            }
            MemoryType::Heap => {
                let memory = self.create(name, id)?;

                InitMemoryArg::Single(memory)
            }
            MemoryType::Timer => {
                let memory = self.create(name, id)?;

                InitMemoryArg::Single(memory)
            }
        };

        T::init(init_arg)
    }
}
