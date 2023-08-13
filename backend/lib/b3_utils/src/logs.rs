use crate::NanoTimeStamp;
use candid::CandidType;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::thread::LocalKey;

pub mod counter;

mod store;
pub use store::*;

mod test;

/// An entry in the canister log.
#[derive(CandidType, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogEntry {
    pub timestamp: NanoTimeStamp,
    // The index of this entry starting from the last canister upgrade.
    pub counter: u64,
    pub message: String,
    pub file: &'static str,
    pub line: u32,
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}:{} {}",
            self.timestamp, self.file, self.line, self.message
        )
    }
}

/// A circular buffer for log messages.
pub struct LogBuffer {
    max_capacity: usize,
    entries: VecDeque<LogEntry>,
}

impl LogBuffer {
    /// Creates a new buffer of the specified max capacity.
    pub fn with_capacity(max_capacity: usize) -> Self {
        Self {
            max_capacity,
            entries: VecDeque::with_capacity(max_capacity),
        }
    }

    /// Changes the max capacity of the buffer.
    /// If the new capacity is smaller than the current capacity, older entries
    /// are evicted.
    /// If the new capacity is larger than the current capacity, the buffer is
    /// not resized.
    pub fn set_capacity(&mut self, new_capacity: usize) {
        while self.entries.len() > new_capacity {
            self.entries.pop_front();
        }
        self.max_capacity = new_capacity;
    }

    /// Adds a new entry to the buffer, potentially evicting older entries.
    pub fn append(&mut self, entry: LogEntry) {
        if self.entries.len() >= self.max_capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    /// Returns an iterator over entries in the order of their insertion.
    pub fn iter(&self) -> impl Iterator<Item = &LogEntry> {
        self.entries.iter()
    }

    /// Returns the number of entries in the buffer.
    /// This is not the same as the max capacity.
    /// The number of entries is always less than or equal to the max capacity.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the buffer is empty.
    /// This is the same as `self.len() == 0`.
    /// This is not the same as the max capacity.
    /// The buffer is empty if and only if the number of entries is zero.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the max capacity of the buffer.
    /// This is the same as the capacity passed to `LogBuffer::with_capacity`.
    /// This is not the same as the number of entries.
    /// The max capacity is the maximum number of entries that can be stored in
    /// the buffer.
    pub fn max_capacity(&self) -> usize {
        self.max_capacity
    }

    /// Returns the first iterator for which p returns false (or past the end
    /// iterator if they all return false).
    ///
    /// Warning: Entries MUST be partitioned by p. That is, p returns true for
    /// all elements in the "front" of the entries list, and false for all
    /// elements in the "back". Otherwise, behavior is not defined. This is
    /// because binary search is used.
    ///
    /// For example,
    ///
    ///   log_buffer.skip_old_entries(|log_entry| log_entry.timestamp <= T)
    ///
    /// In practice, p only uses the timestamp field, because you can partition
    /// on that (since entries are in chronological order, assuming the clock is
    /// monotonic, and the IC, it is).
    ///
    /// If you want an iterator to the first iterator that returns true, but p
    /// does not partition, do this instead:
    ///
    ///    log_buffer.iter().skip_while(opposite_of_p)
    pub fn entries_partition_point<P>(&self, p: P) -> impl Iterator<Item = &LogEntry>
    where
        P: Fn(&LogEntry) -> bool,
    {
        let head_len = self.entries.partition_point(p);
        self.iter().skip(head_len)
    }
}

pub type GlobalBuffer = LocalKey<RefCell<LogBuffer>>;

#[derive(Clone)]
pub struct DevNull;

pub trait Sink {
    fn append(&self, entry: LogEntry);
}

impl Sink for DevNull {
    fn append(&self, _: LogEntry) {}
}

impl Sink for &'static GlobalBuffer {
    fn append(&self, entry: LogEntry) {
        self.with(|cell| cell.borrow_mut().append(entry))
    }
}
