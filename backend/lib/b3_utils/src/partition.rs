use b3_stable_structures::memory_manager::{MemoryId, MemoryManager};
use b3_stable_structures::{
    BoundedStorable, DefaultMemoryImpl, StableBTreeMap, StableCell, StableLog, StableMinHeap,
    StableVec, Storable,
};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;

use self::error::PartitionManagerError;

mod test;

pub mod error;
mod main;
mod types;

pub use main::*;
pub use types::*;

thread_local! {
    pub static PARTITION_MANAGER: RefCell<PartitionManager> = RefCell::new(PartitionManager::init())
}

pub struct PartitionManager {
    memory_manager: MemoryManager<DefaultMemoryImpl>,
    partitions: HashMap<String, u8>,
}

impl PartitionManager {
    pub fn init() -> Self {
        let memory_manager = MemoryManager::init(DefaultMemoryImpl::default());
        let partitions = HashMap::new();

        Self {
            memory_manager,
            partitions,
        }
    }

    pub fn create_partition(
        &mut self,
        name: &str,
        id: u8,
    ) -> Result<DefaultVM, PartitionManagerError> {
        let partitions = self.partitions.borrow_mut();

        match partitions.get(name) {
            Some(_) => return Err(PartitionManagerError::PartitionExists),
            None => {
                for (memory_name, memory_id) in partitions.iter() {
                    if *memory_id == id {
                        return Err(PartitionManagerError::IdAlreadyUsed(
                            memory_name.to_string(),
                        ));
                    }
                }

                partitions.insert(name.to_string(), id);
            }
        }

        let memory = self
            .get_memory(name)
            .ok_or(PartitionManagerError::UnableToCreateMemory(
                name.to_string(),
            ))?;

        Ok(memory)
    }

    pub fn get_partition(&self, name: &str) -> Option<u8> {
        self.partitions.get(name).copied()
    }

    pub fn partitions(&self) -> &HashMap<String, u8> {
        &self.partitions
    }

    pub fn get_memory(&self, name: &str) -> Option<DefaultVM> {
        let memory_id = self.partitions.get(name)?;

        let vm = self.memory_manager.get(MemoryId::new(*memory_id));

        Some(vm)
    }

    pub fn memory_manager(&self) -> &MemoryManager<DefaultMemoryImpl> {
        &self.memory_manager
    }

    pub fn init_stable_vec<T: Storable + BoundedStorable>(
        &mut self,
        name: &str,
        id: u8,
    ) -> Result<StableVec<T, DefaultVM>, PartitionManagerError> {
        let memory = self.create_partition(name, id)?;

        StableVec::init(memory)
            .map_err(|e| PartitionManagerError::UnableToCreateMemory(e.to_string()))
    }

    pub fn init_stable_map<
        K: Ord + Storable + BoundedStorable + Clone,
        V: Storable + BoundedStorable,
    >(
        &mut self,
        name: &str,
        id: u8,
    ) -> Result<StableBTreeMap<K, V, DefaultVM>, PartitionManagerError> {
        let memory = self.create_partition(name, id)?;

        let map = StableBTreeMap::init(memory);

        Ok(map)
    }

    pub fn init_stable_heap<T: Ord + Storable + BoundedStorable>(
        &mut self,
        name: &str,
        id: u8,
    ) -> Result<StableMinHeap<T, DefaultVM>, PartitionManagerError> {
        let memory = self.create_partition(name, id)?;

        StableMinHeap::init(memory)
            .map_err(|e| PartitionManagerError::UnableToCreateMemory(e.to_string()))
    }

    pub fn init_stable_log<T: Storable>(
        &mut self,
        name: &str,
        index_id: u8,
        data_id: u8,
    ) -> Result<StableLog<T, DefaultVM, DefaultVM>, PartitionManagerError> {
        let index_memory = self.create_partition(&format!("{}_index", name), index_id)?;
        let data_memory = self.create_partition(&format!("{}_data", name), data_id)?;

        StableLog::init(index_memory, data_memory)
            .map_err(|e| PartitionManagerError::UnableToCreateMemory(e.to_string()))
    }

    pub fn init_stable_cell<T: Storable + Default>(
        &mut self,
        name: &str,
        id: u8,
    ) -> Result<StableCell<T, DefaultVM>, PartitionManagerError> {
        let memory = self.create_partition(name, id)?;

        StableCell::init(memory, T::default())
            .map_err(|e| PartitionManagerError::UnableToCreateMemory(e.to_string()))
    }
}
