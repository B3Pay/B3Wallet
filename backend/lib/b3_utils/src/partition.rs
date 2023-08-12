use b3_stable_structures::memory_manager::{MemoryId, MemoryManager};
use b3_stable_structures::{
    BoundedStorable, DefaultMemoryImpl, StableBTreeMap, StableCell, StableLog, StableVec, Storable,
};
use std::borrow::BorrowMut;
use std::collections::HashMap;

mod test;

pub mod error;
use error::PartitionManagerError;

mod main;
pub use main::*;

mod store;
pub use store::*;

use self::types::{
    DefaultVM, DefaultVMCell, DefaultVMHeap, DefaultVMLog, DefaultVMMap, DefaultVMVec,
};

pub mod types;

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
    ) -> Result<DefaultVMVec<T>, PartitionManagerError> {
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
    ) -> Result<DefaultVMMap<K, V>, PartitionManagerError> {
        let memory = self.create_partition(name, id)?;

        let map = StableBTreeMap::init(memory);

        Ok(map)
    }

    pub fn init_stable_heap<T: Ord + Storable + BoundedStorable>(
        &mut self,
        name: &str,
        id: u8,
    ) -> Result<DefaultVMHeap<T>, PartitionManagerError> {
        let memory = self.create_partition(name, id)?;

        DefaultVMHeap::init(memory)
            .map_err(|e| PartitionManagerError::UnableToCreateMemory(e.to_string()))
    }

    pub fn init_stable_log<T: Storable>(
        &mut self,
        name: &str,
        index_id: u8,
        data_id: u8,
    ) -> Result<DefaultVMLog<T>, PartitionManagerError> {
        let index_memory = self.create_partition(&format!("{}_index", name), index_id)?;
        let data_memory = self.create_partition(&format!("{}_data", name), data_id)?;

        StableLog::init(index_memory, data_memory)
            .map_err(|e| PartitionManagerError::UnableToCreateMemory(e.to_string()))
    }

    pub fn init_stable_cell<T: Storable + Default>(
        &mut self,
        name: &str,
        id: u8,
    ) -> Result<DefaultVMCell<T>, PartitionManagerError> {
        let memory = self.create_partition(name, id)?;

        StableCell::init(memory, T::default())
            .map_err(|e| PartitionManagerError::UnableToCreateMemory(e.to_string()))
    }
}
