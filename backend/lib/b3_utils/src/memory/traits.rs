use super::{
    error::StableMemoryError,
    timer::DefaultTaskTimer,
    types::{DefaultVM, DefaultVMCell, DefaultVMHeap, DefaultVMLog, DefaultVMMap, DefaultVMVec},
};

pub use ic_stable_structures::{
    cell::InitError as ExternalCellInitError, log::InitError as ExternalLogInitError,
    memory_manager::VirtualMemory, DefaultMemoryImpl, FileMemory, Memory, RestrictedMemory,
    StableBTreeMap, StableCell, StableLog, StableMinHeap, StableVec, Storable, VectorMemory,
};

#[rustfmt::skip]
pub enum InitMemoryArg {
    Single(DefaultVM),
    Double(DefaultVM, DefaultVM),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryType {
    Map,
    Vec,
    Log,
    Cell,
    Heap,
    Timer,
}

pub trait InitMemory<T>: Sized {
    fn memory_type() -> MemoryType;
    fn init(arg: InitMemoryArg) -> Result<Self, StableMemoryError>;
}

impl<T: Storable> InitMemory<DefaultVMVec<T>> for DefaultVMVec<T> {
    fn memory_type() -> MemoryType {
        MemoryType::Vec
    }

    fn init(arg: InitMemoryArg) -> Result<Self, StableMemoryError> {
        if let InitMemoryArg::Single(memory) = arg {
            StableVec::init(memory)
                .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}

impl<K: Ord + Storable + Clone, V: Storable> InitMemory<DefaultVMMap<K, V>> for DefaultVMMap<K, V> {
    fn memory_type() -> MemoryType {
        MemoryType::Map
    }

    fn init(arg: InitMemoryArg) -> Result<Self, StableMemoryError> {
        if let InitMemoryArg::Single(memory) = arg {
            Ok(StableBTreeMap::init(memory))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}

impl<T: Storable> InitMemory<DefaultVMLog<T>> for DefaultVMLog<T> {
    fn memory_type() -> MemoryType {
        MemoryType::Log
    }

    fn init(arg: InitMemoryArg) -> Result<Self, StableMemoryError> {
        if let InitMemoryArg::Double(index_memory, data_memory) = arg {
            StableLog::init(index_memory, data_memory)
                .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}

impl<T: Storable + Default> InitMemory<DefaultVMCell<T>> for DefaultVMCell<T> {
    fn memory_type() -> MemoryType {
        MemoryType::Cell
    }

    fn init(arg: InitMemoryArg) -> Result<Self, StableMemoryError> {
        if let InitMemoryArg::Single(memory) = arg {
            StableCell::init(memory, T::default())
                .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}

impl<T: Ord + Storable> InitMemory<DefaultVMHeap<T>> for DefaultVMHeap<T> {
    fn memory_type() -> MemoryType {
        MemoryType::Heap
    }

    fn init(arg: InitMemoryArg) -> Result<Self, StableMemoryError> {
        if let InitMemoryArg::Single(memory) = arg {
            DefaultVMHeap::init(memory)
                .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}

impl<T: Ord + PartialOrd + Storable> InitMemory<DefaultTaskTimer<T>> for DefaultTaskTimer<T> {
    fn memory_type() -> MemoryType {
        MemoryType::Timer
    }

    fn init(arg: InitMemoryArg) -> Result<Self, StableMemoryError> {
        if let InitMemoryArg::Single(memory) = arg {
            DefaultTaskTimer::init(memory)
                .map_err(|e| StableMemoryError::UnableToCreateMemory(e.to_string()))
        } else {
            Err(StableMemoryError::WrongInitializationArgument)
        }
    }
}
