use candid::CandidType;

pub use b3_stable_structures::{
    memory_manager::VirtualMemory, BoundedStorable, DefaultMemoryImpl, FileMemory, Memory,
    RestrictedMemory, StableBTreeMap, StableCell, StableLog, StableMinHeap, StableVec, Storable,
    VectorMemory,
};

#[derive(CandidType, Clone, Debug)]
pub struct PartitionDetail {
    pub name: String,
    pub size: u64,
}

pub type DefaultVM = VirtualMemory<DefaultMemoryImpl>;

pub type DefaultVMMap<K, V> = StableBTreeMap<K, V, DefaultVM>;
pub type DefaultVMVec<T> = StableVec<T, DefaultVM>;
pub type DefaultVMLog<T> = StableLog<T, DefaultVM, DefaultVM>;
pub type DefaultVMCell<T> = StableCell<T, DefaultVM>;
pub type DefaultVMHeap<T> = StableMinHeap<T, DefaultVM>;
