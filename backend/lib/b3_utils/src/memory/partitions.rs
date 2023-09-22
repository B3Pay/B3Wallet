mod name;

use ic_stable_structures::btreemap::Iter;
pub use name::*;

use super::types::{DefaultVM, DefaultVMMap};

pub struct Partitions(DefaultVMMap<PartitionName, u8>);

impl Partitions {
    pub fn init(default_vm: DefaultVM) -> Self {
        Self(DefaultVMMap::init(default_vm))
    }

    pub fn get(&self, name: &PartitionName) -> Option<u8> {
        self.0.get(name)
    }

    pub fn insert(&mut self, name: PartitionName, id: u8) -> Option<u8> {
        self.0.insert(name, id)
    }

    pub fn iter(&self) -> Iter<PartitionName, u8, DefaultVM> {
        self.0.iter()
    }

    pub fn len(&self) -> u64 {
        self.0.len()
    }
}
